//! Deterministic 60Hz fighting-game simulation.
//!
//! The sim is a pure function of `(World, InputFrame, InputFrame) -> World`.
//! No rendering, no wall clock, no floats in the game state, no filesystem.
//! That is the contract a later rollback netcode layer will need.

pub mod chars;
pub mod collision;
pub mod fighter;
pub mod geom;
pub mod input;
pub mod moves;
pub mod versus;
pub mod world;

pub use chars::{Character, CharacterId, GaugeDef};
pub use fighter::{
    Action, Connect, Fighter, Stance, CANCEL_LATE_FRAMES, FEINT_RECOVERY, GETUP_FRAMES,
    KNOCKDOWN_FRAMES, METER_MAX, PREJUMP, RC_COST, RC_FREEZE_FRAMES, THROW_TECH_WINDOW,
};
pub use geom::{px, Aabb, LocalBox, Vec2i, SUB, TICK_HZ};
pub use input::{stick_to_dir, Btn, Buttons, Chord, InputFrame, Motion};
pub use moves::{
    HitLevel, MoveDef, MoveId, ProjectileClass, ProjectileKind, ShotBehavior, ThrowKind,
};
pub use versus::{Match, Phase, ROUNDS_TO_WIN};
pub use world::{
    CombatEvent, DummyMode, EventKind, Projectile, RoundOutcome, ShotState, World, ROUND_TIME,
    STAGE_W,
};
