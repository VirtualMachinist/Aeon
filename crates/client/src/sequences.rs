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
pub const KOGAN_COIL: [Spec; 1] = [([0, 0, 1254, 1254], 705, 1030)];
pub const KOGAN_UPPERCUT: [Spec; 4] = [
    ([0, 0, 627, 535], 305, 355), ([627, 0, 1254, 535], 890, 350),
    ([0, 535, 627, 1254], 350, 315), ([627, 535, 1254, 1254], 913, 330),
];
pub const RAYA_UPPERCUT: [Spec; 4] = [
    ([0, 0, 627, 580], 346, 430), ([627, 0, 1254, 580], 866, 444),
    ([0, 580, 627, 1254], 334, 455), ([627, 580, 1254, 1254], 888, 432),
];

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
