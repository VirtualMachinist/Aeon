//! Keyed poses. One readable silhouette per state, held for the state's
//! duration; the sim drives smears and afterimages. Files live in
//! `assets/<body>/<pose>.png`, edit-chained from the identity plates.
//! Missing poses fall back along a chain and finally to the box body.

use std::collections::HashMap;

use aeon_sim::{Action, CharacterId, Fighter, MoveId};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pose {
    Idle,
    Walk,
    Run,
    Crouch,
    Hop,
    Jump,
    Hurt,
    Down,
    Getup,
    Block,
    CrouchBlock,
    P,
    K,
    S,
    HS,
    FL,
    ST,
    CrLight,
    CrHeavy,
    Sweep,
    AirLight,
    AirSaber,
    Overhead,
    Throw,
    Thrown,
    Rekka1,
    Rekka2,
    Rekka3,
    Uppercut,
    Grab,
    Dash,
    ShotA,
    ShotB,
    Guard,
    SpecialOverhead,
    AirShot,
    Charge,
    Super,
    Feint,
    Win,
}

impl Pose {
    pub const ALL: [Pose; 40] = [
        Pose::Idle,
        Pose::Walk,
        Pose::Run,
        Pose::Crouch,
        Pose::Hop,
        Pose::Jump,
        Pose::Hurt,
        Pose::Down,
        Pose::Getup,
        Pose::Block,
        Pose::CrouchBlock,
        Pose::P,
        Pose::K,
        Pose::S,
        Pose::HS,
        Pose::FL,
        Pose::ST,
        Pose::CrLight,
        Pose::CrHeavy,
        Pose::Sweep,
        Pose::AirLight,
        Pose::AirSaber,
        Pose::Overhead,
        Pose::Throw,
        Pose::Thrown,
        Pose::Rekka1,
        Pose::Rekka2,
        Pose::Rekka3,
        Pose::Uppercut,
        Pose::Grab,
        Pose::Dash,
        Pose::ShotA,
        Pose::ShotB,
        Pose::Guard,
        Pose::SpecialOverhead,
        Pose::AirShot,
        Pose::Charge,
        Pose::Super,
        Pose::Feint,
        Pose::Win,
    ];

    pub fn file(self) -> &'static str {
        match self {
            Pose::Idle => "idle",
            Pose::Walk => "walk",
            Pose::Run => "run",
            Pose::Crouch => "crouch",
            Pose::Hop => "hop",
            Pose::Jump => "jump",
            Pose::Hurt => "hurt",
            Pose::Down => "down",
            Pose::Getup => "getup",
            Pose::Block => "block",
            Pose::CrouchBlock => "crouch_block",
            Pose::P => "p",
            Pose::K => "k",
            Pose::S => "s",
            Pose::HS => "hs",
            Pose::FL => "fl",
            Pose::ST => "st",
            Pose::CrLight => "cr_light",
            Pose::CrHeavy => "cr_heavy",
            Pose::Sweep => "sweep",
            Pose::AirLight => "air_light",
            Pose::AirSaber => "air_heavy",
            Pose::Overhead => "overhead",
            Pose::Throw => "throw",
            Pose::Thrown => "thrown",
            Pose::Rekka1 => "rekka1",
            Pose::Rekka2 => "rekka2",
            Pose::Rekka3 => "rekka3",
            Pose::Uppercut => "uppercut",
            Pose::Grab => "grab",
            Pose::Dash => "dash",
            Pose::ShotA => "shot_a",
            Pose::ShotB => "shot_b",
            Pose::Guard => "guard",
            Pose::SpecialOverhead => "special_overhead",
            Pose::AirShot => "air_shot",
            Pose::Charge => "charge",
            Pose::Super => "super",
            Pose::Feint => "feint",
            Pose::Win => "win",
        }
    }

    /// What to draw when this pose has no file.
    fn fallback(self) -> Option<Pose> {
        Some(match self {
            Pose::Idle => return None,
            Pose::Walk | Pose::Run | Pose::Block | Pose::Feint | Pose::Win | Pose::Getup => Pose::Idle,
            Pose::CrouchBlock | Pose::CrLight | Pose::CrHeavy | Pose::Sweep => Pose::Crouch,
            Pose::Hop => Pose::Jump,
            Pose::Jump | Pose::AirLight | Pose::AirSaber | Pose::AirShot => Pose::Idle,
            Pose::Hurt | Pose::Thrown => Pose::Idle,
            Pose::Down => Pose::Crouch,
            Pose::P | Pose::K => Pose::S,
            Pose::S | Pose::FL | Pose::ST => Pose::HS,
            Pose::HS => Pose::Idle,
            Pose::Overhead | Pose::SpecialOverhead => Pose::HS,
            Pose::Throw | Pose::Grab => Pose::FL,
            Pose::Rekka1 | Pose::Rekka2 | Pose::Rekka3 => Pose::S,
            Pose::Uppercut | Pose::Super => Pose::HS,
            Pose::Dash => Pose::Run,
            Pose::ShotA | Pose::ShotB | Pose::Guard | Pose::Charge => Pose::FL,
            Pose::Crouch => Pose::Idle,
        })
    }
}

pub struct SpriteSet {
    textures: HashMap<Pose, Texture2D>,
    body: CharacterId,
}

impl SpriteSet {
    pub async fn load(body: CharacterId) -> Self {
        let dir = match body {
            CharacterId::Kogan => "kogan",
            CharacterId::Raya => "raya",
        };
        let mut textures = HashMap::new();
        for pose in Pose::ALL {
            let path = format!("assets/{dir}/{}.png", pose.file());
            if let Ok(tex) = load_texture(&path).await {
                tex.set_filter(FilterMode::Linear);
                textures.insert(pose, tex);
            }
        }
        eprintln!("[aeon] {} sprites: {} poses", body.name(), textures.len());
        Self { textures, body }
    }

    pub fn count(&self) -> usize {
        self.textures.len()
    }

    pub fn body(&self) -> CharacterId {
        self.body
    }

    pub fn get(&self, pose: Pose) -> Option<&Texture2D> {
        let mut p = Some(pose);
        while let Some(cur) = p {
            if let Some(t) = self.textures.get(&cur) {
                return Some(t);
            }
            p = cur.fallback();
        }
        None
    }
}

pub fn pose_for(f: &Fighter) -> Pose {
    let crouch_block = |crouching: bool| if crouching { Pose::CrouchBlock } else { Pose::Block };
    match &f.action {
        Action::Stand => Pose::Idle,
        Action::Crouch => Pose::Crouch,
        Action::Walk { .. } => Pose::Walk,
        Action::Run => Pose::Run,
        Action::BackDash { .. } => Pose::Dash,
        Action::Prejump { hop: true, .. } | Action::Jump { hop: true, .. } => Pose::Hop,
        Action::Prejump { .. } => Pose::Crouch,
        Action::Jump { .. } => Pose::Jump,
        Action::Feint { .. } => Pose::Feint,
        Action::Block { crouching, .. } => crouch_block(*crouching),
        Action::Hit { .. } => Pose::Hurt,
        Action::Knockdown { .. } => Pose::Down,
        Action::Getup { .. } => Pose::Getup,
        Action::Thrown { .. } => Pose::Thrown,
        Action::ThrowTech { .. } => Pose::Block,
        Action::Landing { .. } => Pose::Crouch,
        Action::Attack { move_id, .. } => match move_id {
            MoveId::StP => Pose::P,
            MoveId::StK => Pose::K,
            MoveId::StS => Pose::S,
            MoveId::StHS | MoveId::StHSClose => Pose::HS,
            MoveId::StFL => Pose::FL,
            MoveId::StST => Pose::ST,
            MoveId::CrP | MoveId::CrK | MoveId::CrS | MoveId::CrFL => Pose::CrLight,
            MoveId::CrHS => Pose::CrHeavy,
            MoveId::CrST => Pose::Sweep,
            MoveId::JP | MoveId::JK | MoveId::JFL => Pose::AirLight,
            MoveId::JS | MoveId::JHS | MoveId::JST => Pose::AirSaber,
            MoveId::Overhead => Pose::Overhead,
            MoveId::Throw => Pose::Throw,
            MoveId::Rekka1 => Pose::Rekka1,
            MoveId::Rekka2 => Pose::Rekka2,
            MoveId::Rekka3 => Pose::Rekka3,
            MoveId::Uppercut => Pose::Uppercut,
            MoveId::CommandGrab => Pose::Grab,
            MoveId::CommandDash => Pose::Dash,
            MoveId::ShotA | MoveId::ExB | MoveId::Detonate => Pose::ShotA,
            MoveId::ShotB | MoveId::ExA => Pose::ShotB,
            MoveId::Guard => Pose::Guard,
            MoveId::SpecialOverhead => Pose::SpecialOverhead,
            MoveId::AirShot => Pose::AirShot,
            MoveId::Charge => Pose::Charge,
            MoveId::Super => Pose::Super,
        },
    }
}
