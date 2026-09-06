//! Authored reactions, floor recovery and reversals. Drawings follow the
//! existing action/velocity; they never extend an input or recovery window.
use aeon_sim::{Action, Fighter, MoveId, GETUP_FRAMES};
use macroquad::prelude::*;
use crate::sprites::{key_green, Cell, SpriteFrame};

/// Source region [left, top, right, bottom], projected root x, anatomical
/// standing height, all in reference-image pixels. Green gaps define regions;
/// generated rows are not assumed to be an exact grid.
pub type Spec = ([u16; 4], u16, u16);

pub struct Atlas {
    texture: Texture2D,
    frames: Vec<Frame>,
}

struct Frame {
    source: Rect,
    anchor: Vec2,
    height: f32,
}

impl Atlas {
    pub async fn load(path: &str, reference: (u16, u16), specs: &[Spec]) -> Option<Self> {
        Self::load_with_roots(path, reference, specs, &[]).await
    }

    /// Airborne poses use a projected standing root below their tucked feet.
    /// Grounded cells retain the measured lowest solid pixel as their root.
    pub async fn load_with_roots(path: &str, reference: (u16, u16), specs: &[Spec],
        roots_y: &[Option<u16>]) -> Option<Self> {
        let mut image = match load_image(path).await {
            Ok(image) => image,
            Err(e) => {
                eprintln!("[aeon] sequence fallback {path}: {e}");
                return None;
            }
        };
        key_green(&mut image);
        let width = image.width as usize;
        let height = image.height as usize;
        let mut frames = Vec::new();
        for (i, &(r, root_x, body_height)) in specs.iter().enumerate() {
            let x0 = r[0] as usize * width / reference.0 as usize;
            let x1 = r[2] as usize * width / reference.0 as usize;
            let y0 = r[1] as usize * height / reference.1 as usize;
            let y1 = r[3] as usize * height / reference.1 as usize;
            let root_x = (root_x - r[0]) as f32 / (r[2] - r[0]) as f32;
            let body_height = body_height as f32 / (r[3] - r[1]) as f32;
            // Ignore a two-pixel technical gutter. Measure the lowest visible
            // pixel, including cloth, so a recumbent drawing rests on the floor.
            let mut bottom = None;
            for y in y0 + 2..y1 - 2 {
                for x in x0 + 2..x1 - 2 {
                    if image.bytes[(y * width + x) * 4 + 3] >= 24 {
                        bottom = Some(y + 1);
                    }
                }
            }
            let Some(bottom) = bottom else {
                eprintln!("[aeon] empty sequence cell {i}: {path}");
                return None;
            };
            let bottom = roots_y.get(i).copied().flatten()
                .map(|y| y as usize * height / reference.1 as usize).unwrap_or(bottom);
            let cell_h = (y1 - y0) as f32;
            frames.push(Frame {
                source: Rect::new((x0 + 2) as f32, (y0 + 2) as f32,
                    (x1 - x0 - 4) as f32, cell_h - 4.0),
                anchor: vec2((root_x * (x1 - x0) as f32 - 2.0) / (x1 - x0 - 4) as f32,
                    (bottom - y0 - 2) as f32 / (cell_h - 4.0)),
                height: 1.20 / body_height * (cell_h - 4.0) / cell_h,
            });
        }
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);
        eprintln!("[aeon] {path}: {} authored cells", frames.len());
        Some(Self { texture, frames })
    }

    pub fn frame(&self, cell: usize) -> Option<SpriteFrame<'_>> {
        let f = self.frames.get(cell)?;
        Some(SpriteFrame { texture: &self.texture, source: Some(f.source),
            anchor: f.anchor, height: f.height })
    }
}

// Measured root positions and anatomical standing-height fractions, rather
// than fitting each silhouette (which would enlarge crouches and prone bodies).
// Full provenance and measurement notes live beside the source PNGs.
pub const KOGAN_REACTIONS: [Spec; 12] = [
    ([0, 0, 380, 380], 212, 276), ([380, 0, 700, 380], 570, 304),
    ([700, 0, 1040, 380], 900, 290), ([1040, 0, 1448, 380], 1245, 319),
    ([0, 390, 380, 690], 190, 319), ([380, 390, 720, 690], 575, 319),
    ([720, 390, 1068, 690], 918, 311), ([1068, 390, 1448, 690], 1280, 311),
    ([0, 700, 365, 1086], 220, 311), ([365, 700, 720, 1086], 565, 311),
    ([720, 700, 1075, 1086], 901, 311), ([1075, 700, 1448, 1086], 1228, 319),
];
// Quiet revolver draw/raise/settle, with the saber continuously lowered.
pub const KOGAN_VICTORY: [Spec; 4] = [
    ([0, 0, 760, 510], 425, 480), ([760, 0, 1536, 510], 1083, 480),
    ([0, 508, 760, 1024], 425, 480), ([760, 508, 1536, 1024], 1083, 480),
];

// Quiet cupped-crystal gather, rise and settled offering.
pub const RAYA_VICTORY: [Spec; 4] = [
    ([0, 0, 768, 504], 448, 454),
    ([768, 0, 1536, 504], 1070, 454),
    ([0, 504, 768, 1024], 448, 454),
    ([768, 504, 1536, 1024], 1062, 454),
];

pub fn victory_cell(f: &Fighter, age: u32) -> Option<Cell> {
    if !crate::anim::victory_at_rest(f) { return None; }
    Some(Cell::Victory(match age { 0..=7 => 0, 8..=15 => 1, 16..=23 => 2, _ => 3 }))
}

/// A canceled startup withdraws its own equipment, then regains ready.
/// The existing eight-frame state owns the clock; landing/new actions take over.
pub fn feint_cell(f: &Fighter) -> Option<Cell> {
    let Action::Feint { frame } = f.action else { return None; };
    let move_id = f.last_move?;
    if !f.data().move_def(move_id)?.feintable { return None; }
    let phase = usize::from(frame >= aeon_sim::fighter::FEINT_RECOVERY / 2);
    if f.airborne { return Some([Cell::AirSaber(4), Cell::AirSaber(5)][phase]); }
    if f.id == aeon_sim::CharacterId::Raya {
        if move_id == MoveId::Charge {
            return Some(Cell::Ritual(match frame { 0..=2 => 3, 3..=5 => 4, _ => 5 }));
        }
        let cells = match move_id {
            MoveId::ShotA | MoveId::ExB => [Cell::Ranged(2), Cell::Ranged(3)],
            MoveId::ShotB | MoveId::ExA => [Cell::Ranged(6), Cell::Ranged(7)],
            MoveId::Rekka1 => [Cell::Signature(6), Cell::Signature(9)],
            MoveId::Rekka2 => [Cell::Chant(2), Cell::Chant(3)],
            MoveId::Rekka3 => [Cell::Chant(6), Cell::Chant(7)],
            MoveId::CommandGrab | MoveId::Uppercut => [Cell::Utility(2), Cell::Utility(3)],
            _ => return None,
        };
        return Some(cells[phase]);
    }
    let cells = match move_id {
        MoveId::ShotA | MoveId::ExB => [Cell::Ranged(3), Cell::Ranged(7)],
        MoveId::ShotB => [Cell::Ranged(6), Cell::Ranged(7)],
        MoveId::Guard => [Cell::Disc(2), Cell::Disc(3)],
        MoveId::Rekka1 | MoveId::ExA => [Cell::Atlas(7), Cell::Utility(3)],
        MoveId::Rekka2 => [Cell::Atlas(11), Cell::Utility(3)],
        MoveId::Rekka3 => [Cell::Thrust(3), Cell::Utility(3)],
        MoveId::CommandGrab | MoveId::Uppercut | MoveId::SpecialOverhead => [Cell::Utility(2), Cell::Utility(3)],
        _ => return None,
    };
    Some(cells[phase])
}

// Throw tech is an open-palm separation, then withdrawal and the familiar ready.
// The world advances entry frame 0 immediately; visible phases are 1..=15.
pub const KOGAN_THROW_TECH: [Spec; 2] = [
    ([0, 0, 720, 1024], 440, 670), ([720, 0, 1536, 1024], 1150, 670),
];

// Empty-hand normal reach stays distinct from the Rite's cyan loop.
pub const RAYA_THROW_CONTACT: [Spec; 1] = [([0, 0, 1254, 1254], 665, 940)];

pub fn throw_tech_cell(f: &Fighter) -> Option<Cell> {
    if f.airborne { return None; }
    let Action::ThrowTech { frame } = f.action else { return None; };
    Some(if frame <= 5 {
        if f.id == aeon_sim::CharacterId::Raya { Cell::Recoil(4) } else { Cell::ThrowTech(0) }
    } else if frame <= 10 {
        if f.id == aeon_sim::CharacterId::Raya { Cell::Recoil(5) } else { Cell::ThrowTech(1) }
    } else { Cell::Utility(3) })
}

// Standing overhead keeps a complete raised blade, forward cut and two returns.
// Anatomy is shared rather than fitting the extra height of the raised arms.
pub const KOGAN_OVERHEAD: [Spec; 4] = [
    ([0, 0, 520, 625], 330, 450), ([520, 0, 1254, 625], 900, 450),
    ([0, 625, 615, 1254], 315, 450), ([615, 625, 1254, 1254], 905, 450),
];

// Six deliberate grounded phases; raised hands do not change anatomical scale.
pub const RAYA_OVERHEAD: [Spec; 6] = [
    ([0, 0, 500, 510], 260, 400), ([500, 0, 1024, 510], 680, 400),
    ([0, 510, 500, 970], 300, 400), ([500, 510, 1024, 970], 680, 400),
    ([0, 970, 500, 1536], 260, 420), ([500, 970, 1024, 1536], 680, 420),
];

pub fn overhead_cell(f: &Fighter) -> Option<Cell> {
    if f.airborne { return None; }
    let Action::Attack { move_id: MoveId::Overhead, frame, .. } = f.action else { return None; };
    let mv = f.data().move_def(MoveId::Overhead)?;
    let phase = match f.id {
        aeon_sim::CharacterId::Kogan => {
            if frame < mv.first_active() { 0 }
            else if mv.is_active(frame) { 1 }
            else if frame <= mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
            else { 3 }
        }
        aeon_sim::CharacterId::Raya => {
            if frame < mv.first_active() / 2 { 0 }
            else if frame < mv.first_active() { 1 }
            else if mv.is_active(frame) { 2 }
            else if frame <= mv.last_active() + u16::from(mv.recovery) / 3 { 3 }
            else if frame <= mv.last_active() + u16::from(mv.recovery) * 2 / 3 { 4 }
            else { 5 }
        }
    };
    Some(Cell::Overhead(phase))
}

// Three ritual normals have distinct reach and four drawn phases.
// Source rows overlap only their measured empty green gutter.
pub const RAYA_SIGNATURE: [Spec; 12] = [
    ([0, 0, 410, 328], 230, 302), ([410, 0, 840, 328], 640, 302),
    ([840, 0, 1254, 328], 1015, 302),
    ([0, 328, 410, 643], 225, 302), ([410, 328, 840, 643], 610, 302),
    ([840, 328, 1254, 643], 990, 302),
    ([0, 643, 410, 942], 205, 293), ([410, 643, 840, 942], 575, 293),
    ([840, 643, 1254, 942], 970, 293),
    ([0, 941, 410, 1254], 200, 295), ([410, 941, 840, 1254], 575, 295),
    ([840, 941, 1254, 1254], 970, 295),
];

// V4 aligns the two heavy contacts. Its measured bodies and margins differ
// from V1, so only these two drawings use the revised sheet.
pub const RAYA_SIGNATURE_CONTACTS: [Spec; 2] = [
    ([0, 0, 890, 887], 380, 590),
    ([890, 0, 1774, 887], 1230, 570),
];

pub fn signature_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Raya || f.airborne { return None; }
    let Action::Attack { move_id, frame, .. } = f.action else { return None; };
    let column = match move_id { MoveId::StS => 0, MoveId::StHS => 1,
        MoveId::StHSClose => 2, _ => return None };
    let mv = f.data().move_def(move_id)?;
    let phase = if frame < mv.first_active() { 0 }
        else if mv.is_active(frame) { 1 }
        else if frame <= mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
        else { 3 };
    Some(Cell::Signature(phase * 3 + column))
}

// Chants keep their existing forward travel and follow-up windows. The
// opening syllable deliberately reuses the reviewed low medium-palm drawings.
pub const RAYA_CHANT_II: [Spec; 4] = [
    ([0, 0, 627, 627], 340, 509), ([627, 0, 1254, 627], 895, 506),
    ([0, 627, 627, 1254], 340, 502), ([627, 627, 1254, 1254], 895, 505),
];
pub const RAYA_CHANT_III: [Spec; 4] = [
    ([0, 0, 627, 627], 355, 500), ([627, 0, 1254, 627], 900, 484),
    ([0, 627, 627, 1254], 345, 495), ([627, 627, 1254, 1254], 910, 493),
];

pub fn chant_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Raya || f.airborne { return None; }
    let Action::Attack { move_id, frame, .. } = f.action else { return None; };
    let part = match move_id { MoveId::Rekka1 => 0, MoveId::Rekka2 => 1,
        MoveId::Rekka3 => 2, _ => return None };
    let mv = f.data().move_def(move_id)?;
    let phase = if frame < mv.first_active() { 0 }
        else if mv.is_active(frame) { 1 }
        else if frame <= mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
        else { 3 };
    Some(if part == 0 { Cell::Signature(phase * 3) }
        else { Cell::Chant((part - 1) * 4 + phase) })
}

// Short palm and low boot have their own gather/contact/withdraw/ready drawings.
// The existing move clock, including frozen hitstop, owns every phase.
pub const RAYA_STANDING_LIGHTS: [Spec; 8] = [
    ([0, 0, 500, 385], 255, 330), ([500, 0, 1024, 385], 690, 330),
    ([0, 385, 500, 745], 255, 330), ([500, 385, 1024, 745], 685, 330),
    ([0, 745, 500, 1115], 255, 330), ([500, 745, 1024, 1115], 680, 330),
    ([0, 1115, 500, 1536], 255, 330), ([500, 1115, 1024, 1536], 685, 330),
];

pub fn standing_lights_cell(f: &Fighter) -> Option<Cell> {
    if f.airborne { return None; }
    let Action::Attack { move_id, frame, .. } = f.action else { return None; };
    if f.id == aeon_sim::CharacterId::Kogan && move_id != MoveId::StP { return None; }
    let base = match move_id { MoveId::StP => 0, MoveId::StK => 4, _ => return None };
    let mv = f.data().move_def(move_id)?;
    let phase = if frame < mv.first_active() { 0 }
        else if mv.is_active(frame) { 1 }
        else if frame <= mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
        else { 3 };
    Some(Cell::StandingLights(base + phase))
}

// Distinct low palm and supported ankle kick; the existing action owns timing.
pub const RAYA_CROUCH_LIGHTS: [Spec; 8] = [
    ([0, 0, 500, 385], 285, 450), ([500, 0, 1024, 385], 705, 450),
    ([0, 385, 500, 750], 285, 450), ([500, 385, 1024, 750], 725, 450),
    ([0, 750, 475, 1105], 190, 450), ([475, 750, 1024, 1105], 620, 450),
    ([0, 1105, 480, 1536], 280, 450), ([480, 1105, 1024, 1536], 695, 450),
];

pub fn crouch_lights_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Raya || f.airborne { return None; }
    let Action::Attack { move_id, frame, .. } = f.action else { return None; };
    let base = match move_id { MoveId::CrP => 0, MoveId::CrK => 4, _ => return None };
    let mv = f.data().move_def(move_id)?;
    let phase = if frame < mv.first_active() { 0 }
        else if mv.is_active(frame) { 1 }
        else if frame <= mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
        else { 3 };
    Some(Cell::CrouchLights(base + phase))
}

// The low fist has a gathered arm, contact, bent-elbow withdrawal and low ready.
pub const KOGAN_CROUCH_PUNCH: [Spec; 4] = [
    ([0, 0, 625, 620], 355, 510),
    ([625, 0, 1254, 620], 955, 510),
    ([0, 620, 625, 1254], 355, 510),
    ([625, 620, 1254, 1254], 955, 510),
];

pub fn crouch_punch_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Kogan || f.airborne { return None; }
    let Action::Attack { move_id: MoveId::CrP, frame, .. } = f.action else { return None; };
    let mv = f.data().move_def(MoveId::CrP)?;
    Some(Cell::CrouchPunch(if frame < mv.first_active() { 0 }
        else if mv.is_active(frame) { 1 }
        else if frame < mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
        else { 3 }))
}

// Crouching forward and rising saber paths retain grounded support and common anatomy.
pub const KOGAN_CROUCH_SABER: [Spec; 8] = [
    ([0, 0, 480, 410], 285, 370), ([480, 0, 1024, 410], 750, 370),
    ([0, 410, 480, 750], 285, 370), ([480, 410, 1024, 750], 750, 370),
    ([0, 750, 480, 1140], 270, 370), ([480, 750, 1024, 1140], 740, 370),
    ([0, 1140, 480, 1536], 275, 370), ([480, 1140, 1024, 1536], 735, 370),
];

// Lower trap and sweep: full blades above measured boot/palm support.
pub const KOGAN_CROUCH_LOW: [Spec; 8] = [
    ([0, 0, 500, 430], 275, 350), ([500, 0, 1024, 430], 730, 350),
    ([0, 430, 500, 770], 275, 350), ([500, 430, 1024, 770], 740, 350),
    ([0, 770, 475, 1110], 255, 350), ([475, 770, 1024, 1110], 685, 350),
    ([0, 1110, 500, 1536], 275, 350), ([500, 1110, 1024, 1536], 735, 350),
];

// Raya keeps low support beneath horizontal and vertical crystal gestures.
pub const RAYA_CROUCH_CRYSTALS: [Spec; 8] = [
    ([0, 0, 490, 395], 285, 450), ([490, 0, 1024, 395], 705, 450),
    ([0, 395, 490, 750], 285, 450), ([490, 395, 1024, 750], 710, 450),
    ([0, 750, 490, 1110], 280, 450), ([490, 750, 1024, 1110], 695, 450),
    ([0, 1110, 490, 1536], 280, 450), ([490, 1110, 1024, 1536], 695, 450),
];

// Longer vertical tip meets rising/falling airborne bodies at the existing CrHS reach.
pub const RAYA_ANTI_CRYSTAL: [Spec; 1] = [([100, 400, 920, 1200], 570, 900)];

// Low palm and one-hand-supported sweep have full sandals and distinct returns.
pub const RAYA_CROUCH_LOW: [Spec; 8] = [
    ([0, 0, 480, 420], 285, 500), ([480, 0, 1024, 420], 715, 500),
    ([0, 420, 470, 790], 280, 500), ([470, 420, 1024, 790], 710, 500),
    ([0, 790, 425, 1120], 245, 500), ([425, 790, 1024, 1120], 675, 500),
    ([0, 1120, 450, 1536], 265, 500), ([450, 1120, 1024, 1536], 705, 500),
];

pub fn crouch_saber_cell(f: &Fighter) -> Option<Cell> {
    if f.airborne { return None; }
    let Action::Attack { move_id, frame, .. } = f.action else { return None; };
    let base = match move_id { MoveId::CrS => 0, MoveId::CrHS => 4, MoveId::CrFL => 8, MoveId::CrST => 12, _ => return None };
    let mv = f.data().move_def(move_id)?;
    let phase = if frame < mv.first_active() { 0 }
        else if mv.is_active(frame) { 1 }
        else if frame <= mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
        else { 3 };
    Some(Cell::CrouchSaber(base + phase))
}

// Standing Flash's short pommel and Style's waist-level saber each have four phases.
// Grounded roots and shared anatomy preserve support; both gestures share the sound withdrawal.
pub const KOGAN_FLASH: [Spec; 8] = [
    ([0, 0, 460, 410], 305, 310), ([460, 0, 1024, 410], 715, 310),
    ([0, 1110, 490, 1536], 275, 310), ([460, 410, 1024, 780], 710, 310),
    ([0, 780, 460, 1110], 300, 310), ([460, 780, 1024, 1110], 710, 310),
    ([0, 1110, 490, 1536], 275, 310), ([490, 1110, 1024, 1536], 690, 310),
];

// Raya's low glyph press and supported cape turn keep the authored move clock.
pub const RAYA_FLASH: [Spec; 8] = [
    ([0, 0, 500, 390], 240, 342), ([500, 0, 1024, 390], 685, 342),
    ([0, 390, 500, 745], 240, 342), ([500, 390, 1024, 745], 685, 342),
    ([0, 745, 500, 1105], 250, 342), ([500, 745, 1024, 1105], 685, 342),
    ([0, 1105, 500, 1536], 240, 347), ([500, 1105, 1024, 1536], 685, 347),
];

pub fn flash_cell(f: &Fighter) -> Option<Cell> {
    if f.airborne { return None; }
    let Action::Attack { move_id, frame, .. } = f.action else { return None; };
    let base = match move_id { MoveId::StFL => 0, MoveId::StST => 4, _ => return None };
    let mv = f.data().move_def(move_id)?;
    let phase = if frame < mv.first_active() { 0 }
        else if mv.is_active(frame) { 1 }
        else if frame <= mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
        else { 3 };
    Some(Cell::Flash(base + phase))
}

// Airborne fist, boot and knee share a chamber and gathered withdrawal.
// Anatomical scale and projected roots remain independent of tucked silhouette size.
pub const KOGAN_AIR_LIGHTS: [Spec; 6] = [
    ([0, 0, 512, 510], 330, 430), ([512, 0, 1024, 510], 805, 430),
    ([0, 510, 535, 960], 295, 430), ([535, 510, 1024, 960], 800, 430),
    ([0, 960, 512, 1536], 300, 430), ([512, 960, 1024, 1536], 800, 430),
];
// Corrected lower fist/boot/knee paths; the knee carries the hips into its shorter reach.
pub const KOGAN_AIR_LIGHTS_CONTACT: [Spec; 3] = [
    ([512, 0, 1024, 510], 805, 430),
    ([0, 510, 535, 960], 295, 430),
    ([535, 510, 1024, 960], 760, 430),
];
pub const KOGAN_AIR_LIGHTS_ROOT_Y: [Option<u16>; 6] = [
    Some(535), Some(555), Some(980), Some(980), Some(1420), Some(1460),
];

// Raya's palm, downward boot and small glyph share gathered preparation/return.
// Anatomical scale is independent of each tucked silhouette's measured region.
pub const RAYA_AIR_LIGHTS: [Spec; 6] = [
    ([0, 0, 512, 480], 320, 480), ([512, 0, 1024, 480], 720, 480),
    ([0, 480, 512, 925], 355, 480), ([512, 480, 1024, 925], 710, 480),
    ([0, 925, 500, 1536], 300, 480), ([500, 925, 1024, 1536], 715, 480),
];
pub const RAYA_AIR_LIGHTS_ROOT_Y: [Option<u16>; 6] = [
    Some(525), Some(510), Some(935), Some(920), Some(1410), Some(1420),
];

pub fn air_lights_cell(f: &Fighter) -> Option<Cell> {
    if !f.airborne { return None; }
    if matches!(f.action, Action::Jump { air_ok: false, .. })
        && matches!(f.last_move, Some(MoveId::JP | MoveId::JK | MoveId::JFL)) {
        return Some(Cell::AirLights(5));
    }
    if let Action::Attack { move_id, frame, .. } = f.action {
        let contact = match move_id { MoveId::JP => 1, MoveId::JK => 2, MoveId::JFL => 3, _ => return None };
        let mv = f.data().move_def(move_id)?;
        return Some(Cell::AirLights(if frame < mv.first_active() { 0 }
            else if mv.is_active(frame) { contact }
            else if frame < mv.total_frames().saturating_sub(2) { 4 } else { 5 }));
    }
    None
}

// Short, long and steep downward cuts share a gather and two withdrawal drawings.
// Measured gaps and shared anatomy preserve full blades and consistent scale.
pub const KOGAN_AIR_SABER: [Spec; 6] = [
    ([0, 0, 535, 500], 365, 430), ([535, 0, 1024, 500], 770, 430),
    ([0, 510, 565, 977], 230, 430), ([565, 510, 1024, 977], 735, 430),
    ([0, 980, 520, 1536], 270, 430), ([520, 980, 1024, 1536], 740, 430),
];
pub const KOGAN_AIR_SABER_ROOT_Y: [Option<u16>; 6] = [
    Some(480), Some(480), Some(930), Some(955), Some(1410), Some(1450),
];

// Raya reuses approved AirLights gather/fold/ready; only contacts are new.
pub const RAYA_AIR_CRYSTALS: [Spec; 3] = [
    ([0, 0, 1024, 500], 435, 480),
    ([0, 500, 1024, 1000], 540, 480),
    ([0, 1000, 1024, 1536], 430, 480),
];
pub const RAYA_AIR_CRYSTALS_ROOT_Y: [Option<u16>; 3] = [
    Some(480), Some(1000), Some(1480),
];

pub fn air_saber_cell(f: &Fighter) -> Option<Cell> {
    if !f.airborne { return None; }
    if matches!(f.action, Action::Jump { air_ok: false, .. })
        && matches!(f.last_move, Some(MoveId::JS | MoveId::JHS | MoveId::JST)) {
        return Some(Cell::AirSaber(5));
    }
    if let Action::Attack { move_id, frame, .. } = f.action {
        let contact = match move_id { MoveId::JS => 1, MoveId::JHS => 2, MoveId::JST => 3, MoveId::SpecialOverhead if f.id == aeon_sim::CharacterId::Kogan => 3, _ => return None };
        let mv = f.data().move_def(move_id)?;
        return Some(Cell::AirSaber(if frame < mv.first_active() { 0 }
            else if mv.is_active(frame) { contact }
            else if frame < mv.total_frames().saturating_sub(3) { 4 } else { 5 }));
    }
    None
}

// Shared unfolded anatomy and projected air roots keep tucked feet from
// moving the body origin. Aim aligns with the existing downward projectile.
pub const KOGAN_AIR_SHOT: [Spec; 4] = [
    ([0, 0, 627, 560], 430, 530), ([627, 0, 1254, 560], 940, 530),
    ([0, 560, 627, 1254], 390, 530), ([627, 560, 1254, 1254], 930, 530),
];
pub const KOGAN_AIR_SHOT_ROOT_Y: [Option<u16>; 4] = [
    Some(610), Some(656), Some(1190), Some(1170),
];

pub fn air_shot_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Kogan || !f.airborne { return None; }
    if let Action::Attack { move_id: MoveId::AirShot, frame, .. } = f.action {
        let first = f.data().move_def(MoveId::AirShot)?.first_active();
        return Some(Cell::AirShot(if frame < first / 2 { 0 }
            else if frame <= first { 1 }
            else if frame < first + 3 { 2 } else { 3 }));
    }
    None
}

// Judgment gathers, extends and withdraws its two weapons during the existing rush.
pub const KOGAN_JUDGMENT: [Spec; 4] = [
    ([0, 0, 670, 480], 340, 445), ([670, 0, 1536, 480], 1000, 445),
    ([0, 480, 736, 1024], 420, 445), ([736, 480, 1536, 1024], 1040, 445),
];

pub const RAYA_CONVERGENCE: [Spec; 4] = [
    ([0,0,720,510],460,420), ([720,0,1536,510],1140,420),
    ([0,510,720,1024],460,420), ([720,510,1536,1024],1060,420),
];

pub fn judgment_cell(f: &Fighter) -> Option<Cell> {
    if let Action::Attack { move_id: MoveId::Super, frame, .. } = f.action {
        if f.id == aeon_sim::CharacterId::Raya {
            return Some(Cell::Judgment(match frame {0..=5=>0,6..=11=>1,12..=31=>2,_=>3}));
        }
        let mv = f.data().move_def(MoveId::Super)?;
        return Some(Cell::Judgment(if frame < mv.first_active() { 0 }
            else if mv.is_active(frame) { 1 }
            else if frame < mv.total_frames().saturating_sub(8) { 2 } else { 3 }));
    }
    None
}

// Prone, seated hand support, kneel and half rise share anatomical scale.
pub const KOGAN_FLOOR: [Spec; 4] = [
    ([0, 0, 773, 449], 400, 530), ([773, 0, 1536, 449], 1090, 530),
    ([0, 449, 774, 1024], 410, 530), ([774, 449, 1536, 1024], 1110, 530),
];

pub fn floor_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Kogan { return None; }
    match f.action {
        Action::Knockdown { .. } => Some(Cell::Floor(0)),
        Action::Getup { frame } => Some(Cell::Floor((frame * 4 / GETUP_FRAMES).min(3) as usize)),
        _ => None,
    }
}

// Non-knockdown air recoil releases into a tuck and feet-down descent.
// Anatomical scale is shared; air roots project below the tucked boots.
pub const KOGAN_AIR_RECOVERY: [Spec; 4] = [
    ([0, 0, 625, 600], 400, 500), ([625, 0, 1254, 600], 972, 500),
    ([0, 600, 625, 1254], 375, 500), ([625, 600, 1254, 1254], 950, 500),
];
pub const KOGAN_AIR_RECOVERY_ROOT_Y: [Option<u16>; 4] = [
    Some(540), Some(540), Some(1150), None,
];

pub const RAYA_AIR_RECOVERY: [Spec; 4] = [
    ([0, 0, 625, 600], 400, 480), ([625, 0, 1254, 600], 923, 480),
    ([0, 600, 625, 1254], 428, 480), ([625, 600, 1254, 1254], 945, 480),
];
pub const RAYA_AIR_RECOVERY_ROOT_Y: [Option<u16>; 4] = [
    Some(510), Some(510), Some(1100), None,
];

pub fn air_recovery_cell(f: &Fighter) -> Option<Cell> {
    if !f.airborne { return None; }
    let Action::Hit { stun, knockdown: false } = f.action else { return None; };
    // A continuing knockdown keeps its established tumble. A normal juggle
    // retains recoil until the final four stun ticks; this never returns control.
    Some(Cell::AirRecovery(if f.vel.y > 0 || stun >= 4 { 0 }
        else if f.pos.y > aeon_sim::px(24) { 1 } else { 2 }))
}

// Contact / release pairs retain a shared anatomical scale. The narrow
// second-row gutter is measured rather than split at the nominal midpoint.
pub const KOGAN_RECOIL: [Spec; 8] = [
    ([0, 0, 490, 410], 260, 330), ([490, 0, 1024, 410], 715, 330),
    ([0, 410, 521, 730], 265, 330), ([521, 410, 1024, 730], 710, 330),
    ([0, 730, 465, 1130], 255, 330), ([465, 730, 1024, 1130], 695, 330),
    ([0, 1130, 467, 1536], 260, 330), ([467, 1130, 1024, 1536], 715, 330),
];

/// Show recovery during the final four existing stun frames (3 through 0). Knockdowns
/// retain recoil through the fall; no false return to control is suggested.
/// Remaining stun is frozen by the sim during hitstop and replay pause.
pub fn recoil_cell(f: &Fighter) -> Option<Cell> {
    if f.airborne { return None; }
    match f.action {
        Action::Hit { stun, knockdown } => Some(Cell::Recoil(
            usize::from(f.input().down()) * 2 + usize::from(stun < 4 && !knockdown))),
        Action::Block { crouching, stun } => Some(Cell::Recoil(
            4 + usize::from(crouching) * 2 + usize::from(stun < 4))),
        Action::Thrown { .. } => Some(Cell::Recoil(0)),
        _ => None,
    }
}

// Grounded impact/release pairs share anatomy through deep crouching.
pub const RAYA_RECOIL: [Spec; 8] = [
    ([0, 0, 470, 510], 300, 390), ([470, 0, 941, 510], 690, 390),
    ([0, 510, 470, 860], 300, 390), ([470, 510, 941, 860], 690, 390),
    ([0, 860, 470, 1320], 280, 390), ([470, 860, 941, 1320], 670, 390),
    ([0, 1320, 470, 1672], 285, 390), ([470, 1320, 941, 1672], 685, 390),
];

pub const RAYA_REACTIONS: [Spec; 12] = [
    ([0, 0, 350, 382], 228, 320), ([350, 0, 700, 382], 518, 332),
    ([700, 0, 1055, 382], 905, 324), ([1055, 0, 1448, 382], 1254, 336),
    ([0, 390, 410, 680], 225, 330), ([410, 390, 750, 680], 565, 332),
    ([750, 390, 1080, 680], 930, 332), ([1080, 390, 1448, 680], 1250, 335),
    ([0, 688, 365, 1086], 213, 342), ([365, 688, 700, 1086], 510, 334),
    ([700, 688, 1065, 1086], 920, 340), ([1065, 688, 1448, 1086], 1237, 350),
];
// Two cloth-only glide keys, half/full crouch, and four backward retreat
// phases. Shared anatomy preserves the head size through compression.
pub const KOGAN_GROUND: [Spec; 8] = [
    ([0, 0, 365, 500], 250, 340), ([365, 0, 723, 500], 615, 340),
    ([723, 0, 1135, 500], 915, 340), ([1135, 0, 1536, 500], 1345, 340),
    ([0, 500, 374, 1024], 232, 340), ([374, 500, 780, 1024], 570, 340),
    ([780, 500, 1150, 1024], 953, 340), ([1150, 500, 1536, 1024], 1350, 340),
];

// Raya holds her glide legs while linen/copper advance through two folds.
// A gentle knee bend gathers a run; supported brake/ready serve either direction.
pub const RAYA_GROUND: [Spec; 8] = [
    ([0, 0, 420, 460], 270, 380), ([420, 0, 830, 460], 670, 380),
    ([830, 0, 1210, 460], 1033, 380), ([1210, 0, 1672, 460], 1426, 380),
    ([0, 460, 420, 941], 268, 380), ([420, 460, 830, 941], 632, 380),
    ([830, 460, 1210, 941], 1033, 380), ([1210, 460, 1672, 941], 1412, 380),
];

pub const RAYA_WALK: [Spec; 4] = [
    ([0, 0, 285, 318], 160, 294), ([285, 0, 590, 318], 450, 294),
    ([590, 0, 925, 318], 754, 294), ([925, 0, 1254, 318], 1070, 294),
];

// Four supported steps in the current dark-armor drawing finish.
pub const KOGAN_WALK: [Spec; 4] = [
    ([0, 0, 627, 600], 318, 505), ([627, 0, 1254, 600], 931, 505),
    ([0, 600, 627, 1254], 330, 505), ([627, 600, 1254, 1254], 930, 505),
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GroundState {
    #[default]
    Other,
    Stand,
    Crouch,
    Run,
}

impl GroundState {
    pub fn of(action: &Action) -> Self {
        match action {
            Action::Stand => Self::Stand,
            Action::Crouch => Self::Crouch,
            Action::Run => Self::Run,
            _ => Self::Other,
        }
    }
}

/// Client history only: how long this ground state has been shown and
/// the state it entered from. It never adds a simulation recovery window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GroundContext {
    pub state: GroundState,
    pub from: GroundState,
    pub age: u32,
}

pub fn ground_cell(f: &Fighter, context: GroundContext) -> Option<Cell> {
    let kogan = f.id == aeon_sim::CharacterId::Kogan;
    match f.action {
        Action::Run if context.age < 2 => Some(if kogan { Cell::Utility(4) } else { Cell::Ground(6) }),
        Action::Run => Some(Cell::Ground(((context.age - 2) / 8 % 2) as usize)),
        Action::Crouch => Some(Cell::Ground(if context.age < 2 { 2 } else { 3 })),
        Action::BackDash { frame } => Some(Cell::Ground(if frame < 3 { 4 }
            else if frame < 9 { 5 } else if frame < 12 { 6 } else { 7 })),
        Action::Stand if context.from == GroundState::Run && context.age < 4 => {
            let phase = if context.age < 2 { 6 } else { 7 };
            Some(if kogan { Cell::Utility(phase) } else { Cell::Ground(phase) })
        }
        Action::Stand if context.from == GroundState::Crouch && context.age < 2 => Some(Cell::Ground(2)),
        _ => None,
    }
}

// The old cut rows are not an equal grid. Their neighboring capes
// cross nominal boundaries; measured green gaps preserve each full drawing.
pub const KOGAN_CUTS: [Spec; 8] = [
    ([0, 320, 313, 615], 172, 273), ([313, 320, 630, 615], 457, 273),
    ([630, 320, 916, 615], 752, 273), ([916, 320, 1254, 615], 1049, 273),
    ([0, 615, 313, 940], 155, 273), ([313, 615, 606, 940], 445, 273),
    ([606, 615, 907, 940], 734, 273), ([907, 615, 1254, 940], 1051, 273),
];
// Larger redraws preserve the existing first-cut choreography and clock.
pub const KOGAN_FIRST_CUT: [Spec; 4] = [
    ([0, 0, 627, 620], 315, 390), ([627, 0, 1254, 620], 870, 390),
    ([0, 620, 627, 1254], 302, 390), ([627, 620, 1254, 1254], 895, 390),
];
// Front rising backcut; overhead blade height does not shrink the body.
pub const KOGAN_BACKCUT: [Spec; 4] = [
    ([0, 0, 627, 627], 286, 370), ([627, 0, 1254, 627], 866, 370),
    ([0, 627, 627, 1254], 286, 370), ([627, 627, 1254, 1254], 863, 370),
];
// Wide straight thrusts share anatomical scale; measured gaps preserve each tip.
pub const KOGAN_THRUST_STYLE: [Spec; 4] = [
    ([0, 0, 887, 420], 426, 328), ([887, 0, 1774, 420], 1169, 328),
    ([0, 420, 950, 887], 424, 328), ([950, 420, 1774, 887], 1225, 328),
];
pub const KOGAN_POKE: [Spec; 4] = [
    ([0, 0, 768, 500], 305, 400), ([768, 0, 1536, 500], 1100, 400),
    ([0, 500, 768, 1024], 309, 400), ([768, 500, 1536, 1024], 1050, 400),
];

pub fn poke_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Kogan { return None; }
    let Action::Attack { move_id: MoveId::StS, frame, .. } = f.action else { return None; };
    let mv = f.data().move_def(MoveId::StS)?;
    let phase = if frame < mv.first_active() { 0 }
        else if frame == mv.first_active() { 1 }
        else if frame <= mv.last_active() + u16::from(mv.recovery) / 3 { 2 }
        else { 3 };
    Some(Cell::Poke(phase))
}

// Shared standing anatomy keeps the kneeling head/body at the ready scale.
// Both kneeling blades remain above the measured boot/knee floor anchor.
pub const KOGAN_DISC: [Spec; 4] = [
    ([0, 0, 768, 470], 365, 435), ([768, 0, 1536, 470], 1095, 435),
    ([0, 470, 768, 1024], 375, 435), ([768, 470, 1536, 1024], 1095, 435),
];

/// The full disc exists only during the authored active brace. Dismissal
/// and rising settle fit the existing recovery; no guard frames are added.
pub fn disc_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Kogan { return None; }
    let Action::Attack { move_id: MoveId::Guard, frame, .. } = f.action else { return None; };
    let mv = f.data().move_def(MoveId::Guard)?;
    let phase = if frame < mv.first_active() { 0 }
        else if frame <= mv.last_active() { 1 }
        else if frame <= mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
        else { 3 };
    Some(Cell::Disc(phase))
}

pub const KOGAN_COIL: [Spec; 1] = [([0, 0, 1254, 1254], 705, 1030)];
pub const KOGAN_UPPERCUT: [Spec; 4] = [
    ([0, 0, 627, 535], 305, 355), ([627, 0, 1254, 535], 890, 350),
    ([0, 535, 627, 1254], 350, 315), ([627, 535, 1254, 1254], 913, 330),
];
pub const KOGAN_UPPERCUT_COMPACT: [Spec; 2] = [
    ([0, 0, 705, 1024], 450, 600), ([705, 0, 1536, 1024], 1110, 600),
];

/// Complete the upward blade line during the early rise, then gather the
/// body near the apex. The previous tall pose at maximum height hid the HUD.
pub fn compact_uppercut_cell(f: &Fighter) -> Option<Cell> {
    if f.id == aeon_sim::CharacterId::Raya {
        let mv = f.data().move_def(MoveId::Uppercut)?;
        return match f.action {
            Action::Attack { move_id: MoveId::Uppercut, frame, .. } if mv.is_active(frame) => Some(Cell::UppercutCompact(0)),
            Action::Attack { move_id: MoveId::Uppercut, frame, .. }
                if frame > mv.last_active() && f.airborne && (f.vel.y >= 0 || f.pos.y > aeon_sim::px(100)) => Some(Cell::UppercutCompact(1)),
            Action::Jump { air_ok: false, .. } if f.last_move == Some(MoveId::Uppercut)
                && f.airborne && f.pos.y > aeon_sim::px(100) => Some(Cell::UppercutCompact(1)),
            _ => None,
        };
    }
    if f.id != aeon_sim::CharacterId::Kogan { return None; }
    match f.action {
        Action::Attack { move_id: MoveId::Uppercut, frame, .. } => {
            let first = f.data().move_def(MoveId::Uppercut)?.first_active();
            if frame < first + 2 { None }
            else if frame < first + 6 { Some(Cell::UppercutCompact(0)) }
            else if f.airborne && (f.vel.y >= 0 || f.pos.y > aeon_sim::px(100)) {
                Some(Cell::UppercutCompact(1))
            } else { None }
        }
        Action::Jump { air_ok: false, .. } if f.last_move == Some(MoveId::Uppercut)
            && f.airborne && f.pos.y > aeon_sim::px(100) => Some(Cell::UppercutCompact(1)),
        _ => None,
    }
}

pub const RAYA_UPPERCUT_COMPACT: [Spec; 2] = [
    ([0, 0, 887, 887], 510, 640), ([887, 0, 1774, 887], 1240, 640),
];
pub const RAYA_UPPERCUT: [Spec; 4] = [
    ([0, 0, 627, 580], 346, 430), ([627, 0, 1254, 580], 866, 444),
    ([0, 580, 627, 1254], 334, 455), ([627, 580, 1254, 1254], 888, 432),
];

// Movement sheet: each row has individually measured green gaps. Root y
// for air cells follows the projected body rather than the lowest sword tip.
pub const KOGAN_MOVEMENT: [Spec; 8] = [
    ([0, 0, 410, 440], 245, 330), ([410, 0, 810, 440], 654, 330),
    ([810, 0, 1220, 440], 1047, 330), ([1220, 0, 1672, 440], 1430, 330),
    ([0, 445, 392, 941], 237, 330), ([392, 445, 790, 941], 653, 330),
    ([790, 445, 1200, 941], 1060, 330), ([1200, 445, 1672, 941], 1443, 330),
];
pub const KOGAN_MOVEMENT_ROOT_Y: [Option<u16>; 8] = [
    None, Some(425), Some(415), Some(423),
    Some(800), Some(810), Some(830), None,
];

// Raya keeps a compact hop and a longer full jump at common anatomical scale.
pub const RAYA_MOVEMENT: [Spec; 8] = [
    ([0, 0, 410, 420], 287, 400), ([410, 0, 820, 420], 662, 400),
    ([820, 0, 1210, 420], 1057, 400), ([1210, 0, 1672, 420], 1435, 400),
    ([0, 420, 410, 941], 269, 400), ([410, 420, 820, 941], 646, 400),
    ([820, 420, 1210, 941], 1050, 400), ([1210, 420, 1672, 941], 1437, 400),
];
pub const RAYA_MOVEMENT_ROOT_Y: [Option<u16>; 8] = [
    None, Some(425), Some(380), Some(430),
    Some(855), Some(850), Some(825), None,
];

pub fn movement_cell(f: &Fighter) -> Option<Cell> {
    match f.action {
        Action::Prejump { .. } => Some(Cell::Movement(0)),
        Action::Jump { air_ok: false, .. } if f.last_move == Some(MoveId::Uppercut) => None,
        Action::Jump { hop, .. } if f.airborne => {
            let phase = if f.vel.y > aeon_sim::px(2) { 0 }
                else if f.vel.y >= -aeon_sim::px(2) { 1 } else { 2 };
            Some(Cell::Movement(if hop { 1 } else { 4 } + phase))
        }
        // Only the first existing 2f full-jump landing tick compresses deeply;
        // its second tick uses the established rise-to-stance landing drawing.
        Action::Landing { frame: 0, total: 2 } => Some(Cell::Movement(7)),
        _ => None,
    }
}

// Cape reach and threshold-step share grounded anatomical scale, not a leg cycle.
pub const KOGAN_UTILITY: [Spec; 8] = [
    ([0, 0, 330, 550], 196, 400), ([330, 0, 745, 550], 542, 400),
    ([745, 0, 1090, 550], 935, 400), ([1090, 0, 1448, 550], 1274, 400),
    ([0, 550, 318, 1086], 190, 400), ([318, 550, 732, 1086], 545, 400),
    ([732, 550, 1075, 1086], 936, 400), ([1075, 550, 1448, 1086], 1274, 400),
];

// Complete Rite reach/withdrawal and prayer glide, measured at stable anatomy.
pub const RAYA_UTILITY: [Spec; 8] = [
    ([0, 0, 360, 490], 230, 408), ([360, 0, 775, 490], 560, 408),
    ([775, 0, 1130, 490], 960, 408), ([1130, 0, 1536, 490], 1320, 408),
    ([0, 490, 340, 1024], 220, 426), ([340, 490, 730, 1024], 540, 426),
    ([730, 490, 1125, 1024], 955, 426), ([1125, 490, 1536, 1024], 1310, 426),
];

pub fn utility_cell(f: &Fighter) -> Option<Cell> {
    let Action::Attack { move_id, frame, connected } = f.action else { return None; };
    let mv = f.data().move_def(move_id)?;
    let cell = match move_id {
        MoveId::CommandGrab | MoveId::Throw => {
            let hold = if move_id == MoveId::Throw { aeon_sim::fighter::THROW_TECH_WINDOW }
                else { aeon_sim::fighter::COMMAND_GRAB_HOLD };
            let release = if connected == aeon_sim::Connect::Hit {
                mv.first_active() + u16::from(hold)
            } else if move_id == MoveId::Throw { mv.last_active() } else { mv.last_active() + 1 };
            let ready = if move_id == MoveId::Throw {
                release + (mv.total_frames() - release) / 2
            } else { mv.last_active() + u16::from(mv.recovery) / 2 };
            if frame < mv.first_active() { 0 }
            else if frame < release { 1 }
            else if frame < ready { 2 }
            else { 3 }
        }
        MoveId::CommandDash => {
            let travel = u16::from(mv.vel_frames);
            if frame < travel / 4 { 4 }
            else if frame < travel * 2 / 3 { 5 }
            else if frame < travel { 6 }
            else { 7 }
        }
        _ => return None,
    };
    Some(if f.id == aeon_sim::CharacterId::Raya && move_id == MoveId::Throw && cell == 1 {
        Cell::ThrowContact
    } else { Cell::Utility(cell) })
}

// Grounded revolver and wave phases; green gutters measured per row.
pub const KOGAN_RANGED: [Spec; 8] = [
    ([0, 0, 350, 535], 211, 400), ([350, 0, 720, 535], 530, 400),
    ([720, 0, 1065, 535], 912, 400), ([1065, 0, 1448, 535], 1253, 400),
    ([0, 535, 342, 1086], 210, 400), ([342, 535, 715, 1086], 522, 400),
    ([715, 535, 1050, 1086], 875, 400), ([1050, 535, 1448, 1086], 1250, 400),
];

pub const RAYA_RANGED: [Spec; 8] = [
    ([0,0,390,490],246,395), ([390,0,765,490],610,395),
    ([765,0,1135,490],975,395), ([1135,0,1536,490],1290,395),
    ([0,490,390,1024],235,414), ([390,490,765,1024],610,414),
    ([765,490,1135,1024],970,414), ([1135,490,1536,1024],1290,414),
];

pub const RAYA_RITUAL: [Spec;8] = [
    ([0,0,375,470],230,415), ([375,0,738,470],565,415),
    ([738,0,1090,470],910,415), ([1090,0,1536,470],1310,415),
    ([0,470,395,1024],255,418), ([395,470,770,1024],585,418),
    ([770,470,1120,1024],945,418), ([1120,470,1536,1024],1295,418),
];

pub fn ritual_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Raya { return None; }
    let Action::Attack {move_id,frame,..}=f.action else {return None};
    match move_id {
        MoveId::Charge => {
            // The channel holds attack age10; its own sim counter supplies the
            // restrained breath. Release uses only the existing eight out ticks.
            let cell=match frame {
                0..=4=>0, 5..=9=>1,
                10=>if (f.channel_frames/12).is_multiple_of(2) {1} else {2},
                11..=13=>3, 14..=16=>4, _=>5,
            };
            Some(Cell::Ritual(cell))
        }
        MoveId::Detonate => Some(match frame {
            0..=5=>Cell::Ritual(6),6..=8=>Cell::Ritual(7),
            9..=13=>Cell::Utility(2),_=>Cell::Ritual(5),
        }),
        _=>None,
    }
}

pub fn ranged_cell(f: &Fighter) -> Option<Cell> {
    let Action::Attack { move_id, frame, .. } = f.action else { return None };
    if f.id == aeon_sim::CharacterId::Raya {
        let base = match move_id { MoveId::ShotA | MoveId::ExB => 0, MoveId::ShotB | MoveId::ExA => 4, _ => return None };
        let mv = f.data().move_def(move_id)?;
        // Release the drawn object exactly when the actual projectile appears.
        // The body withdraws while the hanging glyph or planted crystal persists.
        let phase = if frame < mv.first_active() { 0 }
            else if frame <= mv.last_active() + 2 { 1 }
            else if frame <= mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
            else { 3 };
        return Some(Cell::Ranged(base + phase));
    }
    let base = match move_id {
        MoveId::ShotA | MoveId::ExB => 0,
        MoveId::ShotB => 4,
        _ => return None,
    };
    let mv = f.data().move_def(move_id)?;
    // The gun finishes aiming just before discharge. The wave's cut is
    // aligned to release; its low drawing holds briefly before withdrawal.
    let commit = mv.first_active().saturating_sub(if base == 0 { 2 } else { 0 });
    let phase = if frame < commit { 0 }
        else if frame <= mv.last_active() + 2 { 1 }
        else if frame <= mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
        else { 3 };
    Some(Cell::Ranged(base + phase))
}

/// Selection reads authored move phases and actual vertical velocity. Render
/// frequency, hitstop and facing cannot advance the drawing.
pub fn cell_for(f: &Fighter) -> Option<Cell> {
    match f.action {
        Action::Attack { move_id: MoveId::Uppercut, frame, .. } => {
            let mv = f.data().move_def(MoveId::Uppercut)?;
            let cell = if frame < mv.first_active() { 0 }
                else if f.airborne && f.vel.y > aeon_sim::px(2) { 1 }
                else if f.airborne && f.vel.y >= 0 { 2 }
                else { 3 };
            Some(Cell::Uppercut(cell))
        }
        Action::Jump { air_ok: false, .. } if f.airborne && f.last_move == Some(MoveId::Uppercut) => {
            // The attack can finish before its ascent/descent does. Keep the
            // reversal drawing through the remaining committed fall.
            Some(Cell::Uppercut(3))
        }
        Action::Hit { .. } if f.airborne => Some(Cell::Reaction(if f.vel.y > 0 { 2 } else { 3 })),
        Action::Hit { .. } => Some(Cell::Reaction(usize::from(f.input().down()))),
        Action::Thrown { .. } => Some(Cell::Reaction(0)),
        Action::Knockdown { .. } => Some(Cell::Reaction(4)),
        Action::Getup { frame } => Some(Cell::Reaction(4 + (frame * 4 / GETUP_FRAMES).min(3) as usize)),
        Action::Landing { frame, total } => Some(Cell::Reaction(8 + (frame * 4 / total.max(1)).min(3) as usize)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::{px, CharacterId, Connect, World};

    #[test]
    fn recoil_release_uses_four_remaining_frames_and_yields_to_legal_control() {
        for (id, facing) in [CharacterId::Kogan, CharacterId::Raya].into_iter()
            .flat_map(|id| [false, true].map(|facing| (id, facing))) {
            let mut w = World::new(id, CharacterId::Raya);
            w.fighters[0].facing_right = facing;
            w.fighters[0].apply_hit(12, false, 0, 0, false);
            let mut release = 0;
            for _ in 0..20 {
                release += usize::from(recoil_cell(&w.fighters[0]) == Some(Cell::Recoil(1)));
                w.tick(Default::default(), Default::default());
            }
            assert_eq!(release, 4, "stun zero is itself one visible tick");
            assert_eq!(recoil_cell(&w.fighters[0]), None);
            w.fighters[0].start_move(MoveId::StP);
            assert_eq!(recoil_cell(&w.fighters[0]), None, "no art recovery masks an attack");
            w.fighters[0].action = Action::Hit { stun: 2, knockdown: true };
            assert_eq!(recoil_cell(&w.fighters[0]), Some(Cell::Recoil(0)), "a pending knockdown never regains control");
            w.fighters[0].airborne = true;
            assert_eq!(recoil_cell(&w.fighters[0]), None, "air recoil keeps the velocity-driven sequence");
        }
    }

    #[test]
    fn authored_regions_preserve_complete_silhouettes_and_effects() {
        for (name, reference, specs) in [
            ("kogan-air-shot-v3-green.png", (1254, 1254), &KOGAN_AIR_SHOT[..3]),
            ("kogan-air-shot-v1-green.png", (1254, 1254), &KOGAN_AIR_SHOT[3..]),
            ("kogan-judgment-v3-green.png", (1536, 1024), &KOGAN_JUDGMENT[..]),
            ("kogan-floor-v1-green.png", (1536, 1024), &KOGAN_FLOOR[..]),
            ("kogan-recoil-v2-green.png", (1024, 1536), &KOGAN_RECOIL[..]),
            ("kogan-ground-v4-green.png", (1536, 1024), &KOGAN_GROUND[..]),
            ("kogan-walk-style-v1-green.png", (1254, 1254), &KOGAN_WALK[..]),
            ("raya-v1-green.png", (1254, 1254), &RAYA_WALK[..]),
            ("raya-ground-v1-green.png", (1672, 941), &RAYA_GROUND[..]),
            ("raya-recoil-v1-green.png", (941, 1672), &RAYA_RECOIL[..]),
            ("raya-standing-lights-v1-green.png", (1024, 1536), &RAYA_STANDING_LIGHTS[..]),
            ("raya-standing-lights-v2-green.png", (1024, 1536), &RAYA_STANDING_LIGHTS[1..2]),
            ("raya-crouch-lights-v1-green.png", (1024, 1536), &RAYA_CROUCH_LIGHTS[..]),
            ("raya-crouch-crystals-v1-green.png", (1024, 1536), &RAYA_CROUCH_CRYSTALS[..]),
            ("raya-anti-crystal-v1-green.png", (1024, 1536), &RAYA_ANTI_CRYSTAL[..]),
            ("raya-crouch-low-v1-green.png", (1024, 1536), &RAYA_CROUCH_LOW[..]),
            ("raya-crouch-lights-v2-green.png", (1024, 1536), &RAYA_CROUCH_LIGHTS[5..6]),
            ("raya-flash-style-v1-green.png", (1024, 1536), &RAYA_FLASH[..]),
            ("raya-signature-v1-green.png", (1254, 1254), &RAYA_SIGNATURE[..]),
            ("raya-signature-v4-green.png", (1774, 887), &RAYA_SIGNATURE_CONTACTS[..]),
            ("raya-chant2-v2-green.png", (1254, 1254), &RAYA_CHANT_II[..]),
            ("raya-chant3-v1-green.png", (1254, 1254), &RAYA_CHANT_III[..]),
            ("raya-flash-style-v3-green.png", (1024, 1536), &RAYA_FLASH[5..6]),
            ("kogan-throw-tech-v1-green.png", (1536, 1024), &KOGAN_THROW_TECH[..]),
            ("kogan-victory-v1-green.png", (1536, 1024), &KOGAN_VICTORY[..]),
            ("raya-victory-v1-green.png", (1536, 1024), &RAYA_VICTORY[..]),
            ("kogan-overhead-v1-green.png", (1254, 1254), &KOGAN_OVERHEAD[..]),
            ("raya-overhead-v1-green.png", (1024, 1536), &RAYA_OVERHEAD[..]),
            ("kogan-crouch-low-v3-green.png", (1024, 1536), &KOGAN_CROUCH_LOW[..]),
            ("kogan-air-recovery-v1-green.png", (1254, 1254), &KOGAN_AIR_RECOVERY[..]),
            ("raya-air-recovery-v1-green.png", (1254, 1254), &RAYA_AIR_RECOVERY[..]),
            ("kogan-crouch-punch-v1-green.png", (1254, 1254), &KOGAN_CROUCH_PUNCH[..]),
            ("kogan-crouch-saber-v1-green.png", (1024, 1536), &KOGAN_CROUCH_SABER[..]),
            ("kogan-flash-v2-green.png", (1024, 1536), &KOGAN_FLASH[..]),
            ("kogan-air-lights-v1-green.png", (1024, 1536), &KOGAN_AIR_LIGHTS[..]),
            ("kogan-air-lights-v4-green.png", (1024, 1536), &KOGAN_AIR_LIGHTS_CONTACT[..]),
            ("kogan-air-saber-v2-green.png", (1024, 1536), &KOGAN_AIR_SABER[..]),
            ("kogan-disc-v2-green.png", (1536, 1024), &KOGAN_DISC[..]),
            ("kogan-standing-poke-v1-green.png", (1536, 1024), &KOGAN_POKE[..]),
            ("kogan-v1-green.png", (1254, 1254), &KOGAN_CUTS[..]),
            ("kogan-first-cut-style-v1-green.png", (1254, 1254), &KOGAN_FIRST_CUT[..]),
            ("kogan-backcut-style-v1-green.png", (1254, 1254), &KOGAN_BACKCUT[..]),
            ("kogan-thrust-style-v2-green.png", (1774, 887), &KOGAN_THRUST_STYLE[..]),
            ("kogan-uppercut-compact-v1-green.png", (1536, 1024), &KOGAN_UPPERCUT_COMPACT[..]),
            ("kogan-cape-step-v3-green.png", (1448, 1086), &KOGAN_UTILITY[..]),
            ("raya-utility-v1-green.png", (1536, 1024), &RAYA_UTILITY[..]),
            ("raya-throw-contact-v1-green.png", (1254, 1254), &RAYA_THROW_CONTACT[..]),
            ("kogan-ranged-v5-green.png", (1448, 1086), &KOGAN_RANGED[..]),
            ("raya-ranged-v2-green.png", (1536, 1024), &RAYA_RANGED[..]),
            ("raya-ritual-v1-green.png", (1536,1024), &RAYA_RITUAL[..]),
            ("raya-convergence-v1-green.png", (1536,1024), &RAYA_CONVERGENCE[..]),
            ("raya-movement-v1-green.png", (1672, 941), &RAYA_MOVEMENT[..]),
            ("raya-air-lights-v1-green.png", (1024, 1536), &RAYA_AIR_LIGHTS[..]),
            ("raya-air-crystals-v1-green.png", (1024, 1536), &RAYA_AIR_CRYSTALS[..]),
            ("kogan-movement-v2-green.png", (1672, 941), &KOGAN_MOVEMENT[..]),
            ("kogan-reactions-v1-green.png", (1448, 1086), &KOGAN_REACTIONS[..]),
            ("raya-reactions-v1-green.png", (1448, 1086), &RAYA_REACTIONS[..]),
            ("kogan-uppercut-v1-green.png", (1254, 1254), &KOGAN_UPPERCUT[..]),
            ("raya-uppercut-v1-green.png", (1254, 1254), &RAYA_UPPERCUT[..]),
            ("raya-uppercut-compact-v2-green.png", (1774, 887), &RAYA_UPPERCUT_COMPACT[..]),
            ("kogan-uppercut-coil-v1-green.png", (1254, 1254), &KOGAN_COIL[..]),
        ] {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/animation").join(name);
            let bytes = std::fs::read(path).unwrap();
            let mut image = Image::from_file_with_format(&bytes, None).unwrap();
            key_green(&mut image);
            let w = image.width as usize;
            let h = image.height as usize;
            for (i, &(r, root, body_h)) in specs.iter().enumerate() {
                assert!(root > r[0] && root < r[2] && body_h > 0);
                let [x0, y0, x1, y1] = [r[0] as usize * w / reference.0, r[1] as usize * h / reference.1,
                    r[2] as usize * w / reference.0, r[3] as usize * h / reference.1];
                // No opaque drawing may touch the extraction boundary. This
                // catches grid slicing through a foot, cape or raised blade.
                for y in y0..y1 {
                    for x in x0..x1 {
                        if x < x0 + 3 || x >= x1 - 3 || y < y0 + 3 || y >= y1 - 3 {
                            assert!(image.bytes[(y * w + x) * 4 + 3] < 24,
                                "{name} cell {i} cuts drawing at {x},{y}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn airborne_saber_return_survives_attack_expiry_but_yields_to_every_new_state() {
        for (body, right) in [CharacterId::Kogan, CharacterId::Raya].into_iter()
            .flat_map(|body| [false, true].map(|right| (body, right))) {
            for move_id in [MoveId::JS, MoveId::JHS, MoveId::JST] {
                let mut f = Fighter::spawn(body, px(200), right);
                f.airborne = true;
                f.last_move = Some(move_id);
                f.action = Action::Jump { air_ok: false, hop: false };
                assert_eq!(air_saber_cell(&f), Some(Cell::AirSaber(5)));
                for action in [Action::Jump { air_ok: true, hop: false }, Action::Stand,
                    Action::Landing { frame: 0, total: 2 }, Action::Hit { stun: 8, knockdown: false }] {
                    f.action = action;
                    assert_eq!(air_saber_cell(&f), None);
                }
                f.action = Action::Jump { air_ok: false, hop: false };
                f.airborne = false;
                assert_eq!(air_saber_cell(&f), None);
            }
        }
    }

    #[test]
    fn feint_art_requires_a_canceled_special_and_yields_to_every_new_state() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            let mut f = Fighter::spawn(id, px(200), true);
            for mv in id.data().moves.iter() {
                f.last_move = Some(mv.id);
                for airborne in [false, true] {
                    f.airborne = airborne;
                    for frame in 0..aeon_sim::fighter::FEINT_RECOVERY {
                        f.action = Action::Feint { frame };
                        let cell = feint_cell(&f);
                        assert_eq!(cell.is_some(), mv.feintable);
                        if mv.feintable && airborne {
                            assert!(matches!(cell, Some(Cell::AirSaber(4 | 5))));
                        }
                    }
                    for action in [Action::Stand, Action::Crouch, Action::Jump { air_ok: true, hop: false },
                        Action::Landing { frame: 0, total: 1 }, Action::Hit { stun: 8, knockdown: false },
                        Action::Block { stun: 8, crouching: false }, Action::ThrowTech { frame: 0 }] {
                        f.action = action;
                        assert_eq!(feint_cell(&f), None, "later states own their drawings");
                    }
                }
            }
        }
    }

    #[test]
    fn throw_tech_uses_three_supported_phases_and_yields_to_every_new_action() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            let mut f = Fighter::spawn(id, px(200), true);
            f.last_move = Some(MoveId::Throw);
            for frame in 0..16 {
                f.action = Action::ThrowTech { frame };
                let expected = Some(if frame <= 5 {
                    if id == CharacterId::Raya { Cell::Recoil(4) } else { Cell::ThrowTech(0) }
                } else if frame <= 10 {
                    if id == CharacterId::Raya { Cell::Recoil(5) } else { Cell::ThrowTech(1) }
                } else { Cell::Utility(3) });
                assert_eq!(throw_tech_cell(&f), expected);
                assert_eq!(utility_cell(&f), None, "a tech cannot continue the grab");
                f.airborne = true;
                assert_eq!(throw_tech_cell(&f), None);
                f.airborne = false;
            }
            for action in [Action::Stand, Action::Crouch, Action::Feint { frame: 0 },
                Action::Block { stun: 8, crouching: false }, Action::Hit { stun: 8, knockdown: false },
                Action::Jump { air_ok: true, hop: false }, Action::Landing { frame: 0, total: 2 }] {
                f.action = action;
                assert_eq!(throw_tech_cell(&f), None);
                assert_eq!(utility_cell(&f), None);
            }
        }
    }

    #[test]
    fn overhead_art_yields_to_landing_and_never_leaks_into_a_later_jump() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            let mut f = Fighter::spawn(id, px(200), true);
            f.last_move = Some(MoveId::SpecialOverhead);
            for action in [Action::Jump { air_ok: false, hop: false },
                Action::Jump { air_ok: true, hop: false }, Action::Stand,
                Action::Landing { frame: 0, total: 8 }, Action::Hit { stun: 8, knockdown: false }] {
                f.action = action;
                for airborne in [false, true] {
                    f.airborne = airborne;
                    assert_eq!(air_saber_cell(&f), None);
                    assert_eq!(overhead_cell(&f), None);
                }
            }
        }
    }

    #[test]
    fn crouching_saber_commitment_is_active_only_and_every_new_action_owns_its_drawing() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            for facing in [false, true] {
                for (move_id, base) in [(MoveId::CrS, 0), (MoveId::CrHS, 4), (MoveId::CrFL, 8), (MoveId::CrST, 12)] {
                    let mut f = Fighter::spawn(id, px(200), facing);
                    let mv = f.data().move_def(move_id).unwrap();
                    for frame in 0..mv.total_frames() {
                        f.action = Action::Attack { move_id, frame, connected: Connect::None };
                        let cell = crouch_saber_cell(&f);
                        assert_eq!(cell == Some(Cell::CrouchSaber(base + 1)), mv.is_active(frame));
                        if frame == 0 { assert_eq!(cell, Some(Cell::CrouchSaber(base))); }
                        if frame == mv.last_active() + 1 { assert_eq!(cell, Some(Cell::CrouchSaber(base + 2))); }
                        if frame == mv.total_frames() - 1 { assert_eq!(cell, Some(Cell::CrouchSaber(base + 3))); }
                    }
                    f.last_move = Some(move_id);
                    for action in [Action::Stand, Action::Crouch, Action::Hit { stun: 8, knockdown: false },
                        Action::Jump { air_ok: false, hop: true }] {
                        f.action = action;
                        assert_eq!(crouch_saber_cell(&f), None);
                    }
                }
            }
        }
    }

    #[test]
    fn signature_contact_follows_existing_active_frames_and_yields_to_new_states() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            for facing in [false, true] {
                for (move_id, column) in [(MoveId::StS, 0), (MoveId::StHS, 1), (MoveId::StHSClose, 2)] {
                    let mut f = Fighter::spawn(id, px(200), facing);
                    let mv = f.data().move_def(move_id).unwrap();
                    for frame in 0..mv.total_frames() {
                        f.action = Action::Attack { move_id, frame, connected: Connect::None };
                        let cell = signature_cell(&f);
                        if id == CharacterId::Kogan { assert_eq!(cell, None); continue; }
                        assert_eq!(cell == Some(Cell::Signature(3 + column)), mv.is_active(frame));
                        if frame == 0 { assert_eq!(cell, Some(Cell::Signature(column))); }
                        if frame == mv.last_active() + 1 { assert_eq!(cell, Some(Cell::Signature(6 + column))); }
                        if frame == mv.total_frames() - 1 { assert_eq!(cell, Some(Cell::Signature(9 + column))); }
                    }
                    for action in [Action::Stand, Action::Crouch, Action::Hit { stun: 8, knockdown: false },
                        Action::Jump { air_ok: false, hop: true }] {
                        f.action = action;
                        assert_eq!(signature_cell(&f), None);
                    }
                    f.start_move(move_id); f.airborne = true;
                    assert_eq!(signature_cell(&f), None);
                }
            }
        }
    }

    #[test]
    fn chant_contact_obeys_active_frames_and_each_followup_takes_over() {
        for right in [false, true] {
            let mut f = Fighter::spawn(CharacterId::Raya, px(200), right);
            for (move_id, contact) in [(MoveId::Rekka1, Cell::Signature(3)),
                (MoveId::Rekka2, Cell::Chant(1)), (MoveId::Rekka3, Cell::Chant(5))] {
                let mv = f.data().move_def(move_id).unwrap();
                for frame in 0..mv.total_frames() {
                    f.action = Action::Attack { move_id, frame, connected: Connect::None };
                    assert_eq!(chant_cell(&f) == Some(contact), mv.is_active(frame));
                }
            }
            for action in [Action::Stand, Action::Crouch, Action::Feint { frame: 0 },
                Action::Hit { stun: 8, knockdown: false }] {
                f.action = action; assert_eq!(chant_cell(&f), None);
            }
            f.start_move(MoveId::Rekka1); f.airborne = true;
            assert_eq!(chant_cell(&f), None);
            let mut kogan = Fighter::spawn(CharacterId::Kogan, px(200), right);
            kogan.start_move(MoveId::Rekka1); assert_eq!(chant_cell(&kogan), None);
        }
    }

    #[test]
    fn flash_commitment_is_active_only_and_every_new_action_owns_its_drawing() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            for facing in [false, true] {
                for (move_id, base) in [(MoveId::StFL, 0), (MoveId::StST, 4)] {
                    let mut f = Fighter::spawn(id, px(200), facing);
                    let mv = f.data().move_def(move_id).unwrap();
                    for frame in 0..mv.total_frames() {
                        f.action = Action::Attack { move_id, frame, connected: Connect::None };
                        let cell = flash_cell(&f);
                        assert_eq!(cell == Some(Cell::Flash(base + 1)), mv.is_active(frame));
                        if frame == 0 { assert_eq!(cell, Some(Cell::Flash(base))); }
                        if frame == mv.last_active() + 1 { assert_eq!(cell, Some(Cell::Flash(base + 2))); }
                        if frame == mv.total_frames() - 1 { assert_eq!(cell, Some(Cell::Flash(base + 3))); }
                    }
                    f.last_move = Some(move_id);
                    for action in [Action::Stand, Action::Crouch, Action::Hit { stun: 8, knockdown: false },
                        Action::Jump { air_ok: false, hop: true }] {
                        f.action = action;
                        assert_eq!(flash_cell(&f), None);
                    }
                }
            }
        }
    }

    #[test]
    fn airborne_light_return_survives_attack_expiry_but_yields_to_every_new_state() {
        for (body, right) in [CharacterId::Kogan, CharacterId::Raya].into_iter()
            .flat_map(|body| [false, true].map(|right| (body, right))) {
            for move_id in [MoveId::JP, MoveId::JK, MoveId::JFL] {
                let mut f = Fighter::spawn(body, px(200), right);
                f.airborne = true;
                f.last_move = Some(move_id);
                f.action = Action::Jump { air_ok: false, hop: false };
                assert_eq!(air_lights_cell(&f), Some(Cell::AirLights(5)));
                for action in [Action::Jump { air_ok: true, hop: false }, Action::Stand,
                    Action::Landing { frame: 0, total: 2 }, Action::Hit { stun: 8, knockdown: false }] {
                    f.action = action;
                    assert_eq!(air_lights_cell(&f), None);
                }
                f.action = Action::Jump { air_ok: false, hop: false };
                f.airborne = false;
                assert_eq!(air_lights_cell(&f), None);
            }
        }
    }

    #[test]
    fn air_gun_drawings_release_once_and_yield_immediately_to_landing() {
        for facing in [false, true] {
            let mut f = Fighter::spawn(CharacterId::Kogan, px(200), facing);
            f.airborne = true;
            let mv = f.data().move_def(MoveId::AirShot).unwrap();
            let first = mv.first_active();
            let total = mv.total_frames();
            for frame in 0..total {
                f.action = Action::Attack { move_id: MoveId::AirShot, frame, connected: Connect::None };
                let expected = if frame < 4 { 0 } else if frame <= first { 1 }
                    else if frame < first + 3 { 2 } else { 3 };
                assert_eq!(air_shot_cell(&f), Some(Cell::AirShot(expected)));
            }
            for action in [Action::Stand, Action::Landing { frame: 0, total: 2 },
                Action::Hit { stun: 8, knockdown: false }] {
                f.action = action;
                assert_eq!(air_shot_cell(&f), None, "a new legal state owns its drawing");
            }
        }
        let mut raya = Fighter::spawn(CharacterId::Raya, px(200), true);
        raya.airborne = true;
        raya.action = Action::Attack { move_id: MoveId::AirShot, frame: 8, connected: Connect::None };
        assert_eq!(air_shot_cell(&raya), None);
    }

    #[test]
    fn reversal_art_follows_startup_rise_apex_and_descent_for_both_facings() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            for facing in [false, true] {
                let mut f = Fighter::spawn(id, px(200), facing);
                f.start_move(MoveId::Uppercut);
                let first = f.data().move_def(MoveId::Uppercut).unwrap().first_active();
                for (frame, vy, airborne, expected) in [(0, 0, false, 0), (first, px(8), true, 1),
                    (first + 5, px(1), true, 2), (first + 7, -px(3), true, 3)] {
                    f.action = Action::Attack { move_id: MoveId::Uppercut, frame, connected: Connect::None };
                    f.airborne = airborne;
                    f.vel.y = vy;
                    assert_eq!(cell_for(&f), Some(Cell::Uppercut(expected)), "{id:?} {facing}");
                }
            }
        }
    }

    #[test]
    fn a_throw_reaction_reaches_the_floor_and_all_getup_drawings_without_changing_the_sim() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            let mut w = World::new(id, id);
            w.fighters[0].apply_hit(20, true, px(7), px(24), false);
            let mut seen = [false; 8];
            for _ in 0..110 {
                if let Some(Cell::Reaction(c)) = cell_for(&w.fighters[0]) { seen[c] = true; }
                let hash = w.state_hash();
                for _ in 0..4 { let _ = cell_for(&w.fighters[0]); }
                assert_eq!(hash, w.state_hash());
                w.tick(Default::default(), Default::default());
            }
            assert!(seen[2..8].iter().all(|v| *v), "{id:?}: {seen:?}");
        }
    }

    #[test]
    fn raya_ascension_releases_only_while_active_and_gathers_above_the_floor() {
        for facing in [false, true] {
            let mut f = Fighter::spawn(CharacterId::Raya, px(200), facing);
            let mv = f.data().move_def(MoveId::Uppercut).unwrap();
            f.airborne = true; f.pos.y = px(110); f.vel.y = px(1);
            for frame in 0..mv.total_frames() {
                f.action = Action::Attack { move_id: MoveId::Uppercut, frame, connected: Connect::None };
                let cell = compact_uppercut_cell(&f);
                assert_eq!(cell == Some(Cell::UppercutCompact(0)), mv.is_active(frame));
                if frame > mv.last_active() { assert_eq!(cell, Some(Cell::UppercutCompact(1))); }
            }
            f.vel.y = -px(1); f.pos.y = px(99);
            assert_eq!(compact_uppercut_cell(&f), None);
            for action in [Action::Stand, Action::Crouch, Action::Landing { frame: 0, total: 12 },
                Action::Feint { frame: 0 }, Action::Hit { stun: 8, knockdown: false }] {
                f.action = action; assert_eq!(compact_uppercut_cell(&f), None);
            }
        }
    }

    #[test]
    fn an_uppercut_keeps_its_descent_drawing_after_attack_expiry() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            let mut w = World::new(id, id);
            w.fighters[0].start_move(MoveId::Uppercut);
            let mut saw_landing = false;
            for _ in 0..120 {
                let f = &w.fighters[0];
                if f.airborne && matches!(f.action, Action::Jump { air_ok: false, .. }) {
                    assert_eq!(cell_for(f), Some(Cell::Uppercut(3)));
                }
                if matches!(f.action, Action::Landing { .. }) {
                    saw_landing = true;
                    assert!(matches!(cell_for(f), Some(Cell::Reaction(8..=11))));
                }
                w.tick(Default::default(), Default::default());
            }
            assert!(saw_landing);
        }
    }

    #[test]
    fn landing_drawings_do_not_delay_a_two_frame_jump_landing_or_a_safe_hop() {
        let mut f = Fighter::spawn(CharacterId::Kogan, px(200), true);
        f.action = Action::Landing { frame: 0, total: 2 };
        assert_eq!(cell_for(&f), Some(Cell::Reaction(8)));
        f.action = Action::Landing { frame: 1, total: 2 };
        assert_eq!(cell_for(&f), Some(Cell::Reaction(10)));
        f.action = Action::Stand;
        assert_eq!(cell_for(&f), None);
        f.start_move(MoveId::StP);
        assert_eq!(cell_for(&f), None, "a landing never masks an immediate attack");
    }
}
