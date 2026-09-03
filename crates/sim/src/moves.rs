//! Frame data and move identity.
//!
//! Design law (Aeon/DESIGN.md, 2026-08-14):
//! - No normal-to-normal chains. Links and special cancels only.
//! - Whiffing a weapon-heavy is a real mistake: long recovery, no cancel.
//! - Damage lives in 2–3 hit confirms.
//! - Rekkas are the backbone special: parts 1–3 with branch points.
//! - Command grabs beat both blocks and lose to the uppercut.
//!
//! Move ids are *slots*. Both bodies share the machinery; the character file
//! gives each slot its name, motion, and numbers.

use crate::geom::{px, LocalBox};
use crate::input::Btn;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MoveId {
    // Standing normals
    StP,
    StK,
    StS,
    StHS,
    StHSClose,
    StFL,
    StST,
    // Crouching normals (2ST is the sweep)
    CrP,
    CrK,
    CrS,
    CrHS,
    CrFL,
    CrST,
    // Jumping normals (hop and full jump share them)
    JP,
    JK,
    JS,
    JHS,
    JFL,
    JST,
    /// HS+ST — universal standing overhead.
    Overhead,
    /// P+K — normal throw.
    Throw,
    // Special slots
    Rekka1,
    Rekka2,
    Rekka3,
    /// 623 — the uppercut.
    Uppercut,
    /// 63214+FL — command grab.
    CommandGrab,
    /// 236+FL — command dash.
    CommandDash,
    /// 214+S — Kogan revolver / Raya crystal plant.
    ShotA,
    /// 236+HS — Kogan energy wave / Raya voice glyph.
    ShotB,
    /// 214+HS — Kogan disc-shield.
    Guard,
    /// 214+S while a crystal is planted — Raya early detonate.
    Detonate,
    /// 236+ST — Kogan leaping saber overhead.
    SpecialOverhead,
    /// j.FL — Kogan air gun.
    AirShot,
    /// hold 214+FL — Raya consecrate (crystal gauge buff).
    Charge,
    /// 236+S+HS — EX rekka / EX glyph.
    ExA,
    /// 214+S+HS — EX revolver / EX crystal.
    ExB,
    /// 236236+S
    Super,
}

impl MoveId {
    pub const ALL: [MoveId; 37] = [
        MoveId::StP,
        MoveId::StK,
        MoveId::StS,
        MoveId::StHS,
        MoveId::StHSClose,
        MoveId::StFL,
        MoveId::StST,
        MoveId::CrP,
        MoveId::CrK,
        MoveId::CrS,
        MoveId::CrHS,
        MoveId::CrFL,
        MoveId::CrST,
        MoveId::JP,
        MoveId::JK,
        MoveId::JS,
        MoveId::JHS,
        MoveId::JFL,
        MoveId::JST,
        MoveId::Overhead,
        MoveId::Throw,
        MoveId::Rekka1,
        MoveId::Rekka2,
        MoveId::Rekka3,
        MoveId::Uppercut,
        MoveId::CommandGrab,
        MoveId::CommandDash,
        MoveId::ShotA,
        MoveId::ShotB,
        MoveId::Guard,
        MoveId::Detonate,
        MoveId::SpecialOverhead,
        MoveId::AirShot,
        MoveId::Charge,
        MoveId::ExA,
        MoveId::ExB,
        MoveId::Super,
    ];

    /// Generic slot label. Characters override with `Character::move_name`.
    pub fn slot_name(self) -> &'static str {
        match self {
            Self::StP => "5P",
            Self::StK => "5K",
            Self::StS => "5S",
            Self::StHS => "5HS",
            Self::StHSClose => "c.HS",
            Self::StFL => "5FL",
            Self::StST => "5ST",
            Self::CrP => "2P",
            Self::CrK => "2K",
            Self::CrS => "2S",
            Self::CrHS => "2HS",
            Self::CrFL => "2FL",
            Self::CrST => "2ST",
            Self::JP => "j.P",
            Self::JK => "j.K",
            Self::JS => "j.S",
            Self::JHS => "j.HS",
            Self::JFL => "j.FL",
            Self::JST => "j.ST",
            Self::Overhead => "overhead",
            Self::Throw => "throw",
            Self::Rekka1 => "rekka 1",
            Self::Rekka2 => "rekka 2",
            Self::Rekka3 => "rekka 3",
            Self::Uppercut => "uppercut",
            Self::CommandGrab => "command grab",
            Self::CommandDash => "command dash",
            Self::ShotA => "shot A",
            Self::ShotB => "shot B",
            Self::Guard => "guard",
            Self::Detonate => "detonate",
            Self::SpecialOverhead => "special overhead",
            Self::AirShot => "air shot",
            Self::Charge => "charge",
            Self::ExA => "EX A",
            Self::ExB => "EX B",
            Self::Super => "super",
        }
    }

    pub fn is_normal(self) -> bool {
        matches!(
            self,
            Self::StP
                | Self::StK
                | Self::StS
                | Self::StHS
                | Self::StHSClose
                | Self::StFL
                | Self::StST
                | Self::CrP
                | Self::CrK
                | Self::CrS
                | Self::CrHS
                | Self::CrFL
                | Self::CrST
                | Self::JP
                | Self::JK
                | Self::JS
                | Self::JHS
                | Self::JFL
                | Self::JST
                | Self::Overhead
        )
    }

    pub fn is_crouching(self) -> bool {
        matches!(
            self,
            Self::CrP | Self::CrK | Self::CrS | Self::CrHS | Self::CrFL | Self::CrST
        )
    }

    pub fn is_jumping(self) -> bool {
        matches!(
            self,
            Self::JP | Self::JK | Self::JS | Self::JHS | Self::JFL | Self::JST | Self::AirShot
        )
    }

    pub fn is_rekka(self) -> bool {
        matches!(self, Self::Rekka1 | Self::Rekka2 | Self::Rekka3)
    }

    pub fn is_ex(self) -> bool {
        matches!(self, Self::ExA | Self::ExB)
    }

    pub fn is_super(self) -> bool {
        self == Self::Super
    }

    pub fn is_special(self) -> bool {
        !self.is_normal() && self != Self::Throw
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HitLevel {
    /// Must stand-block. Jump-ins, overheads.
    High,
    /// Blocked either way.
    Mid,
    /// Must crouch-block. Sweeps, 2K.
    Low,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CancelRule {
    Never,
    /// Special/super cancel only on hit.
    OnHit,
    /// Special/super cancel on hit or block (lights, some mids).
    OnHitOrBlock,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ThrowKind {
    None,
    /// P+K. Beats both blocks. Techable. Hit beats it.
    Normal,
    /// 63214+FL. Beats both blocks. Untechable. Loses to uppercut invuln.
    Command,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Invuln {
    pub start: u8,
    pub end: u8,
    pub strike: bool,
    pub throw: bool,
}

impl Invuln {
    pub const NONE: Self = Self {
        start: 0,
        end: 0,
        strike: false,
        throw: false,
    };

    /// Full invulnerability (strike + throw) on frames `start..end`.
    pub const fn full(start: u8, end: u8) -> Self {
        Self {
            start,
            end,
            strike: true,
            throw: true,
        }
    }

    pub fn strike_on(self, frame: u16) -> bool {
        self.strike && frame >= self.start as u16 && frame < self.end as u16
    }

    pub fn throw_on(self, frame: u16) -> bool {
        self.throw && frame >= self.start as u16 && frame < self.end as u16
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TimedBox {
    pub start: u8,
    pub end: u8,
    pub hit: LocalBox,
}

impl TimedBox {
    pub const fn span(start: u8, end: u8, hit: LocalBox) -> Self {
        Self { start, end, hit }
    }
}

/// A rekka branch: pressing `button` during frames `from..to` of the current
/// part starts `next`. Follow-ups are legal on hit, block, and whiff — the
/// pause and the spacing are the skill.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Followup {
    pub button: Btn,
    pub next: MoveId,
    pub from: u8,
    pub to: u8,
}

/// Hold-to-channel definition (Raya consecrate). While the button stays held
/// on the channel frame, the move does not advance and the gauge fills.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChannelDef {
    pub button: Btn,
    pub max_frames: u16,
    pub gauge_per_frame: i32,
}

#[derive(Clone, Debug)]
pub struct MoveDef {
    pub id: MoveId,
    pub startup: u8,
    pub active: u8,
    pub recovery: u8,
    pub damage: i32,
    pub chip: i32,
    pub hitstun: u8,
    pub blockstun: u8,
    pub hitstop: u8,
    pub blockstop: u8,
    pub pushback_hit: i32,
    pub pushback_block: i32,
    pub level: HitLevel,
    pub knockdown: bool,
    pub launch: i32,
    pub cancel: CancelRule,
    pub invuln: Invuln,
    pub hitboxes: &'static [TimedBox],
    /// Horizontal velocity applied on frame 0 (dash attacks, DP rise).
    pub vel_x: i32,
    /// Frames the horizontal velocity lasts on the ground. 0 = whole move.
    pub vel_frames: u8,
    pub vel_y: i32,
    pub gravity_override: Option<i32>,
    /// Extra landing recovery if this move leaves the ground (the uppercut tax).
    pub land_recovery: u8,
    pub projectile: Option<ProjectileDef>,
    /// Active hitboxes on this move destroy opposing projectiles.
    pub projectile_guard: bool,
    pub meter_on_hit: i32,
    pub meter_cost: i32,
    /// Character gauge spent on start (EX, revolver chambers).
    pub gauge_cost: i32,
    pub throw: ThrowKind,
    pub followups: &'static [Followup],
    /// FL+ST during startup cancels this special to nothing.
    pub feintable: bool,
    /// Movement special that passes through the opponent's pushbox.
    pub pass_through: bool,
    pub channel: Option<ChannelDef>,
}

impl MoveDef {
    pub fn total_frames(&self) -> u16 {
        self.startup as u16 + self.active as u16 + self.recovery as u16
    }

    pub fn first_active(&self) -> u16 {
        self.startup as u16
    }

    pub fn last_active(&self) -> u16 {
        self.startup as u16 + self.active as u16
    }

    pub fn is_active(&self, frame: u16) -> bool {
        frame >= self.first_active() && frame < self.last_active()
    }

    pub fn in_startup(&self, frame: u16) -> bool {
        frame < self.first_active()
    }

    pub fn in_recovery(&self, frame: u16) -> bool {
        frame >= self.last_active() && frame < self.total_frames()
    }

    pub fn finished(&self, frame: u16) -> bool {
        frame >= self.total_frames()
    }

    /// Advantage if the move connects on its first active frame.
    pub fn advantage_on_hit(&self) -> i32 {
        self.hitstun as i32 - (self.active as i32 - 1) - self.recovery as i32
    }

    pub fn advantage_on_block(&self) -> i32 {
        self.blockstun as i32 - (self.active as i32 - 1) - self.recovery as i32
    }

    pub fn hitboxes_on(&self, frame: u16) -> impl Iterator<Item = LocalBox> + '_ {
        self.hitboxes
            .iter()
            .filter(move |tb| frame >= tb.start as u16 && frame < tb.end as u16)
            .map(|tb| tb.hit)
    }

    pub fn followup_for(&self, button: Btn, frame: u16) -> Option<MoveId> {
        self.followups
            .iter()
            .find(|f| f.button == button && frame >= f.from as u16 && frame < f.to as u16)
            .map(|f| f.next)
    }

    pub fn is_throw(&self) -> bool {
        self.throw != ThrowKind::None
    }

    /// Reasonable defaults for a special: everything off, no boxes.
    pub fn special(id: MoveId) -> Self {
        Self {
            id,
            startup: 0,
            active: 0,
            recovery: 0,
            damage: 0,
            chip: 0,
            hitstun: 0,
            blockstun: 0,
            hitstop: 8,
            blockstop: 6,
            pushback_hit: px(10),
            pushback_block: px(8),
            level: HitLevel::Mid,
            knockdown: false,
            launch: 0,
            cancel: CancelRule::Never,
            invuln: Invuln::NONE,
            hitboxes: &[],
            vel_x: 0,
            vel_frames: 0,
            vel_y: 0,
            gravity_override: None,
            land_recovery: 0,
            projectile: None,
            projectile_guard: false,
            meter_on_hit: 0,
            meter_cost: 0,
            gauge_cost: 0,
            throw: ThrowKind::None,
            followups: &[],
            feintable: !id.is_super() && id != MoveId::Throw,
            pass_through: false,
            channel: None,
        }
    }
}

/// How a shot lives once spawned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShotBehavior {
    /// Moves at `vel` until it hits, clashes, leaves the stage, or expires.
    Travel,
    /// Spawns and stays put (Raya's voice glyph, Kogan's wave is Travel).
    Hang,
    /// Arcs under gravity, lands, arms after `arm_after`, then detonates on
    /// contact or when `armed_life` runs out (harmlessly).
    Plant { arm_after: u16, armed_life: u16 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProjectileDef {
    pub kind: ProjectileKind,
    pub behavior: ShotBehavior,
    /// Spawn offset from the owner's origin, facing-relative.
    pub spawn: LocalBox,
    pub vel_x: i32,
    pub vel_y: i32,
    pub gravity: i32,
    pub lifetime: u16,
    pub damage: i32,
    pub chip: i32,
    pub hitstun: u8,
    pub blockstun: u8,
    pub hitstop: u8,
    pub pushback: i32,
    pub hitbox: LocalBox,
    /// Hitbox used when a planted crystal detonates.
    pub blast: Option<LocalBox>,
    pub level: HitLevel,
    pub knockdown: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProjectileKind {
    Revolver,
    Wave,
    AirShot,
    Glyph,
    Crystal,
}

impl ProjectileKind {
    pub fn clash_class(self) -> ProjectileClass {
        match self {
            Self::Revolver | Self::AirShot | Self::Glyph => ProjectileClass::Light,
            Self::Wave | Self::Crystal => ProjectileClass::Heavy,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Revolver => "revolver",
            Self::Wave => "wave",
            Self::AirShot => "air gun",
            Self::Glyph => "glyph",
            Self::Crystal => "crystal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProjectileClass {
    Light,
    Heavy,
}

/// Hitstop / pushback class by button family.
fn button_class(id: MoveId) -> (u8, i32, i32) {
    match id {
        MoveId::StP | MoveId::StK | MoveId::CrP | MoveId::CrK | MoveId::JP | MoveId::JK => {
            (7, px(6), px(8))
        }
        MoveId::StS | MoveId::StFL | MoveId::CrS | MoveId::CrFL | MoveId::JS | MoveId::JFL => {
            (8, px(12), px(10))
        }
        _ => (10, px(14), px(10)),
    }
}

/// Shared helper so character files stay readable.
#[allow(clippy::too_many_arguments)]
pub fn normal(
    id: MoveId,
    startup: u8,
    active: u8,
    recovery: u8,
    damage: i32,
    hitstun: u8,
    blockstun: u8,
    level: HitLevel,
    cancel: CancelRule,
    hit: LocalBox,
) -> MoveDef {
    let (hitstop, push_hit, push_block) = button_class(id);
    let mut m = MoveDef::special(id);
    m.startup = startup;
    m.active = active;
    m.recovery = recovery;
    m.damage = damage;
    m.hitstun = hitstun;
    m.blockstun = blockstun;
    m.hitstop = hitstop;
    m.blockstop = hitstop.saturating_sub(2);
    m.pushback_hit = push_hit;
    m.pushback_block = push_block;
    m.level = level;
    m.knockdown = id == MoveId::CrST;
    m.cancel = cancel;
    m.hitboxes = leak_box(startup, startup + active, hit);
    m.meter_on_hit = damage / 8;
    m.feintable = false;
    m
}

pub fn leak_box(start: u8, end: u8, hit: LocalBox) -> &'static [TimedBox] {
    // Character data is process-lifetime. Fine for a data-driven fighter.
    Box::leak(Box::new([TimedBox::span(start, end, hit)]))
}

pub fn leak_boxes(boxes: Vec<TimedBox>) -> &'static [TimedBox] {
    Box::leak(boxes.into_boxed_slice())
}

pub fn leak_followups(f: Vec<Followup>) -> &'static [Followup] {
    Box::leak(f.into_boxed_slice())
}
