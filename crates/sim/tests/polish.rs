//! September consultation: free run transitions, strict links, and jump flow.
mod common;

use aeon_sim::fighter::{HOP_LANDING_RECOVERY, LANDING_RECOVERY};
use aeon_sim::{px, Action, CharacterId, Connect, Fighter, MoveId, World};
use common::*;

const BODIES: [CharacterId; 2] = [CharacterId::Kogan, CharacterId::Raya];

fn isolated(id: CharacterId, facing: bool) -> World {
    let mut w = World::new(id, id);
    w.fighters[0] = Fighter::spawn(id, px(if facing { 160 } else { 600 }), facing);
    w.fighters[1].pos.x = px(if facing { 600 } else { 160 });
    w
}

fn airborne(id: CharacterId, facing: bool, hop: bool) -> World {
    let mut w = isolated(id, facing);
    w.tick(dir(9), idle());
    for _ in 0..4 {
        w.tick(if hop { idle() } else { dir(9) }, idle());
    }
    assert!(w.fighters[0].airborne);
    assert_eq!(w.fighters[0].hop, hop);
    w
}

#[test]
fn run_immediately_stops_blocks_crouches_attacks_or_jumps() {
    use aeon_sim::{Btn, HitLevel};
    for id in BODIES {
        for facing in [true, false] {
            for input in [
                idle(),
                back(),
                down_back(),
                dir(3),
                press(Btn::S),
                dir(8),
                dir(7),
            ] {
                let mut w = isolated(id, facing);
                w.tick(dir(6), idle());
                w.tick(idle(), idle());
                w.tick(dir(6), idle());
                assert_eq!(w.fighters[0].action, Action::Run);
                w.tick(input, idle());
                let f = &w.fighters[0];
                if input.buttons.s {
                    assert!(attacking(&w, 0, MoveId::StS));
                } else if input.up() {
                    assert!(matches!(f.action, Action::Prejump { .. }));
                    assert_eq!(f.vel.x, 0);
                } else if input.down() {
                    assert_eq!(f.action, Action::Crouch);
                    assert_eq!(f.vel.x, 0);
                } else if input.dir == 5 {
                    assert_eq!(f.action, Action::Stand);
                    assert_eq!(f.vel.x, 0);
                }
                if input.back() && !input.up() {
                    assert!(f.would_block(if input.down() {
                        HitLevel::Low
                    } else {
                        HitLevel::High
                    }));
                }
            }
        }
    }
}

#[test]
fn air_normals_preserve_travel_and_hop_identity_through_recovery() {
    use aeon_sim::Btn;
    for id in BODIES {
        for facing in [true, false] {
            for hop in [true, false] {
                for button in [Btn::P, Btn::K, Btn::S, Btn::HS, Btn::FL, Btn::ST] {
                    let mut w = airborne(id, facing, hop);
                    let speed = w.fighters[0].vel.x;
                    let x = w.fighters[0].pos.x;
                    w.tick(press(button), idle());
                    assert!(w.fighters[0].action.attacking().is_some());
                    assert_eq!(w.fighters[0].vel.x, speed, "{id:?} {button:?}");
                    assert_eq!(w.fighters[0].pos.x, x + speed);
                    assert_eq!(w.fighters[0].hop, hop);
                    for _ in 0..60 {
                        if !w.fighters[0].airborne {
                            break;
                        }
                        assert_eq!(w.fighters[0].hop, hop);
                        w.tick(idle(), idle());
                    }
                    assert!(!w.fighters[0].airborne);
                    if hop {
                        assert!(w.fighters[0].action.actionable());
                    } else {
                        assert!(matches!(w.fighters[0].action, Action::Landing { total, .. }
                            if total == u16::from(LANDING_RECOVERY)));
                    }
                }
            }
        }
    }
}

#[test]
fn hop_landing_accepts_a_ground_button_and_full_jump_owes_recovery() {
    use aeon_sim::Btn;
    assert_eq!(HOP_LANDING_RECOVERY, 0);
    for id in BODIES {
        for hop in [true, false] {
            let mut w = airborne(id, true, hop);
            while w.fighters[0].pos.y + w.fighters[0].vel.y - w.fighters[0].data().gravity > 0 {
                w.tick(idle(), idle());
            }
            w.tick(dir_press(2, Btn::K), idle());
            if hop {
                assert!(attacking(&w, 0, MoveId::CrK), "ground input on touchdown");
            } else {
                assert!(matches!(w.fighters[0].action, Action::Landing { .. }));
                w.tick(idle(), idle());
                w.tick(press(Btn::P), idle());
                assert!(
                    attacking(&w, 0, MoveId::StP),
                    "input on first free landing frame"
                );
            }
        }
    }
}

#[test]
fn uppercuts_keep_their_authored_landing_tax() {
    for id in BODIES {
        let mut w = isolated(id, true);
        let tax = id.data().move_def(MoveId::Uppercut).unwrap().land_recovery;
        w.fighters[0].start_move(MoveId::Uppercut);
        assert!(!w.fighters[0].hop);
        run_until(&mut w, 120, idle(), idle(), |w| !w.fighters[0].airborne).unwrap();
        assert!(
            matches!(w.fighters[0].action, Action::Landing { total, .. } if total == u16::from(tax))
        );
    }
}

#[test]
fn hop_touchdown_flash_is_a_ground_normal_without_spending_air_gun_gauge() {
    use aeon_sim::Btn;
    let mut w = airborne(CharacterId::Kogan, true, true);
    while w.fighters[0].pos.y + w.fighters[0].vel.y - w.fighters[0].data().gravity > 0 {
        w.tick(idle(), idle());
    }
    let gauge = w.fighters[0].gauge;
    w.tick(press(Btn::FL), idle());
    assert!(attacking(&w, 0, MoveId::StFL));
    assert_eq!(w.fighters[0].gauge, gauge);
}

#[test]
fn landing_does_not_erase_air_hitstun() {
    for id in BODIES {
        let mut w = airborne(id, true, true);
        w.fighters[0].pos.y = 1;
        w.fighters[0].vel.y = -px(1);
        w.fighters[0].apply_hit(15, false, 0, 0, false);
        w.tick(idle(), idle());
        assert!(!w.fighters[0].airborne);
        assert_eq!(
            w.fighters[0].action,
            Action::Hit {
                stun: 14,
                knockdown: false
            }
        );
    }
}

#[test]
fn first_free_frame_accepts_a_link_but_an_early_press_is_not_buffered() {
    use aeon_sim::Btn;
    for id in BODIES {
        let end = id.data().move_def(MoveId::StP).unwrap().total_frames();
        for early in [false, true] {
            let mut w = isolated(id, true);
            w.fighters[0].action = Action::Attack {
                move_id: MoveId::StP,
                frame: end - if early { 2 } else { 1 },
                connected: Connect::Hit,
            };
            w.tick(press(Btn::P), idle());
            if early {
                w.tick(idle(), idle());
                assert_eq!(w.fighters[0].action, Action::Stand);
            } else {
                assert!(matches!(
                    w.fighters[0].action,
                    Action::Attack {
                        move_id: MoveId::StP,
                        frame: 0,
                        ..
                    }
                ));
            }
        }
    }
}

#[test]
fn wakeup_and_blockstun_expiry_accept_the_current_input() {
    use aeon_sim::{Btn, GETUP_FRAMES};
    for id in BODIES {
        for action in [
            Action::Getup {
                frame: GETUP_FRAMES - 1,
            },
            Action::Block {
                crouching: false,
                stun: 0,
            },
            Action::Hit {
                stun: 0,
                knockdown: false,
            },
        ] {
            let mut w = isolated(id, true);
            w.fighters[0].action = action;
            w.tick(press(Btn::P), idle());
            assert!(attacking(&w, 0, MoveId::StP));
        }
    }
}

#[test]
fn kogan_link_into_full_rekka_is_a_natural_five_hit_route() {
    use aeon_sim::Btn;
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    let health = w.fighters[1].health;
    w.tick(press(Btn::P), idle());
    run_until(&mut w, 50, idle(), idle(), |w| {
        w.fighters[0].action.actionable()
    })
    .unwrap();
    w.tick(press(Btn::P), idle());
    for d in [2, 3, 6] {
        w.tick(dir(d), idle());
    }
    run_until(&mut w, 20, dir(6), idle(), |w| w.fighters[1].combo == 2).unwrap();
    w.tick(dir_press(6, Btn::S), idle());
    for combo in [3, 4] {
        run_until(&mut w, 50, idle(), idle(), |w| w.fighters[1].combo == combo).unwrap();
        w.tick(press(Btn::S), idle());
    }
    run_until(&mut w, 50, idle(), idle(), |w| w.fighters[1].combo == 5).unwrap();
    assert!(attacking(&w, 0, MoveId::Rekka3));
    assert!(matches!(
        w.fighters[1].action,
        Action::Hit {
            knockdown: true,
            ..
        }
    ));
    assert_eq!(health - w.fighters[1].health, 170);
}
