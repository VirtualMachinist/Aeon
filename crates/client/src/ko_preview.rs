//! Defeated-body continuity through grounded hits, launches, throws and resets.
use super::*;
use aeon_sim::{Match, Phase};
use crate::render::draw_match_overlay;

const DURATION: u32 = 240;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ending { Hold, NextRound, Rematch }

#[derive(Clone, Copy, Debug)]
struct KoCase { setup: Case, ending: Ending }

fn cases(victim: CharacterId) -> Vec<KoCase> {
    reaction_cases(victim).into_iter().filter(|c| matches!(c.response,Response::Hit|Response::CrouchHit)
        && c.move_id != MoveId::StS).map(|setup| {
        let ending=match setup.move_id { MoveId::StP => Ending::NextRound,
            MoveId::CrK => Ending::Rematch, _ => Ending::Hold };
        KoCase { setup,ending }
    }).collect()
}

impl KoCase {
    fn label(self) -> String {
        format!("{} KO vs {:?} · {:?} · attacker {} · {}",
            if self.setup.body==CharacterId::Raya { "KOGAN" } else { "RAYA" },
            self.setup.move_id,self.ending,if self.setup.right { "right" } else { "left" },
            if self.setup.corner { "corner" } else { "center" })
    }

    fn game(self) -> Match {
        let victim=if self.setup.body==CharacterId::Raya { CharacterId::Kogan } else { CharacterId::Raya };
        let mut m = Match::new(self.setup.body, victim);
        m.world = self.setup.world();
        m.world.fighters[1].health = 1;
        m.phase = Phase::Fight;
        m.wins[0] = u8::from(self.ending != Ending::NextRound);
        m
    }

    fn tick(self, frame: u32, m: &mut Match) -> bool {
        if self.ending == Ending::Rematch && frame == 180 {
            assert!(matches!(m.phase, Phase::MatchOver { winner: 0 }));
            m.rematch();
            return true;
        }
        let input = if frame < 100 { self.setup.inputs_for_world(frame, &m.world) }
            else { [InputFrame::default(); 2] };
        m.tick(input[0], input[1]);
        false
    }
}

pub async fn run(assets: &Assets, args: &[String]) {
    let victim=if args.iter().any(|s| s == "--kit-raya") { CharacterId::Raya } else { CharacterId::Kogan };
    let capture = args.iter().any(|s| s == "--capture");
    let selected = args.iter().find_map(|s| s.strip_prefix("--kit-case=")).map(|n| n.parse::<usize>().expect("case index"));
    let mut all = cases(victim);
    if let Some(name) = args.iter().find_map(|s| s.strip_prefix("--kit-move=")) {
        all.retain(|c| format!("{:?}", c.setup.move_id) == name);
        assert!(!all.is_empty(), "unknown KO move");
    }
    assert!(selected.is_none_or(|n| n < all.len()), "case out of range");
    let mut trace = if capture {
        std::fs::create_dir_all("shots/kit").unwrap();
        let mut file = std::fs::File::create("shots/kit/trace.txt").unwrap();
        writeln!(file, "case\ttick\thash\tphase\twins\tp1\tp2\thitstop\tevents").unwrap();
        let manifest = all.iter().enumerate().filter(|(i,_)| selected.is_none_or(|n| n == *i))
            .map(|(i,c)|format!("{i}\t{}\n",c.label())).collect::<String>();
        std::fs::write("shots/kit/cases.tsv",manifest).unwrap();
        Some(file)
    } else { None };
    let mut clock = FixedClock::default();
    let mut paused = false;
    let mut output_frame = 0;
    for (index, case) in all.into_iter().enumerate() {
        if selected.is_some_and(|n| n != index) { continue; }
        let mut m = case.game();
        let mut pres = Presentation::default();
        let mut frame = 0;
        clock.reset();
        while frame < DURATION {
            if is_key_pressed(KeyCode::Escape) || is_quit_requested() { return; }
            if !capture && is_key_pressed(KeyCode::Space) { paused = !paused; clock.reset(); }
            let step = !capture && is_key_pressed(KeyCode::Period);
            if step { paused = true; clock.reset(); }
            let ticks = if capture || step { 1 } else if paused { 0 } else { clock.advance(get_frame_time() as f64) };
            for _ in 0..ticks.min((DURATION-frame) as usize) {
                let prior = m.world.frame;
                if case.tick(frame, &mut m) || m.world.frame < prior { pres.reset(); }
                frame += 1;
                pres.victory.update(&m.world,m.phase);
                pres.defeat.update(&m.world,m.phase);
                pres.after_tick(assets, &m.world);
                if let Some(file) = &mut trace {
                    writeln!(file,"{index}\t{frame}\t{:016x}\t{:?}\t{:?}\t{:?}\t{:?}\t{}\t{:?}",
                        m.world.state_hash(),m.phase,m.wins,m.world.fighters[0].action,m.world.fighters[1].action,m.world.hitstop,m.world.events).unwrap();
                }
            }
            pres.victory.update(&m.world,m.phase);
            pres.defeat.update(&m.world,m.phase);
            let mut view = View::fit();view.follow(&m.world);
            assets.stage.draw(&view,m.world.frame);
            pres.draw(&view,assets,&m.world,false);
            draw_hud(&view,&m.world,&HudOpts { wins:Some(m.wins),round:Some(m.round) });
            draw_match_overlay(&view,&m,m.world.frame);
            view.text_center(&case.label(),VW/2.0,660.0,22.0,LINEN);
            view.text_center(&format!("KIT REVIEW · case {index} · tick {frame}/{DURATION} · SPACE pause · . step · ESC exit"),VW/2.0,696.0,17.0,INK);
            if capture { get_screen_data().export_png(&format!("shots/kit/{output_frame:04}.png"));output_frame+=1; }
            next_frame().await;
        }
        eprintln!("[aeon] KO {index} {}: {:?}",case.label(),m.phase);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::{Action,RoundOutcome};
    #[test]
    fn both_ko_bodies_reach_real_hits_floor_or_collapse_and_reset() {
        for victim in [CharacterId::Kogan,CharacterId::Raya] {
            let all=cases(victim);assert_eq!(all.len(),24);
            for case in all {
                let mut m=case.game();let mut ko=false;let mut air=false;let mut floor=false;let mut over=false;let mut reset=false;
                for frame in 0..DURATION {
                    let before=m.world.frame;
                    reset |= case.tick(frame,&mut m) || m.world.frame < before;
                    ko |= matches!(m.phase,Phase::RoundEnd { outcome:RoundOutcome::Winner(0),.. });
                    if ko && m.world.fighters[1].health==0 {
                        air |= m.world.fighters[1].airborne;
                        floor |= matches!(m.world.fighters[1].action,Action::Knockdown {..});
                    }
                    over |= matches!(m.phase,Phase::MatchOver {winner:0});
                }
                assert!(ko,"{case:?}: actual KO");
                if matches!(case.setup.move_id,MoveId::Uppercut|MoveId::Throw|MoveId::CommandGrab) { assert!(air && floor,"{case:?}: launch-to-floor"); }
                if case.setup.move_id==MoveId::CrST { assert!(floor,"sweep must reach floor"); }
                match case.ending {
                    Ending::Hold => assert!(over && !reset),
                    Ending::NextRound => { assert!(!over && reset);assert_eq!(m.round,2);assert_eq!(m.wins,[1,0]); },
                    Ending::Rematch => { assert!(over && reset);assert_eq!(m.round,1);assert_eq!(m.wins,[0,0]); },
                }
            }
        }
    }
    #[test]
    fn every_ko_stays_down_after_landing_without_changing_simulation() {
        use crate::{defeat::Clock,sprites::Cell};
        for victim in [CharacterId::Kogan,CharacterId::Raya] {
            let down=if victim==CharacterId::Kogan {Cell::Floor(0)} else {Cell::Reaction(4)};
            for case in cases(victim) {
                let mut m=case.game();let mut clock=Clock::default();let mut reached=false;
                for frame in 0..DURATION {
                    case.tick(frame,&mut m);let hash=m.world.state_hash();clock.update(&m.world,m.phase);
                    let cell=clock.cell(&m.world,1);
                    clock.update(&m.world,m.phase);assert_eq!(clock.cell(&m.world,1),cell,"frozen redraw");
                    assert_eq!(clock.cell(&m.world,0),None,"live winner");
                    if m.world.fighters[1].health>0 { assert_eq!(cell,None); }
                    else if cell==Some(down) { reached=true; }
                    else if reached { panic!("{case:?}: defeated body stood up"); }
                    if m.world.fighters[1].airborne { assert_ne!(cell,Some(down)); }
                    assert_eq!(m.world.state_hash(),hash);
                }
                assert!(reached,"{case:?}: must reach supported hold");
            }
        }
    }

}
