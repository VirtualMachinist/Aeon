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

// Walk uses the existing four drawings, extracted at real green gaps.
pub const KOGAN_WALK: [Spec; 4] = [
    ([0, 0, 313, 320], 175, 277), ([313, 0, 614, 320], 470, 277),
    ([614, 0, 906, 320], 756, 277), ([906, 0, 1254, 320], 1055, 277),
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
    if f.id != aeon_sim::CharacterId::Kogan { return None; }
    match f.action {
        Action::Run if context.age < 2 => Some(Cell::Utility(4)),
        Action::Run => Some(Cell::Ground(((context.age - 2) / 8 % 2) as usize)),
        Action::Crouch => Some(Cell::Ground(if context.age < 2 { 2 } else { 3 })),
        Action::BackDash { frame } => Some(Cell::Ground(if frame < 3 { 4 }
            else if frame < 9 { 5 } else if frame < 12 { 6 } else { 7 })),
        Action::Stand if context.from == GroundState::Run && context.age < 4 => {
            Some(Cell::Utility(if context.age < 2 { 6 } else { 7 }))
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

pub fn movement_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Kogan {
        return None;
    }
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

pub fn utility_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Kogan { return None; }
    let Action::Attack { move_id, frame, connected } = f.action else { return None; };
    let mv = f.data().move_def(move_id)?;
    let cell = match move_id {
        MoveId::CommandGrab => {
            let release = if connected == aeon_sim::Connect::Hit {
                mv.first_active() + u16::from(aeon_sim::fighter::COMMAND_GRAB_HOLD)
            } else { mv.last_active() + 1 };
            if frame < mv.first_active() { 0 }
            else if frame < release { 1 }
            else if frame < mv.last_active() + u16::from(mv.recovery) / 2 { 2 }
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
    Some(Cell::Utility(cell))
}

// Grounded revolver and wave phases; green gutters measured per row.
pub const KOGAN_RANGED: [Spec; 8] = [
    ([0, 0, 350, 535], 211, 400), ([350, 0, 720, 535], 530, 400),
    ([720, 0, 1065, 535], 912, 400), ([1065, 0, 1448, 535], 1253, 400),
    ([0, 535, 342, 1086], 210, 400), ([342, 535, 715, 1086], 522, 400),
    ([715, 535, 1050, 1086], 875, 400), ([1050, 535, 1448, 1086], 1250, 400),
];

pub fn ranged_cell(f: &Fighter) -> Option<Cell> {
    if f.id != aeon_sim::CharacterId::Kogan {
        return None;
    }
    let Action::Attack { move_id, frame, .. } = f.action else { return None };
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
    fn authored_regions_preserve_complete_silhouettes_and_effects() {
        for (name, reference, specs) in [
            ("kogan-ground-v4-green.png", (1536, 1024), &KOGAN_GROUND[..]),
            ("kogan-v1-green.png", (1254, 1254), &KOGAN_WALK[..]),
            ("kogan-disc-v2-green.png", (1536, 1024), &KOGAN_DISC[..]),
            ("kogan-standing-poke-v1-green.png", (1536, 1024), &KOGAN_POKE[..]),
            ("kogan-v1-green.png", (1254, 1254), &KOGAN_CUTS[..]),
            ("kogan-uppercut-compact-v1-green.png", (1536, 1024), &KOGAN_UPPERCUT_COMPACT[..]),
            ("kogan-cape-step-v3-green.png", (1448, 1086), &KOGAN_UTILITY[..]),
            ("kogan-ranged-v5-green.png", (1448, 1086), &KOGAN_RANGED[..]),
            ("kogan-movement-v2-green.png", (1672, 941), &KOGAN_MOVEMENT[..]),
            ("kogan-reactions-v1-green.png", (1448, 1086), &KOGAN_REACTIONS[..]),
            ("raya-reactions-v1-green.png", (1448, 1086), &RAYA_REACTIONS[..]),
            ("kogan-uppercut-v1-green.png", (1254, 1254), &KOGAN_UPPERCUT[..]),
            ("raya-uppercut-v1-green.png", (1254, 1254), &RAYA_UPPERCUT[..]),
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
