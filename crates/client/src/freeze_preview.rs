//! Legal hit, block and Roman Cancel with paused redraws and single ticks.
//! Capture uses 60 Hz output; paused redraws never call World or Presentation.
use super::*;
use aeon_sim::Chord;

#[derive(Clone, Copy, Debug)]
enum Kind { Hit, Block, Rc }

fn cases() -> Vec<(Case, Kind)> {
    let mut result = Vec::new();
    for body in [CharacterId::Kogan, CharacterId::Raya] {
        for kind in [Kind::Hit, Kind::Block, Kind::Rc] {
            let response = if matches!(kind, Kind::Block) { Response::StandBlock } else { Response::Hit };
            for case in normal_cases(body, &[MoveId::StP]).into_iter().filter(|c| c.response == response) {
                result.push((case, kind));
            }
        }
    }
    result
}

fn inputs(case: Case, kind: Kind, tick: u32, world: &World) -> [InputFrame; 2] {
    let mut result = case.inputs_for_world(tick, world);
    if matches!(kind, Kind::Rc) && tick == PRESS + 2 {
        result[0] = InputFrame::chord(Chord::RomanCancel);
    }
    result
}

pub(super) async fn run(assets: &Assets, args: &[String]) {
    let capture = args.iter().any(|a| a == "--capture");
    assert!(capture, "--kit-freeze requires --capture for its fixed 60 Hz redraw schedule");
    let selected = args.iter().find_map(|a| a.strip_prefix("--kit-case="))
        .map(|s| s.parse::<usize>().expect("--kit-case number"));
    assert!(selected.is_none_or(|n| n < 24));
    let mut trace = if capture {
        std::fs::create_dir_all("shots/freeze").unwrap();
        Some(std::fs::File::create("shots/freeze/trace.tsv").unwrap())
    } else { None };
    if let Some(file) = &mut trace {
        writeln!(file, "output\tcase\ttick\tworld\tpaused\thash\thitstop\trc\tevents").unwrap();
    }
    let mut output = 0;
    for (index, (case, kind)) in cases().into_iter().enumerate() {
        if selected.is_some_and(|n| n != index) { continue; }
        let mut world = case.world();
        if matches!(kind, Kind::Rc) { world.fighters[0].meter = 1000; }
        let mut pres = Presentation::default();
        let mut tick = 0;
        let mut pause_left = 0;
        let mut pause_count = 0;
        let mut last_image: Option<Vec<u8>> = None;
        // Sixty actual ticks plus two twelve-render-frame pauses.
        while tick < 60 || pause_left > 0 {
            if is_key_pressed(KeyCode::Escape) || is_quit_requested() { return; }
            let paused = pause_left > 0;
            if paused {
                pause_left -= 1;
            } else {
                let [a, b] = inputs(case, kind, tick, &world);
                world.tick(a, b);
                tick += 1;
                pres.after_tick(assets, &world);
                if pause_count < 2 && (world.hitstop > 0 || world.rc_freeze > 0) {
                    // Hold contact, single-step one frozen tick, then hold again.
                    pause_left = 12;
                    pause_count += 1;
                }
            }
            let mut view = View::fit();
            view.follow(&world);
            assets.stage.draw(&view, world.frame);
            pres.draw(&view, assets, &world, false);
            draw_hud(&view, &world, &HudOpts { wins: None, round: None });
            view.text_center(&format!("FREEZE {index} {kind:?} · {}", case.label()), VW / 2.0, 660.0, 20.0, LINEN);
            // This stable label permits byte-exact paused-redraw comparison.
            view.text_center(&format!("tick {tick}/60 · world {} · pause / single-step review", world.frame), VW / 2.0, 696.0, 17.0, INK);
            if capture {
                let image = get_screen_data();
                if paused { assert_eq!(last_image.as_ref().unwrap(), &image.bytes, "paused redraw changed case {index}"); }
                last_image = Some(image.bytes.clone());
                image.export_png(&format!("shots/freeze/{output:04}.png"));
                writeln!(trace.as_mut().unwrap(), "{output}\t{index}\t{tick}\t{}\t{paused}\t{:016x}\t{}\t{}\t{:?}",
                    world.frame, world.state_hash(), world.hitstop, world.rc_freeze, world.events).unwrap();
            }
            output += 1;
            next_frame().await;
        }
        assert_eq!(pause_count, 2, "fixture must reach actual freeze: {index} {kind:?}");
        pres.reset();
        let fresh = case.world();
        assert_eq!(pres.history.ground_context(&fresh, 0).age, 0);
        eprintln!("[aeon] freeze {index} {kind:?}: 60 ticks, 24 exact paused redraws, reset");
    }
    eprintln!("[aeon] freeze complete: {output} frames");
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::EventKind;
    #[test]
    fn freeze_review_reaches_legal_hit_block_and_rc_in_every_case() {
        assert_eq!(cases().len(), 24);
        for (case, kind) in cases() {
            let mut w = case.world();
            if matches!(kind, Kind::Rc) { w.fighters[0].meter = 1000; }
            let mut seen = false;
            for tick in 0..60 {
                let [a, b] = inputs(case, kind, tick, &w);
                w.tick(a, b);
                seen |= w.events.iter().any(|e| e.kind == match kind {
                    Kind::Hit => EventKind::Hit, Kind::Block => EventKind::Block, Kind::Rc => EventKind::RomanCancel,
                });
            }
            assert!(seen, "{case:?} {kind:?}");
        }
    }
}
