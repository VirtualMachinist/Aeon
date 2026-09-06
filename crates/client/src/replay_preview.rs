//! Capture a legal training input log, save/load the real replay format, and
//! repeat through the same renderer after resetting its complete history.
//! Run in an empty working directory: --replay-review --capture --capture-1x.
use super::*;
use aeon_sim::DummyMode;
use std::io::Write;

const TICKS: u32 = 240;

fn cases() -> [(CharacterId, CharacterId, usize); 4] {
    use CharacterId::{Kogan, Raya};
    [(Kogan, Raya, 0), (Kogan, Raya, 1), (Raya, Kogan, 0), (Raya, Kogan, 1)]
}

fn inputs(tick: u32, attacker: usize) -> [InputFrame; 2] {
    let mut pair = [InputFrame::default(); 2];
    // No position, meter, health or mode changes outside the saved input log.
    if tick < 44 || (98..108).contains(&tick) || (212..230).contains(&tick) {
        pair = [InputFrame::dir(6); 2];
    }
    if tick == 52 || tick == 232 { pair[attacker] = InputFrame::press(Btn::P); }
    if tick == 110 { pair[attacker] = InputFrame::press(Btn::S); }
    if (108..148).contains(&tick) { pair[1-attacker] = InputFrame::dir(4); }
    if (160..169).contains(&tick) { pair[attacker] = InputFrame::dir(8); }
    pair
}

pub(super) async fn run(assets: &Assets) {
    assert!(std::env::args().any(|a| a == "--capture"),
        "--replay-review requires --capture for its fixed 60 Hz output");
    std::fs::create_dir_all("shots/replay-review").unwrap();
    let mut trace = std::fs::File::create("shots/replay-review/trace.tsv").unwrap();
    writeln!(trace, "output\tcase\tpass\ttick\tworld\thash\thitstop\tevents").unwrap();
    let mut output = 0;
    let mut pres = Presentation::default();
    for (case, (p1, p2, attacker)) in cases().into_iter().enumerate() {
        let mut recording = Replay::start(p1, p2, DummyMode::CpuOff);
        let mut loaded: Option<Replay> = None;
        let mut hashes = Vec::new();
        for pass in 0..2 {
            let (a, b, dummy) = loaded.as_ref().map(|rep| (rep.p1.unwrap(), rep.p2.unwrap(), rep.dummy.unwrap()))
                .unwrap_or((p1, p2, DummyMode::CpuOff));
            let mut world = World::training(a, b);
            world.dummy = dummy;
            pres.reset();
            for tick in 0..TICKS {
                if is_key_pressed(KeyCode::Escape) || is_quit_requested() { return; }
                let (a, b) = if let Some(rep) = &loaded {
                    rep.frames[tick as usize]
                } else {
                    let [a, b] = inputs(tick, attacker);
                    recording.push(a, b);
                    (a, b)
                };
                world.tick(a, b);
                pres.after_tick(assets, &world);
                if pass == 0 { hashes.push(world.state_hash()); }
                else { assert_eq!(hashes[tick as usize], world.state_hash(), "case {case} tick {tick}"); }
                let mut view = View::fit();
                view.follow(&world);
                assets.stage.draw(&view, world.frame);
                pres.draw(&view, assets, &world, false);
                draw_hud(&view, &world, &HudOpts { wins: None, round: None });
                // Identical labels allow whole-image recorded/loaded comparison.
                view.text_center(&format!("REPLAY {case} · {} / {} · P{} attacks", p1.name(), p2.name(), attacker+1), VW/2.0, 660.0, 20.0, LINEN);
                view.text_center(&format!("input {}/{} · world {}", tick+1, TICKS, world.frame), VW/2.0, 696.0, 17.0, INK);
                get_screen_data().export_png(&format!("shots/replay-review/{output:04}.png"));
                writeln!(trace, "{output}\t{case}\t{pass}\t{tick}\t{}\t{:016x}\t{}\t{:?}", world.frame, world.state_hash(), world.hitstop, world.events).unwrap();
                output += 1;
                next_frame().await;
            }
            if pass == 0 {
                let path = recording.save().expect("save review replay");
                let replay = Replay::load(&path).expect("load saved review replay");
                assert_eq!(replay.p1, Some(p1));
                assert_eq!(replay.p2, Some(p2));
                assert_eq!(replay.dummy, Some(DummyMode::CpuOff));
                assert_eq!(replay.frames, recording.frames);
                eprintln!("[aeon] replay case {case}: saved {} ({} inputs)", path.display(), replay.frames.len());
                loaded = Some(replay);
            }
        }
        eprintln!("[aeon] replay case {case}: all {TICKS} loaded world hashes match");
    }
    eprintln!("[aeon] replay review complete: {output} frames");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recording_exercises_both_bodies_hit_guard_jump_and_landing() {
        for (p1, p2, attacker) in cases() {
            let mut w = World::training(p1, p2);
            w.dummy = DummyMode::CpuOff;
            let (mut hit, mut block, mut air, mut landed) = (false, false, false, false);
            for tick in 0..TICKS {
                let [a, b] = inputs(tick, attacker);
                w.tick(a, b);
                hit |= w.events.iter().any(|e| e.kind == EventKind::Hit);
                block |= w.events.iter().any(|e| e.kind == EventKind::Block);
                air |= w.fighters[attacker].pos.y > 0;
                landed |= air && w.fighters[attacker].pos.y == 0;
            }
            assert!(hit && block && air && landed, "{p1:?}/{p2:?} P{attacker}: hit={hit} block={block} air={air} landed={landed}");
        }
    }
}
