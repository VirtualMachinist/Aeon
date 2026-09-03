//! Kogan — the duelist. Saber, revolver, disc-shield. Copper cape is aura.
//!
//! Saber reach carries the SamSho tax. The advancing rekka is his backbone.
//! The revolver is an event fed by the firearm gauge, not a stream. The
//! disc answers shots and steals a close turn. Every number here is
//! recorded in docs/FRAME-DATA.md.

use std::sync::OnceLock;

use crate::chars::{Character, CharacterId, GaugeDef, SpecialRoute};
use crate::geom::{px, LocalBox};
use crate::input::{Btn, Motion};
use crate::moves::{
    leak_boxes, leak_followups, normal, CancelRule, Followup, HitLevel, Invuln, MoveDef, MoveId,
    ProjectileDef, ProjectileKind, ShotBehavior, ThrowKind, TimedBox,
};

pub fn kogan() -> &'static Character {
    static CHAR: OnceLock<Character> = OnceLock::new();
    CHAR.get_or_init(build)
}

const NAMES: &[(MoveId, &str)] = &[
    (MoveId::Rekka1, "saber cut"),
    (MoveId::Rekka2, "backcut"),
    (MoveId::Rekka3, "thrust"),
    (MoveId::Uppercut, "sunward cut"),
    (MoveId::CommandGrab, "cape-snare"),
    (MoveId::CommandDash, "threshold-step"),
    (MoveId::ShotA, "revolver"),
    (MoveId::ShotB, "energy wave"),
    (MoveId::Guard, "disc-shield"),
    (MoveId::SpecialOverhead, "falling saber"),
    (MoveId::AirShot, "air gun"),
    (MoveId::ExA, "EX saber cut"),
    (MoveId::ExB, "EX revolver"),
    (MoveId::Super, "judgment"),
];

fn build() -> Character {
    let jab = LocalBox::new(px(8), px(70), px(36), px(14));
    let slash = LocalBox::new(px(10), px(60), px(62), px(18));
    let heavy_slash = LocalBox::new(px(14), px(54), px(88), px(20));
    let short = LocalBox::new(px(8), px(8), px(40), px(18));
    let flash = LocalBox::new(px(8), px(30), px(54), px(24));
    let style = LocalBox::new(px(12), px(40), px(60), px(22));

    let mut moves = vec![
        // Lights are the footsie glue. 4/2/6, hitstun 12 → +5 on hit.
        normal(
            MoveId::StP,
            4,
            2,
            6,
            40,
            12,
            8,
            HitLevel::Mid,
            CancelRule::OnHitOrBlock,
            jab,
        ),
        // 5K: fastest mid kick, conditions the block. +2 / −2.
        normal(
            MoveId::StK,
            5,
            3,
            8,
            45,
            12,
            8,
            HitLevel::Mid,
            CancelRule::OnHitOrBlock,
            short,
        ),
        // 5S: the mid-long poke. Cancellable, and minus: 7/3/14, 15/11 → −1 / −5.
        normal(
            MoveId::StS,
            7,
            3,
            14,
            70,
            15,
            11,
            HitLevel::Mid,
            CancelRule::OnHitOrBlock,
            slash,
        ),
        // Far HS: the SamSho button. 9/3/20, 16/12 → −6 / −10. Death on whiff.
        {
            let mut m = normal(
                MoveId::StHS,
                9,
                3,
                20,
                110,
                16,
                12,
                HitLevel::Mid,
                CancelRule::OnHit,
                heavy_slash,
            );
            m.pushback_hit = px(18);
            m
        },
        // Close HS: the confirm button. 7/2/14, 18/13 → +3 / −2.
        normal(
            MoveId::StHSClose,
            7,
            2,
            14,
            105,
            18,
            13,
            HitLevel::Mid,
            CancelRule::OnHit,
            LocalBox::new(px(6), px(48), px(44), px(28)),
        ),
        // 5FL: the frame-trap button. Fat hitstun: 7/3/12, 19/13 → +6 / 0.
        normal(
            MoveId::StFL,
            7,
            3,
            12,
            75,
            19,
            13,
            HitLevel::Mid,
            CancelRule::OnHit,
            flash,
        ),
        // 5ST: same family, heavier. 9/3/14, 20/14 → +4 / −1.
        normal(
            MoveId::StST,
            9,
            3,
            14,
            90,
            20,
            14,
            HitLevel::Mid,
            CancelRule::OnHit,
            style,
        ),
        normal(
            MoveId::CrP,
            4,
            2,
            6,
            35,
            11,
            7,
            HitLevel::Mid,
            CancelRule::OnHitOrBlock,
            LocalBox::new(px(8), px(40), px(34), px(14)),
        ),
        // 2K: the low poke. Fast, low, slightly minus.
        normal(
            MoveId::CrK,
            4,
            2,
            8,
            40,
            11,
            7,
            HitLevel::Low,
            CancelRule::OnHitOrBlock,
            LocalBox::new(px(8), px(0), px(42), px(16)),
        ),
        normal(
            MoveId::CrS,
            6,
            3,
            12,
            65,
            15,
            11,
            HitLevel::Mid,
            CancelRule::OnHitOrBlock,
            LocalBox::new(px(10), px(38), px(56), px(16)),
        ),
        // 2HS: the tall anti-air slash. −4 / −8.
        normal(
            MoveId::CrHS,
            8,
            3,
            18,
            100,
            16,
            12,
            HitLevel::Mid,
            CancelRule::OnHit,
            LocalBox::new(px(8), px(36), px(40), px(56)),
        ),
        // 2FL: low-profile trap button. 6/3/12, 18/12 → +5 / −1.
        normal(
            MoveId::CrFL,
            6,
            3,
            12,
            70,
            18,
            12,
            HitLevel::Mid,
            CancelRule::OnHit,
            LocalBox::new(px(8), px(14), px(50), px(20)),
        ),
        // 2ST: the sweep. Hard knockdown, −12 on block.
        {
            let mut m = normal(
                MoveId::CrST,
                8,
                3,
                22,
                90,
                20,
                12,
                HitLevel::Low,
                CancelRule::Never,
                LocalBox::new(px(10), px(0), px(62), px(18)),
            );
            m.knockdown = true;
            m
        },
        // Air normals swing downward: boxes sit at and below the airborne
        // body so a hop-in reaches a grounded opponent.
        jump_normal(
            MoveId::JP,
            4,
            8,
            4,
            40,
            LocalBox::new(px(8), px(4), px(34), px(24)),
        ),
        jump_normal(
            MoveId::JK,
            4,
            8,
            4,
            40,
            LocalBox::new(px(6), px(-4), px(38), px(24)),
        ),
        // Air saber: j.S short, j.HS the long line.
        jump_normal(
            MoveId::JS,
            5,
            6,
            6,
            65,
            LocalBox::new(px(6), px(-8), px(56), px(36)),
        ),
        jump_normal(
            MoveId::JHS,
            6,
            5,
            8,
            90,
            LocalBox::new(px(8), px(-12), px(84), px(34)),
        ),
        // j.FL is a knee when the cylinder is empty; otherwise j.FL is the air gun.
        jump_normal(
            MoveId::JFL,
            5,
            6,
            6,
            55,
            LocalBox::new(px(6), px(-2), px(40), px(30)),
        ),
        jump_normal(
            MoveId::JST,
            6,
            5,
            8,
            90,
            LocalBox::new(px(10), px(-10), px(54), px(32)),
        ),
        overhead(),
        throw(),
        rekka1(MoveId::Rekka1, false),
        rekka2(),
        rekka3(),
        wave(),
        revolver(MoveId::ShotA, false),
        disc_shield(),
        sunward_cut(),
        cape_snare(),
        threshold_step(),
        falling_saber(),
        air_gun(),
        rekka1(MoveId::ExA, true),
        revolver(MoveId::ExB, true),
        judgment(),
    ];

    for m in &mut moves {
        if m.id.is_jumping() && m.id != MoveId::AirShot {
            m.level = HitLevel::High;
        }
    }

    Character {
        id: CharacterId::Kogan,
        walk_fwd: px(3),
        walk_back: px(2),
        run_speed: px(6),
        jump_x: px(4),
        jump_y: px(13),
        hop_x: px(4),
        hop_y: px(8),
        gravity: px(1) / 2 + 32, // ~0.625 px/f²
        max_health: 1000,
        push_w: px(32),
        stand_h: px(96),
        crouch_h: px(62),
        throw_range: px(36),
        close_range: px(52),
        color: 0xD4_6A_4C,
        aura: Some(LocalBox::new(px(-54), px(4), px(128), px(112))),
        gauge: GaugeDef {
            max: 6,
            start: 6,
            regen_delay: 90,
            regen_every: 60,
            regen_amount: 1,
            buff_step: 0,
            label: "CYL",
        },
        poke_heavy: MoveId::CrHS,
        weapon_heavy: MoveId::StHS,
        space_controls: &[MoveId::ShotA, MoveId::ShotB],
        reversal: MoveId::Uppercut,
        moves,
        specials: vec![
            SpecialRoute::ground(Motion::SuperQcf, &[Btn::S], MoveId::Super),
            SpecialRoute::ground(Motion::Hcb, &[Btn::FL], MoveId::CommandGrab),
            SpecialRoute::ground(Motion::Dp, &[Btn::S, Btn::HS], MoveId::Uppercut),
            SpecialRoute::ground(Motion::Qcf, &[Btn::S], MoveId::Rekka1),
            SpecialRoute::ground(Motion::Qcf, &[Btn::HS], MoveId::ShotB),
            SpecialRoute::ground(Motion::Qcf, &[Btn::FL], MoveId::CommandDash),
            SpecialRoute::ground(Motion::Qcf, &[Btn::ST], MoveId::SpecialOverhead),
            SpecialRoute::ground(Motion::Qcb, &[Btn::S], MoveId::ShotA),
            SpecialRoute::ground(Motion::Qcb, &[Btn::HS], MoveId::Guard),
            SpecialRoute::air(Motion::None, &[Btn::FL], MoveId::AirShot),
            SpecialRoute::ex(Motion::Qcf, MoveId::ExA),
            SpecialRoute::ex(Motion::Qcb, MoveId::ExB),
        ],
        names: NAMES,
    }
}

fn jump_normal(
    id: MoveId,
    startup: u8,
    active: u8,
    recovery: u8,
    damage: i32,
    hit: LocalBox,
) -> MoveDef {
    let mut m = normal(
        id,
        startup,
        active,
        recovery,
        damage,
        14,
        8,
        HitLevel::High,
        CancelRule::Never,
        hit,
    );
    m.pushback_hit = px(8);
    m
}

/// HS+ST. The universal dust: slow, High, links on hit, minus on block.
/// 22/3/16, 22/12 → +4 / −6.
fn overhead() -> MoveDef {
    normal(
        MoveId::Overhead,
        22,
        3,
        16,
        95,
        22,
        12,
        HitLevel::High,
        CancelRule::Never,
        LocalBox::new(px(6), px(30), px(52), px(60)),
    )
}

/// P+K. Beats both blocks, techable, jabbable. 2/1/20.
fn throw() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Throw);
    m.startup = 2;
    m.active = 1;
    m.recovery = 20;
    m.damage = 140;
    m.hitstop = 4;
    m.pushback_hit = px(0);
    m.knockdown = true;
    m.throw = ThrowKind::Normal;
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        2,
        3,
        LocalBox::new(px(4), px(20), px(32), px(40)),
    )]);
    m.meter_on_hit = 20;
    m.feintable = false;
    m
}

/// 236+S saber cut (part 1). Advancing, safest. 8/3/14, 16/12 → 0 / −4.
/// EX (S+HS, 2 chambers): 6/3/12, 20/16 → +6 / +2.
fn rekka1(id: MoveId, ex: bool) -> MoveDef {
    let mut m = MoveDef::special(id);
    if ex {
        m.startup = 6;
        m.active = 3;
        m.recovery = 12;
        m.damage = 80;
        m.chip = 10;
        m.hitstun = 20;
        m.blockstun = 16;
        m.gauge_cost = 2;
        m.vel_x = px(6);
        m.hitstop = 10;
    } else {
        m.startup = 8;
        m.active = 3;
        m.recovery = 14;
        m.damage = 60;
        m.chip = 6;
        m.hitstun = 16;
        m.blockstun = 12;
        m.vel_x = px(5);
    }
    m.vel_frames = 8;
    m.pushback_hit = px(6);
    m.pushback_block = px(6);
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        m.startup,
        m.startup + m.active,
        LocalBox::new(px(12), px(48), px(70), px(22)),
    )]);
    m.meter_on_hit = 10;
    m.followups = leak_followups(vec![Followup {
        button: Btn::S,
        next: MoveId::Rekka2,
        from: m.startup,
        to: m.startup + m.active + 11,
    }]);
    m
}

/// Part 2 backcut: the confirm. 7/3/14, 17/11 → +1 / −5.
fn rekka2() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Rekka2);
    m.startup = 7;
    m.active = 3;
    m.recovery = 14;
    m.damage = 70;
    m.chip = 6;
    m.hitstun = 17;
    m.blockstun = 11;
    m.vel_x = px(4);
    m.vel_frames = 6;
    m.pushback_hit = px(6);
    m.pushback_block = px(8);
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        7,
        10,
        LocalBox::new(px(10), px(40), px(72), px(26)),
    )]);
    m.meter_on_hit = 10;
    m.followups = leak_followups(vec![Followup {
        button: Btn::S,
        next: MoveId::Rekka3,
        from: 7,
        to: 21,
    }]);
    m
}

/// Part 3 thrust: the tax or the ender. 10/3/22, KD, −12 on block.
fn rekka3() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Rekka3);
    m.startup = 10;
    m.active = 3;
    m.recovery = 22;
    m.damage = 90;
    m.chip = 8;
    m.hitstun = 24;
    m.blockstun = 12;
    m.knockdown = true;
    m.vel_x = px(6);
    m.vel_frames = 8;
    m.pushback_hit = px(16);
    m.pushback_block = px(14);
    m.hitstop = 10;
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        10,
        13,
        LocalBox::new(px(14), px(46), px(84), px(20)),
    )]);
    m.meter_on_hit = 14;
    m
}

/// 236+HS energy wave. Slow, short-lived sword wave: oki and frame traps,
/// not a neutral war. 14/2/22; the shot lives 40f at 3 px/f (~120px).
fn wave() -> MoveDef {
    let mut m = MoveDef::special(MoveId::ShotB);
    m.startup = 14;
    m.active = 2;
    m.recovery = 22;
    m.projectile = Some(ProjectileDef {
        kind: ProjectileKind::Wave,
        behavior: ShotBehavior::Travel,
        spawn: LocalBox::new(px(30), 0, 0, 0),
        vel_x: px(3),
        vel_y: 0,
        gravity: 0,
        lifetime: 40,
        damage: 60,
        chip: 8,
        hitstun: 20,
        blockstun: 18,
        hitstop: 8,
        pushback: px(8),
        hitbox: LocalBox::new(px(0), px(16), px(22), px(44)),
        blast: None,
        level: HitLevel::Mid,
        knockdown: false,
    });
    m.meter_on_hit = 8;
    m
}

/// 214+S revolver, one chamber. 11/1/24. EX (2 chambers): 9/1/22, heavy
/// round that knocks down and wins shot clashes.
fn revolver(id: MoveId, ex: bool) -> MoveDef {
    let mut m = MoveDef::special(id);
    if ex {
        m.startup = 9;
        m.active = 1;
        m.recovery = 22;
        m.gauge_cost = 2;
        m.projectile = Some(ProjectileDef {
            kind: ProjectileKind::Revolver,
            behavior: ShotBehavior::Travel,
            spawn: LocalBox::new(px(28), px(56), 0, 0),
            vel_x: px(12),
            vel_y: 0,
            gravity: 0,
            lifetime: 90,
            damage: 110,
            chip: 14,
            hitstun: 24,
            blockstun: 16,
            hitstop: 10,
            pushback: px(14),
            hitbox: LocalBox::new(px(0), px(0), px(28), px(22)),
            blast: None,
            level: HitLevel::Mid,
            knockdown: true,
        });
    } else {
        m.startup = 11;
        m.active = 1;
        m.recovery = 24;
        m.gauge_cost = 1;
        m.projectile = Some(ProjectileDef {
            kind: ProjectileKind::Revolver,
            behavior: ShotBehavior::Travel,
            spawn: LocalBox::new(px(28), px(56), 0, 0),
            vel_x: px(9),
            vel_y: 0,
            gravity: 0,
            lifetime: 90,
            damage: 70,
            chip: 10,
            hitstun: 16,
            blockstun: 14,
            hitstop: 8,
            pushback: px(10),
            hitbox: LocalBox::new(px(0), px(0), px(24), px(20)),
            blast: None,
            level: HitLevel::Mid,
            knockdown: false,
        });
    }
    m.meter_on_hit = 12;
    m
}

/// 214+HS disc-shield. 8/5/12, hitstun 17 / blockstun 19 → +3 on block.
/// Active frames destroy opposing shots.
fn disc_shield() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Guard);
    m.startup = 8;
    m.active = 5;
    m.recovery = 12;
    m.damage = 75;
    m.chip = 8;
    m.hitstun = 17;
    m.blockstun = 19;
    m.pushback_hit = px(6);
    m.pushback_block = px(4);
    m.projectile_guard = true;
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        8,
        13,
        LocalBox::new(px(2), px(10), px(52), px(82)),
    )]);
    m.meter_on_hit = 10;
    m
}

/// 623+S/HS sunward cut. 3/10/24 + 12 landing. Invuln 1–6. Launch, KD.
fn sunward_cut() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Uppercut);
    m.startup = 3;
    m.active = 10;
    m.recovery = 24;
    m.damage = 140;
    m.chip = 16;
    m.hitstun = 20;
    m.blockstun = 12;
    m.hitstop = 12;
    m.blockstop = 8;
    m.pushback_hit = px(6);
    m.pushback_block = px(12);
    m.knockdown = true;
    m.launch = px(10);
    m.invuln = Invuln::full(1, 7);
    m.hitboxes = leak_boxes(vec![
        TimedBox::span(3, 6, LocalBox::new(px(8), px(40), px(36), px(50))),
        TimedBox::span(6, 13, LocalBox::new(px(4), px(70), px(28), px(48))),
    ]);
    m.vel_x = px(3);
    m.vel_y = px(12);
    m.gravity_override = Some(px(1) / 2);
    m.land_recovery = 12;
    m.meter_on_hit = 18;
    m
}

/// 63214+FL cape-snare. The cape is the visual; the box is his arms.
/// 6/2/38, untechable, beats both blocks, loses to uppercut invuln.
fn cape_snare() -> MoveDef {
    let mut m = MoveDef::special(MoveId::CommandGrab);
    m.startup = 6;
    m.active = 2;
    m.recovery = 38;
    m.damage = 160;
    m.hitstop = 6;
    m.knockdown = true;
    m.throw = ThrowKind::Command;
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        6,
        8,
        LocalBox::new(px(4), px(20), px(44), px(50)),
    )]);
    m.meter_on_hit = 24;
    m
}

/// 236+FL threshold-step. 16 frames, 60px, no hitbox, no cancel.
fn threshold_step() -> MoveDef {
    let mut m = MoveDef::special(MoveId::CommandDash);
    m.startup = 0;
    m.active = 0;
    m.recovery = 16;
    m.vel_x = px(5);
    m.vel_frames = 12;
    m.feintable = false;
    m
}

/// 236+ST falling saber. Leaping overhead: 18/4/14 + 8 landing. High, KD.
fn falling_saber() -> MoveDef {
    let mut m = MoveDef::special(MoveId::SpecialOverhead);
    m.startup = 18;
    m.active = 4;
    m.recovery = 14;
    m.damage = 100;
    m.chip = 12;
    m.hitstun = 22;
    m.blockstun = 14;
    m.hitstop = 10;
    m.level = HitLevel::High;
    m.knockdown = true;
    m.pushback_hit = px(8);
    m.pushback_block = px(10);
    m.vel_x = px(4);
    m.vel_y = px(7);
    m.land_recovery = 8;
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        18,
        22,
        LocalBox::new(px(10), px(10), px(50), px(64)),
    )]);
    m.meter_on_hit = 12;
    m
}

/// j.FL air gun, one chamber. 8/1/12. Angled-down shot.
fn air_gun() -> MoveDef {
    let mut m = MoveDef::special(MoveId::AirShot);
    m.startup = 8;
    m.active = 1;
    m.recovery = 12;
    m.gauge_cost = 1;
    m.projectile = Some(ProjectileDef {
        kind: ProjectileKind::AirShot,
        behavior: ShotBehavior::Travel,
        spawn: LocalBox::new(px(20), px(40), 0, 0),
        vel_x: px(6),
        vel_y: -px(6),
        gravity: 0,
        lifetime: 60,
        damage: 50,
        chip: 6,
        hitstun: 14,
        blockstun: 12,
        hitstop: 7,
        pushback: px(8),
        hitbox: LocalBox::new(px(0), px(0), px(18), px(18)),
        blast: None,
        level: HitLevel::Mid,
        knockdown: false,
    });
    m.meter_on_hit = 8;
    m.feintable = false;
    m
}

/// 236236+S judgment. 4/8/26, 1000 bar, 280.
fn judgment() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Super);
    m.startup = 4;
    m.active = 8;
    m.recovery = 26;
    m.damage = 280;
    m.chip = 24;
    m.hitstun = 18;
    m.blockstun = 14;
    m.hitstop = 14;
    m.blockstop = 8;
    m.pushback_hit = px(16);
    m.pushback_block = px(14);
    m.knockdown = true;
    m.launch = px(4);
    m.invuln = Invuln::full(0, 4);
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        4,
        12,
        LocalBox::new(px(0), px(10), px(70), px(80)),
    )]);
    m.vel_x = px(8);
    m.meter_cost = 1000;
    m
}
