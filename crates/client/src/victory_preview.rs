//! Real knockout, winner hold, next-round and rematch review.
use super::*;
use aeon_sim::{Match, Phase};
use crate::render::draw_match_overlay;

const DURATION: u32 = 240;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ending { Standing, Air, NextRound, Rematch }

#[derive(Clone, Copy, Debug)]
struct VictoryCase { setup: Case, ending: Ending }

fn cases(body: CharacterId) -> Vec<VictoryCase> {
    let mut out = Vec::new();
    for ending in [Ending::Standing, Ending::Air, Ending::NextRound, Ending::Rematch] {
        let (base, mv) = match ending {
            Ending::Standing => (saber_cases(body), MoveId::StS),
            Ending::Air => (saber_cases(body), MoveId::Uppercut),
            Ending::NextRound => (crouching_saber_cases(body), MoveId::CrHS),
            Ending::Rematch => (ranged_cases(body), MoveId::ShotA),
        };
        for setup in base.into_iter().filter(|c| c.move_id == mv && c.response == Response::Hit) {
            out.push(VictoryCase { setup, ending });
        }
    }
    out
}

impl VictoryCase {
    fn label(self) -> String {
        format!("{} victory {:?} · {} · {}", if self.setup.body == CharacterId::Kogan { "KOGAN" } else { "RAYA" }, self.ending,
            if self.setup.right { "right" } else { "left" },
            if self.setup.corner { "corner" } else { "center" })
    }

    fn game(self) -> Match {
        let opponent = if self.setup.body == CharacterId::Kogan { CharacterId::Raya } else { CharacterId::Kogan };
        let mut m = Match::new(self.setup.body, opponent);
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
    let body = if args.iter().any(|s| s == "--kit-raya") { CharacterId::Raya } else { CharacterId::Kogan };
    let capture = args.iter().any(|s| s == "--capture");
    let selected = args.iter().find_map(|s| s.strip_prefix("--kit-case=")).map(|n| n.parse::<usize>().expect("case index"));
    let mut all = cases(body);
    if let Some(name) = args.iter().find_map(|s| s.strip_prefix("--kit-victory-state=")) {
        all.retain(|c| format!("{:?}", c.ending) == name);
        assert!(!all.is_empty(), "unknown victory state");
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
        eprintln!("[aeon] victory {index} {}: {:?}",case.label(),m.phase);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::{Action, RoundOutcome};
    #[test]
    fn victory_cases_reach_real_ko_then_hold_next_round_or_rematch() {
        let all = [CharacterId::Kogan,CharacterId::Raya].into_iter().flat_map(cases).collect::<Vec<_>>();assert_eq!(all.len(),32);
        for case in all {
            let mut m=case.game();let mut ko=false;let mut airborne=false;let mut resting=false;let mut over=false;let mut reset=false;
            for frame in 0..DURATION {
                let before=m.world.frame;
                reset |= case.tick(frame,&mut m) || m.world.frame < before;
                ko |= matches!(m.phase,Phase::RoundEnd { outcome:RoundOutcome::Winner(0),.. });
                airborne |= ko && m.world.fighters[0].airborne;
                resting |= matches!(m.phase,Phase::RoundEnd { frame:30..,.. }) && !m.world.fighters[0].airborne
                    && matches!(m.world.fighters[0].action,Action::Stand|Action::Crouch|Action::Walk { .. });
                over |= matches!(m.phase,Phase::MatchOver { winner:0 });
            }
            assert!(ko && resting,"{case:?}: actual KO and recovered winner");
            if case.ending==Ending::Air { assert!(airborne,"airborne KO must finish landing"); }
            match case.ending {
                Ending::Standing|Ending::Air => assert!(over && !reset),
                Ending::NextRound => { assert!(!over && reset);assert_eq!(m.round,2);assert_eq!(m.wins,[1,0]); },
                Ending::Rematch => { assert!(over && reset);assert_eq!(m.round,1);assert_eq!(m.wins,[0,0]); },
            }
        }
    }
    #[test]
    fn every_real_ko_exposes_all_victory_drawings_then_freezes_or_clears() {
        use crate::{anim::VictoryClock, sequences::victory_cell, sprites::Cell};
        for case in [CharacterId::Kogan,CharacterId::Raya].into_iter().flat_map(cases) {
            let mut m=case.game();let mut clock=VictoryClock::default();
            let mut cells=std::collections::BTreeSet::new();
            for frame in 0..DURATION {
                case.tick(frame,&mut m);
                let hash=m.world.state_hash();
                clock.update(&m.world,m.phase);
                if let Some(age)=clock.age(0) {
                    let Cell::Victory(cell)=victory_cell(&m.world.fighters[0],age).unwrap() else { panic!("winner drawing") };
                    cells.insert(cell);
                    clock.update(&m.world,m.phase);assert_eq!(clock.age(0),Some(age),"a repeated draw cannot advance time");
                }
                assert_eq!(m.world.state_hash(),hash);
                assert_eq!(clock.age(1),None,"loser cannot receive winner art");
                if matches!(m.phase,Phase::Intro { .. }|Phase::Fight) { assert_eq!(clock.age(0),None); }
            }
            assert_eq!(cells,[0,1,2,3].into_iter().collect(),"{case:?}: complete draw/raise/settle/hold");
        }
        let mut m=Match::new(CharacterId::Raya,CharacterId::Kogan);
        let mut clock=VictoryClock::default();
        m.phase=Phase::RoundEnd { outcome:RoundOutcome::Winner(1),frame:30 };
        clock.update(&m.world,m.phase);assert_eq!(clock.age(1),Some(0));assert_eq!(clock.age(0),None);
        assert_eq!(victory_cell(&m.world.fighters[0],0),Some(Cell::Victory(0)),"both bodies have their own loaded winner atlas");
    }

}
