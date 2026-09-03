//! QA scripted trials 1–10 (Aeon/QA.md), driven headlessly so reviewers can
//! grade them with `cargo test` in addition to playing them.
//!
//! Level law (DESIGN.md): a hop-in normal is High — it must be stand-blocked
//! and beats a crouch block. 2K is Low — it must be crouch-blocked and beats
//! a stand block. That pair is the hop mix.

mod common;

use aeon_sim::fighter::Action;
use aeon_sim::geom::px;
use aeon_sim::input::{Btn, Chord, InputFrame};
use aeon_sim::moves::MoveId;
use aeon_sim::{CharacterId, DummyMode, EventKind, ProjectileKind, World, RC_COST};
use common::*;

/// Trial 1 — Jab link. Close P into P on hit. Two hits, not a chain-cancel.
#[test]
fn trial_01_jab_link() {
    let mut w = close_kogan();
    tap(&mut w, press(Btn::P));
    // Mash P during the first jab: a chain would produce a second hit
    // immediately. Nothing should happen until the first jab recovers.
    for _ in 0..6 {
        w.tick(press(Btn::P), idle());
        w.tick(idle(), idle());
    }
    drain_hitstop(&mut w);
    assert_eq!(
        w.fighters[1].combo, 1,
        "mashing inside the first jab does not chain"
    );
    while !w.fighters[0].action.actionable() {
        w.tick(idle(), idle());
    }
    tap(&mut w, press(Btn::P));
    hold(&mut w, 8, idle());
    drain_hitstop(&mut w);
    assert_eq!(w.fighters[1].combo, 2, "the link lands as the second hit");
}

/// Trial 2 — Whiff tax. Far HS whiffs a hair outside its reach; the opponent walks
/// in and jabs before recovery ends.
#[test]
fn trial_02_whiff_tax() {
    for id in [CharacterId::Kogan, CharacterId::Raya] {
        let mut w = free(id, CharacterId::Kogan);
        let hs = id.data().move_def(MoveId::StHS).unwrap();
        let reach = hs.hitboxes[0].hit.x + hs.hitboxes[0].hit.w;
        w.fighters[0].pos.x = px(300);
        // Just past the tip, accounting for the defender's hurtbox front edge.
        w.fighters[1].pos.x = px(300) + reach + px(20) + px(6);
        tap(&mut w, press(Btn::HS));
        // Defender waits for the blade to pass (running into a live heavy
        // gets you cut), then runs in (66, hold 6) and pokes with 5S.
        let mut punished = false;
        let mut poked = false;
        let mut steps = 0;
        for _ in 0..(hs.total_frames() as u32 + 4) {
            let whiffed = w.fighters[0]
                .action
                .attacking()
                .map(|(_, f, _)| f >= hs.last_active())
                .unwrap_or(true);
            let p2 = if !whiffed {
                idle()
            } else if !poked && w.fighters[1].last_distance <= px(90) {
                poked = true;
                dir_press(6, Btn::S)
            } else if poked {
                idle()
            } else {
                steps += 1;
                if steps == 2 {
                    dir(5)
                } else {
                    dir(6)
                }
            };
            w.tick(idle(), p2);
            punished |= has_event(&w, EventKind::Punish);
        }
        drain_hitstop(&mut w);
        assert!(
            w.fighters[0].health < id.data().max_health,
            "{}: the whiffed heavy is punished",
            id.name()
        );
        assert!(punished, "{}: logged as a punish", id.name());
    }
}

/// Trial 3 — Disc vs voice. Raya hangs voice; Kogan 214+HS destroys the shot and
/// keeps his turn (is actionable before she is).
#[test]
fn trial_03_disc_vs_voice() {
    let mut w = free(CharacterId::Kogan, CharacterId::Raya);
    w.fighters[0].pos.x = px(320);
    w.fighters[1].pos.x = px(470);
    // Raya casts. Kogan reads the startup and answers on her frame 6.
    motion_p2(&mut w, &[2, 3, 6], Btn::HS);
    assert!(attacking(&w, 1, MoveId::ShotB));
    hold(&mut w, 5, idle());
    motion(&mut w, &[2, 1, 4], Btn::HS);
    assert!(attacking(&w, 0, MoveId::Guard));
    let guarded = run_until(&mut w, 30, idle(), idle(), |w| {
        w.events
            .iter()
            .any(|e| e.kind == EventKind::ProjectileGuard)
    });
    assert!(guarded.is_some(), "the disc eats the glyph");
    assert_eq!(w.fighters[0].health, 1000, "Kogan took nothing");
    let kogan_free = frames_until_actionable(&mut w, 0, 60);
    assert!(
        !w.fighters[1].action.actionable(),
        "Raya is still recovering: Kogan kept his turn ({} f)",
        kogan_free
    );
}

/// Trial 4 — Sandwich. Raya knockdown → crystal in front → glide behind → the
/// defender is between body and crystal.
#[test]
fn trial_04_sandwich() {
    let mut w = free(CharacterId::Raya, CharacterId::Kogan);
    w.fighters[0].pos.x = px(360);
    w.fighters[1].pos.x = px(410);
    knock_down_p2(&mut w);
    // Plant, motion buffered during the sweep's recovery.
    buffered_special(&mut w, &[2, 1, 4], Btn::S);
    assert!(
        attacking(&w, 0, MoveId::ShotA),
        "got {}",
        w.fighters[0].action.name()
    );
    // Glide through, motion buffered during the plant's recovery.
    buffered_special(&mut w, &[2, 3, 6], Btn::FL);
    assert!(
        attacking(&w, 0, MoveId::CommandDash),
        "got {}",
        w.fighters[0].action.name()
    );
    assert!(w
        .projectiles
        .iter()
        .any(|p| p.def.kind == ProjectileKind::Crystal));
    let glide = CharacterId::Raya
        .data()
        .move_def(MoveId::CommandDash)
        .unwrap();
    hold(&mut w, glide.total_frames() as u32 - 1, idle());
    let raya = w.fighters[0].pos.x;
    let kogan = w.fighters[1].pos.x;
    let crystal = w
        .projectiles
        .iter()
        .find(|p| p.def.kind == ProjectileKind::Crystal)
        .expect("crystal");
    let cbox = crystal.hitbox();
    let body = w.fighters[1].pushbox();
    // Raya is on the far side; the crystal sits on the near side of, or
    // under, the downed body.
    assert!(
        raya > kogan + px(30),
        "Raya glided behind: raya {} kogan {}",
        raya / 256,
        kogan / 256
    );
    assert!(
        cbox.left < body.right && cbox.left < raya - px(40),
        "crystal footprint {}..{} is at or before the body {}..{}, not on Raya's side",
        cbox.left / 256,
        cbox.right / 256,
        body.left / 256,
        body.right / 256
    );
    assert!(
        matches!(
            w.fighters[1].action,
            Action::Knockdown { .. } | Action::Getup { .. }
        ),
        "he is still getting up"
    );
}

/// Trial 5 — Hop mix. Hop-in normal (High) is blocked standing and beats crouch
/// block. Empty hop into 2K (Low) beats stand block.
#[test]
fn trial_05_hop_mix() {
    fn hop_attack(p2: InputFrame) -> bool {
        let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
        w.fighters[0].pos.x = px(360);
        w.fighters[1].pos.x = px(410);
        let hp = w.fighters[1].health;
        w.tick(dir(9), p2);
        for _ in 0..8 {
            w.tick(idle(), p2);
        }
        assert!(w.fighters[0].action.is_hop());
        w.tick(press(Btn::S), p2);
        for _ in 0..30 {
            w.tick(idle(), p2);
        }
        drain_hitstop(&mut w);
        w.fighters[1].health < hp
    }
    assert!(!hop_attack(back()), "hop-in normal is blocked standing");
    assert!(hop_attack(down_back()), "hop-in normal beats crouch block");

    // Empty hop into 2K.
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[0].pos.x = px(360);
    w.fighters[1].pos.x = px(410);
    let hp = w.fighters[1].health;
    w.tick(dir(9), back());
    let landed = run_until(&mut w, 60, idle(), back(), |w| {
        !w.fighters[0].airborne && w.fighters[0].action.actionable()
    });
    assert!(landed.is_some(), "empty hop lands");
    w.tick(dir_press(2, Btn::K), back());
    hold2(&mut w, 8, dir(2), back());
    drain_hitstop(&mut w);
    assert!(w.fighters[1].health < hp, "empty hop 2K beats stand block");
}

/// Trial 6 — Oki triangle. After KD: command grab beats a holding-block dummy;
/// uppercut beats the command grab; a delayed meaty strike beats a button
/// press; a delayed block bait punishes a wakeup 623.
#[test]
fn trial_06_oki_triangle() {
    fn knocked_down(dummy: DummyMode) -> World {
        let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
        w.fighters[0].pos.x = px(370);
        w.fighters[1].pos.x = px(408);
        knock_down_p2(&mut w);
        // Arm the dummy behaviour once it is on the floor.
        w.dummy = dummy;
        while !w.fighters[0].action.actionable() {
            w.tick(idle(), idle());
        }
        w
    }
    fn getup_frames_left(w: &World) -> u32 {
        match w.fighters[1].action {
            Action::Knockdown { frame } => {
                (aeon_sim::KNOCKDOWN_FRAMES - frame + aeon_sim::GETUP_FRAMES) as u32
            }
            Action::Getup { frame } => (aeon_sim::GETUP_FRAMES - frame) as u32,
            _ => 0,
        }
    }

    // a. Command grab beats block.
    let mut w = knocked_down(DummyMode::BlockAll);
    let hp = w.fighters[1].health;
    while getup_frames_left(&w) > 12 {
        w.tick(dir(6), idle());
    }
    w.fighters[0].pos.x = w.fighters[1].pos.x - px(34);
    for d in [6, 3, 2, 1, 4] {
        w.tick(dir(d), idle());
    }
    w.tick(dir_press(4, Btn::FL), idle());
    assert!(attacking(&w, 0, MoveId::CommandGrab));
    hold(&mut w, 30, idle());
    drain_hitstop(&mut w);
    assert!(w.fighters[1].health < hp, "grab beats the blocker");

    // b. Uppercut beats the command grab.
    let mut w = knocked_down(DummyMode::WakeupDp);
    while getup_frames_left(&w) > 8 {
        w.tick(idle(), idle());
    }
    w.fighters[0].pos.x = w.fighters[1].pos.x - px(34);
    for d in [6, 3, 2, 1, 4] {
        w.tick(dir(d), idle());
    }
    w.tick(dir_press(4, Btn::FL), idle());
    assert!(attacking(&w, 0, MoveId::CommandGrab));
    let mut grabbed = false;
    for _ in 0..30 {
        w.tick(idle(), idle());
        grabbed |= has_event(&w, EventKind::Grab);
    }
    drain_hitstop(&mut w);
    assert!(!grabbed, "wakeup 623 is not grabbed");
    assert!(w.fighters[0].health < 1000, "and it hits the grabber");

    // c. Delayed meaty strike beats a button press.
    let mut w = knocked_down(DummyMode::WakeupP);
    while getup_frames_left(&w) > 6 {
        w.tick(idle(), idle());
    }
    w.fighters[0].pos.x = w.fighters[1].pos.x - px(40);
    tap(&mut w, press(Btn::S));
    hold(&mut w, 14, idle());
    drain_hitstop(&mut w);
    assert!(
        w.fighters[1].health < 1000,
        "meaty S beats the wakeup button"
    );
    assert_eq!(w.fighters[0].health, 1000);

    // d. Block bait punishes a wakeup 623.
    let mut w = knocked_down(DummyMode::WakeupDp);
    w.fighters[0].pos.x = w.fighters[1].pos.x - px(44);
    let dp = run_until(&mut w, 80, back(), idle(), |w| {
        attacking(w, 1, MoveId::Uppercut)
    });
    assert!(dp.is_some(), "dummy wakes up with 623");
    // Hold block through it, then punish the landing.
    let landed = run_until(&mut w, 90, back(), idle(), |w| {
        matches!(w.fighters[1].action, Action::Landing { .. })
            || (attacking(w, 1, MoveId::Uppercut)
                && !w.fighters[1].airborne
                && w.fighters[1]
                    .action
                    .attacking()
                    .map(|(_, f, _)| f > 20)
                    .unwrap_or(false))
    });
    assert!(landed.is_some(), "the uppercut whiffs and falls");
    assert_eq!(w.fighters[0].health, 1000, "the bait was not hit");
    w.fighters[0].pos.x = w.fighters[1].pos.x - px(40);
    tap(&mut w, press(Btn::HS));
    let mut punished = false;
    for _ in 0..16 {
        w.tick(idle(), idle());
        punished |= has_event(&w, EventKind::Punish);
    }
    assert!(punished, "the whiffed uppercut eats a heavy");
}

/// Trial 7 — Uppercut xx RC. Defender 623 hits, spends 250, continues. Without
/// 250, they fall into the tax.
#[test]
fn trial_07_uppercut_rc() {
    fn wake_dp(meter: i32) -> (World, bool) {
        let mut w = close_kogan();
        w.dummy = DummyMode::CpuOff;
        w.fighters[1].meter = meter;
        // P1 pokes into P2's uppercut.
        motion_p2(&mut w, &[6, 2, 3], Btn::S);
        assert!(attacking(&w, 1, MoveId::Uppercut));
        hold(&mut w, 4, idle());
        drain_hitstop(&mut w);
        assert!(w.fighters[0].action.in_hitstun(), "uppercut lands");
        let before = w.fighters[1].meter;
        w.tick(idle(), chord(Chord::RomanCancel));
        let spent = w.fighters[1].meter == before - RC_COST;
        (w, spent)
    }
    let (mut w, spent) = wake_dp(250);
    assert!(spent, "RC spent 250");
    drain_hitstop(&mut w);
    let free_in = frames_until_actionable(&mut w, 1, 40);
    assert!(
        free_in <= 4,
        "with 250 the round jumps: actionable in {} frames",
        free_in
    );
    assert!(
        w.fighters[0].action.in_hitstun() || w.fighters[0].airborne,
        "while the opponent is still launched"
    );

    let (mut w, spent) = wake_dp(0);
    assert!(!spent);
    let free_in = frames_until_actionable(&mut w, 1, 120);
    assert!(
        free_in >= 30,
        "without 250 the tax lands: {} frames",
        free_in
    );
}

/// Trial 8 — Feint DP. Attacker feints a special; dummy 623s; attacker punishes.
#[test]
fn trial_08_feint_dp() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[0].pos.x = px(360);
    w.fighters[1].pos.x = px(480);
    motion(&mut w, &[2, 3, 6], Btn::S);
    assert!(attacking(&w, 0, MoveId::Rekka1));
    // P2 reads the flash and mashes 623+S.
    tap(&mut w, chord(Chord::Feint));
    assert!(matches!(w.fighters[0].action, Action::Feint { .. }));
    motion_p2(&mut w, &[6, 2, 3], Btn::S);
    assert!(attacking(&w, 1, MoveId::Uppercut));
    // The uppercut whiffs (P1 is out of its reach and the rekka never came).
    let landed = run_until(&mut w, 90, idle(), idle(), |w| {
        matches!(w.fighters[1].action, Action::Landing { .. })
    });
    assert!(landed.is_some());
    assert_eq!(w.fighters[0].health, 1000, "the feinter was never hit");
    w.fighters[0].pos.x = w.fighters[1].pos.x - px(40);
    tap(&mut w, press(Btn::HS));
    let mut punished = false;
    for _ in 0..16 {
        w.tick(idle(), idle());
        punished |= has_event(&w, EventKind::Punish);
    }
    assert!(punished, "and collects the punish");
}

/// Trial 9 — Throw tech / jab. Simultaneous P+K vs P+K techs. Defender P on the
/// throw frame beats the throw.
#[test]
fn trial_09_throw_tech_and_jab() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.tick(chord(Chord::Throw), chord(Chord::Throw));
    hold(&mut w, 4, idle());
    assert!(matches!(w.fighters[0].action, Action::ThrowTech { .. }));
    assert!(matches!(w.fighters[1].action, Action::ThrowTech { .. }));

    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.tick(idle(), press(Btn::P));
    w.tick(idle(), idle());
    w.tick(chord(Chord::Throw), idle());
    hold(&mut w, 3, idle());
    drain_hitstop(&mut w);
    assert!(w.fighters[0].health < 1000, "hit beats throw");
    assert_eq!(w.fighters[1].health, 1000);
}

/// Trial 10 — Cape. Hurtbox is the man. A jab aimed at cape-only space does not hit.
#[test]
fn trial_10_cape_is_not_a_box() {
    let mut w = free(CharacterId::Kogan, CharacterId::Kogan);
    w.fighters[0].pos.x = px(300);
    let f = &w.fighters[0];
    let aura = f.visual_aura_box().unwrap();
    let hurt = f.hurtboxes()[0];
    assert!(
        aura.right > hurt.right + px(40),
        "the cape flares well past the body"
    );
    // P2 stands so its jab tip lands inside the cape but short of the body.
    w.fighters[1].pos.x = px(300) + px(76);
    w.fighters[1].facing_right = false;
    let jab = CharacterId::Kogan.data().move_def(MoveId::StP).unwrap();
    let jab_world = jab.hitboxes[0].hit.to_world(w.fighters[1].pos, false);
    assert!(jab_world.overlaps(aura), "the jab is inside the aura");
    assert!(!jab_world.overlaps(hurt), "but not inside the hurtbox");
    w.tick(idle(), press(Btn::P));
    hold(&mut w, 8, idle());
    assert_eq!(w.fighters[0].health, 1000, "cape-only space does not hit");
    assert!(!has_event(&w, EventKind::Hit));
}
