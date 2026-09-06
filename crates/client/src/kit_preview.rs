//! Isolated full-kit comparisons using legal inputs and unchanged combat data.
//! --kit-preview [--kit-raya] [--kit-case=N] [--capture].
//! Each case covers preparation, contact/whiff and return to an actionable state.
use super::{Assets, Presentation};
use crate::render::{draw_hud, HudOpts, View, INK, LINEN, VW};
use crate::timing::FixedClock;
use aeon_sim::{px, Btn, Buttons, CharacterId, InputFrame, MoveId, World, STAGE_W};
use macroquad::prelude::*;
use std::io::Write;

#[path = "victory_preview.rs"]
mod victory;
#[path = "ko_preview.rs"]
mod ko;
#[path = "air_exchange_preview.rs"]
mod air_exchange;

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
    EarlyWhiff,
    RisingWhiff,
    Projectile,
    TechEarly,
    TechLate,
    ChargeTap,
    ChargeRelease,
    ChargeMax,
    ChargeInterrupt,
    Whiff,
}

const RESPONSES: [Response; 5] = [
    Response::Hit,
    Response::StandBlock,
    Response::CrouchBlock,
    Response::CrouchHit,
    Response::Whiff,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ground {
    WalkForward, WalkBack, Crouch, RunStop, RunCrouch,
    RunBlock, RunJump, RunAttack, BackDash,
}

#[derive(Clone, Copy, Debug)]
struct Case {
    body: CharacterId,
    move_id: MoveId,
    response: Response,
    right: bool,
    corner: bool,
    jump: Option<(u8, bool)>,
    air: bool,
    ranged: bool,
    utility: bool,
    saber: bool,
    disc: bool,
    reaction: bool,
    ground: Option<Ground>,
    feint: Option<bool>, // true: first startup tick; false: last startup tick
}

fn cases(body: CharacterId) -> Vec<Case> { normal_cases(body, &MOVES) }

fn flash_cases(body: CharacterId) -> Vec<Case> {
    normal_cases(body, &[MoveId::StFL, MoveId::StST])
}

fn throw_cases(body: CharacterId) -> Vec<Case> {
    let mut cases = normal_cases(body, &[MoveId::Throw]);
    for response in [Response::AirEvade, Response::TechEarly, Response::TechLate] {
        for corner in [false, true] {
            for right in [true, false] {
                let mut case = cases[0];
                case.response = response; case.corner = corner; case.right = right;
                cases.push(case);
            }
        }
    }
    cases
}

fn feint_cases(body: CharacterId) -> Vec<Case> {
    let extra = if body == CharacterId::Kogan { disc_cases() } else {
        ritual_cases().into_iter().filter(|c| c.move_id == MoveId::Charge && c.response == Response::ChargeMax)
            .map(|mut c| { c.response = Response::Hit; c }).collect()
    };
    let base = saber_cases(body).into_iter().chain(ranged_cases(body)).chain(utility_cases(body))
        .chain(extra).chain(overhead_cases(body))
        .filter(|c| c.response == Response::Hit
            && body.data().move_def(c.move_id).is_some_and(|m| m.feintable))
        .collect::<Vec<_>>();
    [true, false].into_iter().flat_map(|early| base.iter().copied().map(move |mut c| {
        c.feint = Some(early); c
    })).collect()
}

fn overhead_cases(body: CharacterId) -> Vec<Case> {
    let moves: &[MoveId] = if body == CharacterId::Kogan { &[MoveId::Overhead, MoveId::SpecialOverhead] } else { &[MoveId::Overhead] };
    normal_cases(body, moves)
}

fn crouching_saber_cases(body: CharacterId) -> Vec<Case> {
    normal_cases(body, &[MoveId::CrS, MoveId::CrHS, MoveId::CrFL, MoveId::CrST])
}

fn normal_cases(body: CharacterId, moves: &[MoveId]) -> Vec<Case> {
    let mut result = Vec::new();
    for &move_id in moves {
        for response in RESPONSES {
            for corner in [false, true] {
                for right in [true, false] {
                    result.push(Case { body, move_id, response, right, corner, jump: None, air: false, ranged: false, utility: false, saber: false, disc: false, reaction: false, ground: None, feint: None });
                }
            }
        }
    }
    result
}

fn ritual_cases() -> Vec<Case> {
    let mut result = Vec::new();
    for (move_id, responses) in [
        (MoveId::Charge, [Response::ChargeTap, Response::ChargeRelease, Response::ChargeMax, Response::ChargeInterrupt]),
        (MoveId::Detonate, [Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::Whiff]),
    ] {
        for response in responses { for corner in [false,true] { for right in [true,false] {
            let mut case = normal_cases(CharacterId::Raya, &[move_id])[0];
            case.response = response; case.corner = corner; case.right = right;
            result.push(case);
        } } }
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
                        response: Response::Whiff, right, corner, jump: Some((dir, hop)), air: false, ranged: false, utility: false, saber: false, disc: false, reaction: false, ground: None, feint: None });
                }
            }
        }
    }
    result
}

fn air_cases(body: CharacterId) -> Vec<Case> {
    let mut result = Vec::new();
    for move_id in [MoveId::JP, MoveId::JK, MoveId::JS, MoveId::JHS, MoveId::JFL, MoveId::JST, MoveId::AirShot] {
        if body == CharacterId::Raya && move_id == MoveId::AirShot { continue; }
        for hop in [true, false] {
            for response in [Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::Whiff] {
                for corner in [false, true] {
                    for right in [true, false] {
                        result.push(Case { body, move_id, response, right, corner,
                            jump: Some((8, hop)), air: true, ranged: false, utility: false,
                            saber: false, disc: false, reaction: false, ground: None, feint: None });
                    }
                }
            }
        }
    }
    result
}


// Earlier legal input leaves time to inspect withdrawal above the floor.
fn early_air_cases(body: CharacterId) -> Vec<Case> {
    air_cases(body).into_iter().filter(|c| c.response == Response::Whiff && c.move_id != MoveId::AirShot)
        .map(|mut c| { c.response = Response::EarlyWhiff; c }).collect()
}

// Rising input exposes the full recovery even on Raya's shorter airborne arc.
fn rising_air_cases(body: CharacterId) -> Vec<Case> {
    early_air_cases(body).into_iter().map(|mut c| { c.response = Response::RisingWhiff; c }).collect()
}

fn ranged_cases(body: CharacterId) -> Vec<Case> {
    let mut result = Vec::new();
    for move_id in [MoveId::ShotA, MoveId::ShotB, MoveId::ExA, MoveId::ExB] {
        if body == CharacterId::Kogan && move_id == MoveId::ExA { continue; }
        for response in [Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::Whiff] {
            for corner in [false, true] {
                for right in [true, false] {
                    result.push(Case { body, move_id, response,
                        right, corner, jump: None, air: false, ranged: true, utility: false, saber: false, disc: false, reaction: false, ground: None, feint: None });
                }
            }
        }
    }
    result
}

fn utility_cases(body: CharacterId) -> Vec<Case> {
    let mut result = Vec::new();
    for move_id in [MoveId::CommandGrab, MoveId::CommandDash] {
        let responses = if move_id == MoveId::CommandGrab {
            &[Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::Whiff, Response::AirEvade][..]
        } else { &[Response::Hit, Response::Whiff][..] };
        for &response in responses {
            for corner in [false, true] {
                for right in [true, false] {
                    result.push(Case { body, move_id, response,
                        right, corner, jump: None, air: false, ranged: false, utility: true, saber: false, disc: false, reaction: false, ground: None, feint: None });
                }
            }
        }
    }
    result
}

fn saber_cases(body: CharacterId) -> Vec<Case> {
    let mut result = Vec::new();
    for move_id in [MoveId::StS, MoveId::StHS, MoveId::StHSClose, MoveId::Rekka1,
        MoveId::Rekka2, MoveId::Rekka3, MoveId::ExA, MoveId::Uppercut] {
        // Raya's EX is a placed glyph and belongs to her projectile fixture.
        if body == CharacterId::Raya && move_id == MoveId::ExA { continue; }
        let responses = if body == CharacterId::Raya {
            &[Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::CrouchHit, Response::Whiff][..]
        } else { &[Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::Whiff][..] };
        for &response in responses {
            for corner in [false, true] {
                for right in [true, false] {
                    result.push(Case { body, move_id, response,
                        right, corner, jump: None, air: false, ranged: false, utility: false, saber: true, disc: false, reaction: false, ground: None, feint: None });
                }
            }
        }
    }
    result
}

fn judgment_cases(body: CharacterId) -> Vec<Case> {
    let mut result = Vec::new();
    let responses = if body == CharacterId::Raya {
        &[Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::CrouchHit, Response::Whiff][..]
    } else { &[Response::Hit, Response::StandBlock, Response::CrouchBlock, Response::Whiff][..] };
    for &response in responses {
        for corner in [false, true] {
            for right in [true, false] {
                result.push(Case { body, move_id: MoveId::Super, response,
                    right, corner, jump: None, air: false, ranged: false, utility: false, saber: true,
                    disc: false, reaction: false, ground: None, feint: None });
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
                    response, right, corner, jump: None, air: false, ranged: false, utility: false,
                    saber: false, disc: true, reaction: false, ground: None, feint: None });
            }
        }
    }
    result
}

fn ground_cases(body: CharacterId) -> Vec<Case> {
    let mut result = Vec::new();
    for ground in [Ground::WalkForward, Ground::WalkBack, Ground::Crouch,
        Ground::RunStop, Ground::RunCrouch, Ground::RunBlock, Ground::RunJump,
        Ground::RunAttack, Ground::BackDash] {
        for corner in [false, true] {
            for right in [true, false] {
                result.push(Case { body, move_id: MoveId::StS, response: Response::Whiff,
                    right, corner, jump: None, air: false, ranged: false, utility: false,
                    saber: false, disc: false, reaction: false, ground: Some(ground), feint: None });
            }
        }
    }
    result
}

fn reaction_cases(victim: CharacterId) -> Vec<Case> {
    let body = match victim { CharacterId::Kogan => CharacterId::Raya, CharacterId::Raya => CharacterId::Kogan };
    let mut result = Vec::new();
    for (move_id, response) in [(MoveId::StP, Response::Hit), (MoveId::StS, Response::Hit),
        (MoveId::CrK, Response::CrouchHit), (MoveId::StS, Response::StandBlock),
        (MoveId::CrK, Response::CrouchBlock), (MoveId::Uppercut, Response::Hit),
        (MoveId::CrST, Response::Hit), (MoveId::Throw, Response::Hit),
        (MoveId::CommandGrab, Response::Hit)] {
        for corner in [false, true] {
            for right in [true, false] {
                result.push(Case { body, move_id, response, right, corner, jump: None,
                    air: false, ranged: false, utility: false, saber: false, disc: false,
                    reaction: true, ground: None, feint: None });
            }
        }
    }
    result
}

impl Case {
    fn duration(self) -> u32 {
        if self.feint.is_some() || matches!(self.move_id, MoveId::Charge | MoveId::Detonate) { 120 }
        else if self.body == CharacterId::Raya && self.ranged { 210 }
        else if self.reaction || self.move_id == MoveId::Throw { 150 }
        else if self.disc || self.ground.is_some() { 90 }
        else if self.saber && self.move_id == MoveId::Rekka3 { 180 }
        else if self.air || self.ranged || self.utility || self.saber || matches!(self.move_id, MoveId::CrST | MoveId::SpecialOverhead) { 150 }
        else if matches!(self.move_id, MoveId::CrS | MoveId::CrHS | MoveId::CrFL | MoveId::Overhead) { 90 }
        else { LENGTH }
    }

    fn label(self) -> String {
        if let Some(early) = self.feint {
            return format!("{} {:?} feint · {} · {} · {}", self.body.name(), self.move_id,
                if early { "early" } else { "late" }, if self.right { "right" } else { "left" },
                if self.corner { "corner" } else { "center" });
        }
        if self.reaction {
            let victim = if self.body == CharacterId::Raya { "KOGAN" } else { "RAYA" };
            return format!("{} reaction vs {:?} · {:?} · attacker {} · {}", victim, self.move_id,
                self.response, if self.right { "right" } else { "left" },
                if self.corner { "corner" } else { "center" });
        }
        if let Some(ground) = self.ground {
            return format!("{} {:?} · {} · {}", self.body.name(), ground,
                if self.right { "right" } else { "left" },
                if self.corner { "corner" } else { "center" });
        }
        if self.utility && self.move_id == MoveId::CommandDash {
            return format!("{} {} · {} · {} · {}", self.body.name(),
                if self.body == CharacterId::Kogan { "threshold-step" } else { "processional" },
                if self.response == Response::Whiff { "free travel" } else { "near opponent" },
                if self.right { "right" } else { "left" },
                if self.corner { "corner" } else { "center" });
        }
        if self.air {
            return format!("{} {:?} · {:?} · {} · {} · {}", self.body.name(), self.move_id, self.response,
                if self.jump.unwrap().1 { "hop" } else { "jump" },
                if self.right { "right" } else { "left" }, if self.corner { "corner" } else { "center" });
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
        if self.body == CharacterId::Raya && self.ranged { world.fighters[0].gauge = if matches!(self.move_id, MoveId::ExA | MoveId::ExB) { 50 } else { 0 }; }
        if self.move_id == MoveId::Super { world.fighters[0].meter = 1000; }
        if self.body == CharacterId::Kogan && self.air && self.move_id == MoveId::JFL { world.fighters[0].gauge = 0; }
        if self.move_id == MoveId::Charge { world.fighters[0].gauge = 0; }
        let gap = if self.response == Response::Whiff { 150 } else { 40 };
        let defender = if self.corner { 740 } else { 340 };
        let attacker = defender - gap;
        let (attacker, defender) = if self.move_id == MoveId::Detonate {
            let defender = if self.corner { 740 } else { 480 };
            (defender - if self.response == Response::Whiff { 300 } else { 110 }, defender)
        } else if self.move_id == MoveId::Throw && !self.reaction {
            let defender = if self.corner { 740 } else { 500 };
            let gap = if self.response == Response::Whiff { 150 } else { 35 };
            (defender - gap, defender)
        } else if self.move_id == MoveId::SpecialOverhead {
            let defender = if self.corner { 740 } else { 500 };
            let gap = if self.response == Response::Whiff { 300 } else { 100 };
            (defender - gap, defender)
        } else if self.air {
            let defender = if self.corner { 740 } else { 500 };
            let gap = if matches!(self.response, Response::Whiff | Response::EarlyWhiff | Response::RisingWhiff) { 360 }
                else if self.move_id == MoveId::AirShot { if self.jump.unwrap().1 { 100 } else { 140 } } else { 35 };
            (defender - gap, defender)
        } else if self.reaction {
            let defender = if self.corner { 740 } else { 500 };
            (defender - 35, defender)
        } else if let Some(ground) = self.ground {
            if self.corner && matches!(ground, Ground::WalkBack | Ground::BackDash | Ground::Crouch) {
                (60, 260)
            } else if self.corner { (540, 740) } else { (300, 500) }
        } else if self.disc {
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
            let distance = if self.body == CharacterId::Raya {
                if self.response == Response::Whiff { 300 } else { match self.move_id { MoveId::ShotA => 70, MoveId::ExB => 150, _ => 90 } }
            } else if self.response == Response::Whiff && self.move_id == MoveId::ShotB { 340 } else { 140 };
            let defender = if self.corner { 740 } else { 480 };
            (defender - distance, defender)
        } else if self.jump.is_some() {
            if self.corner { (660, 740) } else { (260, 500) }
        } else { (attacker, defender) };
        for (fighter, x) in world.fighters.iter_mut().zip([attacker, defender]) {
            fighter.pos.x = if self.right { px(x) } else { STAGE_W - px(x) };
        }
        if self.move_id == MoveId::Detonate {
            // Produce the starting armed trap with a real QCB+S and full flight.
            world.fighters[0].gauge = 0;
            for tick in 0..80 {
                let mut a = InputFrame::dir(match tick { 8 => 2, 9 => 1, 10 => 4, _ => 5 });
                if tick == 10 { a.buttons = Buttons::one(Btn::S); }
                world.tick(a, InputFrame::dir(5));
            }
        }
        world
    }

    // Follow-ups are driven by the legal window after active frames, so hitstop
    // cannot make a fixed wall-clock script skip the later rekka actions.
    fn inputs_for_world(self, frame: u32, world: &World) -> [InputFrame; 2] {
        let mut inputs = self.inputs(frame);
        if self.move_id == MoveId::Detonate && self.response == Response::StandBlock {
            let release = world.fighters[0].action.attacking().is_some_and(|(id, age, _)| id == MoveId::Detonate && age >= 5);
            inputs[1] = InputFrame::dir(if release {4} else {5});
        }

        if self.body == CharacterId::Raya && self.ranged && self.response == Response::StandBlock {
            // Begin holding back at the real release/arming boundary so the
            // fixture does not walk out of a stationary glyph or planted trap.
            let glyph_release = matches!(self.move_id, MoveId::ShotB | MoveId::ExA)
                && world.fighters[0].action.attacking().is_some_and(|(id, age, _)|
                    id == self.move_id && age + 1 >= u16::from(self.body.data().move_def(id).unwrap().startup));
            let live = world.projectiles.iter().any(|p| p.owner == 0 && (p.live()
                || matches!(p.state, aeon_sim::ShotState::Planted { armed: false, timer } if timer + 1 >= p.arm_after)));
            inputs[1] = InputFrame::dir(if glyph_release || live { 4 } else { 5 });
        }

        if self.air {
            let f = &world.fighters[0];
            let attack_height = if self.response == Response::EarlyWhiff { 999 }
                else if self.move_id == MoveId::AirShot { 140 } else { 80 };
            let timing = if self.response == Response::RisingWhiff {
                f.vel.y > 0 && f.pos.y >= px(20)
            } else { f.vel.y <= 0 && f.pos.y <= px(attack_height) };
            let ready = world.hitstop == 0 && timing
                && matches!(f.action, aeon_sim::Action::Jump { air_ok: true, .. });
            if ready {
                inputs[0].buttons = Buttons::one(match self.move_id {
                    MoveId::JP => Btn::P, MoveId::JK => Btn::K, MoveId::JS => Btn::S,
                    MoveId::JHS => Btn::HS, MoveId::JST => Btn::ST, _ => Btn::FL,
                });
            }
            let guard = if self.move_id == MoveId::AirShot {
                world.projectiles.iter().any(|p| p.owner == 0
                    && (p.pos.x - world.fighters[1].pos.x).abs() <= px(50))
                    || matches!(world.fighters[1].action, aeon_sim::Action::Block { .. })
            } else { ready || f.action.attacking().is_some() };
            inputs[1] = InputFrame::dir(match self.response {
                Response::StandBlock if guard => 4,
                Response::CrouchBlock => 1, _ => 5,
            });
        }
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
        if let Some(early) = self.feint {
            let f = &world.fighters[0];
            // Release the held channel button after canceling; otherwise the
            // next edge would issue an unrelated normal during the return.
            if self.move_id == MoveId::Charge && f.last_move == Some(MoveId::Charge)
                && !matches!(f.action, aeon_sim::Action::Attack { move_id: MoveId::Charge, .. }) {
                inputs[0] = InputFrame::dir(5);
            }
            if world.hitstop == 0 {
                if let aeon_sim::Action::Attack { move_id, frame: action_frame, .. } = f.action {
                    let mv = f.data().move_def(move_id).unwrap();
                    let cancel_frame = if early { 0 } else { mv.first_active() - 1 };
                    // A held FL is outside the chord window by late Charge startup.
                    // Release for one startup tick, then press FL+ST together.
                    if move_id == MoveId::Charge && self.move_id == move_id
                        && action_frame + 1 == cancel_frame {
                        inputs[0] = InputFrame::dir(5);
                    }
                    if move_id == self.move_id && action_frame == cancel_frame {
                        inputs[0] = InputFrame::chord(aeon_sim::Chord::Feint);
                    }
                }
            }
        }
        inputs
    }

    fn inputs(self, frame: u32) -> [InputFrame; 2] {
        if matches!(self.move_id, MoveId::Charge | MoveId::Detonate) {
            let mut a = InputFrame::dir(match frame { n if n == PRESS-2 => 2, n if n == PRESS-1 => 1, n if n == PRESS => 4, _ => 5 });
            let hold_to = match self.response { Response::ChargeTap => PRESS+1, Response::ChargeRelease => 42, _ => 100 };
            if self.move_id == MoveId::Charge && (PRESS..hold_to).contains(&frame) { a.buttons = Buttons::one(Btn::FL); }
            if self.move_id == MoveId::Detonate && frame == PRESS { a.buttons = Buttons::one(Btn::S); }
            let mut b = InputFrame::dir(if self.response == Response::CrouchBlock {1} else {5});
            if self.response == Response::ChargeInterrupt && frame == 30 { b.buttons = Buttons::one(Btn::S); }
            return [a,b];
        }

        if self.move_id == MoveId::Throw && !self.reaction {
            let mut attacker = InputFrame::dir(5);
            if frame == PRESS { attacker.buttons = Buttons::two(Btn::P, Btn::K); }
            let mut defender = InputFrame::dir(match self.response {
                Response::StandBlock if frame >= PRESS => 4,
                Response::CrouchBlock => 1, Response::CrouchHit => 2,
                Response::AirEvade if (PRESS - 8..=PRESS).contains(&frame) => 8,
                _ => 5,
            });
            if self.response == Response::TechEarly && frame == PRESS + 4
                || self.response == Response::TechLate && frame == PRESS + 8 {
                defender.buttons = Buttons::two(Btn::P, Btn::K);
            }
            return [attacker, defender];
        }
        if matches!(self.move_id, MoveId::Overhead | MoveId::SpecialOverhead) {
            let leaping = self.move_id == MoveId::SpecialOverhead;
            let dir = if leaping {
                match frame {
                    n if n == PRESS - 3 => 2, n if n == PRESS - 2 => 3,
                    n if n == PRESS - 1 || n == PRESS => 6, _ => 5,
                }
            } else { 5 };
            let mut attacker = InputFrame::dir(dir);
            if frame == PRESS {
                attacker.buttons = if leaping { Buttons::one(Btn::ST) }
                    else { Buttons::two(Btn::HS, Btn::ST) };
            }
            let guard_start = PRESS + if leaping { 16 } else { 20 };
            let defender = InputFrame::dir(match self.response {
                Response::StandBlock if frame >= guard_start => 4,
                Response::CrouchBlock => 1, Response::CrouchHit => 2, _ => 5,
            });
            return [attacker, defender];
        }
        if self.reaction {
            let crouch = matches!(self.move_id, MoveId::CrK | MoveId::CrST);
            let dir = if self.move_id == MoveId::CommandGrab {
                match frame {
                    n if n == PRESS - 5 => 6, n if n == PRESS - 4 => 3,
                    n if n == PRESS - 3 => 2, n if n == PRESS - 2 => 1,
                    n if n == PRESS - 1 || n == PRESS => 4, _ => 5,
                }
            } else if self.move_id == MoveId::Uppercut {
                match frame {
                    n if n == PRESS - 3 => 6, n if n == PRESS - 2 => 2,
                    n if n == PRESS - 1 || n == PRESS => 3, _ => 5,
                }
            } else if crouch { 2 } else { 5 };
            let mut attacker = InputFrame::dir(dir);
            if frame == PRESS {
                attacker.buttons = match self.move_id {
                    MoveId::StP => Buttons::one(Btn::P), MoveId::CrK => Buttons::one(Btn::K),
                    MoveId::CrST => Buttons::one(Btn::ST), MoveId::Throw => Buttons::two(Btn::P, Btn::K),
                    MoveId::CommandGrab => Buttons::one(Btn::FL), _ => Buttons::one(Btn::S),
                };
            }
            let defender = InputFrame::dir(match self.response {
                Response::StandBlock if frame >= PRESS => 4,
                Response::CrouchBlock => 1, Response::CrouchHit => 2, _ => 5,
            });
            return [attacker, defender];
        }
        if let Some(ground) = self.ground {
            let dir = match ground {
                Ground::WalkForward if (12..60).contains(&frame) => 6,
                Ground::WalkBack if (12..60).contains(&frame) => 4,
                Ground::Crouch if (12..40).contains(&frame) => 2,
                Ground::BackDash if frame == 12 || frame == 14 => 4,
                Ground::RunStop | Ground::RunCrouch | Ground::RunBlock
                    | Ground::RunJump | Ground::RunAttack if frame == 12 || (14..40).contains(&frame) => 6,
                Ground::RunCrouch if (40..66).contains(&frame) => 2,
                Ground::RunBlock if (40..74).contains(&frame) => 4,
                Ground::RunJump if (40..46).contains(&frame) => 9,
                _ => 5,
            };
            let mut p1 = InputFrame::dir(dir);
            if ground == Ground::RunAttack && frame == 40 { p1.buttons = Buttons::one(Btn::S); }
            let p2 = if ground == Ground::RunBlock && frame == 40 { InputFrame::press(Btn::S) }
                else { InputFrame::dir(5) };
            return [p1, p2];
        }
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
            let dir = if self.move_id == MoveId::Super {
                match frame {
                    n if n == PRESS - 6 || n == PRESS - 3 => 2,
                    n if n == PRESS - 5 || n == PRESS - 2 => 3,
                    n if n == PRESS - 4 || n == PRESS - 1 || n == PRESS => 6,
                    _ => 5,
                }
            } else if special || dp {
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
                Response::CrouchHit => 2,
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
            let wave = matches!(self.move_id, MoveId::ShotB | MoveId::ExA);
            let direction = match frame {
                n if n == PRESS - 3 => 2,
                n if n == PRESS - 2 => if wave { 3 } else { 1 },
                n if n == PRESS - 1 || n == PRESS => if wave { 6 } else { 4 },
                _ => 5,
            };
            let mut attacker = InputFrame::dir(direction);
            if frame == PRESS {
                attacker.buttons = match self.move_id {
                    MoveId::ExA | MoveId::ExB => Buttons::two(Btn::S, Btn::HS),
                    MoveId::ShotB => Buttons::one(Btn::HS),
                    _ => Buttons::one(Btn::S),
                };
            }
            let guard_start = if wave { PRESS + 34 } else { PRESS + 13 };
            let jump_start = if self.move_id == MoveId::ExB { PRESS - 4 } else { PRESS + 4 };
            let defense = match self.response {
                Response::StandBlock if frame >= guard_start => 4,
                Response::CrouchBlock => 1,
                Response::Whiff if self.body == CharacterId::Kogan && !wave && (jump_start..jump_start + 7).contains(&frame) => 8,
                _ => 5,
            };
            return [attacker, InputFrame::dir(defense)];
        }
        if let Some((dir, hop)) = self.jump {
            let up = frame == PRESS || (!hop && (PRESS..PRESS + 7).contains(&frame));
            return [InputFrame::dir(if up { dir } else { 5 }), InputFrame::default()];
        }
        let crouch = matches!(self.move_id, MoveId::CrP | MoveId::CrK | MoveId::CrS | MoveId::CrHS | MoveId::CrFL | MoveId::CrST);
        let attacker = if frame == PRESS {
            let button = match self.move_id {
                MoveId::StP | MoveId::CrP => Btn::P,
                MoveId::StFL | MoveId::CrFL => Btn::FL, MoveId::StST | MoveId::CrST => Btn::ST,
                MoveId::CrS => Btn::S, MoveId::CrHS => Btn::HS,
                _ => Btn::K,
            };
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
    if args.iter().any(|a| a == "--kit-air-exchange") { return air_exchange::run(assets, &args).await; }
    if args.iter().any(|a| a == "--kit-ko") { return ko::run(assets, &args).await; }
    if args.iter().any(|a| a == "--kit-victory") { return victory::run(assets, &args).await; }
    let capture = args.iter().any(|a| a == "--capture");
    let body = if args.iter().any(|a| a == "--kit-raya") {
        CharacterId::Raya
    } else {
        CharacterId::Kogan
    };
    let selected = args.iter().find_map(|a| a.strip_prefix("--kit-case=")).map(|n| {
        n.parse::<usize>().expect("--kit-case must be a nonnegative integer")
    });
    let mut all = if args.iter().any(|a| a == "--kit-ritual") {
        ritual_cases()
    } else if args.iter().any(|a| a == "--kit-crp") {
        normal_cases(body, &[MoveId::CrP])
    } else if args.iter().any(|a| a == "--kit-feint") {
        feint_cases(body)
    } else if args.iter().any(|a| a == "--kit-throw") {
        throw_cases(body)
    } else if args.iter().any(|a| a == "--kit-overhead") {
        overhead_cases(body)
    } else if args.iter().any(|a| a == "--kit-crouch") {
        crouching_saber_cases(body)
    } else if args.iter().any(|a| a == "--kit-flash") {
        flash_cases(body)
    } else if args.iter().any(|a| a == "--kit-air") {
        if args.iter().any(|a| a == "--kit-air-rising") { rising_air_cases(body) }
            else if args.iter().any(|a| a == "--kit-air-early") { early_air_cases(body) } else { air_cases(body) }
    } else if args.iter().any(|a| a == "--kit-reaction") {
        reaction_cases(body)
    } else if args.iter().any(|a| a == "--kit-super") {
        judgment_cases(body)
    } else if args.iter().any(|a| a == "--kit-ground") {
        ground_cases(body)
    } else if args.iter().any(|a| a == "--kit-disc") {
        assert!(body == CharacterId::Kogan, "disc cases cover Kogan");
        disc_cases()
    } else if args.iter().any(|a| a == "--kit-saber") {
        saber_cases(body)
    } else if args.iter().any(|a| a == "--kit-utility") {
        utility_cases(body)
    } else if args.iter().any(|a| a == "--kit-ranged") {
        ranged_cases(body)
    } else if args.iter().any(|a| a == "--kit-movement") {
        movement_cases(body)
    } else { cases(body) };
    if let Some(name) = args.iter().find_map(|a| a.strip_prefix("--kit-feint-timing=")) {
        assert!(name == "early" || name == "late", "--kit-feint-timing must be early or late");
        all.retain(|c| c.feint == Some(name == "early"));
        assert!(!all.is_empty(), "--kit-feint-timing requires feint cases");
    }
    if let Some(name) = args.iter().find_map(|a| a.strip_prefix("--kit-jump=")) {
        assert!(name == "hop" || name == "full", "--kit-jump must be hop or full");
        all.retain(|case| case.jump.is_some_and(|(_, hop)| hop == (name == "hop")));
        assert!(!all.is_empty(), "--kit-jump requires jumping cases");
    }
    if let Some(name) = args.iter().find_map(|a| a.strip_prefix("--kit-ground-state=")) {
        all.retain(|case| case.ground.is_some_and(|g| format!("{g:?}") == name));
        assert!(!all.is_empty(), "--kit-ground-state must name a ground case");
    }
    if let Some(name) = args.iter().find_map(|a| a.strip_prefix("--kit-response=")) {
        all.retain(|case| format!("{:?}", case.response) == name);
        assert!(!all.is_empty(), "--kit-response must name a response in the selected family");
    }
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
    fn raya_ritual_preview_uses_legal_channel_and_manual_armed_detonation() {
        let cases = ritual_cases(); assert_eq!(cases.len(),32);
        for case in cases {
            let mut world = case.world();
            let hp = [world.fighters[0].health,world.fighters[1].health];
            if case.move_id == MoveId::Detonate {
                assert!(world.projectiles.iter().any(|p| p.owner==0 && p.armed()), "{case:?}: real armed starting trap");
                assert_eq!(hp[1],World::new(CharacterId::Raya,CharacterId::Kogan).fighters[1].health, "trap must not touch before command");
            }
            let mut drawings=std::collections::HashSet::new();
            let mut frozen=None;
            let mut started=false; let mut detonated=false; let mut channels=0; let mut hit=false; let mut block=false;
            for tick in 0..case.duration() {
                let [a,b]=case.inputs_for_world(tick,&world);world.tick(a,b);
                let f=&world.fighters[0];let cell=crate::sequences::ritual_cell(f);
                if let Some(cell)=cell {drawings.insert(cell);}
                if let Some((action,channel,previous))=frozen {if f.action==action && f.channel_frames==channel {assert_eq!(cell,previous,"pause/channel freeze");}}
                frozen=Some((f.action.clone(),f.channel_frames,cell));
                started |= world.fighters[0].action.attacking().is_some_and(|(id,_,_)| id==case.move_id);
                channels=channels.max(world.fighters[0].channel_frames);
                detonated |= world.events.iter().any(|e| e.kind==EventKind::Detonate && e.move_id==Some(MoveId::Detonate));
                hit |= matches!(world.fighters[0].action,Action::Hit {..});
                block |= matches!(world.fighters[1].action,Action::Block {..});
            }
            assert!(started,"{case:?}: legal action");
            if case.move_id==MoveId::Charge {
                match case.response {
                    Response::ChargeTap => {assert_eq!(world.fighters[0].gauge,0);assert_eq!(channels,0);},
                    Response::ChargeRelease => {assert_eq!(world.fighters[0].gauge,38);assert_eq!(channels,19);},
                    Response::ChargeMax => {for phase in 0..6 {assert!(drawings.contains(&crate::sprites::Cell::Ritual(phase)));}assert_eq!(world.fighters[0].gauge,100);assert_eq!(channels,60);},
                    Response::ChargeInterrupt => {assert!(hit);assert!(world.fighters[0].health<hp[0]);assert!((1..60).contains(&channels));},
                    _=>unreachable!(),
                }
            } else {
                assert!(detonated,"{case:?}: manual command event");
                for cell in [crate::sprites::Cell::Ritual(6),crate::sprites::Cell::Ritual(7),crate::sprites::Cell::Utility(2),crate::sprites::Cell::Ritual(5)] {assert!(drawings.contains(&cell));}
                let guard=matches!(case.response,Response::StandBlock|Response::CrouchBlock);
                assert_eq!(block,guard,"{case:?}");
                assert_eq!(hp[1]-world.fighters[1].health,if case.response==Response::Whiff {0} else if guard {12} else {90},"{case:?}: one commanded blast");
                assert!(world.projectiles.is_empty());
            }
            assert!(world.fighters.iter().all(|f|f.action.actionable()),"{case:?}: full recovery");
        }
    }

    #[test]
    fn feint_preview_covers_every_legal_special_at_first_and_last_startup_tick() {
        for body in [CharacterId::Kogan, CharacterId::Raya] {
        let all = feint_cases(body);
        assert_eq!(all.len(), if body == CharacterId::Kogan {88} else {80});
        for mv in body.data().moves.iter().filter(|m| m.feintable) {
            assert_eq!(all.iter().filter(|c| c.move_id == mv.id).count(), 8, "every legal special: {:?}", mv.id);
        }
        for case in all {
            let mut world = case.world();
            let mut seen = false;
            let mut frames = Vec::new();
            let mut feint_tick = None;
            for tick in 0..case.duration() {
                let before = world.fighters[0].clone();
                let inputs = case.inputs_for_world(tick, &world);
                world.tick(inputs[0], inputs[1]);
                if let Action::Feint { frame } = world.fighters[0].action {
                    if !seen {
                        let Action::Attack { move_id, frame: prior, .. } = before.action else { panic!("{case:?}: legal attack entry") };
                        assert_eq!(move_id, case.move_id);
                        let mv = before.data().move_def(move_id).unwrap();
                        assert_eq!(prior, if case.feint == Some(true) { 0 } else { mv.first_active() - 1 });
                        assert_eq!(world.fighters[0].last_move, Some(case.move_id));
                        feint_tick = Some(tick);
                    }
                    seen = true; frames.push(frame);
                    assert!(world.events.iter().all(|e| !matches!(e.kind, EventKind::Hit | EventKind::Grab)), "{case:?}: canceled commitment cannot contact");
                    assert!(world.projectiles.iter().all(|p| p.owner != 0), "{case:?}: canceled shot cannot release");
                }
                if seen {
                    assert!(!matches!(world.fighters[0].action, Action::Attack { .. }), "{case:?}: no unintended attack after cancel");
                }
                if seen && tick > feint_tick.unwrap() + 20 {
                    assert!(!matches!(world.fighters[0].action, Action::Feint { .. }), "{case:?}: cancel must finish");
                }
            }
            assert!(seen, "{case:?}: must reach feint through legal input");
            if case.move_id == MoveId::Charge {
                assert_eq!(world.fighters[0].gauge, 0, "startup cancel never channels");
            }
            assert_eq!(frames.first(), Some(&0));
            assert!(frames.len() <= usize::from(aeon_sim::fighter::FEINT_RECOVERY));
            assert_eq!(frames, (0..frames.len() as u16).collect::<Vec<_>>(), "{case:?}: uninterrupted phase clock until legal landing or return");
            assert!(matches!(world.fighters[0].action, Action::Stand));
            assert!(!world.fighters[0].airborne);
        }
        }
    }


    #[test]
    fn crouching_lights_preserve_contact_guard_freeze_and_control() {
        use crate::{sequences::{crouch_punch_cell, crouch_lights_cell}, sprites::Cell};
        for (body, moves) in [(CharacterId::Kogan, &[MoveId::CrP][..]),
            (CharacterId::Raya, &[MoveId::CrP, MoveId::CrK][..])] {
            let cases = normal_cases(body, moves);
            assert_eq!(cases.len(), 20 * moves.len());
            for case in cases {
                let mut world = case.world();
                let hp = world.fighters[1].health;
                let damage = world.fighters[0].data().move_def(case.move_id).unwrap().damage;
                let mut seen = std::collections::HashSet::new();
                let mut blocked = false;
                let mut frozen = None;
                for tick in 0..case.duration() {
                    let [a, b] = case.inputs_for_world(tick, &world);
                    world.tick(a, b);
                    let f = &world.fighters[0];
                    let hash = world.state_hash();
                    let cell = if body == CharacterId::Kogan { crouch_punch_cell(f) } else { crouch_lights_cell(f) };
                    assert_eq!(world.state_hash(), hash);
                    if let Some(cell) = cell { seen.insert(cell); }
                    if let Some((action, previous)) = frozen {
                        if f.action == action { assert_eq!(cell, previous, "pause/hitstop keeps its drawing"); }
                    }
                    frozen = Some((f.action.clone(), cell));
                    if let Action::Attack { move_id, frame, .. } = f.action {
                        let mv = f.data().move_def(move_id).unwrap();
                        let active = if body == CharacterId::Kogan { Cell::CrouchPunch(1) }
                            else { Cell::CrouchLights(if move_id == MoveId::CrK { 5 } else { 1 }) };
                        assert_eq!(cell == Some(active), mv.is_active(frame));
                    } else { assert_eq!(cell, None, "new actions own the body immediately"); }
                    blocked |= matches!(world.fighters[1].action, Action::Block { .. });
                }
                assert_eq!(seen.len(), 4, "{}: all four authored phases", case.label());
                let low_beats_stand = body == CharacterId::Raya && case.move_id == MoveId::CrK;
                match case.response {
                    Response::Hit | Response::CrouchHit => assert_eq!(hp - world.fighters[1].health, damage),
                    Response::StandBlock if low_beats_stand => { assert!(!blocked); assert_eq!(hp - world.fighters[1].health, damage); }
                    Response::StandBlock | Response::CrouchBlock => { assert!(blocked); assert_eq!(hp, world.fighters[1].health); }
                    Response::Whiff => { assert!(!blocked); assert_eq!(hp, world.fighters[1].health); }
                    _ => unreachable!(),
                }
                assert!(world.fighters.iter().all(|f| f.action.actionable()));
                assert!(matches!(world.fighters[0].action, Action::Crouch));
            }
        }
    }

    #[test]
    fn air_saber_preview_preserves_contact_freeze_and_landing_with_complete_early_recovery() {
        use crate::{sequences::air_saber_cell, sprites::Cell};
        for case in [CharacterId::Kogan, CharacterId::Raya].into_iter()
            .flat_map(|body| air_cases(body).into_iter().chain(early_air_cases(body)).chain(rising_air_cases(body)))
            .filter(|c| matches!(c.move_id, MoveId::JS | MoveId::JHS | MoveId::JST)) {
            let mut world = case.world();
            let hp = world.fighters[1].health;
            let mut seen = std::collections::HashSet::new();
            let mut frozen = None;
            for tick in 0..case.duration() {
                let [a, b] = case.inputs_for_world(tick, &world);
                world.tick(a, b);
                let f = &world.fighters[0];
                let cell = air_saber_cell(f);
                if let Some(cell) = cell { seen.insert(cell); }
                if let Some((frame, previous)) = frozen {
                    if world.frame == frame { assert_eq!(cell, previous, "hitstop holds the selected phase"); }
                }
                frozen = Some((world.frame, cell));
                if let Action::Attack { move_id, frame, .. } = f.action {
                    let mv = f.data().move_def(move_id).unwrap();
                    let contact = match move_id { MoveId::JS => 1, MoveId::JHS => 2, _ => 3 };
                    assert_eq!(cell == Some(Cell::AirSaber(contact)), mv.is_active(frame));
                }
                if !f.airborne { assert_eq!(cell, None, "landing immediately owns the body"); }
            }
            if matches!(case.response, Response::EarlyWhiff | Response::RisingWhiff) {
                assert_eq!(hp, world.fighters[1].health, "early fixture remains a spaced miss");
                if !case.jump.unwrap().1 && (case.body == CharacterId::Kogan || case.response == Response::RisingWhiff) {
                    assert_eq!(seen.len(), 4, "{}: gather/contact/withdraw/ready", case.label());
                    assert!(seen.contains(&Cell::AirSaber(4)) && seen.contains(&Cell::AirSaber(5)));
                }
            }
            assert!(world.fighters.iter().all(|f| !f.airborne && f.action.actionable()));
        }
    }

    #[test]
    fn air_lights_preview_preserves_contact_freeze_and_landing_with_complete_early_recovery() {
        use crate::{sequences::air_lights_cell, sprites::Cell};
        for case in [CharacterId::Kogan, CharacterId::Raya].into_iter()
            .flat_map(|body| air_cases(body).into_iter().chain(early_air_cases(body)).chain(rising_air_cases(body)))
            .filter(|c| matches!(c.move_id, MoveId::JP | MoveId::JK | MoveId::JFL)) {
            let mut world = case.world();
            let hp = world.fighters[1].health;
            let mut seen = std::collections::HashSet::new();
            let mut frozen = None;
            for tick in 0..case.duration() {
                let [a, b] = case.inputs_for_world(tick, &world);
                world.tick(a, b);
                let f = &world.fighters[0];
                let cell = air_lights_cell(f);
                if let Some(cell) = cell { seen.insert(cell); }
                if let Some((frame, previous)) = frozen {
                    if world.frame == frame { assert_eq!(cell, previous, "hitstop holds the selected phase"); }
                }
                frozen = Some((world.frame, cell));
                if let Action::Attack { move_id, frame, .. } = f.action {
                    let mv = f.data().move_def(move_id).unwrap();
                    let contact = match move_id { MoveId::JP => 1, MoveId::JK => 2, _ => 3 };
                    assert_eq!(cell == Some(Cell::AirLights(contact)), mv.is_active(frame));
                }
                if !f.airborne { assert_eq!(cell, None, "landing immediately owns the body"); }
            }
            if matches!(case.response, Response::EarlyWhiff | Response::RisingWhiff) {
                assert_eq!(hp, world.fighters[1].health, "early fixture remains a spaced miss");
                if !case.jump.unwrap().1 && (case.body == CharacterId::Kogan || case.response == Response::RisingWhiff) {
                    assert_eq!(seen.len(), 4, "{}: gather/contact/withdraw/ready", case.label());
                    assert!(seen.contains(&Cell::AirLights(4)) && seen.contains(&Cell::AirLights(5)));
                }
            }
            assert!(world.fighters.iter().all(|f| !f.airborne && f.action.actionable()));
        }
    }

    #[test]
    fn air_preview_recognizes_every_loaded_and_empty_cylinder_move() {
        use aeon_sim::Action;
        let cases = [CharacterId::Kogan, CharacterId::Raya].into_iter().flat_map(air_cases).collect::<Vec<_>>();
        assert_eq!(cases.len(), 416);
        for case in cases {
            let mut world = case.world();
            let initial_health = world.fighters[1].health;
            let mut seen = false;
            let mut blocked = false;
            for frame in 0..case.duration() {
                let inputs = case.inputs_for_world(frame, &world);
                world.tick(inputs[0], inputs[1]);
                seen |= matches!(world.fighters[0].action, Action::Attack { move_id, .. } if move_id == case.move_id);
                blocked |= matches!(world.fighters[1].action, Action::Block { .. });
            }
            assert!(seen, "{} did not recognize its legal air input", case.label());
            let damage = initial_health - world.fighters[1].health;
            match case.response {
                Response::Hit => assert!(damage > 0 && !blocked, "{}: damage {damage}, block {blocked}", case.label()),
                Response::StandBlock => assert!(blocked, "{}: damage {damage}, no guard", case.label()),
                Response::CrouchBlock if case.move_id == MoveId::AirShot => assert!(blocked, "{}: damage {damage}, no low guard", case.label()),
                Response::CrouchBlock => assert!(damage > 0 && !blocked, "{}: high attack must defeat low guard", case.label()),
                Response::Whiff => assert_eq!(damage, 0, "{}: spaced miss connected", case.label()),
                _ => unreachable!(),
            }
            assert!(world.fighters.iter().all(|f| !f.airborne && f.action.actionable()), "{}: incomplete return", case.label());
        }
    }

    #[test]
    fn judgment_preview_uses_legal_metered_input_and_covers_hit_guard_and_miss() {
        for case in judgment_cases(CharacterId::Kogan) {
            let mut w = case.world();
            let initial_x = w.fighters[0].pos.x;
            let hp = w.fighters[1].health;
            let mut super_started = false;
            let mut drawings = std::collections::HashSet::new();
            let mut frozen = None;
            let mut block = false;
            let mut down = false;
            for frame in 0..case.duration() {
                let [a, b] = case.inputs_for_world(frame, &w); w.tick(a, b);
                let f = &w.fighters[0];
                let cell = crate::sequences::judgment_cell(f);
                if let Some(cell) = cell { drawings.insert(cell); }
                if let Some((world_frame, previous)) = frozen {
                    if w.frame == world_frame { assert_eq!(cell, previous, "hitstop freezes judgment"); }
                }
                frozen = Some((w.frame, cell));
                if let Action::Attack { move_id: MoveId::Super, frame: action_frame, .. } = f.action {
                    let mv = f.data().move_def(MoveId::Super).unwrap();
                    assert_eq!(cell == Some(crate::sprites::Cell::Judgment(1)), mv.is_active(action_frame), "extended drawing only during active frames");
                }
                if matches!(w.fighters[0].action, Action::Attack { move_id: MoveId::Super, .. }) {
                    super_started = true;
                    assert!(w.fighters[0].meter < 1000, "super spends the prepared bar");
                }
                block |= matches!(w.fighters[1].action, Action::Block { .. });
                down |= matches!(w.fighters[1].action, Action::Knockdown { .. });
            }
            assert!(super_started, "{case:?}: double quarter-circle starts judgment");
            assert_eq!(drawings.len(), 4, "{case:?}: all four drawings");
            assert_eq!(crate::sequences::judgment_cell(&w.fighters[0]), None, "no art recovery delays control");
            match case.response {
                Response::Hit => assert!(down && hp - w.fighters[1].health == 280, "{case:?}"),
                Response::StandBlock | Response::CrouchBlock => assert!(block && !down && hp - w.fighters[1].health == 24, "{case:?}"),
                Response::Whiff => assert!(w.fighters[1].health == hp && !down && !block, "{case:?}"),
                _ => unreachable!(),
            }
            assert!(w.fighters[0].pos.x != initial_x, "rush advances at authored velocity");
            assert!(w.fighters.iter().all(|f| f.action.actionable()), "{case:?}: both recover");
        }
    }

    #[test]
    fn convergence_preview_uses_legal_metered_input_and_covers_hit_guard_and_miss() {
        let cases = judgment_cases(CharacterId::Raya);
        assert_eq!(cases.len(), 20);
        for case in cases {
            let mut w = case.world();
            let initial_x = w.fighters[0].pos.x;
            let hp = w.fighters[1].health;
            let mut seen = false;
            let mut block = false;
            let mut down = false;
            let mut drawings = std::collections::HashSet::new();
            let mut frozen = None;
            for tick in 0..case.duration() {
                let [a,b] = case.inputs_for_world(tick,&w); w.tick(a,b);
                let f = &w.fighters[0];
                let cell = crate::sequences::judgment_cell(f);
                if let Some(cell) = cell { drawings.insert(cell); }
                if let Some((world_frame, previous)) = frozen {
                    if w.frame == world_frame { assert_eq!(cell,previous,"super hitstop holds the drawing"); }
                }
                frozen = Some((w.frame,cell));
                if let Action::Attack {move_id:MoveId::Super,frame,..}=f.action {
                    assert_eq!(cell==Some(crate::sprites::Cell::Judgment(1)),f.data().move_def(MoveId::Super).unwrap().is_active(frame));
                }
                if matches!(w.fighters[0].action, Action::Attack { move_id: MoveId::Super, .. }) {
                    seen = true;
                    assert!(w.fighters[0].meter < 1000, "super spends the prepared bar");
                }
                block |= matches!(w.fighters[1].action, Action::Block { .. });
                down |= matches!(w.fighters[1].action, Action::Knockdown { .. });
            }
            assert!(seen, "{case:?}: double quarter-circle starts convergence");
            assert_eq!(drawings.len(),4,"gather, expansion, dismissal and ready");
            assert_eq!(crate::sequences::judgment_cell(&w.fighters[0]),None,"control immediately clears the super drawing");
            match case.response {
                Response::Hit | Response::CrouchHit => assert!(down && hp-w.fighters[1].health==340, "{case:?}"),
                Response::StandBlock | Response::CrouchBlock => assert!(block && !down && hp-w.fighters[1].health==28, "{case:?}"),
                Response::Whiff => assert!(w.fighters[1].health==hp && !down && !block, "{case:?}"),
                _ => unreachable!(),
            }
            assert_ne!(w.fighters[0].pos.x, initial_x, "authored rush travel");
            assert!(w.fighters.iter().all(|f| f.action.actionable()), "{case:?}: full return");
        }
    }

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
                let base = if case.jump.unwrap().1 { 1 } else { 4 };
                for cell in [0, base, base + 1, base + 2] {
                    assert!(drawings.contains(&crate::sprites::Cell::Movement(cell)), "{case:?} cell {cell}");
                }
                assert_eq!(drawings.contains(&crate::sprites::Cell::Movement(7)), !case.jump.unwrap().1,
                    "only the authored full-jump landing displays compression");
            }
        }
    }

    #[test]
    fn reaction_preview_covers_grounded_guard_recoil_launch_and_floor_recovery() {
        for case in [CharacterId::Kogan, CharacterId::Raya].into_iter().flat_map(reaction_cases) {
            let mut w = case.world();
            let hp = w.fighters[1].health;
            let mut hit = false; let mut block = false; let mut thrown = false;
            let mut rising = false; let mut falling = false; let mut down = false;
            let mut getup = std::collections::HashSet::new();
            let mut recoil = std::collections::HashSet::new();
            let mut frozen = None;
            for frame in 0..case.duration() {
                let [a, b] = case.inputs_for_world(frame, &w); w.tick(a, b);
                let f = &w.fighters[1];
                hit |= matches!(f.action, Action::Hit { .. });
                block |= matches!(f.action, Action::Block { .. });
                thrown |= matches!(f.action, Action::Thrown { .. });
                rising |= f.airborne && f.vel.y > 0;
                falling |= f.airborne && f.vel.y < 0;
                down |= matches!(f.action, Action::Knockdown { .. });
                let cell = crate::sequences::recoil_cell(f);
                if let Some(cell) = cell { recoil.insert(cell); }
                if let Some((world_frame, previous)) = frozen {
                    if w.frame == world_frame { assert_eq!(cell, previous, "hitstop holds recoil"); }
                }
                frozen = Some((w.frame, cell));
                let hash = w.state_hash();
                for _ in 0..4 { let _ = crate::sequences::recoil_cell(f); let _ = crate::sequences::floor_cell(f); }
                assert_eq!(w.state_hash(), hash, "presentation cannot mutate the simulation");
                if matches!(f.action, Action::Getup { .. }) {
                    getup.insert(crate::sequences::floor_cell(f).or_else(|| crate::sequences::cell_for(f)).unwrap());
                }
            }
            match case.response {
                Response::StandBlock | Response::CrouchBlock => {
                    assert!(block && !hit && w.fighters[1].health == hp, "{case:?}: guarded without damage");
                }
                _ => assert!(w.fighters[1].health < hp && (hit || down), "{case:?}: real consequence"),
            }
            let pair = match case.response {
                Response::StandBlock => Some(4), Response::CrouchBlock => Some(6),
                Response::CrouchHit => Some(2),
                Response::Hit if matches!(case.move_id, MoveId::StP | MoveId::StS) => Some(0),
                _ => None,
            };
            if let Some(first) = pair {
                assert!(recoil.contains(&crate::sprites::Cell::Recoil(first))
                    && recoil.contains(&crate::sprites::Cell::Recoil(first + 1)), "{case:?}: impact and release {recoil:?}");
            }
            if matches!(case.move_id, MoveId::Uppercut | MoveId::CrST | MoveId::Throw | MoveId::CommandGrab) {
                assert!(down && getup.len() == 4, "{case:?}: full floor recovery {getup:?}");
            }
            if matches!(case.move_id, MoveId::Uppercut | MoveId::Throw | MoveId::CommandGrab) {
                assert!(rising && falling, "{case:?}: complete launch");
            }
            if matches!(case.move_id, MoveId::Throw | MoveId::CommandGrab) { assert!(thrown, "{case:?}"); }
            assert!(w.fighters.iter().all(|f| f.action.actionable()), "{case:?}: both recover");
        }
    }

    #[test]
    fn ground_preview_preserves_immediate_run_exits_and_complete_backdash() {
        for case in [CharacterId::Kogan, CharacterId::Raya].into_iter().flat_map(ground_cases) {
            let ground = case.ground.unwrap();
            let mut world = case.world();
            let mut run = false; let mut blocked = false; let mut hit = false;
            let mut backdash = 0; let mut walk = 0; let mut crouch = false;
            let mut jumped = false; let mut landing = 0;
            for frame in 0..case.duration() {
                let [p1, p2] = case.inputs_for_world(frame, &world);
                world.tick(p1, p2);
                let f = &world.fighters[0];
                run |= f.action == Action::Run;
                blocked |= matches!(f.action, Action::Block { .. });
                backdash += usize::from(matches!(f.action, Action::BackDash { .. }));
                walk += usize::from(matches!(f.action, Action::Walk { .. }));
                crouch |= f.action == Action::Crouch;
                jumped |= matches!(f.action, Action::Jump { hop: false, .. });
                landing += usize::from(matches!(f.action, Action::Landing { total: 2, .. }));
                hit |= world.events.iter().any(|e| e.attacker == 0 && matches!(e.kind, EventKind::Hit | EventKind::Punish));
                if frame == 40 {
                    match ground {
                        Ground::RunStop => assert_eq!(f.action, Action::Stand, "{case:?}: stop immediately"),
                        Ground::RunCrouch => assert_eq!(f.action, Action::Crouch, "{case:?}: crouch immediately"),
                        Ground::RunBlock => assert_eq!(f.action, Action::Walk { forward: false }, "{case:?}: guard input immediately"),
                        Ground::RunJump => assert!(matches!(f.action, Action::Prejump { frame: 0, .. }), "{case:?}"),
                        Ground::RunAttack => assert!(f.action.attacking().is_some_and(|(id, f, _)| id == MoveId::StS && f == 0), "{case:?}"),
                        _ => {},
                    }
                }
            }
            match ground {
                Ground::WalkForward | Ground::WalkBack => assert!(walk >= 48, "{case:?}"),
                Ground::Crouch => assert!(crouch, "{case:?}"),
                Ground::BackDash => assert_eq!(backdash, aeon_sim::fighter::BACKDASH_FRAMES as usize, "{case:?}"),
                Ground::RunBlock => assert!(run && blocked, "{case:?}"),
                Ground::RunJump => assert!(run && jumped && landing == 2, "{case:?}: full jump landing {landing}"),
                Ground::RunAttack => assert!(run && hit, "{case:?}"),
                _ => assert!(run, "{case:?}"),
            }
            assert!(world.fighters.iter().all(|f| f.action.actionable()), "{case:?}: fully recovered");
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
        for case in saber_cases(CharacterId::Kogan).into_iter().filter(|c| c.move_id == MoveId::Uppercut) {
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
        for case in saber_cases(CharacterId::Kogan).into_iter().filter(|c| c.move_id == MoveId::Rekka3) {
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
        for case in [CharacterId::Kogan, CharacterId::Raya].into_iter().flat_map(saber_cases) {
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
            let crouched = matches!(case.response, Response::CrouchBlock | Response::CrouchHit);
            let passes_above = crouched && (case.move_id == MoveId::Uppercut
                || case.body == CharacterId::Kogan && case.move_id == MoveId::StS);
            if case.response == Response::Whiff || passes_above {
                assert_eq!((hits, blocks), (0, 0), "{case:?}");
            } else {
                assert!(target_contact, "{case:?} target contact");
                if matches!(case.response, Response::Hit | Response::CrouchHit) { assert!(hits > 0 && blocks == 0, "{case:?}"); }
                else { assert!(blocks > 0 && hits == 0, "{case:?}"); }
            }
            if case.move_id == MoveId::Uppercut { assert!(air && landing, "{case:?} reversal landing"); }
            assert!(world.fighters.iter().all(|f| f.action.actionable()), "{case:?} full recovery");
        }
    }

    #[test]
    fn utility_preview_exercises_snare_evasion_and_step_without_new_mechanics() {
        for case in [CharacterId::Kogan, CharacterId::Raya].into_iter().flat_map(utility_cases) {
            let mut world = case.world();
            let start_x = world.fighters[0].pos.x;
            let defender_x = world.fighters[1].pos.x;
            let hp = world.fighters[1].health;
            let mut frozen = None;
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
                let cell = crate::sequences::utility_cell(&world.fighters[0]);
                if let Some(cell) = cell { drawings.insert(cell); }
                if let Some((frame, previous)) = frozen {
                    if world.frame == frame { assert_eq!(cell, previous, "freeze holds utility drawing"); }
                }
                frozen = Some((world.frame, cell));
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
            let damage = if case.body == CharacterId::Raya { 180 } else { case.body.data().move_def(MoveId::CommandGrab).unwrap().damage };
            assert_eq!(hp-world.fighters[1].health, if connects { damage } else { 0 }, "{case:?}: original damage");
            if case.move_id == MoveId::CommandDash {
                let limit = if case.body == CharacterId::Kogan { px(60) } else { px(112) };
                assert!(travel > 0 && travel <= limit, "{case:?} authored travel");
                if case.response == Response::Whiff { assert_eq!(travel, limit, "{case:?}"); }
                if case.body == CharacterId::Raya && case.response == Response::Hit && !case.corner {
                    assert_ne!((start_x-defender_x).signum(), (world.fighters[0].pos.x-defender_x).signum(), "processional passes through the body");
                }
            }
        }
    }

    #[test]
    fn raya_ranged_preview_uses_real_glyphs_and_armed_crystals() {
        let cases = ranged_cases(CharacterId::Raya);
        assert_eq!(cases.len(), 64);
        for case in cases {
            let mut world = case.world();
            let hp = world.fighters[1].health;
            let mut started = false;
            let mut spawned = false;
            let mut armed = false;
            let mut hits = 0;
            let mut blocks = 0;
            let mut drawings = std::collections::HashSet::new();
            let mut frozen = None;
            for tick in 0..case.duration() {
                let [a,b] = case.inputs_for_world(tick, &world);
                world.tick(a,b);
                let cell = crate::sequences::ranged_cell(&world.fighters[0]);
                if let Some(cell) = cell { drawings.insert(cell); }
                if let Some((frame,previous)) = frozen { if world.frame == frame { assert_eq!(cell,previous,"freeze holds release drawing"); } }
                frozen = Some((world.frame,cell));
                started |= world.fighters[0].action.attacking().is_some_and(|(id,_,_)| id == case.move_id);
                spawned |= !world.projectiles.is_empty();
                for e in &world.events {
                    armed |= e.kind == EventKind::Armed;
                    hits += usize::from(matches!(e.kind, EventKind::Hit | EventKind::Knockdown | EventKind::Punish));
                    blocks += usize::from(e.kind == EventKind::Block);
                }
            }
            let base = if matches!(case.move_id, MoveId::ShotA | MoveId::ExB) { 0 } else { 4 };
            for phase in 0..4 { assert!(drawings.contains(&crate::sprites::Cell::Ranged(base+phase)), "{case:?}: every drawn phase"); }
            let miss = case.response == Response::Whiff;
            let guard = matches!(case.response, Response::StandBlock | Response::CrouchBlock);
            assert!(started && (spawned || hits + blocks > 0), "{case:?}: legal release");
            let contacts = if matches!(case.move_id, MoveId::ShotA | MoveId::ExB) { 2 } else { 1 };
            assert_eq!((hits,blocks), if miss {(0,0)} else if guard {(0,contacts)} else {(contacts,0)}, "{case:?}");
            let def = case.body.data().move_def(case.move_id).unwrap().projectile.unwrap();
            assert_eq!(hp-world.fighters[1].health, if miss {0} else if guard {def.chip * contacts as i32} else {def.damage + if contacts == 2 { def.damage * 80 / 100 } else { 0 }}, "{case:?}: unchanged damage");
            assert_eq!(world.fighters[0].gauge, 0, "{case:?}: original gauge cost");
            assert_eq!(armed, matches!(case.move_id, MoveId::ShotA | MoveId::ExB), "{case:?}: crystal must plant and arm");
            assert!(world.projectiles.is_empty(), "{case:?}: complete contact or expiry");
            assert!(world.fighters.iter().all(|f| f.action.actionable()), "{case:?}: complete recovery");
        }
    }

    #[test]
    fn ranged_preview_exercises_release_guard_whiff_and_recovery() {
        for case in ranged_cases(CharacterId::Kogan) {
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
    fn throw_preview_checks_both_guards_jump_escape_and_early_late_techs() {
        let cases: Vec<_> = [CharacterId::Kogan, CharacterId::Raya].into_iter().flat_map(throw_cases).collect();
        assert_eq!(cases.len(), 64);
        for case in cases {
            let mut world = case.world();
            let hp = world.fighters[1].health;
            let (mut started, mut grabs, mut throws, mut techs) = (false, 0, 0, 0);
            let mut tech_frames = [Vec::new(), Vec::new()];
            let mut tech_drawings = std::collections::HashSet::new();
            let mut drawings = std::collections::HashSet::new();
            let mut frozen = None;
            for tick in 0..case.duration() {
                let [a, b] = case.inputs_for_world(tick, &world);
                world.tick(a, b);
                started |= world.fighters[0].action.attacking().is_some_and(|(id, _, _)| id == MoveId::Throw);
                let f = &world.fighters[0];
                if let Some(cell) = crate::sequences::utility_cell(f) { drawings.insert(cell); }
                if let Some(cell) = crate::sequences::throw_tech_cell(f) { tech_drawings.insert(cell); }
                let cells = world.fighters.each_ref().map(|f| crate::sequences::throw_tech_cell(f).or_else(|| crate::sequences::utility_cell(f)));
                if let Some((frame, previous)) = frozen {
                    if world.frame == frame { assert_eq!(cells, previous, "freeze holds both throw drawings"); }
                }
                frozen = Some((world.frame, cells));
                if let Action::Attack { move_id: MoveId::Throw, frame, connected } = f.action {
                    let release = if connected == aeon_sim::Connect::Hit { 9 } else { 3 };
                    let reach = if case.body == CharacterId::Raya { crate::sprites::Cell::ThrowContact } else { crate::sprites::Cell::Utility(1) };
                    assert_eq!(crate::sequences::utility_cell(f) == Some(reach), (2..release).contains(&frame), "{case:?}: reach holds exactly until release");
                }
                for event in &world.events {
                    grabs += usize::from(event.kind == EventKind::Grab);
                    throws += usize::from(event.kind == EventKind::Throw);
                    techs += usize::from(event.kind == EventKind::ThrowTech);
                }
                for (f, frames) in world.fighters.iter().zip(&mut tech_frames) {
                    if let Action::ThrowTech { frame } = f.action { frames.push(frame); }
                }
            }
            assert!(started, "{case:?}: legal P+K");
            let tech = matches!(case.response, Response::TechEarly | Response::TechLate);
            let miss = matches!(case.response, Response::Whiff | Response::AirEvade);
            if !tech {
                for phase in 0..4 {
                    let cell = if case.body == CharacterId::Raya && phase == 1 { crate::sprites::Cell::ThrowContact } else { crate::sprites::Cell::Utility(phase) };
                    assert!(drawings.contains(&cell), "{case:?}: all throw phases");
                }
            }
            assert_eq!((grabs, throws, techs), if tech { (1,0,1) } else if miss { (0,0,0) } else { (1,1,0) }, "{case:?}");
            assert_eq!(hp - world.fighters[1].health, if tech || miss { 0 } else { case.body.data().move_def(MoveId::Throw).unwrap().damage }, "{case:?}");
            if tech {
                let phases = if case.body == CharacterId::Raya {
                    [crate::sprites::Cell::Recoil(4), crate::sprites::Cell::Recoil(5), crate::sprites::Cell::Utility(3)]
                } else { [crate::sprites::Cell::ThrowTech(0), crate::sprites::Cell::ThrowTech(1), crate::sprites::Cell::Utility(3)] };
                assert_eq!(tech_drawings, phases.into_iter().collect(), "{case:?}: every separation phase");
            }
            for frames in tech_frames {
                assert_eq!(frames, if tech { (1..16).collect() } else { Vec::new() }, "{case:?}: original separation duration");
            }
            assert!(world.fighters.iter().all(|f| !f.airborne && f.action.actionable()), "{case:?}: full return");
        }
    }

    #[test]
    fn raya_overhead_preview_preserves_high_guard_and_grounded_recovery() {
        use crate::{sequences::overhead_cell, sprites::Cell};
        let cases = overhead_cases(CharacterId::Raya);
        assert_eq!(cases.len(), 20);
        for case in cases {
            let mut world = case.world();
            let hp = world.fighters[1].health;
            let (mut started, mut hits, mut blocks) = (false, 0, 0);
            let mut seen = std::collections::HashSet::new();
            let mut frozen = None;
            for tick in 0..case.duration() {
                let [a, b] = case.inputs_for_world(tick, &world);
                world.tick(a, b);
                let f = &world.fighters[0];
                let cell = overhead_cell(f);
                if let Some(cell) = cell { seen.insert(cell); }
                if let Some((frame, previous)) = frozen {
                    if world.frame == frame { assert_eq!(cell, previous, "freeze holds the drawing"); }
                }
                frozen = Some((world.frame, cell));
                if let Action::Attack { move_id, frame, .. } = f.action {
                    let mv = f.data().move_def(move_id).unwrap();
                    assert_eq!(cell == Some(Cell::Overhead(2)), mv.is_active(frame), "{case:?}: active-only downstroke");
                }
                started |= f.action.attacking().is_some_and(|(id, _, _)| id == MoveId::Overhead);
                assert!(!f.airborne, "{case:?}: grounded overhead");
                hits += world.events.iter().filter(|e| matches!(e.kind, EventKind::Hit | EventKind::Knockdown)).count();
                blocks += world.events.iter().filter(|e| e.kind == EventKind::Block).count();
            }
            for phase in 0..6 { assert!(seen.contains(&Cell::Overhead(phase)), "{case:?}: phase {phase}"); }
            let expected = match case.response { Response::Whiff => (0, 0), Response::StandBlock => (0, 1), _ => (1, 0) };
            assert!(started, "{case:?}: legal HS+ST chord");
            assert_eq!((hits, blocks), expected, "{case:?}");
            assert_eq!(hp-world.fighters[1].health, if expected.0==1 { 100 } else { 0 }, "{case:?}: authored damage");
            assert!(world.fighters.iter().all(|f| !f.airborne && f.action.actionable()), "{case:?}: full recovery");
        }
    }

    #[test]
    fn overhead_preview_uses_legal_chord_and_motion_with_high_guard_and_landing() {
        use crate::{sequences::{air_saber_cell, overhead_cell, cell_for}, sprites::Cell};
        let cases = overhead_cases(CharacterId::Kogan);
        assert_eq!(cases.len(), 40);
        for case in cases {
            let mut world = case.world();
            let (mut started, mut airborne, mut knocked_down) = (false, false, false);
            let (mut hits, mut blocks) = (0, 0);
            let mut landing = Vec::new();
            let mut seen = std::collections::HashSet::new();
            let mut frozen = None;
            for tick in 0..case.duration() {
                let [a, b] = case.inputs_for_world(tick, &world);
                world.tick(a, b);
                let f = &world.fighters[0];
                let cell = overhead_cell(f).or_else(|| air_saber_cell(f)).or_else(|| cell_for(f));
                if let Some(cell) = cell { seen.insert(cell); }
                if let Some((frame, previous)) = frozen {
                    if world.frame == frame { assert_eq!(cell, previous, "freeze holds the drawing"); }
                }
                frozen = Some((world.frame, cell));
                if let Action::Attack { move_id, frame, .. } = f.action {
                    let mv = f.data().move_def(move_id).unwrap();
                    let contact = if move_id == MoveId::Overhead { Cell::Overhead(1) } else { Cell::AirSaber(3) };
                    assert_eq!(cell == Some(contact), mv.is_active(frame), "{case:?}: active-only commitment");
                }
                if !f.airborne { assert_eq!(air_saber_cell(f), None); }
                started |= f.action.attacking().is_some_and(|(id, _, _)| id == case.move_id);
                airborne |= f.airborne;
                if let Action::Landing { frame, total } = f.action {
                    let remaining = total - frame;
                    if landing.last() != Some(&remaining) { landing.push(remaining); }
                }
                hits += world.events.iter().filter(|e| matches!(e.kind, EventKind::Hit | EventKind::Knockdown)).count();
                blocks += world.events.iter().filter(|e| e.kind == EventKind::Block).count();
                knocked_down |= matches!(world.fighters[1].action, Action::Hit { knockdown: true, .. } | Action::Knockdown { .. });
            }
            assert!(started, "{case:?}: legal recognition");
            let expected = match case.response { Response::Whiff => (0, 0), Response::StandBlock => (0, 1), _ => (1, 0) };
            assert_eq!((hits, blocks), expected, "{case:?}");
            let leaping = case.move_id == MoveId::SpecialOverhead;
            assert_eq!(airborne, leaping, "{case:?}: original lift");
            assert_eq!(knocked_down, leaping && expected.0 == 1, "{case:?}");
            if leaping { assert_eq!(landing, vec![8,7,6,5,4,3,2,1], "{case:?}: authored landing tax"); }
            if leaping {
                for c in 8..12 { assert!(seen.contains(&Cell::Reaction(c)), "{case:?}: full landing sequence"); }
            } else {
                for c in 0..4 { assert!(seen.contains(&Cell::Overhead(c)), "{case:?}: complete grounded gesture"); }
            }
            assert!(world.fighters.iter().all(|f| !f.airborne && f.action.actionable()), "{case:?}: complete recovery");
        }
    }

    #[test]
    fn crouching_saber_preview_preserves_low_guard_rules_and_supported_recovery() {
        let cases = [CharacterId::Kogan, CharacterId::Raya].into_iter().flat_map(crouching_saber_cases).collect::<Vec<_>>();
        assert_eq!(cases.len(), 160);
        for case in cases {
            let mut world = case.world();
            let mut started = false;
            let (mut hits, mut blocks, mut knockdown) = (0, 0, false);
            for tick in 0..case.duration() {
                let [a, b] = case.inputs_for_world(tick, &world);
                world.tick(a, b);
                started |= world.fighters[0].action.attacking()
                    .is_some_and(|(id, _, _)| id == case.move_id);
                hits += world.events.iter().filter(|e| matches!(e.kind, EventKind::Hit | EventKind::Knockdown)).count();
                blocks += world.events.iter().filter(|e| e.kind == EventKind::Block).count();
                knockdown |= matches!(world.fighters[1].action, Action::Hit { knockdown: true, .. } | Action::Knockdown { .. });
            }
            assert!(started, "{case:?}: recognized through legal crouching input");
            let expected = match case.response {
                Response::Whiff => (0, 0), Response::CrouchBlock => (0, 1),
                Response::StandBlock if case.move_id != MoveId::CrST => (0, 1),
                _ => (1, 0),
            };
            assert_eq!((hits, blocks), expected, "{case:?}");
            assert_eq!(knockdown, case.move_id == MoveId::CrST && expected.0 == 1, "{case:?}");
            assert!(matches!(world.fighters[0].action, Action::Crouch), "{case:?}: supported crouched return");
            assert!(world.fighters.iter().all(|f| !f.airborne && f.action.actionable()), "{case:?}: full defender recovery");
        }
    }

    #[test]
    fn flash_preview_reaches_both_mid_normals_and_recovers_after_contact() {
        let cases = [CharacterId::Kogan, CharacterId::Raya].into_iter().flat_map(flash_cases).collect::<Vec<_>>();
        assert_eq!(cases.len(), 80);
        for case in cases {
            let mut world = case.world();
            let mut started = false;
            let mut held = None;
            let mut seen = std::collections::HashSet::new();
            let (mut hits, mut blocks) = (0, 0);
            for tick in 0..case.duration() {
                let [a, b] = case.inputs_for_world(tick, &world);
                world.tick(a, b);
                let cell = crate::sequences::flash_cell(&world.fighters[0]);
                if let Some(cell) = cell { seen.insert(cell); }
                if let Some((frame, previous)) = held {
                    if frame == world.frame { assert_eq!(cell, previous, "frozen contact holds"); }
                }
                held = Some((world.frame, cell));
                started |= world.fighters[0].action.attacking()
                    .is_some_and(|(id, _, _)| id == case.move_id);
                hits += world.events.iter().filter(|e| e.kind == EventKind::Hit).count();
                blocks += world.events.iter().filter(|e| e.kind == EventKind::Block).count();
            }
            assert_eq!(seen.len(), 4, "{case:?}: all four drawn phases occur");
            assert!(started, "{case:?}: recognized through legal inputs");
            let expected = match case.response {
                Response::Whiff => (0, 0),
                Response::StandBlock | Response::CrouchBlock => (0, 1),
                _ => (1, 0),
            };
            assert_eq!((hits, blocks), expected, "{case:?}");
            assert!(world.fighters.iter().all(|f| !f.airborne && f.action.actionable()), "{case:?}: legal return");
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
                let mut seen = std::collections::HashSet::new();
                let mut held = None;
                for frame in 0..LENGTH {
                    let [p1, p2] = case.inputs_for_world(frame, &world);
                    world.tick(p1, p2);
                    let cell = crate::sequences::standing_lights_cell(&world.fighters[0]);
                    if let Some(cell) = cell { seen.insert(cell); }
                    if let Some((tick, previous)) = held {
                        if tick == world.frame { assert_eq!(cell, previous, "hitstop freezes the drawn phase"); }
                    }
                    held = Some((world.frame, cell));
                    started |= world.fighters[0].action.attacking()
                        .is_some_and(|(id, _, _)| id == case.move_id);
                    for event in &world.events {
                        hits += usize::from(event.kind == EventKind::Hit);
                        blocks += usize::from(event.kind == EventKind::Block);
                    }
                    crouched_hit |= matches!(world.fighters[1].action, Action::Hit { .. })
                        && world.fighters[1].input().down();
                }
                assert_eq!(seen.len(), if case.move_id == MoveId::StP || (body == CharacterId::Raya && case.move_id == MoveId::StK) { 4 } else { 0 },
                    "{case:?}: four supported light phases appear;unmodified moves retain their selector");
                assert_eq!(crate::sequences::standing_lights_cell(&world.fighters[0]), None, "no extra art recovery");
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
