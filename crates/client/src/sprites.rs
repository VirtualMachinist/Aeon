//! Authored animation cells selected by simulation phase. Existing keyed
//! poses cover the rest of each kit and provide a complete fallback.

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
            Pose::Walk | Pose::Run | Pose::Block | Pose::Feint | Pose::Win | Pose::Getup => {
                Pose::Idle
            }
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
    atlas: Option<Texture2D>,
    thrust: Option<Texture2D>,
    reactions: Option<crate::sequences::Atlas>,
    uppercut: Option<crate::sequences::Atlas>,
    compact_uppercut: Option<crate::sequences::Atlas>,
    cuts: Option<crate::sequences::Atlas>,
    poke: Option<crate::sequences::Atlas>,
    disc: Option<crate::sequences::Atlas>,
    judgment: Option<crate::sequences::Atlas>,
    air_shot: Option<crate::sequences::Atlas>,
    air_saber: Option<crate::sequences::Atlas>,
    flash: Option<crate::sequences::Atlas>,
    overhead: Option<crate::sequences::Atlas>,
    crouch_saber: Option<crate::sequences::Atlas>,
    crouch_low: Option<crate::sequences::Atlas>,
    air_lights: Option<crate::sequences::Atlas>,
    air_lights_contact: Option<crate::sequences::Atlas>,
    air_shot_return: Option<crate::sequences::Atlas>,
    floor: Option<crate::sequences::Atlas>,
    recoil: Option<crate::sequences::Atlas>,
    ground: Option<crate::sequences::Atlas>,
    walk: Option<crate::sequences::Atlas>,
    coil: Option<crate::sequences::Atlas>,
    movement: Option<crate::sequences::Atlas>,
    ranged: Option<crate::sequences::Atlas>,
    utility: Option<crate::sequences::Atlas>,
}

/// A source rectangle and its foot anchor. Geometry stays in the sim.
pub struct SpriteFrame<'a> {
    pub texture: &'a Texture2D,
    pub source: Option<Rect>,
    pub anchor: Vec2,
    pub height: f32,
}

/// Which picture a fighter shows this frame: a keyed pose, an atlas cell, or
/// a wide thrust cell. Cheap to copy, so the motion layer can remember what
/// was on screen a few frames ago for crossfades and afterimages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Cell {
    Pose(Pose),
    Atlas(usize),
    Thrust(usize),
    Reaction(usize),
    Uppercut(usize),
    UppercutCompact(usize),
    Poke(usize),
    Disc(usize),
    Judgment(usize),
    AirShot(usize),
    AirSaber(usize),
    AirLights(usize),
    Flash(usize),
    CrouchSaber(usize),
    Overhead(usize),
    Floor(usize),
    Recoil(usize),
    Ground(usize),
    Movement(usize),
    Ranged(usize),
    Utility(usize),
}

/// Shared normalized layout lets framing checks measure the same complete
/// source region and projected root used by the renderer.
pub fn thrust_layout(cell: usize) -> (Rect, Vec2, f32) {
    // The lower thrust extends beyond the nominal half-width.
    let regions = [
        (0.0, 0.0, 0.5, 0.5), (0.5, 0.0, 0.5, 0.5),
        (0.0, 0.5, 845.0 / 1536.0, 0.5),
        (845.0 / 1536.0, 0.5, 691.0 / 1536.0, 0.5),
    ];
    let anchors = [(0.3809, 0.8359), (0.3262, 0.8340), (0.3385, 0.7754), (0.2829, 0.7910)];
    let (x, y, w, h) = regions[cell % 4];
    let (ax, ay) = anchors[cell % 4];
    (Rect::new(x, y, w, h), vec2(ax, ay), 1.20 / 0.72)
}

// Foot anchors measured from the generated sheets, including their uneven
// row baselines. A single hardcoded sheet baseline would make poses jump.
const KOGAN_ANCHORS: [(f32, f32); 16] = [
    (0.509, 0.976),
    (0.531, 0.973),
    (0.405, 0.970),
    (0.360, 0.970),
    (0.549, 0.928),
    (0.458, 0.928),
    (0.399, 0.928),
    (0.345, 0.928),
    (0.496, 0.928),
    (0.421, 0.925),
    (0.340, 0.928),
    (0.351, 0.928),
    (0.487, 0.869),
    (0.409, 0.869),
    (0.331, 0.873),
    (0.444, 0.869),
];
const RAYA_ANCHORS: [(f32, f32); 16] = [
    (0.439, 0.998),
    (0.389, 0.995),
    (0.370, 0.989),
    (0.372, 0.989),
    (0.412, 0.998),
    (0.335, 0.995),
    (0.368, 0.998),
    (0.354, 0.998),
    (0.410, 0.998),
    (0.359, 0.998),
    (0.362, 0.998),
    (0.337, 0.998),
    (0.389, 0.992),
    (0.351, 0.995),
    (0.364, 1.000),
    (0.356, 1.000),
];

/// Extract the technical green background once at load, preserving cyan
/// writing and Raya's linen. Source sheets and provenance remain intact.
pub(crate) fn key_green(image: &mut Image) {
    for rgba in image.bytes.chunks_exact_mut(4) {
        let other = rgba[0].max(rgba[2]);
        let dominance = i16::from(rgba[1]) - i16::from(other);
        if dominance > 90 && rgba[1] > 140 {
            rgba[3] = 0;
        } else if dominance > 18 && rgba[1] > 85 {
            let coverage = 1.0 - (dominance - 18) as f32 / 72.0;
            rgba[3] = (rgba[3] as f32 * coverage.clamp(0.0, 1.0)) as u8;
            rgba[1] = other;
        } else if dominance > 4 && rgba[1] > 65 {
            // Weak key spill can survive on already-antialiased copper edges.
            // Desaturate the key channel without eroding their existing alpha.
            rgba[1] = other;
        }
    }
    // Green mixed into copper can become yellow after the first despill.
    // Correct only warm edge pixels adjacent to transparent background, leaving
    // interior gold ornament, linen and blue/cyan writing untouched.
    let width = image.width as usize;
    let height = image.height as usize;
    let alpha: Vec<_> = image.bytes.chunks_exact(4).map(|p| p[3]).collect();
    for y in 1..height.saturating_sub(1) {
        for x in 1..width.saturating_sub(1) {
            let i = y * width + x;
            let p = &mut image.bytes[i * 4..i * 4 + 4];
            if p[3] > 0 && u16::from(p[1]) * 100 > u16::from(p[0]) * 70
                && u16::from(p[1]) > u16::from(p[2]) + 15
                && [i - 1, i + 1, i - width, i + width].iter().any(|&n| alpha[n] < 24) {
                p[1] = ((u16::from(p[0]) * 65 / 100) as u8).max(p[2]);
            }
        }
    }
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
            if let Ok(mut image) = load_image(&path).await {
                key_green(&mut image);
                let tex = Texture2D::from_image(&image);
                tex.set_filter(FilterMode::Linear);
                textures.insert(pose, tex);
            }
        }
        eprintln!("[aeon] {} sprites: {} poses", body.name(), textures.len());
        let atlas = match load_image(&format!("assets/animation/{dir}-v1-green.png")).await {
            Ok(mut image) => {
                key_green(&mut image);
                let texture = Texture2D::from_image(&image);
                texture.set_filter(FilterMode::Linear);
                eprintln!("[aeon] {} animation: 16 cells", body.name());
                Some(texture)
            }
            Err(e) => {
                eprintln!("[aeon] {} animation fallback: {e}", body.name());
                None
            }
        };
        let thrust = if body == CharacterId::Kogan {
            match load_image("assets/animation/kogan-thrust-v2-green.png").await {
                Ok(mut image) => {
                    key_green(&mut image);
                    let texture = Texture2D::from_image(&image);
                    texture.set_filter(FilterMode::Linear);
                    eprintln!("[aeon] KOGAN thrust animation: 4 wide cells");
                    Some(texture)
                }
                Err(e) => {
                    eprintln!("[aeon] KOGAN thrust fallback: {e}");
                    None
                }
            }
        } else {
            None
        };
        use crate::sequences::*;
        let (reaction_roots, uppercut_roots) = match body {
            CharacterId::Kogan => (&KOGAN_REACTIONS, &KOGAN_UPPERCUT),
            CharacterId::Raya => (&RAYA_REACTIONS, &RAYA_UPPERCUT),
        };
        let reactions = Atlas::load(&format!("assets/animation/{dir}-reactions-v1-green.png"),
            (1448, 1086), reaction_roots).await;
        let uppercut = Atlas::load(&format!("assets/animation/{dir}-uppercut-v1-green.png"),
            (1254, 1254), uppercut_roots).await;
        let coil = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-uppercut-coil-v1-green.png", (1254, 1254), &KOGAN_COIL).await
        } else { None };
        let legacy_movement = std::env::args().any(|a| a == "--kit-legacy-movement");
        let movement = if body == CharacterId::Kogan && !legacy_movement {
            Atlas::load_with_roots("assets/animation/kogan-movement-v2-green.png",
                (1672, 941), &KOGAN_MOVEMENT, &KOGAN_MOVEMENT_ROOT_Y).await
        } else { None };
        let ranged = if body == CharacterId::Kogan
            && !std::env::args().any(|a| a == "--kit-legacy-ranged") {
            Atlas::load("assets/animation/kogan-ranged-v5-green.png",
                (1448, 1086), &KOGAN_RANGED).await
        } else { None };
        let utility = if body == CharacterId::Kogan
            && !std::env::args().any(|a| a == "--kit-legacy-utility") {
            Atlas::load("assets/animation/kogan-cape-step-v3-green.png",
                (1448, 1086), &KOGAN_UTILITY).await
        } else { None };
        let compact_uppercut = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-uppercut-compact-v1-green.png",
                (1536, 1024), &KOGAN_UPPERCUT_COMPACT).await
        } else { None };
        let cuts = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-v1-green.png", (1254, 1254), &KOGAN_CUTS).await
        } else { None };
        let poke = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-standing-poke-v1-green.png", (1536, 1024), &KOGAN_POKE).await
        } else { None };
        let disc = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-disc-v2-green.png", (1536, 1024), &KOGAN_DISC).await
        } else { None };
        let walk = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-v1-green.png", (1254, 1254), &KOGAN_WALK).await
        } else { None };
        let ground = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-ground-v4-green.png", (1536, 1024), &KOGAN_GROUND).await
        } else { None };
        let overhead = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-overhead-v1-green.png", (1254, 1254), &KOGAN_OVERHEAD).await
        } else { None };
        let crouch_low = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-crouch-low-v3-green.png", (1024, 1536), &KOGAN_CROUCH_LOW).await
        } else { None };
        let crouch_saber = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-crouch-saber-v1-green.png", (1024, 1536), &KOGAN_CROUCH_SABER).await
        } else { None };
        let flash = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-flash-v2-green.png", (1024, 1536), &KOGAN_FLASH).await
        } else { None };
        let air_lights = if body == CharacterId::Kogan {
            Atlas::load_with_roots("assets/animation/kogan-air-lights-v1-green.png", (1024, 1536),
                &KOGAN_AIR_LIGHTS, &KOGAN_AIR_LIGHTS_ROOT_Y).await
        } else { None };
        // Keep the reviewed V1 gather/return; only the contact limbs use the corrected atlas.
        let air_lights_contact = if body == CharacterId::Kogan {
            Atlas::load_with_roots("assets/animation/kogan-air-lights-v4-green.png", (1024, 1536),
                &KOGAN_AIR_LIGHTS_CONTACT, &KOGAN_AIR_LIGHTS_ROOT_Y[1..4]).await
        } else { None };
        let air_saber = if body == CharacterId::Kogan {
            Atlas::load_with_roots("assets/animation/kogan-air-saber-v2-green.png", (1024, 1536),
                &KOGAN_AIR_SABER, &KOGAN_AIR_SABER_ROOT_Y).await
        } else { None };
        let air_shot = if body == CharacterId::Kogan {
            Atlas::load_with_roots("assets/animation/kogan-air-shot-v3-green.png", (1254, 1254),
                &KOGAN_AIR_SHOT[..3], &KOGAN_AIR_SHOT_ROOT_Y[..3]).await
        } else { None };
        // Keep the original sound return; later edits damaged that source cell.
        let air_shot_return = if body == CharacterId::Kogan {
            Atlas::load_with_roots("assets/animation/kogan-air-shot-v1-green.png", (1254, 1254),
                &KOGAN_AIR_SHOT[3..], &KOGAN_AIR_SHOT_ROOT_Y[3..]).await
        } else { None };
        let judgment = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-judgment-v3-green.png", (1536, 1024), &KOGAN_JUDGMENT).await
        } else { None };
        let floor = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-floor-v1-green.png", (1536, 1024), &KOGAN_FLOOR).await
        } else { None };
        let recoil = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-recoil-v2-green.png", (1024, 1536), &KOGAN_RECOIL).await
        } else { None };
        Self { textures, body, atlas, thrust, reactions, uppercut, compact_uppercut, cuts, poke, disc, judgment, air_shot, air_shot_return, air_saber, air_lights, air_lights_contact, flash, overhead, crouch_saber, crouch_low, floor, recoil, ground, walk, coil, movement, ranged, utility }
    }

    /// A set with no textures: cells resolve to pose names only.
    #[cfg(test)]
    pub fn empty(body: CharacterId) -> Self {
        Self {
            textures: HashMap::new(),
            body,
            atlas: None,
            thrust: None,
            reactions: None,
            uppercut: None,
            compact_uppercut: None,
            cuts: None,
            poke: None,
            disc: None,
            judgment: None,
            air_shot: None,
            air_saber: None,
            flash: None,
            overhead: None,
            crouch_saber: None,
            crouch_low: None,
            air_lights: None,
            air_lights_contact: None,
            air_shot_return: None,
            floor: None,
            recoil: None,
            ground: None,
            walk: None,
            coil: None,
            movement: None,
            ranged: None,
            utility: None,
        }
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

    pub fn cell_for_with_ground(&self, fighter: &Fighter, tick: u32,
        context: crate::sequences::GroundContext) -> Cell {
        if self.ground.is_some() && self.utility.is_some() {
            if let Some(cell) = crate::sequences::ground_cell(fighter, context) { return cell; }
        }
        self.cell_for(fighter, tick)
    }

    /// The picture for this fighter on this simulation tick.
    pub fn cell_for(&self, fighter: &Fighter, tick: u32) -> Cell {
        if self.overhead.is_some() {
            if let Some(cell) = crate::sequences::overhead_cell(fighter) { return cell; }
        }
        if self.crouch_saber.is_some() && self.crouch_low.is_some() {
            if let Some(cell) = crate::sequences::crouch_saber_cell(fighter) { return cell; }
        }
        if self.flash.is_some() {
            if let Some(cell) = crate::sequences::flash_cell(fighter) { return cell; }
        }
        if self.air_lights.is_some() && self.air_lights_contact.is_some() {
            if let Some(cell) = crate::sequences::air_lights_cell(fighter) { return cell; }
        }
        if self.air_saber.is_some() {
            if let Some(cell) = crate::sequences::air_saber_cell(fighter) { return cell; }
        }
        if self.air_shot.is_some() && self.air_shot_return.is_some() {
            if let Some(cell) = crate::sequences::air_shot_cell(fighter) { return cell; }
        }
        if self.judgment.is_some() {
            if let Some(cell) = crate::sequences::judgment_cell(fighter) { return cell; }
        }
        if self.floor.is_some() {
            if let Some(cell) = crate::sequences::floor_cell(fighter) { return cell; }
        }
        if self.recoil.is_some() {
            if let Some(cell) = crate::sequences::recoil_cell(fighter) { return cell; }
        }
        if self.disc.is_some() {
            if let Some(cell) = crate::sequences::disc_cell(fighter) { return cell; }
        }
        if self.poke.is_some() {
            if let Some(cell) = crate::sequences::poke_cell(fighter) { return cell; }
        }
        if self.utility.is_some() {
            if let Some(cell) = crate::sequences::utility_cell(fighter) { return cell; }
        }
        if self.ranged.is_some() {
            if let Some(cell) = crate::sequences::ranged_cell(fighter) {
                return cell;
            }
        }
        if self.movement.is_some() {
            if let Some(cell) = crate::sequences::movement_cell(fighter) {
                return cell;
            }
        }
        if self.compact_uppercut.is_some() {
            if let Some(cell) = crate::sequences::compact_uppercut_cell(fighter) { return cell; }
        }
        if let Some(cell) = crate::sequences::cell_for(fighter) {
            if matches!(cell, Cell::Reaction(_)) && self.reactions.is_some()
                || matches!(cell, Cell::Uppercut(_)) && self.uppercut.is_some() {
                return cell;
            }
        }
        if let Some(cell) = animation_cell(fighter, tick) {
            let thrust = matches!(
                fighter.action,
                Action::Attack {
                    move_id: MoveId::Rekka3,
                    ..
                }
            );
            if thrust && self.thrust.is_some() {
                return Cell::Thrust(cell % 4);
            }
            if self.atlas.is_some() {
                return Cell::Atlas(cell);
            }
        }
        Cell::Pose(phase_pose(fighter))
    }

    /// Resolve a cell to its texture, source rectangle and foot anchor.
    pub fn frame(&self, cell: Cell) -> Option<SpriteFrame<'_>> {
        match cell {
            Cell::Movement(cell) => self.movement.as_ref()?.frame(cell),
            Cell::Ranged(cell) => self.ranged.as_ref()?.frame(cell),
            Cell::Utility(cell) => self.utility.as_ref()?.frame(cell),
            Cell::Overhead(cell) => self.overhead.as_ref()?.frame(cell),
            Cell::CrouchSaber(cell @ 8..=15) => self.crouch_low.as_ref()?.frame(cell - 8),
            Cell::CrouchSaber(cell) => self.crouch_saber.as_ref()?.frame(cell),
            Cell::Flash(cell) => self.flash.as_ref()?.frame(cell),
            Cell::AirSaber(cell) => self.air_saber.as_ref()?.frame(cell),
            Cell::AirLights(cell @ 1..=3) => self.air_lights_contact.as_ref()?.frame(cell - 1),
            Cell::AirLights(cell) => self.air_lights.as_ref()?.frame(cell),
            Cell::AirShot(3) => self.air_shot_return.as_ref()?.frame(0),
            Cell::AirShot(cell) => self.air_shot.as_ref()?.frame(cell),
            Cell::Judgment(cell) => self.judgment.as_ref()?.frame(cell),
            Cell::Floor(cell) => self.floor.as_ref()?.frame(cell),
            Cell::Recoil(cell) => self.recoil.as_ref()?.frame(cell),
            Cell::Reaction(cell) => self.reactions.as_ref()?.frame(cell),
            Cell::Poke(cell) => self.poke.as_ref()?.frame(cell),
            Cell::Disc(cell) => self.disc.as_ref()?.frame(cell),
            Cell::Ground(cell) => self.ground.as_ref()?.frame(cell),
            Cell::UppercutCompact(cell) => self.compact_uppercut.as_ref()?.frame(cell),
            Cell::Uppercut(0) if self.coil.is_some() => self.coil.as_ref()?.frame(0),
            Cell::Uppercut(cell) => self.uppercut.as_ref()?.frame(cell),
            Cell::Thrust(cell) => {
                let texture = self.thrust.as_ref()?;
                let (region, anchor, height) = thrust_layout(cell);
                Some(SpriteFrame {
                    texture,
                    source: Some(Rect::new(region.x * texture.width(), region.y * texture.height(),
                        region.w * texture.width(), region.h * texture.height())),
                    anchor,
                    height,
                })
            }
            Cell::Atlas(cell @ 0..=3) if self.walk.is_some() => self.walk.as_ref()?.frame(cell),
            Cell::Atlas(cell @ 4..=11) if self.cuts.is_some() => self.cuts.as_ref()?.frame(cell - 4),
            Cell::Atlas(cell) => {
                let texture = self.atlas.as_ref()?;
                let cell = cell % 16;
                let side = texture.width() / 4.0;
                let row_height = texture.height() / 4.0;
                let (anchors, height) = match self.body {
                    CharacterId::Kogan => (&KOGAN_ANCHORS, 1.20 / 0.87),
                    CharacterId::Raya => (&RAYA_ANCHORS, 1.20 / 0.92),
                };
                let (x, y) = anchors[cell];
                // Fractional grid boundaries and linear filtering can pick up
                // a neighboring row's feet or blade. Keep a small row gutter,
                // preserving scale and anchoring the visible sole to the floor.
                let gutter = 3.0;
                let source_height = row_height - gutter * 2.0;
                Some(SpriteFrame {
                    texture,
                    source: Some(Rect::new(
                        (cell % 4) as f32 * side,
                        (cell / 4) as f32 * row_height + gutter,
                        side,
                        source_height,
                    )),
                    anchor: vec2(x, ((y * row_height - gutter) / source_height).min(1.0)),
                    height: height * source_height / row_height,
                })
            }
            Cell::Pose(pose) => self.get(pose).map(|texture| SpriteFrame {
                texture,
                source: None,
                anchor: vec2(0.5, 0.94),
                height: 1.55,
            }),
        }
    }

}

/// Animation contact coincides with the move's active frames. Hitstop and
/// pause naturally freeze these samples because the sim frame stays still.
pub(crate) fn animation_cell(f: &Fighter, tick: u32) -> Option<usize> {
    match f.action {
        Action::Walk { forward } => {
            let cell = (tick / 6 % 4) as usize;
            Some(if forward { cell } else { 3 - cell })
        }
        Action::Attack { move_id, frame, .. } => {
            let row = match move_id {
                MoveId::StS | MoveId::Rekka1 => 1,
                MoveId::ExA if f.id == CharacterId::Kogan => 1,
                MoveId::Rekka2 => 2,
                MoveId::Rekka3 => 3,
                MoveId::StHS | MoveId::StHSClose => {
                    if f.id == CharacterId::Kogan {
                        2
                    } else {
                        3
                    }
                }
                _ => return None,
            };
            let mv = f.data().move_def(move_id)?;
            if frame < mv.first_active() {
                return Some(row * 4);
            }
            if mv.is_active(frame) {
                return Some(row * 4 + if frame == mv.first_active() { 1 } else { 2 });
            }
            let recovery = frame.saturating_sub(mv.last_active() + 1);
            if recovery < u16::from(mv.recovery) / 3 {
                Some(row * 4 + 2)
            } else {
                // Raya chant I's last cell is an extended glyph, so its
                // gathered-hands frame supplies her withdrawal instead.
                Some(
                    row * 4
                        + if f.id == CharacterId::Raya && row == 1 {
                            0
                        } else {
                            3
                        },
                )
            }
        }
        _ => None,
    }
}

fn phase_pose(f: &Fighter) -> Pose {
    if let Action::Attack { move_id, frame, .. } = f.action {
        if let Some(mv) = f.data().move_def(move_id) {
            let rest = if f.airborne {
                if f.hop {
                    Pose::Hop
                } else {
                    Pose::Jump
                }
            } else if move_id.is_crouching() {
                Pose::Crouch
            } else {
                Pose::Idle
            };
            if frame < mv.first_active() / 3 {
                return rest;
            }
        }
    }
    if let Action::Getup { frame } = f.action {
        return if frame < 8 {
            Pose::Down
        } else if frame < 18 {
            Pose::Crouch
        } else {
            Pose::Idle
        };
    }
    pose_for(f)
}

pub fn pose_for(f: &Fighter) -> Pose {
    let crouch_block = |crouching: bool| {
        if crouching {
            Pose::CrouchBlock
        } else {
            Pose::Block
        }
    };
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
            MoveId::ShotB => Pose::ShotB,
            MoveId::ExA => {
                if f.id == CharacterId::Kogan {
                    Pose::Rekka1
                } else {
                    Pose::ShotB
                }
            }
            MoveId::Guard => Pose::Guard,
            MoveId::SpecialOverhead => Pose::SpecialOverhead,
            MoveId::AirShot => Pose::AirShot,
            MoveId::Charge => Pose::Charge,
            MoveId::Super => Pose::Super,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::px;

    #[test]
    fn signature_animation_contact_matches_sim_active_frames() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            for move_id in [
                MoveId::StS,
                MoveId::StHS,
                MoveId::Rekka1,
                MoveId::Rekka2,
                MoveId::Rekka3,
            ] {
                let mut f = Fighter::spawn(id, px(200), true);
                f.start_move(move_id);
                let mv = id.data().move_def(move_id).unwrap();
                for frame in 0..mv.total_frames() {
                    f.action = Action::Attack {
                        move_id,
                        frame,
                        connected: aeon_sim::Connect::None,
                    };
                    let cell = animation_cell(&f, 0);
                    if mv.is_active(frame) {
                        assert!(matches!(cell.map(|n| n % 4), Some(1 | 2)));
                    } else if frame < mv.first_active() {
                        assert_eq!(cell.unwrap() % 4, 0);
                    }
                    // Render count cannot advance an attack or hitstop pose.
                    assert_eq!(cell, animation_cell(&f, 999));
                }
            }
        }
    }

    #[test]
    fn kogan_ex_rekka_uses_saber_pose() {
        let mut f = Fighter::spawn(CharacterId::Kogan, px(200), true);
        f.start_move(MoveId::ExA);
        assert_eq!(pose_for(&f), Pose::Rekka1);
        assert_eq!(animation_cell(&f, 0), Some(4));
    }

    #[test]
    fn keying_despills_existing_translucent_edges_without_increasing_alpha() {
        let mut image = Image { width: 3, height: 1,
            bytes: vec![70, 125, 88, 96, 184, 115, 51, 160, 100, 112, 30, 80] };
        key_green(&mut image);
        assert_eq!(image.bytes[1], 88);
        assert!(image.bytes[3] > 0 && image.bytes[3] < 96);
        assert_eq!(&image.bytes[4..8], &[184, 115, 51, 160], "copper stays intact");
        assert_eq!(&image.bytes[8..], &[100, 100, 30, 80], "weak spill loses green, not alpha");
    }

    #[test]
    fn warm_edge_despill_preserves_interior_ornament_and_shape() {
        let copper_edge = [170, 170, 35, 255];
        let mut image = Image { width: 3, height: 3, bytes: vec![0; 36] };
        image.bytes[16..20].copy_from_slice(&copper_edge);
        key_green(&mut image);
        assert_eq!(&image.bytes[16..20], &[170, 110, 35, 255]);
        let mut interior = Image { width: 3, height: 3, bytes: copper_edge.repeat(9) };
        key_green(&mut interior);
        assert_eq!(&interior.bytes[16..20], &copper_edge);
    }

    #[test]
    fn green_key_preserves_cyan_and_white() {
        let mut image = Image {
            width: 3,
            height: 1,
            bytes: vec![0, 230, 0, 255, 70, 230, 255, 255, 245, 245, 240, 255],
        };
        key_green(&mut image);
        assert_eq!(image.bytes[3], 0);
        assert_eq!(&image.bytes[4..], &[70, 230, 255, 255, 245, 245, 240, 255]);
    }
}
