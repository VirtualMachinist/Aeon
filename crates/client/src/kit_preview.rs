//! Isolated full-kit comparisons using legal inputs and unchanged combat data.
//! --kit-preview [--kit-raya] [--kit-case=N] [--capture].
//! Each case covers preparation, contact/whiff and return to an actionable state.
use super::{Assets, Presentation};
use crate::render::{draw_hud, HudOpts, View, INK, LINEN, VW};
use crate::timing::FixedClock;
use aeon_sim::{px, Btn, CharacterId, InputFrame, MoveId, World, STAGE_W};
use macroquad::prelude::*;
use std::io::Write;

const LENGTH: u32 = 60;
const PRESS: u32 = 12;
const MOVES: [MoveId; 3] = [MoveId::StP, MoveId::StK, MoveId::CrK];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Response {
    Hit,
    StandBlock,
    CrouchBlock,
    CrouchHit,
    Whiff,
}

const RESPONSES: [Response; 5] = [
    Response::Hit,
    Response::StandBlock,
    Response::CrouchBlock,
    Response::CrouchHit,
    Response::Whiff,
];

#[derive(Clone, Copy, Debug)]
struct Case {
    body: CharacterId,
    move_id: MoveId,
    response: Response,
    right: bool,
    corner: bool,
    jump: Option<(u8, bool)>,
}

fn cases(body: CharacterId) -> Vec<Case> {
    let mut result = Vec::new();
    for move_id in MOVES {
        for response in RESPONSES {
            for corner in [false, true] {
                for right in [true, false] {
                    result.push(Case { body, move_id, response, right, corner, jump: None });
                }
            }
        }
    }
    result
}

fn movement_cases(body: CharacterId) -> Vec<Case> {
    let mut result = Vec::new();
    for hop in [true, false] {
        for dir in [8, 9, 7] {
            for corner in [false, true] {
                for right in [true, false] {
                    result.push(Case { body, move_id: MoveId::StP,
                        response: Response::Whiff, right, corner, jump: Some((dir, hop)) });
                }
            }
        }
    }
    result
}

impl Case {
    fn label(self) -> String {
        if let Some((dir, hop)) = self.jump {
            return format!("{} {} dir {} · {} · {}", self.body.name(),
                if hop { "hop" } else { "jump" }, dir,
                if self.right { "right" } else { "left" },
                if self.corner { "corner" } else { "center" });
        }
        format!(
            "{} {:?} · {:?} · {} · {}",
            self.body.name(),
            self.move_id,
            self.response,
            if self.right { "right" } else { "left" },
            if self.corner { "corner" } else { "center" },
        )
    }

    fn world(self) -> World {
        let opponent = match self.body {
            CharacterId::Kogan => CharacterId::Raya,
            CharacterId::Raya => CharacterId::Kogan,
        };
        let mut world = World::new(self.body, opponent);
        let gap = if self.response == Response::Whiff { 150 } else { 40 };
        let defender = if self.corner { 740 } else { 340 };
        let attacker = defender - gap;
        let (attacker, defender) = if self.jump.is_some() {
            if self.corner { (660, 740) } else { (260, 500) }
        } else { (attacker, defender) };
        for (fighter, x) in world.fighters.iter_mut().zip([attacker, defender]) {
            fighter.pos.x = if self.right { px(x) } else { STAGE_W - px(x) };
        }
        world
    }

    fn inputs(self, frame: u32) -> [InputFrame; 2] {
        if let Some((dir, hop)) = self.jump {
            let up = frame == PRESS || (!hop && (PRESS..PRESS + 7).contains(&frame));
            return [InputFrame::dir(if up { dir } else { 5 }), InputFrame::default()];
        }
        let crouch = self.move_id == MoveId::CrK;
        let attacker = if frame == PRESS {
            let button = if self.move_id == MoveId::StP { Btn::P } else { Btn::K };
            InputFrame::dir_press(if crouch { 2 } else { 5 }, button)
        } else {
            InputFrame::dir(if crouch { 2 } else { 5 })
        };
        // InputFrame is facing-relative. Back is 4 for either fighter.
        // Delay backwalk until the attack input so a center case stays in reach.
        let defender = InputFrame::dir(match self.response {
            Response::StandBlock if frame >= PRESS => 4,
            Response::CrouchBlock => 1,
            Response::CrouchHit => 2,
            _ => 5,
        });
        [attacker, defender]
    }
}

pub async fn run(assets: &Assets) {
    let args: Vec<String> = std::env::args().collect();
    let capture = args.iter().any(|a| a == "--capture");
    let body = if args.iter().any(|a| a == "--kit-raya") {
        CharacterId::Raya
    } else {
        CharacterId::Kogan
    };
    let selected = args.iter().find_map(|a| a.strip_prefix("--kit-case=")).map(|n| {
        n.parse::<usize>().expect("--kit-case must be a nonnegative integer")
    });
    let all = if args.iter().any(|a| a == "--kit-movement") {
        movement_cases(body)
    } else { cases(body) };
    assert!(selected.is_none_or(|n| n < all.len()), "--kit-case out of range");
    let mut trace = if capture {
        std::fs::create_dir_all("shots/kit").expect("kit preview directory");
        let mut file = std::fs::File::create("shots/kit/trace.txt").expect("kit preview trace");
        writeln!(file, "case\ttick\thash\tp1\tp2\thitstop\tevents").unwrap();
        let manifest = all.iter().enumerate()
            .filter(|(i, _)| selected.is_none_or(|n| n == *i))
            .map(|(i, case)| format!("{i}\t{}\n", case.label())).collect::<String>();
        std::fs::write("shots/kit/cases.tsv", manifest).expect("kit case manifest");
        Some(file)
    } else {
        None
    };
    let mut clock = FixedClock::default();
    let mut output_frame = 0;
    let mut paused = false;
    for (index, case) in all.into_iter().enumerate() {
        if selected.is_some_and(|n| n != index) {
            continue;
        }
        let mut world = case.world();
        let mut pres = Presentation::default();
        let mut frame = 0;
        clock.reset();
        while frame < LENGTH {
            if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
                return;
            }
            if !capture && is_key_pressed(KeyCode::Space) {
                paused = !paused;
                clock.reset();
            }
            let step = !capture && is_key_pressed(KeyCode::Period);
            if step {
                paused = true;
                clock.reset();
            }
            let ticks = if capture || step {
                1
            } else if paused {
                0
            } else {
                clock.advance(get_frame_time() as f64)
            };
            for _ in 0..ticks.min((LENGTH - frame) as usize) {
                let [p1, p2] = case.inputs(frame);
                world.tick(p1, p2);
                frame += 1;
                pres.after_tick(assets, &world);
                if let Some(file) = &mut trace {
                    writeln!(
                        file, "{index}\t{frame}\t{:016x}\t{:?}\t{:?}\t{}\t{:?}",
                        world.state_hash(), world.fighters[0].action,
                        world.fighters[1].action, world.hitstop, world.events,
                    ).unwrap();
                }
            }
            let mut view = View::fit();
            view.follow(&world);
            assets.stage.draw(&view, world.frame);
            pres.draw(&view, assets, &world, false);
            draw_hud(&view, &world, &HudOpts { wins: None, round: None });
            view.text_center(&case.label(), VW / 2.0, 146.0, 22.0, LINEN);
            view.text_center(
                &format!("KIT REVIEW · case {index} · tick {frame}/{LENGTH} · SPACE pause · . step · ESC exit"),
                VW / 2.0, 696.0, 17.0, INK,
            );
            if capture {
                get_screen_data().export_png(&format!("shots/kit/{output_frame:04}.png"));
                output_frame += 1;
            }
            next_frame().await;
        }
        eprintln!("[aeon] kit {index} {}: hash {:016x}, defender health {}",
            case.label(), world.state_hash(), world.fighters[1].health);
    }
    eprintln!("[aeon] kit preview complete: {output_frame} captured frames");
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::{Action, EventKind};

    #[test]
    fn movement_preview_preserves_hop_and_jump_landings() {
        for body in [CharacterId::Kogan, CharacterId::Raya] {
            for case in movement_cases(body) {
                let mut world = case.world();
                let mut rising = false;
                let mut falling = false;
                let mut landed = false;
                let mut landing_ticks = 0;
                let mut drawings = std::collections::HashSet::new();
                for frame in 0..LENGTH {
                    let [p1, p2] = case.inputs(frame);
                    world.tick(p1, p2);
                    let f = &world.fighters[0];
                    if let Action::Jump { hop, .. } = f.action {
                        assert_eq!(hop, case.jump.unwrap().1, "{case:?}");
                        rising |= f.vel.y > 0;
                        falling |= f.vel.y < 0;
                    }
                    if let Action::Landing { total, .. } = f.action {
                        assert_eq!(total, 2, "{case:?}");
                        landing_ticks += 1;
                    }
                    landed |= falling && !f.airborne && f.action.actionable();
                    if let Some(cell) = crate::sequences::movement_cell(f) {
                        drawings.insert(cell);
                    }
                }
                assert!(rising && falling && landed, "{case:?} complete arc");
                assert_eq!(landing_ticks, if case.jump.unwrap().1 { 0 } else { 2 }, "{case:?}");
                if body == CharacterId::Kogan {
                    let base = if case.jump.unwrap().1 { 1 } else { 4 };
                    for cell in [0, base, base + 1, base + 2] {
                        assert!(drawings.contains(&crate::sprites::Cell::Movement(cell)), "{case:?} cell {cell}");
                    }
                }
            }
        }
    }

    #[test]
    fn lights_preview_exercises_legal_moves_and_guard_outcomes() {
        for body in [CharacterId::Kogan, CharacterId::Raya] {
            for case in cases(body) {
                let mut world = case.world();
                let mut started = false;
                let mut hits = 0;
                let mut blocks = 0;
                let mut crouched_hit = false;
                for frame in 0..LENGTH {
                    let [p1, p2] = case.inputs(frame);
                    world.tick(p1, p2);
                    started |= world.fighters[0].action.attacking()
                        .is_some_and(|(id, _, _)| id == case.move_id);
                    for event in &world.events {
                        hits += usize::from(event.kind == EventKind::Hit);
                        blocks += usize::from(event.kind == EventKind::Block);
                    }
                    crouched_hit |= matches!(world.fighters[1].action, Action::Hit { .. })
                        && world.fighters[1].input().down();
                }
                assert!(started, "{case:?} must start through real inputs");
                assert!(world.fighters[0].action.actionable(), "{case:?} must recover");
                // Standing jabs pass above a crouched hurtbox. A standing guard
                // does not stop the authored low crouching kick.
                let ducked = case.move_id == MoveId::StP
                    && matches!(case.response, Response::CrouchBlock | Response::CrouchHit);
                let expected = if case.response == Response::Whiff || ducked {
                    (0, 0)
                } else if case.response == Response::CrouchBlock
                    || (case.response == Response::StandBlock && case.move_id != MoveId::CrK) {
                    (0, 1)
                } else {
                    (1, 0)
                };
                assert_eq!((hits, blocks), expected, "{case:?}");
                if case.response == Response::CrouchHit && case.move_id == MoveId::CrK {
                    assert!(crouched_hit, "{case:?} dedicated crouched-hit exchange");
                }
            }
        }
    }
}
