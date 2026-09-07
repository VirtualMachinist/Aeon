//! Authored animation cells selected by simulation phase. Existing keyed
//! poses cover the rest of each kit and provide a complete fallback.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

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
    /// Packed pages, when `assets/packed/` exists. Then every other atlas
    /// field is `None` and `frame` reads the manifest map.
    packed: Option<Packed>,
    textures: HashMap<Pose, Texture2D>,
    body: CharacterId,
    atlas: Option<Texture2D>,
    thrust: Option<Texture2D>,
    thrust_style: Option<crate::sequences::Atlas>,
    reactions: Option<crate::sequences::Atlas>,
    uppercut: Option<crate::sequences::Atlas>,
    compact_uppercut: Option<crate::sequences::Atlas>,
    cuts: Option<crate::sequences::Atlas>,
    first_cut: Option<crate::sequences::Atlas>,
    backcut: Option<crate::sequences::Atlas>,
    poke: Option<crate::sequences::Atlas>,
    disc: Option<crate::sequences::Atlas>,
    judgment: Option<crate::sequences::Atlas>,
    air_shot: Option<crate::sequences::Atlas>,
    air_saber: Option<crate::sequences::Atlas>,
    flash: Option<crate::sequences::Atlas>,
    flash_contact: Option<crate::sequences::Atlas>,
    overhead: Option<crate::sequences::Atlas>,
    throw_tech: Option<crate::sequences::Atlas>,
    throw_contact: Option<crate::sequences::Atlas>,
    victory: Option<crate::sequences::Atlas>,
    standing_lights: Option<crate::sequences::Atlas>,
    signature: Option<crate::sequences::Atlas>,
    signature_contacts: Option<crate::sequences::Atlas>,
    chant: Option<crate::sequences::Atlas>,
    chant_finisher: Option<crate::sequences::Atlas>,
    standing_palm_contact: Option<crate::sequences::Atlas>,
    jab_contact: Option<crate::sequences::Atlas>,
    crouch_lights: Option<crate::sequences::Atlas>,
    crouch_kick_contact: Option<crate::sequences::Atlas>,
    crouch_punch: Option<crate::sequences::Atlas>,
    crouch_saber: Option<crate::sequences::Atlas>,
    crouch_saber_contact: Option<crate::sequences::Atlas>,
    crouch_low: Option<crate::sequences::Atlas>,
    air_lights: Option<crate::sequences::Atlas>,
    air_lights_contact: Option<crate::sequences::Atlas>,
    air_shot_return: Option<crate::sequences::Atlas>,
    floor: Option<crate::sequences::Atlas>,
    air_recovery: Option<crate::sequences::Atlas>,
    recoil: Option<crate::sequences::Atlas>,
    ground: Option<crate::sequences::Atlas>,
    walk: Option<crate::sequences::Atlas>,
    coil: Option<crate::sequences::Atlas>,
    movement: Option<crate::sequences::Atlas>,
    ranged: Option<crate::sequences::Atlas>,
    ritual: Option<crate::sequences::Atlas>,
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
    StandingLights(usize),
    Signature(usize),
    Chant(usize),
    CrouchLights(usize),
    CrouchPunch(usize),
    CrouchSaber(usize),
    Overhead(usize),
    ThrowTech(usize),
    ThrowContact,
    Victory(usize),
    Floor(usize),
    AirRecovery(usize),
    Recoil(usize),
    Ground(usize),
    Movement(usize),
    Ranged(usize),
    Ritual(usize),
    Utility(usize),
}

/// A cell's family: the variant without its index. Styles and manifest keys
/// are per family.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Family {
    Pose,
    Walk,
    Cut,
    Thrust,
    Reaction,
    Uppercut,
    UppercutCompact,
    Poke,
    Disc,
    Judgment,
    AirShot,
    AirSaber,
    AirLights,
    Flash,
    StandingLights,
    Signature,
    Chant,
    CrouchLights,
    CrouchPunch,
    CrouchSaber,
    Overhead,
    ThrowTech,
    ThrowContact,
    Victory,
    Floor,
    AirRecovery,
    Recoil,
    Ground,
    Movement,
    Ranged,
    Ritual,
    Utility,
}

/// How the motion layer may treat a picture. Authored drawings already
/// contain their bend, weight and weapon line; the flags say what the
/// procedural layer must leave alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellStyle {
    /// No rotation or stretch: the drawing carries its own commitment.
    pub rigid: bool,
    /// No dx/dy shove: shifting would detach the weapon from its contact line.
    pub pinned: bool,
    /// Afterimages only from the same picture, never from an older stance.
    pub trail_same_cell: bool,
    /// Afterimages are capped at two.
    pub trail_short: bool,
    /// A change of picture cuts; the previous picture is not faded over it.
    pub cut: bool,
}

// Flag bits per body for the style table below.
const A: u8 = 1; // authored: rigid and cut
const P: u8 = 2; // pinned
const S: u8 = 4; // trail only from the same cell
const T: u8 = 8; // trail capped at two

/// One row per family: Kogan flags, Raya flags. Kogan's combat drawings all
/// cap their trails and trail only from the same cell; that is the body-level
/// `T` on every Kogan row rather than a special case in the motion code.
const STYLES: &[(Family, u8, u8)] = &[
    (Family::Pose, T, 0),
    (Family::Walk, A | S | T, A),
    (Family::Cut, A | S | T, 0),
    (Family::Thrust, A | S | T, 0),
    (Family::Reaction, A | S | T, A),
    (Family::Uppercut, A | S | T, A | S),
    (Family::UppercutCompact, A | S | T, A | S),
    (Family::Poke, A | S | T, 0),
    (Family::Disc, A | S | T, 0),
    (Family::Judgment, A | S | T, A | P | S | T),
    (Family::AirShot, A | P | S | T, 0),
    (Family::AirSaber, A | P | S | T, A | P | S),
    (Family::AirLights, A | P | S | T, A | P | S),
    (Family::Flash, A | P | S | T, A | P | S),
    (Family::StandingLights, A | P | S | T, A | P | S),
    (Family::Signature, A | P | S | T, A | P | S),
    (Family::Chant, A | P | S | T, A | P | S),
    (Family::CrouchLights, A | P | S | T, A | P | S),
    (Family::CrouchPunch, A | P | S | T, 0),
    (Family::CrouchSaber, A | P | S | T, A | P | S),
    (Family::Overhead, A | P | S | T, A | P | S),
    (Family::ThrowTech, A | P | S | T, 0),
    (Family::ThrowContact, A | S | T, A | P | S),
    (Family::Victory, A | P | S | T, A | P | S),
    (Family::Floor, A | S | T, 0),
    (Family::AirRecovery, A | S | T, A | S),
    (Family::Recoil, A | S | T, A | S),
    (Family::Ground, A | S | T, A | S | T),
    (Family::Movement, A | T, A),
    (Family::Ranged, A | T, A | P),
    (Family::Ritual, A | T, A | P),
    (Family::Utility, A | S | T, A | P | S),
];

impl Cell {
    pub fn family(self) -> Family {
        match self {
            Cell::Pose(_) => Family::Pose,
            Cell::Atlas(0..=3) => Family::Walk,
            Cell::Atlas(_) => Family::Cut,
            Cell::Thrust(_) => Family::Thrust,
            Cell::Reaction(_) => Family::Reaction,
            Cell::Uppercut(_) => Family::Uppercut,
            Cell::UppercutCompact(_) => Family::UppercutCompact,
            Cell::Poke(_) => Family::Poke,
            Cell::Disc(_) => Family::Disc,
            Cell::Judgment(_) => Family::Judgment,
            Cell::AirShot(_) => Family::AirShot,
            Cell::AirSaber(_) => Family::AirSaber,
            Cell::AirLights(_) => Family::AirLights,
            Cell::Flash(_) => Family::Flash,
            Cell::StandingLights(_) => Family::StandingLights,
            Cell::Signature(_) => Family::Signature,
            Cell::Chant(_) => Family::Chant,
            Cell::CrouchLights(_) => Family::CrouchLights,
            Cell::CrouchPunch(_) => Family::CrouchPunch,
            Cell::CrouchSaber(_) => Family::CrouchSaber,
            Cell::Overhead(_) => Family::Overhead,
            Cell::ThrowTech(_) => Family::ThrowTech,
            Cell::ThrowContact => Family::ThrowContact,
            Cell::Victory(_) => Family::Victory,
            Cell::Floor(_) => Family::Floor,
            Cell::AirRecovery(_) => Family::AirRecovery,
            Cell::Recoil(_) => Family::Recoil,
            Cell::Ground(_) => Family::Ground,
            Cell::Movement(_) => Family::Movement,
            Cell::Ranged(_) => Family::Ranged,
            Cell::Ritual(_) => Family::Ritual,
            Cell::Utility(_) => Family::Utility,
        }
    }

    pub fn style(self, body: CharacterId) -> CellStyle {
        let family = self.family();
        let bits = STYLES
            .iter()
            .find(|(f, _, _)| *f == family)
            .map(|(_, k, r)| match body {
                CharacterId::Kogan => *k,
                CharacterId::Raya => *r,
            })
            .unwrap_or(0);
        CellStyle {
            rigid: bits & A != 0,
            pinned: bits & P != 0,
            trail_same_cell: bits & S != 0,
            trail_short: bits & T != 0,
            cut: bits & A != 0,
        }
    }

    /// Manifest key: `family:index`, or `pose:<file>`.
    pub fn key(self) -> String {
        let (name, index) = match self {
            Cell::Pose(p) => return format!("pose:{}", p.file()),
            Cell::Atlas(i) => ("atlas", i),
            Cell::Thrust(i) => ("thrust", i),
            Cell::Reaction(i) => ("reaction", i),
            Cell::Uppercut(i) => ("uppercut", i),
            Cell::UppercutCompact(i) => ("uppercutcompact", i),
            Cell::Poke(i) => ("poke", i),
            Cell::Disc(i) => ("disc", i),
            Cell::Judgment(i) => ("judgment", i),
            Cell::AirShot(i) => ("airshot", i),
            Cell::AirSaber(i) => ("airsaber", i),
            Cell::AirLights(i) => ("airlights", i),
            Cell::Flash(i) => ("flash", i),
            Cell::StandingLights(i) => ("standinglights", i),
            Cell::Signature(i) => ("signature", i),
            Cell::Chant(i) => ("chant", i),
            Cell::CrouchLights(i) => ("crouchlights", i),
            Cell::CrouchPunch(i) => ("crouchpunch", i),
            Cell::CrouchSaber(i) => ("crouchsaber", i),
            Cell::Overhead(i) => ("overhead", i),
            Cell::ThrowTech(i) => ("throwtech", i),
            Cell::ThrowContact => ("throwcontact", 0),
            Cell::Victory(i) => ("victory", i),
            Cell::Floor(i) => ("floor", i),
            Cell::AirRecovery(i) => ("airrecovery", i),
            Cell::Recoil(i) => ("recoil", i),
            Cell::Ground(i) => ("ground", i),
            Cell::Movement(i) => ("movement", i),
            Cell::Ranged(i) => ("ranged", i),
            Cell::Ritual(i) => ("ritual", i),
            Cell::Utility(i) => ("utility", i),
        };
        format!("{name}:{index}")
    }

    pub fn parse(key: &str) -> Option<Cell> {
        let (name, rest) = key.split_once(':')?;
        if name == "pose" {
            return Pose::ALL.iter().copied().find(|p| p.file() == rest).map(Cell::Pose);
        }
        let i: usize = rest.parse().ok()?;
        Some(match name {
            "atlas" => Cell::Atlas(i),
            "thrust" => Cell::Thrust(i),
            "reaction" => Cell::Reaction(i),
            "uppercut" => Cell::Uppercut(i),
            "uppercutcompact" => Cell::UppercutCompact(i),
            "poke" => Cell::Poke(i),
            "disc" => Cell::Disc(i),
            "judgment" => Cell::Judgment(i),
            "airshot" => Cell::AirShot(i),
            "airsaber" => Cell::AirSaber(i),
            "airlights" => Cell::AirLights(i),
            "flash" => Cell::Flash(i),
            "standinglights" => Cell::StandingLights(i),
            "signature" => Cell::Signature(i),
            "chant" => Cell::Chant(i),
            "crouchlights" => Cell::CrouchLights(i),
            "crouchpunch" => Cell::CrouchPunch(i),
            "crouchsaber" => Cell::CrouchSaber(i),
            "overhead" => Cell::Overhead(i),
            "throwtech" => Cell::ThrowTech(i),
            "throwcontact" => Cell::ThrowContact,
            "victory" => Cell::Victory(i),
            "floor" => Cell::Floor(i),
            "airrecovery" => Cell::AirRecovery(i),
            "recoil" => Cell::Recoil(i),
            "ground" => Cell::Ground(i),
            "movement" => Cell::Movement(i),
            "ranged" => Cell::Ranged(i),
            "ritual" => Cell::Ritual(i),
            "utility" => Cell::Utility(i),
            _ => return None,
        })
    }

    /// Every cell the selectors could name, for the packer to try.
    pub fn candidates() -> Vec<Cell> {
        let mut out: Vec<Cell> = Pose::ALL.iter().map(|p| Cell::Pose(*p)).collect();
        for i in 0..16 {
            out.extend([
                Cell::Atlas(i),
                Cell::Thrust(i),
                Cell::Reaction(i),
                Cell::Uppercut(i),
                Cell::UppercutCompact(i),
                Cell::Poke(i),
                Cell::Disc(i),
                Cell::Judgment(i),
                Cell::AirShot(i),
                Cell::AirSaber(i),
                Cell::AirLights(i),
                Cell::Flash(i),
                Cell::StandingLights(i),
                Cell::Signature(i),
                Cell::Chant(i),
                Cell::CrouchLights(i),
                Cell::CrouchPunch(i),
                Cell::CrouchSaber(i),
                Cell::Overhead(i),
                Cell::ThrowTech(i),
                Cell::Victory(i),
                Cell::Floor(i),
                Cell::AirRecovery(i),
                Cell::Recoil(i),
                Cell::Ground(i),
                Cell::Movement(i),
                Cell::Ranged(i),
                Cell::Ritual(i),
                Cell::Utility(i),
            ]);
        }
        out.push(Cell::ThrowContact);
        out
    }
}

/// Set by the packer so every keyed source image is retained beside its
/// texture; the game never pays for this.
pub static KEEP_IMAGES: AtomicBool = AtomicBool::new(false);
static IMAGES: Mutex<Vec<(miniquad::TextureId, Image)>> = Mutex::new(Vec::new());

/// Upload a keyed image, remembering it when the packer asked.
pub(crate) fn upload(image: &Image) -> Texture2D {
    let texture = Texture2D::from_image(image);
    texture.set_filter(FilterMode::Linear);
    if KEEP_IMAGES.load(Ordering::Relaxed) {
        IMAGES
            .lock()
            .unwrap()
            .push((texture.raw_miniquad_id(), image.clone()));
    }
    texture
}

/// The retained source image for a texture (packer only).
pub fn retained_image(texture: &Texture2D) -> Option<Image> {
    let id = texture.raw_miniquad_id();
    IMAGES
        .lock()
        .unwrap()
        .iter()
        .find(|(t, _)| *t == id)
        .map(|(_, image)| image.clone())
}

/// A cell in a packed page.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PackedFrame {
    pub page: usize,
    pub source: Rect,
    pub anchor: Vec2,
    pub height: f32,
}

/// Re-express a frame's anchor and height for a crop of its source
/// rectangle. Feet stay where the sim puts them; the drawn body keeps its
/// size, because height is relative to the source rectangle's height.
pub fn refit(source: Rect, anchor: Vec2, height: f32, crop: Rect) -> (Vec2, f32) {
    let anchor = vec2(
        (anchor.x * source.w - (crop.x - source.x)) / crop.w,
        (anchor.y * source.h - (crop.y - source.y)) / crop.h,
    );
    (anchor, height * crop.h / source.h)
}

pub struct Packed {
    pages: Vec<Texture2D>,
    frames: HashMap<Cell, PackedFrame>,
}

impl Packed {
    pub fn parse(manifest: &str) -> HashMap<Cell, PackedFrame> {
        let mut frames = HashMap::new();
        for line in manifest.lines() {
            let mut it = line.split_whitespace();
            let (Some(key), Some(page)) = (it.next(), it.next()) else { continue };
            let nums: Vec<f32> = it.filter_map(|v| v.parse().ok()).collect();
            let (Some(cell), Ok(page)) = (Cell::parse(key), page.parse::<usize>()) else {
                continue;
            };
            let [x, y, w, h, ax, ay, height] = nums[..] else {
                continue;
            };
            frames.insert(
                cell,
                PackedFrame {
                    page,
                    source: Rect::new(x, y, w, h),
                    anchor: vec2(ax, ay),
                    height,
                },
            );
        }
        frames
    }

    fn frame(&self, cell: Cell) -> Option<SpriteFrame<'_>> {
        let lookup = |cell: Cell| {
            self.frames.get(&cell).map(|f| SpriteFrame {
                texture: &self.pages[f.page],
                source: Some(f.source),
                anchor: f.anchor,
                height: f.height,
            })
        };
        if let Cell::Pose(pose) = cell {
            let mut p = Some(pose);
            while let Some(cur) = p {
                if let Some(f) = lookup(Cell::Pose(cur)) {
                    return Some(f);
                }
                p = cur.fallback();
            }
            return None;
        }
        lookup(cell)
    }
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
            // Linear filtering still interpolates invisible RGB. Remove the
            // technical green so it cannot tint adjacent visible silhouette.
            rgba[1] = other;
        } else if dominance > 18 && rgba[1] > 85 {
            let coverage = 1.0 - (dominance - 18) as f32 / 72.0;
            rgba[3] = (rgba[3] as f32 * coverage.clamp(0.0, 1.0)) as u8;
            rgba[1] = other;
        } else if dominance > 4 {
            // Weak or dark key spill survives on antialiased copper edges.
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

/// A selector in priority order: what it picks, and which cells must be
/// present before any of its family is used, so a partly missing family
/// never mixes authored and fallback pictures inside one move.
pub struct Selector {
    pub name: &'static str,
    pub pick: fn(&Fighter) -> Option<Cell>,
    pub requires: &'static [(Option<CharacterId>, Cell)],
}

const K: Option<CharacterId> = Some(CharacterId::Kogan);
const R: Option<CharacterId> = Some(CharacterId::Raya);

pub const SELECTORS: &[Selector] = &[
    Selector { name: "feint", pick: crate::sequences::feint_cell, requires: &[] },
    Selector { name: "throw tech", pick: crate::sequences::throw_tech_cell, requires: &[] },
    Selector { name: "overhead", pick: crate::sequences::overhead_cell, requires: &[(None, Cell::Overhead(0))] },
    Selector {
        name: "chant",
        pick: crate::sequences::chant_cell,
        requires: &[(None, Cell::Signature(0)), (None, Cell::Chant(0)), (None, Cell::Chant(4))],
    },
    Selector {
        name: "signature",
        pick: crate::sequences::signature_cell,
        requires: &[(None, Cell::Signature(0)), (None, Cell::Signature(4))],
    },
    Selector {
        name: "standing lights",
        pick: crate::sequences::standing_lights_cell,
        requires: &[(None, Cell::StandingLights(0)), (None, Cell::StandingLights(1))],
    },
    Selector {
        name: "crouch lights",
        pick: crate::sequences::crouch_lights_cell,
        requires: &[(None, Cell::CrouchLights(0)), (None, Cell::CrouchLights(5))],
    },
    Selector { name: "crouch punch", pick: crate::sequences::crouch_punch_cell, requires: &[(None, Cell::CrouchPunch(0))] },
    Selector {
        name: "crouch saber",
        pick: crate::sequences::crouch_saber_cell,
        requires: &[(None, Cell::CrouchSaber(0)), (None, Cell::CrouchSaber(8))],
    },
    Selector { name: "flash", pick: crate::sequences::flash_cell, requires: &[(None, Cell::Flash(0)), (R, Cell::Flash(5))] },
    Selector {
        name: "air lights",
        pick: crate::sequences::air_lights_cell,
        requires: &[(None, Cell::AirLights(0)), (K, Cell::AirLights(1))],
    },
    Selector {
        name: "air saber",
        pick: crate::sequences::air_saber_cell,
        requires: &[(None, Cell::AirSaber(1)), (R, Cell::AirSaber(0))],
    },
    Selector { name: "air shot", pick: crate::sequences::air_shot_cell, requires: &[(None, Cell::AirShot(0)), (None, Cell::AirShot(3))] },
    Selector { name: "judgment", pick: crate::sequences::judgment_cell, requires: &[(None, Cell::Judgment(0))] },
    Selector { name: "floor", pick: crate::sequences::floor_cell, requires: &[(None, Cell::Floor(0))] },
    Selector { name: "air recovery", pick: crate::sequences::air_recovery_cell, requires: &[(None, Cell::AirRecovery(0))] },
    Selector { name: "recoil", pick: crate::sequences::recoil_cell, requires: &[(None, Cell::Recoil(0))] },
    Selector { name: "disc", pick: crate::sequences::disc_cell, requires: &[(None, Cell::Disc(0))] },
    Selector { name: "poke", pick: crate::sequences::poke_cell, requires: &[(None, Cell::Poke(0))] },
    Selector { name: "utility", pick: crate::sequences::utility_cell, requires: &[(None, Cell::Utility(0))] },
    Selector { name: "ritual", pick: crate::sequences::ritual_cell, requires: &[(None, Cell::Ritual(0))] },
    Selector { name: "ranged", pick: crate::sequences::ranged_cell, requires: &[(None, Cell::Ranged(0))] },
    Selector { name: "movement", pick: crate::sequences::movement_cell, requires: &[(None, Cell::Movement(0))] },
    Selector { name: "compact uppercut", pick: crate::sequences::compact_uppercut_cell, requires: &[(None, Cell::UppercutCompact(0))] },
    Selector { name: "reactions and reversals", pick: crate::sequences::cell_for, requires: &[] },
];

impl SpriteSet {
    fn dir(body: CharacterId) -> &'static str {
        match body {
            CharacterId::Kogan => "kogan",
            CharacterId::Raya => "raya",
        }
    }

    /// Packed pages when the packer has run, else every source sheet.
    pub async fn load(body: CharacterId) -> Self {
        if let Some(set) = Self::load_packed(body).await {
            return set;
        }
        Self::load_unpacked(body).await
    }

    /// `assets/packed/<body>.manifest` plus its pages: already keyed, already
    /// cropped, a few textures instead of sixty.
    pub async fn load_packed(body: CharacterId) -> Option<Self> {
        let dir = Self::dir(body);
        let manifest = load_string(&format!("assets/packed/{dir}.manifest")).await.ok()?;
        let frames = Packed::parse(&manifest);
        let page_count = frames.values().map(|f| f.page + 1).max()?;
        let mut bytes = Vec::with_capacity(page_count);
        for n in 0..page_count {
            bytes.push(load_file(&format!("assets/packed/{dir}-{n}.png")).await.ok()?);
        }
        // PNG inflate is the whole startup cost now. Decode two pages at a
        // time on worker threads and upload each pair before decoding the
        // next, so the transient memory stays at two pages, not all of them.
        let mut pages: Vec<Texture2D> = Vec::with_capacity(page_count);
        for pair in bytes.chunks(2) {
            let images: Vec<Image> = std::thread::scope(|scope| {
                let handles: Vec<_> = pair
                    .iter()
                    .map(|b| scope.spawn(|| Image::from_file_with_format(b, Some(ImageFormat::Png))))
                    .collect();
                handles.into_iter().filter_map(|h| h.join().ok()?.ok()).collect()
            });
            if images.len() != pair.len() {
                return None;
            }
            for image in &images {
                let page = Texture2D::from_image(image);
                page.set_filter(FilterMode::Linear);
                pages.push(page);
            }
        }
        eprintln!(
            "[aeon] {} packed: {} cells on {} pages",
            body.name(),
            frames.len(),
            pages.len()
        );
        let mut set = Self::empty_set(body);
        set.packed = Some(Packed { pages, frames });
        set.log_selectors();
        Some(set)
    }

    /// Which authored families this set can draw, in selector order.
    fn log_selectors(&self) {
        let ready: Vec<&str> = SELECTORS
            .iter()
            .filter(|s| !s.requires.is_empty() && self.have(s.requires))
            .map(|s| s.name)
            .collect();
        eprintln!("[aeon] {} selectors ready: {}", self.body.name(), ready.join(", "));
    }

    /// One line for the title screen.
    pub fn describe(&self) -> String {
        match &self.packed {
            Some(p) => format!("{} cells packed on {} page(s)", p.frames.len(), p.pages.len()),
            None => format!("{} poses from sheets", self.textures.len()),
        }
    }

    pub async fn load_unpacked(body: CharacterId) -> Self {
        let dir = Self::dir(body);
        let mut textures = HashMap::new();
        for pose in Pose::ALL {
            let path = format!("assets/{dir}/{}.png", pose.file());
            if let Ok(mut image) = load_image(&path).await {
                key_green(&mut image);
                textures.insert(pose, upload(&image));
            }
        }
        eprintln!("[aeon] {} sprites: {} poses", body.name(), textures.len());
        let atlas = match load_image(&format!("assets/animation/{dir}-v1-green.png")).await {
            Ok(mut image) => {
                key_green(&mut image);
                let texture = upload(&image);
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
                    let texture = upload(&image);
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
        let movement = if legacy_movement { None } else {
            let (file, specs, roots) = match body {
                CharacterId::Kogan => ("kogan-movement-v2-green.png", &KOGAN_MOVEMENT, &KOGAN_MOVEMENT_ROOT_Y),
                CharacterId::Raya => ("raya-movement-v1-green.png", &RAYA_MOVEMENT, &RAYA_MOVEMENT_ROOT_Y),
            };
            Atlas::load_with_roots(&format!("assets/animation/{file}"), (1672, 941), specs, roots).await
        };
        let ritual = if body == CharacterId::Raya && !std::env::args().any(|a|a=="--kit-legacy-ritual") {
            Atlas::load("assets/animation/raya-ritual-v1-green.png",(1536,1024),&RAYA_RITUAL).await
        } else {None};
        let ranged = if std::env::args().any(|a| a == "--kit-legacy-ranged") { None } else {
            match body {
                CharacterId::Kogan => Atlas::load("assets/animation/kogan-ranged-v5-green.png", (1448,1086), &KOGAN_RANGED).await,
                CharacterId::Raya => Atlas::load("assets/animation/raya-ranged-v2-green.png", (1536,1024), &RAYA_RANGED).await,
            }
        };
        let throw_contact = if body == CharacterId::Raya {
            Atlas::load("assets/animation/raya-throw-contact-v1-green.png",
                (1254, 1254), &RAYA_THROW_CONTACT).await
        } else { None };
        let utility = if std::env::args().any(|a| a == "--kit-legacy-utility") { None } else {
            match body {
                CharacterId::Kogan => Atlas::load("assets/animation/kogan-cape-step-v3-green.png",
                    (1448, 1086), &KOGAN_UTILITY).await,
                CharacterId::Raya => Atlas::load("assets/animation/raya-utility-v1-green.png",
                    (1536, 1024), &RAYA_UTILITY).await,
            }
        };
        let compact_uppercut = match body {
            CharacterId::Kogan => Atlas::load("assets/animation/kogan-uppercut-compact-v1-green.png",
                (1536, 1024), &KOGAN_UPPERCUT_COMPACT).await,
            CharacterId::Raya => Atlas::load("assets/animation/raya-uppercut-compact-v2-green.png",
                (1774, 887), &RAYA_UPPERCUT_COMPACT).await,
        };
        let cuts = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-v1-green.png", (1254, 1254), &KOGAN_CUTS).await
        } else { None };
        let first_cut = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-first-cut-style-v1-green.png",
                (1254, 1254), &KOGAN_FIRST_CUT).await
        } else { None };
        let backcut = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-backcut-style-v1-green.png",
                (1254, 1254), &KOGAN_BACKCUT).await
        } else { None };
        let thrust_style = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-thrust-style-v2-green.png",
                (1774, 887), &KOGAN_THRUST_STYLE).await
        } else { None };
        let poke = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-standing-poke-v1-green.png", (1536, 1024), &KOGAN_POKE).await
        } else { None };
        let disc = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-disc-v2-green.png", (1536, 1024), &KOGAN_DISC).await
        } else { None };
        let (walk_file, walk_specs) = match body {
            CharacterId::Kogan => ("kogan-walk-style-v1-green.png", &KOGAN_WALK),
            CharacterId::Raya => ("raya-v1-green.png", &RAYA_WALK),
        };
        let walk = Atlas::load(&format!("assets/animation/{walk_file}"),
            (1254, 1254), walk_specs).await;
        let (ground_file, ground_size, ground_specs) = match body {
            CharacterId::Kogan => ("kogan-ground-v4-green.png", (1536, 1024), &KOGAN_GROUND),
            CharacterId::Raya => ("raya-ground-v1-green.png", (1672, 941), &RAYA_GROUND),
        };
        let ground = Atlas::load(&format!("assets/animation/{ground_file}"),
            ground_size, ground_specs).await;
        let victory = match body {
            CharacterId::Kogan => Atlas::load("assets/animation/kogan-victory-v1-green.png", (1536, 1024), &KOGAN_VICTORY).await,
            CharacterId::Raya => Atlas::load("assets/animation/raya-victory-v1-green.png", (1536, 1024), &RAYA_VICTORY).await,
        };
        let throw_tech = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-throw-tech-v1-green.png", (1536, 1024), &KOGAN_THROW_TECH).await
        } else { None };
        let overhead = match body {
            CharacterId::Kogan => Atlas::load("assets/animation/kogan-overhead-v1-green.png", (1254, 1254), &KOGAN_OVERHEAD).await,
            CharacterId::Raya => Atlas::load("assets/animation/raya-overhead-v1-green.png", (1024, 1536), &RAYA_OVERHEAD).await,
        };
        let crouch_low = match body {
            CharacterId::Kogan => Atlas::load("assets/animation/kogan-crouch-low-v3-green.png", (1024, 1536), &KOGAN_CROUCH_LOW).await,
            CharacterId::Raya => Atlas::load("assets/animation/raya-crouch-low-v1-green.png", (1024, 1536), &RAYA_CROUCH_LOW).await,
        };
        let standing_lights = match body {
            CharacterId::Kogan => Atlas::load("assets/animation/kogan-standing-kick-v5-green.png", (1254, 1254), &KOGAN_STANDING_KICK).await,
            CharacterId::Raya => Atlas::load("assets/animation/raya-standing-lights-v1-green.png", (1024, 1536), &RAYA_STANDING_LIGHTS).await,
        };
        let jab_contact = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-jab-contact-v2-green.png", (1254, 1254), &KOGAN_JAB_CONTACT).await
        } else { None };
        // Keep seven sound original drawings; only the lowered contact uses V2.
        let standing_palm_contact = if body == CharacterId::Raya {
            Atlas::load("assets/animation/raya-standing-lights-v2-green.png", (1024, 1536), &RAYA_STANDING_LIGHTS[1..2]).await
        } else { None };
        let crouch_lights = match body {
            CharacterId::Kogan => Atlas::load("assets/animation/kogan-crouching-kick-v5-green.png", (1254, 1254), &KOGAN_CROUCH_KICK).await,
            CharacterId::Raya => Atlas::load("assets/animation/raya-crouch-lights-v1-green.png", (1024, 1536), &RAYA_CROUCH_LIGHTS).await,
        };
        // Keep seven sound V1 drawings and use the compact V2 ankle contact.
        let crouch_kick_contact = if body == CharacterId::Raya {
            Atlas::load("assets/animation/raya-crouch-lights-v2-green.png", (1024, 1536), &RAYA_CROUCH_LIGHTS[5..6]).await
        } else { None };
        let crouch_punch = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-crouch-punch-v1-green.png", (1254, 1254), &KOGAN_CROUCH_PUNCH).await
        } else { None };
        let crouch_saber = match body {
            CharacterId::Kogan => Atlas::load("assets/animation/kogan-crouch-saber-v1-green.png", (1024, 1536), &KOGAN_CROUCH_SABER).await,
            CharacterId::Raya => Atlas::load("assets/animation/raya-crouch-crystals-v1-green.png", (1024, 1536), &RAYA_CROUCH_CRYSTALS).await,
        };
        let crouch_saber_contact = if body == CharacterId::Raya {
            Atlas::load("assets/animation/raya-anti-crystal-v1-green.png", (1024, 1536), &RAYA_ANTI_CRYSTAL).await
        } else { None };
        let (flash_file, flash_specs) = match body {
            CharacterId::Kogan => ("kogan-flash-v2-green.png", &KOGAN_FLASH),
            CharacterId::Raya => ("raya-flash-style-v1-green.png", &RAYA_FLASH),
        };
        let flash = Atlas::load(&format!("assets/animation/{flash_file}"), (1024, 1536), flash_specs).await;
        // Retain seven V1 cells and the corrected short cape-contact hem.
        let flash_contact = if body == CharacterId::Raya {
            Atlas::load("assets/animation/raya-flash-style-v3-green.png", (1024, 1536), &RAYA_FLASH[5..6]).await
        } else { None };
        let (air_file, air_specs, air_roots) = match body {
            CharacterId::Kogan => ("kogan-air-lights-v1-green.png", &KOGAN_AIR_LIGHTS, &KOGAN_AIR_LIGHTS_ROOT_Y),
            CharacterId::Raya => ("raya-air-lights-v1-green.png", &RAYA_AIR_LIGHTS, &RAYA_AIR_LIGHTS_ROOT_Y),
        };
        let air_lights = Atlas::load_with_roots(&format!("assets/animation/{air_file}"),
            (1024, 1536), air_specs, air_roots).await;
        // Kogan keeps V1 gather/return and reviewed V4 contacts. Raya uses one coherent atlas.
        let air_lights_contact = if body == CharacterId::Kogan {
            Atlas::load_with_roots("assets/animation/kogan-air-lights-v4-green.png", (1024, 1536),
                &KOGAN_AIR_LIGHTS_CONTACT, &KOGAN_AIR_LIGHTS_ROOT_Y[1..4]).await
        } else { None };
        let air_saber = if body == CharacterId::Kogan {
            Atlas::load_with_roots("assets/animation/kogan-air-saber-v2-green.png", (1024, 1536),
                &KOGAN_AIR_SABER, &KOGAN_AIR_SABER_ROOT_Y).await
        } else {
            Atlas::load_with_roots("assets/animation/raya-air-crystals-v1-green.png", (1024, 1536),
                &RAYA_AIR_CRYSTALS, &RAYA_AIR_CRYSTALS_ROOT_Y).await
        };
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
        } else if !std::env::args().any(|a|a=="--kit-legacy-super") {
            Atlas::load("assets/animation/raya-convergence-v1-green.png", (1536,1024), &RAYA_CONVERGENCE).await
        } else { None };
        let floor = if body == CharacterId::Kogan {
            Atlas::load("assets/animation/kogan-floor-v1-green.png", (1536, 1024), &KOGAN_FLOOR).await
        } else { None };
        let (air_file, air_specs, air_roots) = match body {
            CharacterId::Kogan => ("kogan-air-recovery-v1-green.png", &KOGAN_AIR_RECOVERY, &KOGAN_AIR_RECOVERY_ROOT_Y),
            CharacterId::Raya => ("raya-air-recovery-v1-green.png", &RAYA_AIR_RECOVERY, &RAYA_AIR_RECOVERY_ROOT_Y),
        };
        let air_recovery = Atlas::load_with_roots(&format!("assets/animation/{air_file}"),
            (1254, 1254), air_specs, air_roots).await;
        let (recoil_file, recoil_size, recoil_specs) = match body {
            CharacterId::Kogan => ("kogan-recoil-v2-green.png", (1024, 1536), &KOGAN_RECOIL),
            CharacterId::Raya => ("raya-recoil-v1-green.png", (941, 1672), &RAYA_RECOIL),
        };
        let recoil = Atlas::load(&format!("assets/animation/{recoil_file}"), recoil_size, recoil_specs).await;
        let signature = if body == CharacterId::Raya {
            Atlas::load("assets/animation/raya-signature-v1-green.png", (1254, 1254), &RAYA_SIGNATURE).await
        } else { None };
        let signature_contacts = if body == CharacterId::Raya {
            Atlas::load("assets/animation/raya-signature-v4-green.png", (1774, 887), &RAYA_SIGNATURE_CONTACTS).await
        } else { None };
        let chant = if body == CharacterId::Raya {
            Atlas::load("assets/animation/raya-chant2-v2-green.png", (1254, 1254), &RAYA_CHANT_II).await
        } else { None };
        let chant_finisher = if body == CharacterId::Raya {
            Atlas::load("assets/animation/raya-chant3-v1-green.png", (1254, 1254), &RAYA_CHANT_III).await
        } else { None };
        let set = Self { packed: None, textures, body, atlas, thrust, thrust_style, reactions, uppercut, compact_uppercut, cuts, first_cut, backcut, poke, disc, judgment, air_shot, air_shot_return, air_saber, air_lights, air_lights_contact, flash, flash_contact, overhead, throw_tech, throw_contact, victory, standing_lights, signature, signature_contacts, chant, chant_finisher, standing_palm_contact, jab_contact, crouch_lights, crouch_kick_contact, crouch_punch, crouch_saber, crouch_saber_contact, crouch_low, floor, air_recovery, recoil, ground, walk, coil, movement, ranged, ritual, utility };
        set.log_selectors();
        set
    }

    /// A set with no textures: cells resolve to pose names only.
    #[cfg(test)]
    pub fn empty(body: CharacterId) -> Self {
        Self::empty_set(body)
    }

    fn empty_set(body: CharacterId) -> Self {
        Self {
            packed: None,
            textures: HashMap::new(),
            body,
            atlas: None,
            thrust: None,
            thrust_style: None,
            reactions: None,
            uppercut: None,
            compact_uppercut: None,
            cuts: None,
            first_cut: None,
            backcut: None,
            poke: None,
            disc: None,
            judgment: None,
            air_shot: None,
            air_saber: None,
            flash: None,
            flash_contact: None,
            overhead: None,
            throw_tech: None,
            throw_contact: None,
            victory: None,
            standing_lights: None,
            signature: None,
            signature_contacts: None,
            chant: None,
            chant_finisher: None,
            standing_palm_contact: None,
            jab_contact: None,
            crouch_lights: None,
            crouch_kick_contact: None,
            crouch_punch: None,
            crouch_saber: None,
            crouch_saber_contact: None,
            crouch_low: None,
            air_lights: None,
            air_lights_contact: None,
            air_shot_return: None,
            floor: None,
            air_recovery: None,
            recoil: None,
            ground: None,
            walk: None,
            coil: None,
            movement: None,
            ranged: None,
            ritual: None,
            utility: None,
        }
    }

    /// Every cell that resolves to a picture in this set.
    pub fn available_cells(&self) -> Vec<Cell> {
        Cell::candidates()
            .into_iter()
            .filter(|c| self.frame(*c).is_some())
            .collect()
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
        if let Some(cell) = crate::sequences::ground_cell(fighter, context) {
            if self.frame(cell).is_some() { return cell; }
        }
        self.cell_for(fighter, tick)
    }

    fn have(&self, requires: &[(Option<CharacterId>, Cell)]) -> bool {
        requires
            .iter()
            .filter(|(body, _)| body.is_none_or(|b| b == self.body))
            .all(|(_, cell)| self.frame(*cell).is_some())
    }

    /// The picture for this fighter on this simulation tick: the first
    /// selector in `SELECTORS` whose family is present and whose pick
    /// resolves, else the signature atlas, else a keyed pose.
    pub fn cell_for(&self, fighter: &Fighter, tick: u32) -> Cell {
        for selector in SELECTORS {
            if !self.have(selector.requires) {
                continue;
            }
            if let Some(cell) = (selector.pick)(fighter) {
                if self.frame(cell).is_some() {
                    return cell;
                }
            }
        }
        // Quiet neutral shares the authored ready pose used by attack returns.
        if fighter.id == CharacterId::Kogan
            && !fighter.airborne
            && matches!(fighter.action, Action::Stand)
            && self.frame(Cell::Flash(3)).is_some()
        {
            return Cell::Flash(3);
        }
        if let Some(cell) = animation_cell(fighter, tick) {
            let thrust = matches!(
                fighter.action,
                Action::Attack {
                    move_id: MoveId::Rekka3,
                    ..
                }
            );
            if thrust && self.frame(Cell::Thrust(cell % 4)).is_some() {
                return Cell::Thrust(cell % 4);
            }
            if self.frame(Cell::Atlas(cell)).is_some() {
                return Cell::Atlas(cell);
            }
        }
        Cell::Pose(phase_pose(fighter))
    }

    /// Resolve a cell to its texture, source rectangle and foot anchor.
    pub fn frame(&self, cell: Cell) -> Option<SpriteFrame<'_>> {
        if let Some(packed) = &self.packed {
            return packed.frame(cell);
        }
        match cell {
            Cell::Movement(cell) => self.movement.as_ref()?.frame(cell),
            Cell::Ranged(cell) => self.ranged.as_ref()?.frame(cell),
            Cell::Ritual(cell) => self.ritual.as_ref()?.frame(cell),
            Cell::Utility(cell) => self.utility.as_ref()?.frame(cell),
            Cell::Victory(cell) => self.victory.as_ref()?.frame(cell),
            Cell::ThrowTech(cell) => self.throw_tech.as_ref()?.frame(cell),
            Cell::ThrowContact => self.throw_contact.as_ref()?.frame(0),
            Cell::Overhead(cell) => self.overhead.as_ref()?.frame(cell),
            Cell::CrouchLights(cell @ 4..=7) if self.body == CharacterId::Kogan => self.crouch_lights.as_ref()?.frame(cell - 4),
            Cell::CrouchLights(5) => self.crouch_kick_contact.as_ref()?.frame(0),
            Cell::CrouchLights(cell) => self.crouch_lights.as_ref()?.frame(cell),
            Cell::CrouchPunch(cell) => self.crouch_punch.as_ref()?.frame(cell),
            Cell::CrouchSaber(cell @ 8..=15) => self.crouch_low.as_ref()?.frame(cell - 8),
            Cell::CrouchSaber(5) if self.body == CharacterId::Raya && self.crouch_saber_contact.is_some() => self.crouch_saber_contact.as_ref()?.frame(0),
            Cell::CrouchSaber(cell) => self.crouch_saber.as_ref()?.frame(cell),
            Cell::Flash(5) if self.body == CharacterId::Raya => self.flash_contact.as_ref()?.frame(0),
            Cell::Flash(cell) => self.flash.as_ref()?.frame(cell),
            Cell::AirSaber(cell @ (0 | 4 | 5)) if self.body == CharacterId::Raya => self.air_lights.as_ref()?.frame(cell),
            Cell::AirSaber(cell @ 1..=3) if self.body == CharacterId::Raya => self.air_saber.as_ref()?.frame(cell - 1),
            Cell::AirSaber(cell) => self.air_saber.as_ref()?.frame(cell),
            Cell::AirLights(cell @ 1..=3) if self.body == CharacterId::Kogan => self.air_lights_contact.as_ref()?.frame(cell - 1),
            Cell::AirLights(cell) => self.air_lights.as_ref()?.frame(cell),
            Cell::AirShot(3) => self.air_shot_return.as_ref()?.frame(0),
            Cell::AirShot(cell) => self.air_shot.as_ref()?.frame(cell),
            Cell::Judgment(cell) => self.judgment.as_ref()?.frame(cell),
            Cell::Floor(cell) => self.floor.as_ref()?.frame(cell),
            Cell::AirRecovery(cell) => self.air_recovery.as_ref()?.frame(cell),
            // The darker jab contact keeps the original fallback and is bracketed
            // by the existing drawn fist gather/withdrawal and relaxed ready.
            Cell::StandingLights(cell) if self.body == CharacterId::Kogan => {
                match cell {
                    0 | 2 => self.flash.as_ref()?.frame(2),
                    3 => self.flash.as_ref()?.frame(3),
                    1 => self.jab_contact.as_ref().and_then(|atlas| atlas.frame(0))
                        .or_else(|| self.frame(Cell::Pose(Pose::P))),
                    4..=7 => self.standing_lights.as_ref()?.frame(cell - 4),
                    _ => None,
                }
            }
            Cell::StandingLights(1) => self.standing_palm_contact.as_ref()?.frame(0),
            Cell::StandingLights(cell) => self.standing_lights.as_ref()?.frame(cell),
            Cell::Signature(cell @ 4..=5) => self.signature_contacts.as_ref()?.frame(cell - 4),
            Cell::Signature(cell) => self.signature.as_ref()?.frame(cell),
            Cell::Chant(cell @ 4..=7) => self.chant_finisher.as_ref()?.frame(cell - 4),
            Cell::Chant(cell) => self.chant.as_ref()?.frame(cell),
            Cell::Recoil(cell) => self.recoil.as_ref()?.frame(cell),
            Cell::Reaction(cell) => self.reactions.as_ref()?.frame(cell),
            Cell::Poke(cell) => self.poke.as_ref()?.frame(cell),
            Cell::Disc(cell) => self.disc.as_ref()?.frame(cell),
            Cell::Ground(cell) => self.ground.as_ref()?.frame(cell),
            Cell::UppercutCompact(cell) => self.compact_uppercut.as_ref()?.frame(cell),
            Cell::Uppercut(0) if self.coil.is_some() => self.coil.as_ref()?.frame(0),
            Cell::Uppercut(cell) => self.uppercut.as_ref()?.frame(cell),
            Cell::Thrust(cell) if self.thrust_style.is_some() => self.thrust_style.as_ref()?.frame(cell),
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
            Cell::Atlas(cell @ 4..=7) if self.first_cut.is_some() => self.first_cut.as_ref()?.frame(cell - 4),
            Cell::Atlas(cell @ 8..=11) if self.backcut.is_some() => self.backcut.as_ref()?.frame(cell - 8),
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
    fn transparent_key_cannot_reintroduce_green_through_linear_filtering() {
        let copper = [184, 115, 51, 255];
        let mut image = Image { width: 2, height: 1,
            bytes: [copper, [0, 230, 0, 255]].concat() };
        key_green(&mut image);
        assert_eq!(&image.bytes[..4], &copper, "opaque copper is untouched");
        assert_eq!(image.bytes[7], 0);
        // Straight-alpha texture filtering interpolates RGB even for the
        // transparent texel. Sample near the outside edge over black.
        for fraction in [0.25_f32, 0.5, 0.75] {
            let sample: [f32; 4] = std::array::from_fn(|c|
                f32::from(image.bytes[c]) * (1.0-fraction)
                    + f32::from(image.bytes[c+4]) * fraction);
            let rgb = [0, 1, 2].map(|c| sample[c] * sample[3] / 255.0);
            assert!(rgb[0] > rgb[1] && rgb[1] > rgb[2], "copper edge became green: {rgb:?}");
            assert!((sample[3] - 255.0 * (1.0-fraction)).abs() < 0.01);
        }
    }

    #[test]
    fn dark_key_spill_is_neutralized_without_eroding_coverage() {
        let pixels = [[21, 42, 15, 255], [0, 63, 0, 4], [14, 17, 22, 255],
            [20, 60, 64, 255], [38, 27, 19, 255]];
        let mut image = Image { width: 5, height: 1, bytes: pixels.concat() };
        key_green(&mut image);
        assert_eq!(&image.bytes[0..4], &[21, 21, 15, 255]);
        assert_eq!(&image.bytes[4..8], &[0, 0, 0, 4]);
        assert_eq!(&image.bytes[8..], &pixels[2..].concat(),
            "dark blue, cyan writing and copper remain untouched");
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
