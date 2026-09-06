//! Legal anti-air and uppercut/RC/air-normal exchanges; no forced airborne state.
use super::*;
use aeon_sim::{Connect, RC_FREEZE_FRAMES};

const DURATION: u32 = 150;

#[derive(Clone, Copy, Debug)]
enum Route { Anti { hop: bool, falling: bool }, Juggle { rc_at: u16 } }

#[derive(Clone, Copy, Debug)]
struct AirCase { setup: Case, route: Route }

#[derive(Default)]
struct Driver { canceled: Option<u32>, fired: bool }

fn cases(body: CharacterId) -> Vec<AirCase> {
    let mut result = Vec::new();
    if body == CharacterId::Kogan {
        for hop in [true, false] {
            for falling in [false, true] {
                result.extend(normal_cases(body, &[MoveId::CrHS]).into_iter()
                    .filter(|c| c.response == Response::Hit)
                    .map(|setup| AirCase { setup, route: Route::Anti { hop, falling } }));
            }
        }
    }
    for setup in normal_cases(body, &[MoveId::JP, MoveId::JK, MoveId::JS, MoveId::JHS, MoveId::JFL, MoveId::JST])
        .into_iter().filter(|c| c.response == Response::Hit) {
        let rc_at = match (body, setup.move_id) {
            (CharacterId::Kogan, MoveId::JHS | MoveId::JST) => 6,
            (CharacterId::Raya, MoveId::JHS) => 8,
            (CharacterId::Raya, MoveId::JST) => 7,
            _ => 4,
        };
        result.push(AirCase { setup, route: Route::Juggle { rc_at } });
    }
    result
}

impl AirCase {
    fn label(self) -> String {
        let route = match self.route {
            Route::Anti { hop, falling } => format!("anti-air {} {}", if hop { "hop" } else { "jump" }, if falling { "fall" } else { "rise" }),
            Route::Juggle { .. } => "uppercut RC juggle".into(),
        };
        format!("{} {:?} · {route} · {} · {}", self.setup.body.name(), self.setup.move_id,
            if self.setup.right { "right" } else { "left" }, if self.setup.corner { "corner" } else { "center" })
    }

    fn world(self) -> World {
        let mut w = self.setup.world();
        if matches!(self.route, Route::Juggle { .. }) {
            w.fighters[0].pos.x = w.fighters[1].pos.x - px(35) * if self.setup.right { 1 } else { -1 };
            w.fighters[0].meter = 1000;
            if self.setup.body == CharacterId::Kogan && self.setup.move_id == MoveId::JFL { w.fighters[0].gauge = 0; }
        }
        w
    }

    fn inputs(self, tick: u32, w: &World, driver: &mut Driver) -> [InputFrame; 2] {
        match self.route {
            Route::Anti { hop, falling } => {
                let offset = match (hop, falling) { (true, false) => 1, (true, true) => 9, (false, false) => 3, (false, true) => 20 };
                let a = if tick == PRESS + offset { InputFrame::dir_press(2, Btn::HS) } else { InputFrame::dir(2) };
                let b = InputFrame::dir(if (PRESS..PRESS + if hop { 1 } else { 9 }).contains(&tick) { 8 } else { 5 });
                [a, b]
            }
            Route::Juggle { rc_at } => {
                let f = &w.fighters[0];
                let mut a = match tick { 12 => InputFrame::dir(6), 13 => InputFrame::dir(2), 14 => InputFrame::dir_press(3, Btn::S), _ => InputFrame::default() };
                if driver.canceled.is_none() && w.hitstop == 0
                    && f.action.attacking().is_some_and(|(id, frame, connect)| id == MoveId::Uppercut && frame >= rc_at && connect == Connect::Hit) {
                    a = InputFrame { dir: 5, buttons: Buttons { s: true, fl: true, ..Buttons::NONE } };
                    driver.canceled = Some(tick);
                } else if driver.canceled.is_some_and(|t| tick > t + u32::from(RC_FREEZE_FRAMES)) && !driver.fired {
                    let button = match self.setup.move_id { MoveId::JP => Btn::P, MoveId::JK => Btn::K,
                        MoveId::JS => Btn::S, MoveId::JHS => Btn::HS, MoveId::JFL => Btn::FL, _ => Btn::ST };
                    a = InputFrame::press(button); driver.fired = true;
                }
                [a, InputFrame::default()]
            }
        }
    }
}

pub async fn run(assets: &Assets, args: &[String]) {
    let body = if args.iter().any(|s| s == "--kit-raya") { CharacterId::Raya } else { CharacterId::Kogan };
    let capture = args.iter().any(|s| s == "--capture");
    let selected = args.iter().find_map(|s| s.strip_prefix("--kit-case=")).map(|n| n.parse::<usize>().expect("case index"));
    let mut all = cases(body);
    if let Some(name) = args.iter().find_map(|s| s.strip_prefix("--kit-move=")) {
        all.retain(|c| format!("{:?}", c.setup.move_id) == name);
        assert!(!all.is_empty(), "unknown air-exchange move");
    }
    assert!(selected.is_none_or(|n| n < all.len()), "case out of range");
    let mut trace = if capture {
        std::fs::create_dir_all("shots/kit").unwrap();
        let mut file = std::fs::File::create("shots/kit/trace.txt").unwrap();
        writeln!(file, "case\ttick\thash\tp1\tp2\thitstop\trc_freeze\tevents").unwrap();
        let manifest = all.iter().enumerate().filter(|(i, _)| selected.is_none_or(|n| n == *i))
            .map(|(i, c)| format!("{i}\t{}\n", c.label())).collect::<String>();
        std::fs::write("shots/kit/cases.tsv", manifest).unwrap();
        Some(file)
    } else { None };
    let mut clock = FixedClock::default();
    let mut paused = false;
    let mut output_frame = 0;
    for (index, case) in all.into_iter().enumerate() {
        if selected.is_some_and(|n| n != index) { continue; }
        let mut w = case.world();
        let mut driver = Driver::default();
        let mut pres = Presentation::default();
        let mut frame = 0;
        clock.reset();
        while frame < DURATION {
            if is_key_pressed(KeyCode::Escape) || is_quit_requested() { return; }
            if !capture && is_key_pressed(KeyCode::Space) { paused = !paused; clock.reset(); }
            let step = !capture && is_key_pressed(KeyCode::Period);
            if step { paused = true; clock.reset(); }
            let ticks = if capture || step { 1 } else if paused { 0 } else { clock.advance(get_frame_time() as f64) };
            for _ in 0..ticks.min((DURATION - frame) as usize) {
                let [a, b] = case.inputs(frame, &w, &mut driver); w.tick(a, b); frame += 1;
                pres.after_tick(assets, &w);
                if let Some(file) = &mut trace {
                    writeln!(file, "{index}\t{frame}\t{:016x}\t{:?}\t{:?}\t{}\t{}\t{:?}",
                        w.state_hash(), w.fighters[0].action, w.fighters[1].action, w.hitstop, w.rc_freeze, w.events).unwrap();
                }
            }
            let mut view = View::fit(); view.follow(&w);
            assets.stage.draw(&view, w.frame); pres.draw(&view, assets, &w, false);
            draw_hud(&view, &w, &HudOpts { wins: None, round: None });
            view.text_center(&case.label(), VW / 2.0, 660.0, 22.0, LINEN);
            view.text_center(&format!("KIT REVIEW · case {index} · tick {frame}/{DURATION} · SPACE pause · . step · ESC exit"), VW / 2.0, 696.0, 17.0, INK);
            if capture { get_screen_data().export_png(&format!("shots/kit/{output_frame:04}.png")); output_frame += 1; }
            next_frame().await;
        }
        eprintln!("[aeon] air exchange {index} {}: {:016x}", case.label(), w.state_hash());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::{Action, EventKind};
    #[test]
    fn real_jumps_and_rc_inputs_produce_air_hits_and_scaled_juggles() {
        for body in [CharacterId::Kogan, CharacterId::Raya] {
            let all = cases(body); assert_eq!(all.len(), if body == CharacterId::Kogan { 40 } else { 24 });
            for case in all {
                let mut w = case.world(); let mut driver = Driver::default();
                let mut hits = 0; let mut target_hit = false; let mut canceled = false; let mut legal_landing = false;
                for tick in 0..DURATION {
                    let air = w.fighters[1].airborne;
                    let hit = matches!(w.fighters[1].action, Action::Hit { .. });
                    let velocity = w.fighters[1].vel.y;
                    let [a, b] = case.inputs(tick, &w, &mut driver); w.tick(a, b);
                    canceled |= w.events.iter().any(|e| e.kind == EventKind::RomanCancel);
                    for event in &w.events {
                        if event.attacker == 0 && matches!(event.kind, EventKind::Hit | EventKind::Knockdown) {
                            hits += 1;
                            if event.move_id == Some(case.setup.move_id) {
                                assert!(air, "{case:?}: victim must already be airborne"); target_hit = true;
                                match case.route {
                                    Route::Anti { falling, .. } => assert_eq!(velocity < 0, falling, "{case:?}: chosen rise/fall phase"),
                                    Route::Juggle { .. } => { assert!(hit && canceled, "{case:?}: real continuation"); assert_eq!(w.fighters[1].combo, 2); },
                                }
                            }
                        }
                    }
                    if target_hit && air && !w.fighters[1].airborne {
                        assert!(matches!(w.fighters[1].action, Action::Landing { total: 2, .. } | Action::Hit { stun: 1.., knockdown: false }), "{case:?}: preserve remaining stun or existing two-tick landing");
                        legal_landing = true;
                    }
                }
                assert!(target_hit && legal_landing, "{case:?}: airborne hit and authored landing");
                assert_eq!(hits, if matches!(case.route, Route::Anti { .. }) { 1 } else { 2 }, "{case:?}");
                assert!(w.fighters.iter().all(|f| f.action.actionable() && !f.airborne), "{case:?}: both recover");
            }
        }
    }
    #[test]
    fn kogan_juggles_show_recoil_tuck_and_feet_without_changing_simulation() {
        use crate::{sequences::air_recovery_cell, sprites::Cell};
        for case in cases(CharacterId::Raya) {
            let mut w = case.world(); let mut driver = Driver::default();
            let mut seen = std::collections::HashSet::new();
            for tick in 0..DURATION {
                let [a, b] = case.inputs(tick, &w, &mut driver); w.tick(a, b);
                let hash = w.state_hash(); let f = &w.fighters[1];
                let cell = air_recovery_cell(f);
                if let Some(cell) = cell { seen.insert(cell); }
                if matches!(f.action, Action::Hit { knockdown: true, .. }) || !f.airborne {
                    assert_eq!(cell, None, "knockdowns and grounded actions retain their own drawings");
                }
                assert_eq!(w.state_hash(), hash);
            }
            assert_eq!(seen, [Cell::AirRecovery(0), Cell::AirRecovery(1), Cell::AirRecovery(2)].into_iter().collect(), "{case:?}");
        }
    }

}
