//! Isolated full-kit comparisons using legal inputs and unchanged combat data.
//! --kit-preview [--kit-raya] [--kit-case=N] [--capture].
//! Each case covers preparation, contact/whiff and return to an actionable state.
use super::{Assets, Presentation};
use crate::render::{draw_hud, HudOpts, View, INK, LINEN, VW};
use crate::timing::FixedClock;
use aeon_sim::{px, Btn, Buttons, CharacterId, InputFrame, MoveId, World, STAGE_W};
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
    AirEvade,
    Projectile,
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
    ranged: bool,
    utility: bool,
    saber: bool,
    disc: bool,
}

fn cases(body: CharacterId) -> Vec<Case> {
    let mut result = Vec::new();
    for move_id in MOVES {
        for response in RESPONSES {
            for corner in [false, true] {
                for right in [true, false] {
                    result.push(Case { body, move_id, response, right, corner, jump: None, ranged: false, utility: false, saber: false, disc: false });
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
                        response: Response::Whiff, right, corner, jump: Some((dir, hop)), ranged: false, utility: false, saber: false, disc: false });
                }
            }
        }
    }
    result
}

fn ranged_cases() -> Vec<Case> {
    let mut result = Vec::new();
    for move_id in [MoveId::ShotA, MoveId::ShotB, MoveId::ExB] {
        for response in [Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::Whiff] {
            for corner in [false, true] {
                for right in [true, false] {
                    result.push(Case { body: CharacterId::Kogan, move_id, response,
                        right, corner, jump: None, ranged: true, utility: false, saber: false, disc: false });
                }
            }
        }
    }
    result
}

fn utility_cases() -> Vec<Case> {
    let mut result = Vec::new();
    for move_id in [MoveId::CommandGrab, MoveId::CommandDash] {
        let responses = if move_id == MoveId::CommandGrab {
            &[Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::Whiff, Response::AirEvade][..]
        } else { &[Response::Hit, Response::Whiff][..] };
        for &response in responses {
            for corner in [false, true] {
                for right in [true, false] {
                    result.push(Case { body: CharacterId::Kogan, move_id, response,
                        right, corner, jump: None, ranged: false, utility: true, saber: false, disc: false });
                }
            }
        }
    }
    result
}

fn saber_cases() -> Vec<Case> {
    let mut result = Vec::new();
    for move_id in [MoveId::StS, MoveId::StHS, MoveId::StHSClose, MoveId::Rekka1,
        MoveId::Rekka2, MoveId::Rekka3, MoveId::ExA, MoveId::Uppercut] {
        for response in [Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::Whiff] {
            for corner in [false, true] {
                for right in [true, false] {
                    result.push(Case { body: CharacterId::Kogan, move_id, response,
                        right, corner, jump: None, ranged: false, utility: false, saber: true, disc: false });
                }
            }
        }
    }
    result
}

fn disc_cases() -> Vec<Case> {
    let mut result = Vec::new();
    for response in [Response::Hit, Response::StandBlock, Response::CrouchBlock,
        Response::Whiff, Response::Projectile] {
        for corner in [false, true] {
            for right in [true, false] {
                result.push(Case { body: CharacterId::Kogan, move_id: MoveId::Guard,
                    response, right, corner, jump: None, ranged: false, utility: false,
                    saber: false, disc: true });
            }
        }
    }
    result
}

impl Case {
    fn duration(self) -> u32 {
        if self.disc { 90 }
        else if self.saber && self.move_id == MoveId::Rekka3 { 180 }
        else if self.ranged || self.utility || self.saber { 150 } else { LENGTH }
    }

    fn label(self) -> String {
        if self.utility && self.move_id == MoveId::CommandDash {
            return format!("KOGAN threshold-step · {} · {} · {}",
                if self.response == Response::Whiff { "free travel" } else { "near opponent" },
                if self.right { "right" } else { "left" },
                if self.corner { "corner" } else { "center" });
        }
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
        let (attacker, defender) = if self.disc {
            let distance = match self.response {
                Response::Whiff => 200, Response::Projectile => 150, _ => 40,
            };
            let defender = if self.corner { 740 } else { 500 };
            (defender - distance, defender)
        } else if self.saber {
            let distance = if self.move_id == MoveId::StHSClose { 40 }
                else if self.response == Response::Whiff { 360 }
                else if self.move_id == MoveId::StHS { 80 } else { 40 };
            let defender = if self.corner { 740 } else { 500 };
            (defender - distance, defender)
        } else if self.utility {
            let distance = if self.move_id == MoveId::CommandDash {
                if self.response == Response::Whiff { 200 } else { 70 }
            } else if self.response == Response::Whiff { 160 } else { 40 };
            let defender = if self.corner { 740 } else { 400 };
            (defender - distance, defender)
        } else if self.ranged {
            let distance = if self.response == Response::Whiff && self.move_id == MoveId::ShotB { 340 } else { 140 };
            let defender = if self.corner { 740 } else { 480 };
            (defender - distance, defender)
        } else if self.jump.is_some() {
            if self.corner { (660, 740) } else { (260, 500) }
        } else { (attacker, defender) };
        for (fighter, x) in world.fighters.iter_mut().zip([attacker, defender]) {
            fighter.pos.x = if self.right { px(x) } else { STAGE_W - px(x) };
        }
        world
    }

    // Follow-ups are driven by the legal window after active frames, so hitstop
    // cannot make a fixed wall-clock script skip the later rekka actions.
    fn inputs_for_world(self, frame: u32, world: &World) -> [InputFrame; 2] {
        let mut inputs = self.inputs(frame);
        if self.saber && world.hitstop == 0 {
            if let aeon_sim::Action::Attack { move_id, frame: action_frame, .. } = world.fighters[0].action {
                let follow = move_id == MoveId::Rekka1 && matches!(self.move_id, MoveId::Rekka2 | MoveId::Rekka3)
                    || move_id == MoveId::Rekka2 && self.move_id == MoveId::Rekka3;
                let mv = world.fighters[0].data().move_def(move_id).unwrap();
                if follow && action_frame == mv.last_active() + 1 {
                    inputs[0] = InputFrame::press(Btn::S);
                }
            }
        }
        inputs
    }

    fn inputs(self, frame: u32) -> [InputFrame; 2] {
        if self.disc {
            let mut attacker = InputFrame::dir(match frame {
                n if n == PRESS - 2 => 2,
                n if n == PRESS - 1 => 1,
                n if n == PRESS => 4,
                _ => 5,
            });
            if frame == PRESS { attacker.buttons = Buttons::one(Btn::HS); }
            let defender = if self.response == Response::Projectile {
                // The established disc-vs-voice trial: answer the cast eight ticks later.
                let mut input = InputFrame::dir(match frame {
                    n if n == PRESS - 10 => 2,
                    n if n == PRESS - 9 => 3,
                    n if n == PRESS - 8 => 6,
                    _ => 5,
                });
                if frame == PRESS - 8 { input.buttons = Buttons::one(Btn::HS); }
                input
            } else {
                InputFrame::dir(match self.response {
                    Response::StandBlock if frame >= PRESS => 4,
                    Response::CrouchBlock => 1,
                    _ => 5,
                })
            };
            return [attacker, defender];
        }
        if self.saber {
            let special = matches!(self.move_id, MoveId::Rekka1 | MoveId::Rekka2 | MoveId::Rekka3 | MoveId::ExA);
            let dp = self.move_id == MoveId::Uppercut;
            let dir = if special || dp {
                match frame {
                    n if n == PRESS - 3 => if dp { 6 } else { 2 },
                    n if n == PRESS - 2 => if dp { 2 } else { 3 },
                    n if n == PRESS - 1 || n == PRESS => if dp { 3 } else { 6 },
                    _ => 5,
                }
            } else { 5 };
            let mut attacker = InputFrame::dir(dir);
            if frame == PRESS {
                attacker.buttons = match self.move_id {
                    MoveId::StHS | MoveId::StHSClose => Buttons::one(Btn::HS),
                    MoveId::ExA => Buttons::two(Btn::S, Btn::HS),
                    _ => Buttons::one(Btn::S),
                };
            }
            // A far-away HS becomes far HS. Evade the close variant by jumping
            // while remaining inside its range selector, preserving legal input.
            let close_evade = self.move_id == MoveId::StHSClose && self.response == Response::Whiff;
            let defender = match self.response {
                Response::StandBlock if frame >= PRESS => 4,
                Response::CrouchBlock => 1,
                _ if close_evade && (PRESS - 8..PRESS - 1).contains(&frame) => 8,
                _ => 5,
            };
            return [attacker, InputFrame::dir(defender)];
        }

        if self.utility {
            let grab = self.move_id == MoveId::CommandGrab;
            let direction = if grab {
                match frame {
                    n if n == PRESS - 5 => 6,
                    n if n == PRESS - 4 => 3,
                    n if n == PRESS - 3 => 2,
                    n if n == PRESS - 2 => 1,
                    n if n == PRESS - 1 || n == PRESS => 4,
                    _ => 5,
                }
            } else {
                match frame {
                    n if n == PRESS - 3 => 2,
                    n if n == PRESS - 2 => 3,
                    n if n == PRESS - 1 || n == PRESS => 6,
                    _ => 5,
                }
            };
            let attacker = if frame == PRESS { InputFrame::dir_press(direction, Btn::FL) }
                else { InputFrame::dir(direction) };
            let defender = InputFrame::dir(match self.response {
                Response::StandBlock if frame >= PRESS => 4,
                Response::CrouchBlock => 1,
                Response::AirEvade if (PRESS - 6..PRESS + 1).contains(&frame) => 8,
                _ => 5,
            });
            return [attacker, defender];
        }
        if self.ranged {
            let wave = self.move_id == MoveId::ShotB;
            let direction = match frame {
                n if n == PRESS - 3 => 2,
                n if n == PRESS - 2 => if wave { 3 } else { 1 },
                n if n == PRESS - 1 || n == PRESS => if wave { 6 } else { 4 },
                _ => 5,
            };
            let mut attacker = InputFrame::dir(direction);
            if frame == PRESS {
                attacker.buttons = match self.move_id {
                    MoveId::ExB => Buttons::two(Btn::S, Btn::HS),
                    MoveId::ShotB => Buttons::one(Btn::HS),
                    _ => Buttons::one(Btn::S),
                };
            }
            let guard_start = if wave { PRESS + 34 } else { PRESS + 13 };
            let jump_start = if self.move_id == MoveId::ExB { PRESS - 4 } else { PRESS + 4 };
            let defense = match self.response {
                Response::StandBlock if frame >= guard_start => 4,
                Response::CrouchBlock => 1,
                Response::Whiff if !wave && (jump_start..jump_start + 7).contains(&frame) => 8,
                _ => 5,
            };
            return [attacker, InputFrame::dir(defense)];
        }
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
    let mut all = if args.iter().any(|a| a == "--kit-disc") {
        assert!(body == CharacterId::Kogan, "disc cases cover Kogan");
        disc_cases()
    } else if args.iter().any(|a| a == "--kit-saber") {
        assert!(body == CharacterId::Kogan, "saber cases currently cover Kogan");
        saber_cases()
    } else if args.iter().any(|a| a == "--kit-utility") {
        assert!(body == CharacterId::Kogan, "utility cases currently cover Kogan");
        utility_cases()
    } else if args.iter().any(|a| a == "--kit-ranged") {
        assert!(body == CharacterId::Kogan, "ranged cases currently cover Kogan");
        ranged_cases()
    } else if args.iter().any(|a| a == "--kit-movement") {
        movement_cases(body)
    } else { cases(body) };
    if let Some(name) = args.iter().find_map(|a| a.strip_prefix("--kit-move=")) {
        all.retain(|case| format!("{:?}", case.move_id) == name);
        assert!(!all.is_empty(), "--kit-move must name a move in the selected family");
    }
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
        let duration = case.duration();
        clock.reset();
        while frame < duration {
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
            for _ in 0..ticks.min((duration - frame) as usize) {
                let [p1, p2] = case.inputs_for_world(frame, &world);
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
            view.text_center(&case.label(), VW / 2.0, 660.0, 22.0, LINEN);
            view.text_center(
                &format!("KIT REVIEW · case {index} · tick {frame}/{duration} · SPACE pause · . step · ESC exit"),
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
                    let [p1, p2] = case.inputs_for_world(frame, &world);
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
    fn disc_preview_covers_close_contact_and_legal_projectile_absorption() {
        for case in disc_cases() {
            let mut world = case.world();
            let mut started = false;
            let mut drawings = std::collections::HashSet::new();
            let mut hits = 0; let mut blocks = 0; let mut absorbed = 0;
            for frame in 0..case.duration() {
                let [p1, p2] = case.inputs_for_world(frame, &world);
                world.tick(p1, p2);
                if let Some(cell) = crate::sequences::disc_cell(&world.fighters[0]) { drawings.insert(cell); }
                started |= world.fighters[0].action.attacking().is_some_and(|(id, _, _)| id == MoveId::Guard);
                for e in &world.events {
                    hits += usize::from(matches!(e.kind, EventKind::Hit | EventKind::Punish | EventKind::Knockdown));
                    blocks += usize::from(e.kind == EventKind::Block);
                    absorbed += usize::from(e.kind == EventKind::ProjectileGuard);
                }
            }
            assert_eq!(drawings.len(), 4, "{case:?}: all disc phases");
            assert!(started, "{case:?}: legal disc input");
            match case.response {
                Response::Hit => assert!(hits > 0 && blocks == 0 && absorbed == 0, "{case:?}"),
                Response::StandBlock | Response::CrouchBlock => assert!(hits == 0 && blocks > 0 && absorbed == 0, "{case:?}"),
                Response::Whiff => assert_eq!((hits, blocks, absorbed), (0, 0, 0), "{case:?}"),
                Response::Projectile => {
                    assert_eq!((hits, blocks, absorbed), (0, 0, 1), "{case:?}: glyph absorbed");
                    assert_eq!(world.fighters[0].health, world.fighters[0].data().max_health);
                    assert!(world.projectiles.is_empty());
                }
                _ => unreachable!(),
            }
            assert!(world.fighters.iter().all(|f| f.action.actionable()), "{case:?}: full recovery");
        }
    }

    #[test]
    fn kogan_reversal_keeps_complete_drawings_below_the_gameplay_hud() {
        use crate::sequences::{KOGAN_COIL, KOGAN_UPPERCUT, KOGAN_UPPERCUT_COMPACT};
        use crate::sprites::{key_green, Cell};
        let mut extents = std::collections::HashMap::new();
        for (file, specs, cells) in [
            ("kogan-uppercut-coil-v1-green.png", &KOGAN_COIL[..], vec![Cell::Uppercut(0)]),
            ("kogan-uppercut-v1-green.png", &KOGAN_UPPERCUT[..], (0..4).map(Cell::Uppercut).collect()),
            ("kogan-uppercut-compact-v1-green.png", &KOGAN_UPPERCUT_COMPACT[..], (0..2).map(Cell::UppercutCompact).collect()),
        ] {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/animation").join(file);
            let bytes = std::fs::read(path).unwrap();
            let mut image = Image::from_file_with_format(&bytes, None).unwrap();
            key_green(&mut image);
            for (&(r, _, anatomical), cell) in specs.iter().zip(cells) {
                // The standalone coil supersedes the older sheet coil.
                if file == "kogan-uppercut-v1-green.png" && cell == Cell::Uppercut(0) { continue; }
                let mut top = r[3];
                let mut bottom = r[1];
                for y in r[1] + 2..r[3] - 2 {
                    for x in r[0] + 2..r[2] - 2 {
                        if image.bytes[(y as usize * image.width as usize + x as usize) * 4 + 3] >= 24 {
                            top = top.min(y); bottom = bottom.max(y + 1);
                        }
                    }
                }
                assert!(bottom > top);
                extents.insert(cell, (bottom - top) as f32 / anatomical as f32);
            }
        }
        let mut seen = std::collections::HashSet::new();
        for case in saber_cases().into_iter().filter(|c| c.move_id == MoveId::Uppercut) {
            let mut world = case.world();
            for frame in 0..case.duration() {
                let [p1, p2] = case.inputs_for_world(frame, &world);
                world.tick(p1, p2);
                let f = &world.fighters[0];
                let cell = crate::sequences::compact_uppercut_cell(f).or_else(|| crate::sequences::cell_for(f));
                let Some(cell) = cell.filter(|c| matches!(c, Cell::Uppercut(_) | Cell::UppercutCompact(_))) else { continue; };
                seen.insert(cell);
                let visible_height = extents[&cell] * 1.20 * crate::render::sub_to_px(f.data().stand_h) * crate::render::WS;
                let top = crate::render::GROUND - crate::render::sub_to_px(f.pos.y) * crate::render::WS - visible_height;
                assert!(top >= 120.0, "{case:?} tick {frame} {cell:?} top {top}: timer/round-label clearance");
            }
        }
        for c in [Cell::Uppercut(0), Cell::Uppercut(1), Cell::UppercutCompact(0), Cell::UppercutCompact(1), Cell::Uppercut(3)] {
            assert!(seen.contains(&c), "complete frontward cut and descent: {c:?}");
        }
    }

    #[test]
    fn kogan_thrust_keeps_the_complete_blade_inside_both_corners() {
        use crate::render::{sub_to_px, WS};
        use crate::sprites::{animation_cell, key_green, thrust_layout};
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets/animation/kogan-thrust-v2-green.png");
        let mut image = Image::from_file_with_format(&std::fs::read(path).unwrap(), None).unwrap();
        key_green(&mut image);
        let extents: Vec<_> = (0..4).map(|cell| {
            let (r, anchor, height) = thrust_layout(cell);
            let (x0, x1) = ((r.x * image.width as f32).round() as usize,
                ((r.x + r.w) * image.width as f32).round() as usize);
            let (y0, y1) = ((r.y * image.height as f32).round() as usize,
                ((r.y + r.h) * image.height as f32).round() as usize);
            let mut left = x1; let mut right = x0;
            for y in y0..y1 { for x in x0..x1 {
                if image.bytes[(y * image.width as usize + x) * 4 + 3] >= 24 {
                    left = left.min(x); right = right.max(x + 1);
                }
            }}
            let scale = height / (y1 - y0) as f32;
            let root = x0 as f32 + anchor.x * (x1 - x0) as f32;
            ((left as f32 - root) * scale, (right as f32 - root) * scale)
        }).collect();
        for case in saber_cases().into_iter().filter(|c| c.move_id == MoveId::Rekka3) {
            let mut world = case.world();
            for frame in 0..case.duration() {
                let [p1, p2] = case.inputs_for_world(frame, &world);
                world.tick(p1, p2);
                let f = &world.fighters[0];
                if !matches!(f.action, Action::Attack { move_id: MoveId::Rekka3, .. }) { continue; }
                let cell = animation_cell(f, world.frame).unwrap() % 4;
                let (left, right) = extents[cell];
                let mut view = View { scale: 1.0, ox: 0.0, oy: 0.0, cam_x: 0.0 };
                view.follow(&world);
                let root = view.world(sub_to_px(f.pos.x), 0.0).x;
                let scale = sub_to_px(f.data().stand_h) * WS;
                let (left, right) = if f.facing_right { (root + left * scale, root + right * scale) }
                    else { (root - right * scale, root - left * scale) };
                // Reserve 3 world pixels for the authored contact lean plus hitstop impulse.
                let impulse = 3.0 * WS;
                assert!(left - impulse >= 8.0 && right + impulse <= VW - 8.0,
                    "{case:?} tick {frame} thrust {cell}: full silhouette {left}..{right}");
            }
        }
    }

    #[test]
    fn saber_preview_reaches_normals_rekka_followups_ex_and_reversal_legally() {
        for case in saber_cases() {
            let mut world = case.world();
            let mut started = std::collections::HashSet::new();
            let mut hits = 0;
            let mut blocks = 0;
            let mut target_contact = false;
            let mut air = false;
            let mut landing = false;
            for frame in 0..case.duration() {
                let [p1, p2] = case.inputs_for_world(frame, &world);
                world.tick(p1, p2);
                if let Some((id, _, connected)) = world.fighters[0].action.attacking() {
                    started.insert(id);
                    target_contact |= id == case.move_id && connected != aeon_sim::Connect::None;
                }
                air |= world.fighters[0].airborne;
                landing |= matches!(world.fighters[0].action, Action::Landing { total: 12, .. });
                for e in &world.events {
                    hits += usize::from(matches!(e.kind, EventKind::Hit | EventKind::Knockdown | EventKind::Punish));
                    blocks += usize::from(e.kind == EventKind::Block);
                }
            }
            assert!(started.contains(&case.move_id), "{case:?} legal target: {started:?}");
            if case.move_id == MoveId::Rekka3 {
                assert!(started.contains(&MoveId::Rekka1) && started.contains(&MoveId::Rekka2));
            }
            if case.response == Response::Whiff
                || matches!(case.move_id, MoveId::StS | MoveId::Uppercut) && case.response == Response::CrouchBlock {
                assert_eq!((hits, blocks), (0, 0), "{case:?}");
            } else {
                assert!(target_contact, "{case:?} target contact");
                if case.response == Response::Hit { assert!(hits > 0 && blocks == 0, "{case:?}"); }
                else { assert!(blocks > 0 && hits == 0, "{case:?}"); }
            }
            if case.move_id == MoveId::Uppercut { assert!(air && landing, "{case:?} reversal landing"); }
            assert!(world.fighters.iter().all(|f| f.action.actionable()), "{case:?} full recovery");
        }
    }

    #[test]
    fn utility_preview_exercises_snare_evasion_and_step_without_new_mechanics() {
        for case in utility_cases() {
            let mut world = case.world();
            let mut started = false;
            let mut grabs = 0;
            let mut throws = 0;
            let mut blocks = 0;
            let mut travel = 0;
            let mut drawings = std::collections::HashSet::new();
            for frame in 0..case.duration() {
                let x = world.fighters[0].pos.x;
                let [p1, p2] = case.inputs_for_world(frame, &world);
                world.tick(p1, p2);
                started |= world.fighters[0].action.attacking()
                    .is_some_and(|(id, _, _)| id == case.move_id);
                if frame >= PRESS { travel += (world.fighters[0].pos.x - x).abs(); }
                if let Some(cell) = crate::sequences::utility_cell(&world.fighters[0]) { drawings.insert(cell); }
                for event in &world.events {
                    grabs += usize::from(event.kind == EventKind::Grab);
                    throws += usize::from(event.kind == EventKind::Throw);
                    blocks += usize::from(event.kind == EventKind::Block);
                }
            }
            assert!(started, "{case:?} must start legally");
            let connects = case.move_id == MoveId::CommandGrab
                && !matches!(case.response, Response::Whiff | Response::AirEvade);
            assert_eq!((grabs, throws, blocks), if connects { (1, 1, 0) } else { (0, 0, 0) }, "{case:?}");
            assert!(world.fighters.iter().all(|f| f.action.actionable()), "{case:?} complete recovery");
            let base = if case.move_id == MoveId::CommandDash { 4 } else { 0 };
            for phase in 0..4 {
                assert!(drawings.contains(&crate::sprites::Cell::Utility(base + phase)), "{case:?} phase {phase}");
            }
            if case.move_id == MoveId::CommandDash {
                assert!(travel > 0 && travel <= px(60), "{case:?} authored travel");
                if case.response == Response::Whiff { assert_eq!(travel, px(60), "{case:?}"); }
            }
        }
    }

    #[test]
    fn ranged_preview_exercises_release_guard_whiff_and_recovery() {
        for case in ranged_cases() {
            let mut world = case.world();
            let mut started = false;
            let mut hits = 0;
            let mut blocks = 0;
            let mut drawings = std::collections::HashSet::new();
            for frame in 0..case.duration() {
                let [p1, p2] = case.inputs_for_world(frame, &world);
                world.tick(p1, p2);
                started |= world.fighters[0].action.attacking()
                    .is_some_and(|(id, _, _)| id == case.move_id);
                if let Some(cell) = crate::sequences::ranged_cell(&world.fighters[0]) {
                    drawings.insert(cell);
                }
                for event in &world.events {
                    hits += usize::from(matches!(event.kind, EventKind::Hit | EventKind::Knockdown | EventKind::Punish));
                    blocks += usize::from(event.kind == EventKind::Block);
                }
            }
            assert!(started, "{case:?} must start through motion inputs");
            let expected = if case.response == Response::Whiff { (0, 0) }
                else if matches!(case.response, Response::StandBlock | Response::CrouchBlock) { (0, 1) }
                else { (1, 0) };
            assert_eq!((hits, blocks), expected, "{case:?}");
            assert!(world.fighters.iter().all(|f| f.action.actionable()), "{case:?} complete recovery");
            let base = if case.move_id == MoveId::ShotB { 4 } else { 0 };
            for phase in 0..4 {
                assert!(drawings.contains(&crate::sprites::Cell::Ranged(base + phase)), "{case:?} phase {phase}");
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
                    let [p1, p2] = case.inputs_for_world(frame, &world);
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
