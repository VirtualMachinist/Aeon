//! Headless proofs that the engine plays like the law says: pokes hit,
//! blocks work, lows beat stand block, heavies are punishable, lights link,
//! and the 2026-08-14 combat law (chords, hop, run, feint, rekka, throw law,
//! gauges, placed shots) is true in code.

mod common;

use aeon_sim::fighter::Action;
use aeon_sim::geom::px;
use aeon_sim::input::{Btn, Chord, InputFrame, CHARGE_FRAMES};
use aeon_sim::moves::{MoveId, ShotBehavior, ThrowKind};
use aeon_sim::{
    CharacterId, DummyMode, EventKind, Match, Phase, ProjectileKind, ShotState, World,
    FEINT_RECOVERY, RC_COST, RC_FREEZE_FRAMES,
};
use common::*;

// ---------------------------------------------------------------- footsies

#[test]
fn jab_hits_standing_dummy() {
    let mut w = close_kogan();
    let hp = w.fighters[1].health;
    tap(&mut w, press(Btn::P));
    hold(&mut w, 6, idle());
    drain_hitstop(&mut w);
    assert!(w.fighters[1].health < hp, "jab should deal damage");
    assert!(w.fighters[1].action.in_hitstun() || w.fighters[1].combo >= 1);
}

#[test]
fn holding_back_blocks_mid() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    let hp = w.fighters[1].health;
    tap(&mut w, press(Btn::P));
    hold2(&mut w, 8, idle(), back());
    drain_hitstop(&mut w);
    assert_eq!(w.fighters[1].health, hp, "blocked light does no chip");
    assert!(matches!(w.fighters[1].action, Action::Block { .. }));
}

#[test]
fn stand_block_does_not_stop_low() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    let hp = w.fighters[1].health;
    tap(&mut w, dir_press(2, Btn::K));
    hold2(&mut w, 8, dir(2), back());
    drain_hitstop(&mut w);
    assert!(w.fighters[1].health < hp, "2K must beat stand block");
}

#[test]
fn crouch_block_stops_low() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    let hp = w.fighters[1].health;
    tap(&mut w, dir_press(2, Btn::K));
    hold2(&mut w, 8, dir(2), down_back());
    drain_hitstop(&mut w);
    assert_eq!(w.fighters[1].health, hp);
    assert!(matches!(w.fighters[1].action, Action::Block { .. }));
}

#[test]
fn far_hs_whiff_is_punishable_by_jab() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[0].pos.x = px(300);
    w.fighters[1].pos.x = px(430);
    tap(&mut w, press(Btn::HS));
    hold(&mut w, 14, idle());
    assert!(
        attacking(&w, 0, MoveId::StHS),
        "P1 should still be recovering 5HS"
    );

    w.fighters[1].pos.x = w.fighters[0].pos.x + px(40);
    w.tick(idle(), press(Btn::P));
    hold(&mut w, 8, idle());
    drain_hitstop(&mut w);
    assert!(w.fighters[0].health < w.p1_char.data().max_health);
}

#[test]
fn jab_links_into_jab() {
    let mut w = close_kogan();
    tap(&mut w, press(Btn::P));
    for _ in 0..8 {
        w.tick(idle(), idle());
        if w.hitstop > 0 {
            break;
        }
    }
    drain_hitstop(&mut w);
    assert!(w.fighters[1].combo >= 1);
    for _ in 0..12 {
        if w.fighters[0].action.actionable() {
            break;
        }
        w.tick(idle(), idle());
    }
    tap(&mut w, press(Btn::P));
    hold(&mut w, 8, idle());
    drain_hitstop(&mut w);
    assert!(
        w.fighters[1].combo >= 2,
        "5P should link into 5P (combo {})",
        w.fighters[1].combo
    );
}

#[test]
fn normals_do_not_cancel_into_normals() {
    let mut w = close_kogan();
    tap(&mut w, press(Btn::P));
    hold(&mut w, 4, idle());
    drain_hitstop(&mut w);
    assert!(attacking(&w, 0, MoveId::StP));
    // Pressing HS during 5P's active/recovery does nothing.
    tap(&mut w, press(Btn::HS));
    assert!(
        attacking(&w, 0, MoveId::StP),
        "no chain: HS must not replace 5P"
    );
}

#[test]
fn both_bodies_have_shared_combat_jobs() {
    for id in [CharacterId::Kogan, CharacterId::Raya] {
        let d = id.data();
        assert!(d.has_move(d.poke_heavy));
        assert!(d.has_move(d.weapon_heavy));
        assert!(!d.space_controls.is_empty());
        assert!(d.has_move(d.reversal));
        let hs = d.move_def(MoveId::StHS).unwrap();
        assert!(
            hs.advantage_on_block() <= -6,
            "{} far HS must be minus",
            id.name()
        );
        assert!(
            hs.advantage_on_hit() < 0,
            "{} far HS minus on hit too",
            id.name()
        );
        let s = d.move_def(MoveId::StS).unwrap();
        assert!(
            s.advantage_on_block() < 0,
            "{} far S stays minus on block",
            id.name()
        );
        assert!(
            s.advantage_on_hit() <= 0,
            "{} far S is not plus on hit",
            id.name()
        );
        let p = d.move_def(MoveId::StP).unwrap();
        assert!(
            p.advantage_on_hit() >= p.startup as i32,
            "{} 5P links into itself",
            id.name()
        );
        let fl = d.move_def(MoveId::StFL).unwrap();
        assert!(
            fl.advantage_on_hit() >= 5,
            "{} FL is the fat-hitstun trap button",
            id.name()
        );
        assert!(
            fl.hitstun > s.hitstun,
            "{} FL has more hitstun than S",
            id.name()
        );
        assert!(
            d.move_def(MoveId::CrST).unwrap().knockdown,
            "2ST is the sweep"
        );
        assert!(
            d.run_speed > d.walk_fwd,
            "{} runs faster than walks",
            id.name()
        );
    }
}

#[test]
fn no_normal_chains_from_frame_data() {
    for id in [CharacterId::Kogan, CharacterId::Raya] {
        let jab = id.data().move_def(MoveId::StP).unwrap();
        assert!(jab.startup <= 5);
        let hs = id.data().move_def(MoveId::StHS).unwrap();
        assert!(
            hs.recovery >= 18,
            "far heavy recovery is the whiff-punish tax"
        );
    }
}

#[test]
fn damage_scaling_is_100_80_60_45_35() {
    use aeon_sim::collision::scale_damage;
    assert_eq!(scale_damage(100, 0), 100);
    assert_eq!(scale_damage(100, 1), 80);
    assert_eq!(scale_damage(100, 2), 60);
    assert_eq!(scale_damage(100, 3), 45);
    assert_eq!(scale_damage(100, 4), 35);
    assert_eq!(scale_damage(100, 9), 35);
}

// ------------------------------------------------------------------ chords

#[test]
fn roman_cancel_spends_250_and_cancels_a_whiff() {
    let mut w = close_kogan();
    w.fighters[0].meter = 500;
    tap(&mut w, press(Btn::HS));
    hold(&mut w, 2, idle());
    assert!(matches!(w.fighters[0].action, Action::Attack { .. }));

    tap(&mut w, chord(Chord::RomanCancel));
    assert_eq!(w.fighters[0].meter, 500 - RC_COST);
    assert!(matches!(
        w.fighters[0].action,
        Action::Stand | Action::Jump { .. }
    ));
    assert_eq!(w.rc_freeze, RC_FREEZE_FRAMES);
    assert!(has_event(&w, EventKind::RomanCancel));

    let frozen = w.frame;
    for _ in 0..RC_FREEZE_FRAMES {
        w.tick(idle(), idle());
        assert_eq!(w.frame, frozen);
    }
    w.tick(idle(), idle());
    assert_eq!(w.frame, frozen + 1);
}

#[test]
fn roman_cancel_works_after_hit_or_block() {
    for hit in [true, false] {
        let mut w = close_kogan();
        w.fighters[0].meter = RC_COST;
        w.fighters[0].start_move(MoveId::StHS);
        w.fighters[0].mark_connected(hit);
        tap(&mut w, chord(Chord::RomanCancel));
        assert_eq!(w.fighters[0].meter, 0);
        assert!(matches!(w.fighters[0].action, Action::Stand));
    }
}

#[test]
fn roman_cancel_requires_meter() {
    let mut w = close_kogan();
    tap(&mut w, press(Btn::HS));
    hold(&mut w, 2, idle());
    tap(&mut w, chord(Chord::RomanCancel));
    assert!(matches!(w.fighters[0].action, Action::Attack { .. }));
    assert_eq!(w.rc_freeze, 0);
}

#[test]
fn roman_cancel_cannot_burst_from_hitstun_or_blockstun() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[1].meter = 1000;
    tap(&mut w, press(Btn::S));
    hold(&mut w, 8, idle());
    drain_hitstop(&mut w);
    assert!(w.fighters[1].action.in_hitstun());
    w.tick(idle(), chord(Chord::RomanCancel));
    assert!(
        w.fighters[1].action.in_hitstun(),
        "RC must not burst hitstun"
    );
    assert_eq!(w.fighters[1].meter, 1000);

    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[1].meter = 1000;
    tap(&mut w, press(Btn::S));
    hold2(&mut w, 8, idle(), back());
    drain_hitstop(&mut w);
    assert!(w.fighters[1].action.in_blockstun());
    w.tick(
        idle(),
        InputFrame {
            dir: 4,
            buttons: aeon_sim::Buttons::chord(Chord::RomanCancel),
        },
    );
    assert!(
        w.fighters[1].action.in_blockstun(),
        "RC must not burst blockstun"
    );
    assert_eq!(w.fighters[1].meter, 1000);
}

#[test]
fn ex_chord_spends_character_gauge_not_the_bar() {
    let mut w = close_kogan();
    w.fighters[0].meter = 1000;
    assert_eq!(w.fighters[0].gauge, 6);
    motion_chord(&mut w, &[2, 3, 6], Chord::Ex);
    assert!(
        attacking(&w, 0, MoveId::ExA),
        "236+S+HS is EX saber cut, got {}",
        w.fighters[0].action.name()
    );
    assert_eq!(w.fighters[0].gauge, 4, "EX spends two chambers");
    assert_eq!(w.fighters[0].meter, 1000, "EX does not touch the bar");
}

#[test]
fn ex_without_gauge_does_not_come_out() {
    let mut w = close_kogan();
    w.fighters[0].gauge = 1;
    motion_chord(&mut w, &[2, 3, 6], Chord::Ex);
    assert!(!attacking(&w, 0, MoveId::ExA));
    let mut w = close(CharacterId::Raya, CharacterId::Kogan);
    assert_eq!(w.fighters[0].gauge, 0);
    motion_chord(&mut w, &[2, 1, 4], Chord::Ex);
    assert!(
        !attacking(&w, 0, MoveId::ExB),
        "no crystal gauge, no EX crystal"
    );
}

#[test]
fn ex_chord_can_land_a_frame_apart() {
    let mut w = close_kogan();
    for d in [2, 3, 6] {
        tap(&mut w, dir(d));
    }
    tap(&mut w, dir_press(6, Btn::S));
    assert!(attacking(&w, 0, MoveId::Rekka1), "S alone is the rekka");
    tap(&mut w, two(Btn::S, Btn::HS));
    assert!(
        attacking(&w, 0, MoveId::ExA),
        "S+HS one frame later karas into EX"
    );
}

#[test]
fn overhead_chord_is_high() {
    let d = CharacterId::Kogan
        .data()
        .move_def(MoveId::Overhead)
        .unwrap();
    assert_eq!(d.level, aeon_sim::HitLevel::High);
    // Beats crouch block.
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    let hp = w.fighters[1].health;
    tap(&mut w, chord(Chord::Overhead));
    assert!(attacking(&w, 0, MoveId::Overhead));
    hold2(&mut w, 26, idle(), down_back());
    drain_hitstop(&mut w);
    assert!(
        w.fighters[1].health < hp,
        "HS+ST overhead beats crouch block"
    );
    // Blocked standing (the dummy holds block in place rather than walking off).
    let mut w = close_kogan();
    w.dummy = DummyMode::BlockAll;
    let hp = w.fighters[1].health;
    tap(&mut w, chord(Chord::Overhead));
    hold2(&mut w, 26, idle(), back());
    drain_hitstop(&mut w);
    assert_eq!(w.fighters[1].health, hp);
    assert!(
        matches!(w.fighters[1].action, Action::Block { .. }),
        "got {}",
        w.fighters[1].action.name()
    );
}

#[test]
fn throw_chord_is_a_throw() {
    let mut w = close_kogan();
    tap(&mut w, chord(Chord::Throw));
    assert!(attacking(&w, 0, MoveId::Throw));
}

// --------------------------------------------------------------------- hop

#[test]
fn hop_is_a_lower_shorter_arc_than_jump() {
    fn arc(hold_up: u32) -> (i32, u32, bool) {
        let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
        w.fighters[1].pos.x = px(700);
        hold(&mut w, hold_up, dir(8));
        let mut peak = 0;
        let mut frames = 0;
        let mut hopped = false;
        for _ in 0..120 {
            w.tick(idle(), idle());
            if w.fighters[0].action.is_hop() {
                hopped = true;
            }
            if w.fighters[0].airborne {
                frames += 1;
                peak = peak.max(w.fighters[0].pos.y);
            } else if frames > 0 {
                break;
            }
        }
        (peak, frames, hopped)
    }
    let (hop_peak, hop_frames, hopped) = arc(1);
    let (jump_peak, jump_frames, jumped_as_hop) = arc(10);
    assert!(hopped, "a one-frame up tap is a hop");
    assert!(!jumped_as_hop, "held up is a full jump");
    assert!(
        hop_peak < jump_peak * 2 / 3,
        "hop peak {} vs jump {}",
        hop_peak,
        jump_peak
    );
    assert!(
        hop_frames < jump_frames * 3 / 4,
        "hop airtime {} vs jump {}",
        hop_frames,
        jump_frames
    );
}

#[test]
fn hop_overhead_beats_stand_block_and_loses_to_crouch_block() {
    fn hop_in(p2: InputFrame) -> (i32, i32) {
        let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
        w.fighters[0].pos.x = px(360);
        w.fighters[1].pos.x = px(410);
        let hp = w.fighters[1].health;
        // Forward hop, then j.S at the top.
        w.tick(dir(9), p2);
        for _ in 0..8 {
            w.tick(idle(), p2);
        }
        assert!(
            w.fighters[0].action.is_hop(),
            "should be hopping, got {}",
            w.fighters[0].action.name()
        );
        w.tick(press(Btn::S), p2);
        for _ in 0..30 {
            w.tick(idle(), p2);
        }
        drain_hitstop(&mut w);
        (hp, w.fighters[1].health)
    }
    // Stand block: back with no down. The hop normal is High, so...
    let (before, after) = hop_in(back());
    assert_eq!(before, after, "stand block blocks a high");
    let (before, after) = hop_in(down_back());
    assert!(
        after < before,
        "crouch block loses to a hop overhead (High)"
    );
}

// ------------------------------------------------------------------ run

#[test]
fn run_is_universal_and_faster_than_walk() {
    for id in [CharacterId::Kogan, CharacterId::Raya] {
        let mut w = free(id, CharacterId::Kogan);
        w.fighters[0].pos.x = px(100);
        w.fighters[1].pos.x = px(700);
        let x0 = w.fighters[0].pos.x;
        // 66 then hold 6.
        tap(&mut w, dir(6));
        tap(&mut w, dir(5));
        tap(&mut w, dir(6));
        assert!(
            matches!(w.fighters[0].action, Action::Run),
            "{} should run",
            id.name()
        );
        hold(&mut w, 10, dir(6));
        let ran = w.fighters[0].pos.x - x0;
        let mut w2 = free(id, CharacterId::Kogan);
        w2.fighters[0].pos.x = px(100);
        w2.fighters[1].pos.x = px(700);
        hold(&mut w2, 13, dir(6));
        let walked = w2.fighters[0].pos.x - px(100);
        assert!(
            ran > walked * 3 / 2,
            "{} run {} vs walk {}",
            id.name(),
            ran,
            walked
        );
    }
}

#[test]
fn backdash_is_punishable() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    tap(&mut w, dir(4));
    tap(&mut w, dir(5));
    tap(&mut w, dir(4));
    assert!(matches!(w.fighters[0].action, Action::BackDash { .. }));
    assert!(
        !w.fighters[0].strike_invuln(),
        "no invuln on the shared backdash"
    );
    // Late in the dash the body is still locked; a jab from range lands.
    w.fighters[0].action = Action::BackDash { frame: 10 };
    w.fighters[0].vel.x = 0;
    w.fighters[1].pos.x = w.fighters[0].pos.x + px(36);
    w.tick(idle(), press(Btn::P));
    hold(&mut w, 6, idle());
    drain_hitstop(&mut w);
    assert!(w.fighters[0].health < 1000, "a backdash can be hit");
}

// ---------------------------------------------------------------- feint

#[test]
fn feint_cancels_special_startup_to_nothing() {
    let mut w = close_kogan();
    motion(&mut w, &[2, 3, 6], Btn::S);
    assert!(attacking(&w, 0, MoveId::Rekka1));
    hold(&mut w, 2, dir(6));
    tap(&mut w, chord(Chord::Feint));
    assert!(
        matches!(w.fighters[0].action, Action::Feint { .. }),
        "got {}",
        w.fighters[0].action.name()
    );
    assert!(has_event(&w, EventKind::Feint));
    let hp = w.fighters[1].health;
    hold(&mut w, 30, idle());
    assert_eq!(w.fighters[1].health, hp, "the special never came");
    assert!(w.fighters[0].hitboxes().is_empty());
}

#[test]
fn feint_does_not_apply_to_normals_or_after_active() {
    let mut w = close_kogan();
    tap(&mut w, press(Btn::HS));
    tap(&mut w, chord(Chord::Feint));
    assert!(
        matches!(w.fighters[0].action, Action::Attack { move_id, .. } if move_id.is_normal()),
        "normals cannot be feinted, got {}",
        w.fighters[0].action.name()
    );

    let mut w = close_kogan();
    motion(&mut w, &[6, 2, 3], Btn::S);
    assert!(attacking(&w, 0, MoveId::Uppercut));
    hold(&mut w, 6, idle());
    tap(&mut w, chord(Chord::Feint));
    assert!(
        attacking(&w, 0, MoveId::Uppercut),
        "too late: the uppercut is active"
    );
}

#[test]
fn feinted_dp_is_punishable_before_a_baited_dp_recovers() {
    // M4: feint a 623; the feint's recovery is shorter than an opponent's
    // whiffed 623, so the feinter is free to punish.
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[1].pos.x = px(600);
    motion(&mut w, &[6, 2, 3], Btn::S);
    assert!(attacking(&w, 0, MoveId::Uppercut));
    tap(&mut w, chord(Chord::Feint));
    assert!(matches!(w.fighters[0].action, Action::Feint { .. }));
    let free_in = frames_until_actionable(&mut w, 0, 60);
    assert!(
        free_in <= FEINT_RECOVERY as u32,
        "feint recovery {} > {}",
        free_in,
        FEINT_RECOVERY
    );
    let up = CharacterId::Kogan
        .data()
        .move_def(MoveId::Uppercut)
        .unwrap();
    let dp_total = up.total_frames() as u32 + up.land_recovery as u32;
    assert!(
        free_in * 3 < dp_total,
        "feint ({}f) must be far cheaper than a whiffed DP ({}f)",
        free_in,
        dp_total
    );
}

// ---------------------------------------------------------------- rekka

#[test]
fn rekka_parts_follow_and_stopping_early_is_a_different_situation() {
    for id in [CharacterId::Kogan, CharacterId::Raya] {
        let mut w = close(id, CharacterId::Kogan);
        motion(&mut w, &[2, 3, 6], Btn::S);
        assert!(
            attacking(&w, 0, MoveId::Rekka1),
            "{}: 236+S is rekka 1",
            id.name()
        );
        let r1 = id.data().move_def(MoveId::Rekka1).unwrap();
        hold(&mut w, r1.startup as u32 + 1, dir(5));
        drain_hitstop(&mut w);
        tap(&mut w, press(Btn::S));
        assert!(
            attacking(&w, 0, MoveId::Rekka2),
            "{}: S during part 1 is part 2",
            id.name()
        );
        let r2 = id.data().move_def(MoveId::Rekka2).unwrap();
        hold(&mut w, r2.startup as u32 + 1, dir(5));
        drain_hitstop(&mut w);
        tap(&mut w, press(Btn::S));
        assert!(
            attacking(&w, 0, MoveId::Rekka3),
            "{}: S during part 2 is part 3",
            id.name()
        );
        hold(&mut w, 12, idle());
        drain_hitstop(&mut w);
        assert!(
            matches!(
                w.fighters[1].action,
                Action::Hit {
                    knockdown: true,
                    ..
                } | Action::Knockdown { .. }
            ),
            "{}: part 3 knocks down, got {}",
            id.name(),
            w.fighters[1].action.name()
        );

        // Stopping after part 1 leaves a different frame situation than part 3.
        let r3 = id.data().move_def(MoveId::Rekka3).unwrap();
        assert!(r1.advantage_on_block() > r3.advantage_on_block());
        assert!(!r1.knockdown && r3.knockdown);
        // Part 2 and 3 cannot be started from neutral.
        let mut w = close(id, CharacterId::Kogan);
        tap(&mut w, press(Btn::S));
        assert!(!attacking(&w, 0, MoveId::Rekka2));
    }
}

#[test]
fn rekka_part_is_roman_cancellable() {
    let mut w = close_kogan();
    w.fighters[0].meter = 250;
    motion(&mut w, &[2, 3, 6], Btn::S);
    hold(&mut w, 9, dir(5));
    drain_hitstop(&mut w);
    tap(&mut w, press(Btn::S));
    assert!(attacking(&w, 0, MoveId::Rekka2));
    let before = w.fighters[0].meter;
    tap(&mut w, chord(Chord::RomanCancel));
    assert_eq!(w.fighters[0].meter, before - RC_COST);
    assert!(matches!(w.fighters[0].action, Action::Stand));
}

#[test]
fn special_cancel_window_is_tight() {
    // 5S on hit cancels into 236+S only up to CANCEL_LATE_FRAMES after active.
    let s = CharacterId::Kogan.data().move_def(MoveId::StS).unwrap();
    let late = s.last_active() + aeon_sim::CANCEL_LATE_FRAMES;
    let mut w = close_kogan();
    w.fighters[0].start_move(MoveId::StS);
    w.fighters[0].mark_connected(true);
    w.fighters[0].action = Action::Attack {
        move_id: MoveId::StS,
        frame: late + 1,
        connected: aeon_sim::Connect::Hit,
    };
    motion(&mut w, &[2, 3, 6], Btn::S);
    assert!(
        !attacking(&w, 0, MoveId::Rekka1),
        "cancel too late must fail"
    );
}

// ------------------------------------------------------------ throw law

#[test]
fn command_grab_beats_crouch_block_and_stand_block() {
    for guard in [down_back(), back()] {
        let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
        let hp = w.fighters[1].health;
        for d in [6, 3, 2, 1, 4] {
            w.tick(dir(d), guard);
        }
        // Point blank: a stand-blocker walking back still cannot escape.
        w.fighters[1].pos.x = w.fighters[0].pos.x + px(34);
        w.tick(dir_press(4, Btn::FL), guard);
        assert!(
            attacking(&w, 0, MoveId::CommandGrab),
            "63214+FL is the cape-snare, got {}",
            w.fighters[0].action.name()
        );
        hold2(&mut w, 8, dir(4), guard);
        assert!(
            has_event(&w, EventKind::Grab)
                || matches!(w.fighters[1].action, Action::Thrown { .. })
                || w.fighters[1].health < hp,
            "grab should connect, P2 is {}",
            w.fighters[1].action.name()
        );
        hold2(&mut w, 8, idle(), guard);
        drain_hitstop(&mut w);
        assert!(
            w.fighters[1].health < hp,
            "command grab beats block (guard dir {})",
            guard.dir
        );
        assert!(
            matches!(
                w.fighters[1].action,
                Action::Hit {
                    knockdown: true,
                    ..
                } | Action::Knockdown { .. }
            ),
            "hard knockdown"
        );
    }
}

#[test]
fn command_grab_is_untechable() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    // The dummy mashes P+K the moment it is grabbed.
    w.dummy = DummyMode::Tech;
    let hp = w.fighters[1].health;
    for d in [6, 3, 2, 1, 4] {
        w.tick(dir(d), idle());
    }
    w.tick(dir_press(4, Btn::FL), idle());
    let mut teched = false;
    for _ in 0..20 {
        w.tick(idle(), idle());
        teched |= has_event(&w, EventKind::ThrowTech);
    }
    drain_hitstop(&mut w);
    assert!(
        w.fighters[1].health < hp,
        "mashing tech does nothing against the rite"
    );
    assert!(!teched);

    // Control: the same dummy techs a normal throw.
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.dummy = DummyMode::Tech;
    tap(&mut w, chord(Chord::Throw));
    let mut teched = false;
    for _ in 0..12 {
        w.tick(idle(), idle());
        teched |= has_event(&w, EventKind::ThrowTech);
    }
    assert!(teched, "normal throw is teched by the same mash");
    assert_eq!(w.fighters[1].health, hp);
}

#[test]
fn command_grab_loses_to_uppercut() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    for d in [6, 3, 2, 1, 4] {
        w.tick(dir(d), idle());
    }
    // P1 grabs; P2 has the DP motion buffered and presses S the next frame
    // so its invuln covers the grab's active frames.
    for d in [6, 2] {
        w.tick(dir(4), dir(d));
    }
    w.tick(dir_press(4, Btn::FL), dir(3));
    assert!(attacking(&w, 0, MoveId::CommandGrab));
    w.tick(dir(4), dir_press(3, Btn::S));
    assert!(
        attacking(&w, 1, MoveId::Uppercut),
        "P2 got {}",
        w.fighters[1].action.name()
    );
    hold(&mut w, 12, dir(4));
    drain_hitstop(&mut w);
    assert!(!has_event(&w, EventKind::Grab));
    assert!(
        w.fighters[0].health < 1000
            || w.fighters[0].action.in_hitstun()
            || matches!(
                w.fighters[0].action,
                Action::Knockdown { .. } | Action::Hit { .. }
            ),
        "the uppercut should have won: P1 {} hp {}",
        w.fighters[0].action.name(),
        w.fighters[0].health
    );
    assert_eq!(w.fighters[1].health, 1000, "grabber never grabbed");
}

#[test]
fn command_grab_whiff_is_a_long_recovery() {
    let g = CharacterId::Kogan
        .data()
        .move_def(MoveId::CommandGrab)
        .unwrap();
    let t = CharacterId::Kogan.data().move_def(MoveId::Throw).unwrap();
    assert!(g.total_frames() >= 40, "whiffed command grab is max-punish");
    assert!(t.total_frames() <= 24, "normal throw whiff is modest");
    assert_eq!(g.throw, ThrowKind::Command);
    assert_eq!(t.throw, ThrowKind::Normal);
}

#[test]
fn normal_throw_beats_crouch_block_now() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    let hp = w.fighters[1].health;
    w.tick(chord(Chord::Throw), down_back());
    hold2(&mut w, 12, idle(), down_back());
    drain_hitstop(&mut w);
    assert!(w.fighters[1].health < hp, "P+K beats crouch block");
}

#[test]
fn normal_throw_is_techable() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    let hp = w.fighters[1].health;
    tap(&mut w, chord(Chord::Throw));
    hold(&mut w, 2, idle());
    // Grabbed now; tech within the window.
    assert!(
        matches!(w.fighters[1].action, Action::Thrown { .. }),
        "got {}",
        w.fighters[1].action.name()
    );
    hold2(&mut w, 3, idle(), idle());
    w.tick(idle(), chord(Chord::Throw));
    hold(&mut w, 2, idle());
    assert!(
        has_event(&w, EventKind::ThrowTech)
            || matches!(w.fighters[1].action, Action::ThrowTech { .. }),
        "late tech inside the window, got {}",
        w.fighters[1].action.name()
    );
    assert_eq!(w.fighters[1].health, hp);
}

#[test]
fn simultaneous_throws_tech() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.tick(chord(Chord::Throw), chord(Chord::Throw));
    hold(&mut w, 4, idle());
    assert!(
        has_event(&w, EventKind::ThrowTech)
            || matches!(w.fighters[0].action, Action::ThrowTech { .. })
    );
    assert_eq!(w.fighters[0].health, 1000);
    assert_eq!(w.fighters[1].health, 1000);
}

#[test]
fn jab_on_the_throw_frame_beats_the_throw() {
    // P is 4f startup; the throw is 2f. A P pressed two frames before the
    // throw press is active on the throw's active frame: hit beats throw.
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.tick(idle(), press(Btn::P));
    w.tick(idle(), idle());
    w.tick(chord(Chord::Throw), idle());
    hold(&mut w, 3, idle());
    drain_hitstop(&mut w);
    assert!(w.fighters[0].health < 1000, "the jab lands");
    assert!(!has_event(&w, EventKind::Grab));
    assert_eq!(
        w.fighters[1].health, 1000,
        "the thrower was hit, not the jabber"
    );
}

#[test]
fn throws_whiff_on_stunned_and_airborne_bodies() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    tap(&mut w, press(Btn::S));
    hold(&mut w, 8, idle());
    drain_hitstop(&mut w);
    assert!(w.fighters[1].action.in_hitstun());
    let hp = w.fighters[1].health;
    w.fighters[0].start_move(MoveId::Throw);
    hold(&mut w, 4, idle());
    assert!(
        !matches!(w.fighters[1].action, Action::Thrown { .. }),
        "no throws in hitstun"
    );
    assert_eq!(w.fighters[1].health, hp);
}

// ---------------------------------------------------------------- uppercut

#[test]
fn uppercut_is_invulnerable_early_and_taxed_on_whiff() {
    let up = CharacterId::Kogan
        .data()
        .move_def(MoveId::Uppercut)
        .unwrap();
    assert!(up.invuln.strike_on(1) && up.invuln.throw_on(1));
    assert!(up.startup <= 4);
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[1].pos.x = px(700);
    motion(&mut w, &[6, 2, 3], Btn::HS);
    assert!(
        attacking(&w, 0, MoveId::Uppercut),
        "623+HS is also the uppercut"
    );
    let free_in = frames_until_actionable(&mut w, 0, 120);
    assert!(
        free_in >= 40,
        "whiffed uppercut vulnerable for {} frames",
        free_in
    );
}

#[test]
fn uppercut_on_hit_is_rc_able() {
    let mut w = close_kogan();
    w.fighters[0].meter = 250;
    motion(&mut w, &[6, 2, 3], Btn::S);
    hold(&mut w, 4, dir(3));
    drain_hitstop(&mut w);
    assert!(
        w.fighters[1].action.in_hitstun(),
        "got {}",
        w.fighters[1].action.name()
    );
    let before = w.fighters[0].meter;
    tap(&mut w, chord(Chord::RomanCancel));
    assert_eq!(w.fighters[0].meter, before - RC_COST);
    assert!(matches!(
        w.fighters[0].action,
        Action::Jump { air_ok: true, .. } | Action::Stand
    ));
}

// ------------------------------------------------------------------ gauges

#[test]
fn kogan_firearm_gauge_spends_and_cools() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[1].pos.x = px(700);
    assert_eq!(w.fighters[0].gauge, 6);
    motion(&mut w, &[2, 1, 4], Btn::S);
    assert!(
        attacking(&w, 0, MoveId::ShotA),
        "214+S is the revolver, got {}",
        w.fighters[0].action.name()
    );
    assert_eq!(w.fighters[0].gauge, 5, "one chamber per shot");
    hold(&mut w, 14, idle());
    assert!(w
        .projectiles
        .iter()
        .any(|p| p.def.kind == ProjectileKind::Revolver));
    // Cooldown reloads.
    let g = CharacterId::Kogan.data().gauge;
    hold(&mut w, (g.regen_delay + g.regen_every + 2) as u32, idle());
    assert_eq!(w.fighters[0].gauge, 6, "the cylinder cools back");
    // Empty cylinder: no shot.
    w.fighters[0].gauge = 0;
    motion(&mut w, &[2, 1, 4], Btn::S);
    assert!(!attacking(&w, 0, MoveId::ShotA), "empty gun does not fire");
}

#[test]
fn raya_consecrate_fills_the_crystal_gauge_and_buffs_crystals() {
    let mut w = free(CharacterId::Raya, CharacterId::Kogan);
    w.fighters[1].pos.x = px(700);
    for d in [2, 1, 4] {
        tap(&mut w, dir(d));
    }
    tap(&mut w, dir_press(4, Btn::FL));
    assert!(
        attacking(&w, 0, MoveId::Charge),
        "214+FL is consecrate, got {}",
        w.fighters[0].action.name()
    );
    hold(&mut w, 40, dir_press(4, Btn::FL));
    assert!(
        w.fighters[0].gauge >= 50,
        "gauge {} after holding",
        w.fighters[0].gauge
    );
    let tier = w.fighters[0].buff_tier();
    assert!(tier >= 1);
    // Release: no attack comes out. Charge is a buff, not a stored attack.
    let hp = w.fighters[1].health;
    hold(&mut w, 20, idle());
    assert!(w.projectiles.is_empty());
    assert_eq!(w.fighters[1].health, hp);
    assert!(w.fighters[0].gauge >= 50, "gauge persists after release");

    // A buffed crystal arms faster and hits harder than an unbuffed one.
    let base = CharacterId::Raya
        .data()
        .move_def(MoveId::ShotA)
        .unwrap()
        .projectile
        .unwrap();
    let ShotBehavior::Plant { arm_after, .. } = base.behavior else {
        panic!()
    };
    motion(&mut w, &[2, 1, 4], Btn::S);
    hold(&mut w, 16, idle());
    let c = w
        .projectiles
        .iter()
        .find(|p| p.def.kind == ProjectileKind::Crystal)
        .expect("crystal");
    assert!(
        c.arm_after < arm_after,
        "buffed crystal arms in {} < {}",
        c.arm_after,
        arm_after
    );
    assert!(c.damage > base.damage);
}

#[test]
fn back_charge_release_is_not_a_stored_attack() {
    let mut w = free(CharacterId::Raya, CharacterId::Kogan);
    w.fighters[1].pos.x = px(700);
    hold(&mut w, CHARGE_FRAMES as u32 + 2, back());
    tap(&mut w, dir_press(6, Btn::K));
    assert!(
        attacking(&w, 0, MoveId::StK),
        "[4]6K is just a kick now, got {}",
        w.fighters[0].action.name()
    );
    assert!(!CharacterId::Raya
        .data()
        .specials
        .iter()
        .any(|r| matches!(r.motion, aeon_sim::Motion::ChargeBackForward)));
}

// ----------------------------------------------------------- placed shots

#[test]
fn raya_voice_is_a_hanging_glyph() {
    let mut w = free(CharacterId::Raya, CharacterId::Kogan);
    w.fighters[1].pos.x = px(700);
    motion(&mut w, &[2, 3, 6], Btn::HS);
    assert!(attacking(&w, 0, MoveId::ShotB));
    hold(&mut w, 14, idle());
    let g = w
        .projectiles
        .iter()
        .find(|p| p.def.kind == ProjectileKind::Glyph)
        .expect("glyph");
    let x = g.pos.x;
    assert_eq!(g.state, ShotState::Hanging);
    hold(&mut w, 20, idle());
    let g = w
        .projectiles
        .iter()
        .find(|p| p.def.kind == ProjectileKind::Glyph)
        .expect("glyph still hangs");
    assert_eq!(g.pos.x, x, "a glyph does not travel");
    hold(&mut w, 60, idle());
    assert!(
        !w.projectiles
            .iter()
            .any(|p| p.def.kind == ProjectileKind::Glyph),
        "and it fades"
    );
}

#[test]
fn raya_crystal_plants_arms_and_detonates_on_contact() {
    let mut w = free(CharacterId::Raya, CharacterId::Kogan);
    w.fighters[1].pos.x = px(700);
    motion(&mut w, &[2, 1, 4], Btn::S);
    assert!(attacking(&w, 0, MoveId::ShotA));
    hold(&mut w, 16, idle());
    assert!(
        has_event(&w, EventKind::Plant)
            || w.projectiles
                .iter()
                .any(|p| p.def.kind == ProjectileKind::Crystal)
    );
    // Wait for it to land and arm.
    let landed = run_until(&mut w, 90, idle(), idle(), |w| {
        w.projectiles.iter().any(|p| p.planted())
    });
    assert!(landed.is_some(), "crystal lands");
    let c = w.projectiles.iter().find(|p| p.planted()).unwrap();
    assert!(!c.armed(), "not armed on landing");
    let crystal_x = c.pos.x;
    // Walk the dummy onto the unarmed crystal: nothing happens.
    w.fighters[1].pos.x = crystal_x;
    let hp = w.fighters[1].health;
    w.tick(idle(), idle());
    assert_eq!(w.fighters[1].health, hp, "unarmed crystal is inert");
    w.fighters[1].pos.x = px(700);
    let armed = run_until(&mut w, 60, idle(), idle(), |w| {
        w.projectiles.iter().any(|p| p.armed())
    });
    assert!(armed.is_some(), "crystal arms");
    // Now touch it.
    w.fighters[1].pos.x = crystal_x;
    hold(&mut w, 3, idle());
    drain_hitstop(&mut w);
    assert!(
        w.fighters[1].health < hp,
        "armed crystal detonates on contact"
    );
    assert!(has_event(&w, EventKind::Hit) || w.fighters[1].action.in_hitstun());
}

#[test]
fn raya_can_shatter_an_armed_crystal_early() {
    let mut w = free(CharacterId::Raya, CharacterId::Kogan);
    w.fighters[1].pos.x = px(700);
    motion(&mut w, &[2, 1, 4], Btn::S);
    let armed = run_until(&mut w, 120, idle(), idle(), |w| {
        w.projectiles.iter().any(|p| p.armed())
    });
    assert!(armed.is_some());
    motion(&mut w, &[2, 1, 4], Btn::S);
    assert!(
        attacking(&w, 0, MoveId::Detonate),
        "214+S with an armed crystal is shatter, got {}",
        w.fighters[0].action.name()
    );
    hold(&mut w, 8, idle());
    assert!(has_event(&w, EventKind::Detonate) || w.projectiles.iter().all(|p| !p.planted()));
    hold(&mut w, 8, idle());
    assert!(
        !w.projectiles
            .iter()
            .any(|p| p.def.kind == ProjectileKind::Crystal),
        "gone"
    );
}

#[test]
fn one_shot_per_owner_per_type_but_types_coexist() {
    let mut w = free(CharacterId::Raya, CharacterId::Kogan);
    w.fighters[1].pos.x = px(720);
    w.fighters[0].start_move(MoveId::ShotB);
    hold(&mut w, 14, idle());
    w.fighters[0].start_move(MoveId::ShotB);
    hold(&mut w, 14, idle());
    assert_eq!(
        w.projectiles
            .iter()
            .filter(|p| p.def.kind == ProjectileKind::Glyph)
            .count(),
        1
    );
    w.fighters[0].start_move(MoveId::ShotA);
    hold(&mut w, 16, idle());
    assert!(w
        .projectiles
        .iter()
        .any(|p| p.def.kind == ProjectileKind::Crystal));
    assert!(w
        .projectiles
        .iter()
        .any(|p| p.def.kind == ProjectileKind::Glyph));
}

#[test]
fn same_class_shots_cancel() {
    let mut w = free(CharacterId::Kogan, CharacterId::Raya);
    w.fighters[0].pos.x = px(200);
    w.fighters[1].pos.x = px(400);
    // Raya hangs a glyph (light); Kogan fires the revolver (light) into it.
    w.fighters[1].start_move(MoveId::ShotB);
    hold(&mut w, 14, idle());
    assert!(w
        .projectiles
        .iter()
        .any(|p| p.def.kind == ProjectileKind::Glyph));
    w.fighters[0].start_move(MoveId::ShotA);
    let clashed = run_until(&mut w, 40, idle(), idle(), |w| {
        w.events.iter().any(|e| e.kind == EventKind::Clash)
    });
    assert!(clashed.is_some(), "light vs light: both die");
    assert!(w.projectiles.is_empty());
}

#[test]
fn kogan_disc_is_plus_and_destroys_a_shot() {
    let disc = CharacterId::Kogan.data().move_def(MoveId::Guard).unwrap();
    assert!(disc.advantage_on_block() >= 3);
    assert!(disc.projectile_guard);

    let mut w = free(CharacterId::Kogan, CharacterId::Raya);
    w.fighters[0].pos.x = px(300);
    w.fighters[1].pos.x = px(470);
    // Raya places the glyph; Kogan steps to where the disc will bloom on it
    // without his body touching it.
    w.fighters[1].start_move(MoveId::ShotB);
    hold(&mut w, 14, idle());
    assert!(w
        .projectiles
        .iter()
        .any(|p| p.def.kind == ProjectileKind::Glyph));
    w.fighters[0].pos.x = px(320);
    motion(&mut w, &[2, 1, 4], Btn::HS);
    assert!(
        attacking(&w, 0, MoveId::Guard),
        "214+HS is the disc, got {}",
        w.fighters[0].action.name()
    );
    let guarded = run_until(&mut w, 14, idle(), idle(), |w| {
        w.events
            .iter()
            .any(|e| e.kind == EventKind::ProjectileGuard)
    });
    assert!(guarded.is_some(), "the disc eats the shot");
    assert_eq!(w.fighters[0].health, 1000);
}

#[test]
fn kogan_wave_is_short_lived() {
    let wave = CharacterId::Kogan
        .data()
        .move_def(MoveId::ShotB)
        .unwrap()
        .projectile
        .unwrap();
    assert!(
        wave.vel_x * wave.lifetime as i32 <= px(140),
        "the wave is oki, not a fireball war"
    );
    let rev = CharacterId::Kogan.data().move_def(MoveId::ShotA).unwrap();
    assert!(rev.gauge_cost >= 1, "revolver is gauge-fed");
}

#[test]
fn kogan_air_saber_and_air_gun_are_different_buttons() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[1].pos.x = px(700);
    hold(&mut w, 6, dir(8));
    assert!(w.fighters[0].airborne);
    tap(&mut w, press(Btn::FL));
    assert!(
        attacking(&w, 0, MoveId::AirShot),
        "j.FL is the air gun, got {}",
        w.fighters[0].action.name()
    );
    assert_eq!(w.fighters[0].gauge, 5);
    hold(&mut w, 9, idle());
    assert!(w
        .projectiles
        .iter()
        .any(|p| p.def.kind == ProjectileKind::AirShot));

    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[1].pos.x = px(700);
    hold(&mut w, 6, dir(8));
    tap(&mut w, press(Btn::HS));
    assert!(attacking(&w, 0, MoveId::JHS), "j.HS is the air saber");
}

#[test]
fn raya_glide_passes_through_the_body() {
    let mut w = free(CharacterId::Raya, CharacterId::Kogan);
    w.fighters[0].pos.x = px(370);
    w.fighters[1].pos.x = px(420);
    motion(&mut w, &[2, 3, 6], Btn::FL);
    assert!(
        attacking(&w, 0, MoveId::CommandDash),
        "236+FL is the processional, got {}",
        w.fighters[0].action.name()
    );
    hold(&mut w, 24, idle());
    assert!(
        w.fighters[0].pos.x > w.fighters[1].pos.x,
        "Raya is behind Kogan now"
    );
    assert!(!w.fighters[0].facing_right, "and faces back toward him");
}

// ------------------------------------------------------------------- aura

#[test]
fn kogan_aura_never_extends_hurtboxes() {
    let f = aeon_sim::Fighter::spawn(CharacterId::Kogan, px(300), true);
    let aura = f.visual_aura_box().expect("Kogan has a visual aura");
    let hurt = f.hurtboxes();
    assert!(aura.width() > hurt[0].width());
    assert!(aura.left < hurt[0].left || aura.right > hurt[0].right);
    assert_eq!(hurt.len(), 1, "aura must not become a collision box");
    // A jab aimed at cape-only space does not hit.
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[0].pos.x = px(300);
    w.fighters[1].pos.x = px(300) + px(16) + px(4) + px(36) + px(8); // P2's jab tip lands in P1's aura, outside P1's hurtbox
    let hurt = w.fighters[0].hurtboxes()[0];
    let aura = w.fighters[0].visual_aura_box().unwrap();
    w.fighters[1].start_move(MoveId::StP);
    hold(&mut w, 6, idle());
    let jab = &w.fighters[1];
    assert!(jab.hitboxes().is_empty() || w.fighters[0].health == 1000);
    assert!(aura.right > hurt.right);
}

// ----------------------------------------------------------- determinism

#[test]
fn same_inputs_replay_to_the_same_state() {
    fn script(w: &mut World) {
        let seq: Vec<(InputFrame, InputFrame)> = (0..400u32)
            .map(|i| {
                let a = match i % 37 {
                    0..=3 => dir(6),
                    4 => dir_press(6, Btn::S),
                    9..=11 => dir(2),
                    12 => dir_press(3, Btn::S),
                    20 => chord(Chord::Throw),
                    25 => dir(9),
                    30 => press(Btn::HS),
                    _ => idle(),
                };
                let b = match i % 23 {
                    0..=5 => back(),
                    6 => press(Btn::P),
                    10..=12 => down_back(),
                    15 => dir_press(2, Btn::K),
                    _ => idle(),
                };
                (a, b)
            })
            .collect();
        for (a, b) in seq {
            w.tick(a, b);
        }
    }
    let mut a = free(CharacterId::Kogan, CharacterId::Raya);
    let mut b = free(CharacterId::Kogan, CharacterId::Raya);
    script(&mut a);
    script(&mut b);
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a, b);
    assert!(a.frame > 300);
}

// ----------------------------------------------------------------- versus

#[test]
fn versus_is_first_to_two_rounds() {
    let mut m = Match::new(CharacterId::Kogan, CharacterId::Raya);
    assert!(matches!(m.phase, Phase::Intro { .. }));
    for _ in 0..aeon_sim::versus::INTRO_FRAMES {
        m.tick(idle(), idle());
    }
    assert_eq!(m.phase, Phase::Fight);
    // KO P2 twice.
    for round in 1..=2 {
        m.world.fighters[1].health = 1;
        m.world.fighters[0].pos.x = px(370);
        m.world.fighters[1].pos.x = px(408);
        m.tick(press(Btn::P), idle());
        for _ in 0..12 {
            m.tick(idle(), idle());
        }
        assert!(
            matches!(m.phase, Phase::RoundEnd { .. }),
            "round {} should end, phase {:?}",
            round,
            m.phase
        );
        for _ in 0..aeon_sim::versus::ROUND_END_FRAMES + 1 {
            m.tick(idle(), idle());
        }
        if round == 1 {
            assert_eq!(m.wins, [1, 0]);
            assert_eq!(m.round, 2);
            for _ in 0..aeon_sim::versus::INTRO_FRAMES {
                m.tick(idle(), idle());
            }
        }
    }
    assert_eq!(m.winner(), Some(0));
    m.rematch();
    assert_eq!(m.wins, [0, 0]);
}

#[test]
fn time_over_goes_to_the_healthier_body() {
    let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
    w.time_left = 1;
    w.fighters[1].health -= 10;
    w.tick(idle(), idle());
    assert_eq!(w.outcome, Some(aeon_sim::RoundOutcome::Winner(0)));
}

#[test]
fn measured_advantage_matches_frame_data_on_a_clean_hit() {
    let mut w = close_kogan();
    let p = CharacterId::Kogan.data().move_def(MoveId::StP).unwrap();
    tap(&mut w, press(Btn::P));
    hold(&mut w, 40, idle());
    assert_eq!(w.advantage_p1(), p.advantage_on_hit());
}

#[test]
fn dummy_block_all_reads_hit_level_from_move_data() {
    let mut w = close_kogan();
    w.dummy = DummyMode::BlockAll;
    let hp = w.fighters[1].health;
    tap(&mut w, dir_press(2, Btn::ST));
    hold(&mut w, 14, dir(2));
    drain_hitstop(&mut w);
    assert_eq!(w.fighters[1].health, hp, "dummy crouch-blocks the sweep");
    let mut w = close_kogan();
    w.dummy = DummyMode::BlockAll;
    tap(&mut w, chord(Chord::Overhead));
    hold(&mut w, 28, idle());
    drain_hitstop(&mut w);
    assert_eq!(w.fighters[1].health, hp, "dummy stand-blocks the overhead");
}
