//! Raya — the officiant. Voice and energy-infused crystals.
//!
//! Voice is a *placed* hanging glyph, not a hadoken. Crystals arc, plant,
//! arm, and detonate. The rite is the command grab. Charge is a buff on the
//! crystal gauge, not a stored attack. She still footsies: her far HS is the
//! identity button. Every number here is recorded in docs/FRAME-DATA.md.

use std::sync::OnceLock;

use crate::chars::{Character, CharacterId, GaugeDef, SpecialRoute};
use crate::geom::{px, LocalBox};
use crate::input::{Btn, Motion};
use crate::moves::{
    leak_boxes, leak_followups, normal, CancelRule, ChannelDef, Followup, HitLevel, Invuln,
    MoveDef, MoveId, ProjectileDef, ProjectileKind, ShotBehavior, ThrowKind, TimedBox,
};

pub fn raya() -> &'static Character {
    static CHAR: OnceLock<Character> = OnceLock::new();
    CHAR.get_or_init(build)
}

const NAMES: &[(MoveId, &str)] = &[
    (MoveId::Rekka1, "chant I"),
    (MoveId::Rekka2, "chant II"),
    (MoveId::Rekka3, "chant III"),
    (MoveId::Uppercut, "ascension"),
    (MoveId::CommandGrab, "the rite"),
    (MoveId::CommandDash, "processional"),
    (MoveId::ShotA, "crystal"),
    (MoveId::ShotB, "voice glyph"),
    (MoveId::Detonate, "shatter"),
    (MoveId::Charge, "consecrate"),
    (MoveId::ExA, "EX glyph"),
    (MoveId::ExB, "EX crystal"),
    (MoveId::Super, "convergence"),
];

fn build() -> Character {
    // The far slash: a weapon. 88px of reach. 11/3/22. Whiff and die.
    let far_slash = LocalBox::new(px(16), px(58), px(88), px(16));
    let close_slash = LocalBox::new(px(6), px(40), px(48), px(36));
    let med_slash = LocalBox::new(px(12), px(56), px(70), px(18));
    let palm = LocalBox::new(px(8), px(62), px(46), px(14));
    let kick = LocalBox::new(px(8), px(8), px(40), px(18));
    let flash = LocalBox::new(px(8), px(34), px(52), px(24));
    let style = LocalBox::new(px(12), px(44), px(58), px(22));

    let mut moves = vec![
        // 5P: a palm of light. 5/2/8, 14/9 → +5 / 0. Links into itself.
        {
            let mut m = normal(
                MoveId::StP,
                5,
                2,
                8,
                55,
                14,
                9,
                HitLevel::Mid,
                CancelRule::OnHitOrBlock,
                palm,
            );
            m.pushback_hit = px(10);
            m
        },
        normal(
            MoveId::StK,
            5,
            3,
            9,
            50,
            12,
            8,
            HitLevel::Mid,
            CancelRule::OnHitOrBlock,
            kick,
        ),
        // 5S: 8/3/14, 16/12 → 0 / −4. Cancellable.
        {
            let mut m = normal(
                MoveId::StS,
                8,
                3,
                14,
                90,
                16,
                12,
                HitLevel::Mid,
                CancelRule::OnHitOrBlock,
                med_slash,
            );
            m.pushback_hit = px(16);
            m
        },
        // Far HS — the identity button. 11/3/22, 18/13 → −6 / −8. You do not
        // combo from this. You *whiff punish* with it.
        {
            let mut m = normal(
                MoveId::StHS,
                11,
                3,
                22,
                150,
                18,
                13,
                HitLevel::Mid,
                CancelRule::Never,
                far_slash,
            );
            m.pushback_hit = px(22);
            m.pushback_block = px(16);
            m
        },
        // Close HS: the confirm. 7/2/16, 20/14 → +3 / −3.
        {
            let mut m = normal(
                MoveId::StHSClose,
                7,
                2,
                16,
                130,
                20,
                14,
                HitLevel::Mid,
                CancelRule::OnHit,
                close_slash,
            );
            m.pushback_hit = px(10);
            m
        },
        // 5FL: frame-trap button. 7/3/13, 20/14 → +6 / 0.
        normal(
            MoveId::StFL,
            7,
            3,
            13,
            80,
            20,
            14,
            HitLevel::Mid,
            CancelRule::OnHit,
            flash,
        ),
        // 5ST: 10/3/16, 22/15 → +4 / −2.
        normal(
            MoveId::StST,
            10,
            3,
            16,
            100,
            22,
            15,
            HitLevel::Mid,
            CancelRule::OnHit,
            style,
        ),
        normal(
            MoveId::CrP,
            5,
            2,
            8,
            50,
            12,
            8,
            HitLevel::Mid,
            CancelRule::OnHitOrBlock,
            LocalBox::new(px(8), px(42), px(48), px(14)),
        ),
        normal(
            MoveId::CrK,
            5,
            2,
            9,
            50,
            12,
            8,
            HitLevel::Low,
            CancelRule::OnHitOrBlock,
            LocalBox::new(px(8), px(0), px(46), px(16)),
        ),
        normal(
            MoveId::CrS,
            8,
            3,
            14,
            90,
            16,
            12,
            HitLevel::Mid,
            CancelRule::OnHit,
            LocalBox::new(px(10), px(40), px(66), px(16)),
        ),
        // 2HS: anti-air crystal. −4 / −9.
        normal(
            MoveId::CrHS,
            10,
            3,
            20,
            140,
            18,
            13,
            HitLevel::Mid,
            CancelRule::OnHit,
            LocalBox::new(px(10), px(20), px(50), px(72)),
        ),
        // 2FL: 7/3/13, 19/13 → +5 / −1.
        normal(
            MoveId::CrFL,
            7,
            3,
            13,
            75,
            19,
            13,
            HitLevel::Mid,
            CancelRule::OnHit,
            LocalBox::new(px(8), px(14), px(50), px(20)),
        ),
        // 2ST: the sweep. Hard knockdown, −11 on block. 9/3/22 leaves her
        // exactly enough time to plant and glide inside the knockdown.
        {
            let mut m = normal(
                MoveId::CrST,
                9,
                3,
                22,
                110,
                22,
                13,
                HitLevel::Low,
                CancelRule::Never,
                LocalBox::new(px(12), px(0), px(72), px(18)),
            );
            m.knockdown = true;
            m
        },
        // Air normals swing downward so a hop-in reaches a grounded body.
        jump_normal(
            MoveId::JP,
            5,
            8,
            6,
            50,
            LocalBox::new(px(8), px(4), px(36), px(22)),
        ),
        jump_normal(
            MoveId::JK,
            5,
            8,
            6,
            50,
            LocalBox::new(px(6), px(-4), px(38), px(24)),
        ),
        jump_normal(
            MoveId::JS,
            6,
            6,
            6,
            80,
            LocalBox::new(px(8), px(-8), px(60), px(34)),
        ),
        jump_normal(
            MoveId::JHS,
            7,
            5,
            6,
            120,
            LocalBox::new(px(10), px(-12), px(84), px(32)),
        ),
        // j.FL: the downward glyph. Dive-flavored, hits deep.
        jump_normal(
            MoveId::JFL,
            6,
            6,
            6,
            70,
            LocalBox::new(px(4), px(-14), px(44), px(40)),
        ),
        jump_normal(
            MoveId::JST,
            7,
            5,
            6,
            110,
            LocalBox::new(px(10), px(-10), px(52), px(32)),
        ),
        overhead(),
        throw(),
        chant1(),
        chant2(),
        chant3(),
        glyph(MoveId::ShotB, false),
        crystal(MoveId::ShotA, false),
        shatter(),
        processional(),
        the_rite(),
        consecrate(),
        ascension(),
        glyph(MoveId::ExA, true),
        crystal(MoveId::ExB, true),
        convergence(),
    ];

    for m in &mut moves {
        if m.id.is_jumping() {
            m.level = HitLevel::High;
        }
    }

    Character {
        id: CharacterId::Raya,
        walk_fwd: px(2) + 128,
        walk_back: px(2) - 64,
        run_speed: px(5),
        jump_x: px(3),
        jump_y: px(12),
        hop_x: px(3),
        hop_y: px(7),
        gravity: px(1) / 2 + 48,
        max_health: 950,
        push_w: px(30),
        stand_h: px(98),
        crouch_h: px(60),
        throw_range: px(34),
        close_range: px(48),
        color: 0xE8_DCC5,
        aura: None,
        gauge: GaugeDef {
            max: 100,
            start: 0,
            regen_delay: 0,
            regen_every: 0,
            regen_amount: 0,
            buff_step: 50,
            label: "CRYSTAL",
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
            SpecialRoute::ground(Motion::Qcb, &[Btn::S], MoveId::ShotA),
            SpecialRoute::ground(Motion::Qcb, &[Btn::FL], MoveId::Charge),
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

/// HS+ST. 24/3/16, 23/13 → +5 / −6. High.
fn overhead() -> MoveDef {
    normal(
        MoveId::Overhead,
        24,
        3,
        16,
        100,
        23,
        13,
        HitLevel::High,
        CancelRule::Never,
        LocalBox::new(px(6), px(30), px(50), px(62)),
    )
}

/// P+K. 2/1/22, 170, techable.
fn throw() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Throw);
    m.startup = 2;
    m.active = 1;
    m.recovery = 22;
    m.damage = 170;
    m.hitstop = 4;
    m.pushback_hit = 0;
    m.knockdown = true;
    m.throw = ThrowKind::Normal;
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        2,
        3,
        LocalBox::new(px(4), px(20), px(30), px(40)),
    )]);
    m.meter_on_hit = 16;
    m.feintable = false;
    m
}

/// 236+S chant I. 9/3/14, 16/12 → −1 / −4. Advances a step.
fn chant1() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Rekka1);
    m.startup = 9;
    m.active = 3;
    m.recovery = 14;
    m.damage = 65;
    m.chip = 6;
    m.hitstun = 16;
    m.blockstun = 12;
    m.vel_x = px(3);
    m.vel_frames = 8;
    m.pushback_hit = px(6);
    m.pushback_block = px(6);
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        9,
        12,
        LocalBox::new(px(12), px(36), px(60), px(44)),
    )]);
    m.meter_on_hit = 10;
    m.followups = leak_followups(vec![Followup {
        button: Btn::S,
        next: MoveId::Rekka2,
        from: 9,
        to: 23,
    }]);
    m
}

/// Chant II. 8/3/14, 17/11 → +1 / −5.
fn chant2() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Rekka2);
    m.startup = 8;
    m.active = 3;
    m.recovery = 14;
    m.damage = 70;
    m.chip = 6;
    m.hitstun = 17;
    m.blockstun = 11;
    m.vel_x = px(3);
    m.vel_frames = 6;
    m.pushback_hit = px(6);
    m.pushback_block = px(8);
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        8,
        11,
        LocalBox::new(px(10), px(30), px(66), px(50)),
    )]);
    m.meter_on_hit = 10;
    m.followups = leak_followups(vec![Followup {
        button: Btn::S,
        next: MoveId::Rekka3,
        from: 8,
        to: 22,
    }]);
    m
}

/// Chant III. 11/3/24, KD, −14 on block. A tall glyph.
fn chant3() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Rekka3);
    m.startup = 11;
    m.active = 3;
    m.recovery = 24;
    m.damage = 100;
    m.chip = 10;
    m.hitstun = 24;
    m.blockstun = 12;
    m.knockdown = true;
    m.vel_x = px(4);
    m.vel_frames = 8;
    m.pushback_hit = px(14);
    m.pushback_block = px(14);
    m.hitstop = 10;
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        11,
        14,
        LocalBox::new(px(12), px(10), px(60), px(84)),
    )]);
    m.meter_on_hit = 14;
    m
}

/// 236+HS voice glyph: placed ~80px ahead at chest height, hangs 60f.
/// 13/1/22. EX (50 gauge): larger, 90f, heavy class. One on screen.
fn glyph(id: MoveId, ex: bool) -> MoveDef {
    let mut m = MoveDef::special(id);
    if ex {
        m.startup = 11;
        m.active = 1;
        m.recovery = 20;
        m.gauge_cost = 50;
        m.projectile = Some(ProjectileDef {
            kind: ProjectileKind::Glyph,
            behavior: ShotBehavior::Hang,
            spawn: LocalBox::new(px(70), px(30), 0, 0),
            vel_x: 0,
            vel_y: 0,
            gravity: 0,
            lifetime: 90,
            damage: 90,
            chip: 12,
            hitstun: 22,
            blockstun: 18,
            hitstop: 9,
            pushback: px(10),
            hitbox: LocalBox::new(px(0), px(0), px(50), px(62)),
            blast: None,
            level: HitLevel::Mid,
            knockdown: false,
        });
    } else {
        m.startup = 13;
        m.active = 1;
        m.recovery = 22;
        m.projectile = Some(ProjectileDef {
            kind: ProjectileKind::Glyph,
            behavior: ShotBehavior::Hang,
            spawn: LocalBox::new(px(76), px(40), 0, 0),
            vel_x: 0,
            vel_y: 0,
            gravity: 0,
            lifetime: 60,
            damage: 70,
            chip: 10,
            hitstun: 18,
            blockstun: 16,
            hitstop: 8,
            pushback: px(10),
            hitbox: LocalBox::new(px(0), px(0), px(34), px(40)),
            blast: None,
            level: HitLevel::Mid,
            knockdown: false,
        });
    }
    m.meter_on_hit = 10;
    m
}

/// 214+S crystal: a low toss that lands ~60px out (on a downed body from
/// sweep range), plants, arms after 20f, lives 90f armed. 15/1/16 — fast
/// enough to plant and glide inside a hard knockdown.
/// The blast itself is also a knockdown (D2 / FRAME-DATA currency).
/// EX (50 gauge): a longer arc (~120px), arms in 1f, lives 120f, bigger blast.
fn crystal(id: MoveId, ex: bool) -> MoveDef {
    let mut m = MoveDef::special(id);
    let (arm_after, armed_life, damage, blast, vel_x, vel_y) = if ex {
        (
            1,
            120,
            110,
            LocalBox::new(px(-26), 0, px(76), px(84)),
            px(5),
            px(6),
        )
    } else {
        (
            20,
            90,
            90,
            LocalBox::new(px(-18), 0, px(60), px(70)),
            px(2),
            px(4),
        )
    };
    if ex {
        m.startup = 13;
        m.active = 1;
        m.recovery = 18;
        m.gauge_cost = 50;
    } else {
        m.startup = 15;
        m.active = 1;
        m.recovery = 16;
    }
    m.projectile = Some(ProjectileDef {
        kind: ProjectileKind::Crystal,
        behavior: ShotBehavior::Plant {
            arm_after,
            armed_life,
        },
        spawn: LocalBox::new(px(24), px(24), 0, 0),
        vel_x,
        vel_y,
        gravity: px(1) / 2 + 32,
        lifetime: 200,
        damage,
        chip: 12,
        hitstun: 22,
        blockstun: 16,
        hitstop: 9,
        pushback: px(12),
        hitbox: LocalBox::new(px(0), px(0), px(24), px(30)),
        blast: Some(blast),
        level: HitLevel::Mid,
        knockdown: true,
    });
    m.meter_on_hit = 12;
    m
}

/// 214+S with an armed crystal planted: shatter it now. 6/1/12.
fn shatter() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Detonate);
    m.startup = 6;
    m.active = 1;
    m.recovery = 12;
    m.feintable = false;
    m
}

/// 236+FL processional glide: 18 frames, 112px, passes through the body.
/// Crystal in front, glide behind — the sandwich.
fn processional() -> MoveDef {
    let mut m = MoveDef::special(MoveId::CommandDash);
    m.startup = 0;
    m.active = 0;
    m.recovery = 18;
    m.vel_x = px(8);
    m.vel_frames = 14;
    m.pass_through = true;
    m.feintable = false;
    m
}

/// 63214+FL the rite. She concludes you. 7/2/40, untechable.
fn the_rite() -> MoveDef {
    let mut m = MoveDef::special(MoveId::CommandGrab);
    m.startup = 7;
    m.active = 2;
    m.recovery = 40;
    m.damage = 180;
    m.hitstop = 6;
    m.knockdown = true;
    m.throw = ThrowKind::Command;
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        7,
        9,
        LocalBox::new(px(4), px(20), px(42), px(50)),
    )]);
    m.meter_on_hit = 24;
    m
}

/// Hold 214+FL: consecrate. 10f in, then hold to fill the crystal gauge at
/// 2/frame (50f to full), 8f out. Interruptible. The gauge is the buff.
fn consecrate() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Charge);
    m.startup = 10;
    m.active = 1;
    m.recovery = 8;
    m.channel = Some(ChannelDef {
        button: Btn::FL,
        max_frames: 60,
        gauge_per_frame: 2,
    });
    m
}

/// 623+S/HS ascension. 4/8/28 + 12 landing. Invuln 1–6. Launch, KD.
fn ascension() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Uppercut);
    m.startup = 4;
    m.active = 8;
    m.recovery = 28;
    m.damage = 130;
    m.chip = 14;
    m.hitstun = 19;
    m.blockstun = 11;
    m.hitstop = 11;
    m.blockstop = 7;
    m.pushback_hit = px(8);
    m.pushback_block = px(12);
    m.knockdown = true;
    m.launch = px(9);
    m.invuln = Invuln::full(1, 7);
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        4,
        12,
        LocalBox::new(px(2), px(34), px(46), px(62)),
    )]);
    m.vel_x = px(2);
    m.vel_y = px(11);
    m.gravity_override = Some(px(1) / 2);
    m.land_recovery = 12;
    m.meter_on_hit = 16;
    m
}

/// 236236+S convergence. Dash-through, 6/6/30, 340, 1000 bar.
fn convergence() -> MoveDef {
    let mut m = MoveDef::special(MoveId::Super);
    m.startup = 6;
    m.active = 6;
    m.recovery = 30;
    m.damage = 340;
    m.chip = 28;
    m.hitstun = 20;
    m.blockstun = 12;
    m.hitstop = 16;
    m.blockstop = 8;
    m.pushback_hit = px(4);
    m.pushback_block = px(18);
    m.knockdown = true;
    m.launch = px(6);
    m.invuln = Invuln::full(0, 8);
    m.hitboxes = leak_boxes(vec![TimedBox::span(
        6,
        12,
        LocalBox::new(px(-10), px(8), px(100), px(80)),
    )]);
    m.vel_x = px(14);
    m.meter_cost = 1000;
    m
}
