//! Motion on top of pose-hold sprites.
//!
//! Every state in both kits gets anticipation, contact and recovery motion:
//! leans, stretch and squash, launch tumbles, the knockdown bounce, run and
//! processional afterimages, and a short crossfade whenever the picture
//! changes. All of it is a pure function of simulation state plus a few
//! frames of what was drawn before, so hitstop, pause, frame-step and
//! replays hold exactly. Geometry stays in the sim; nothing here is a box.

use std::collections::VecDeque;
use std::f32::consts::PI;

use aeon_sim::{Action, CharacterId, Fighter, MoveDef, MoveId, World, SUB};
use macroquad::prelude::*;

use crate::sprites::{Cell, Pose, SpriteSet};

/// Frames of drawn history kept per body (afterimages look back this far).
const TRAIL: usize = 12;
/// A changed picture fades the previous one out over this many frames.
const CROSSFADE: u32 = 2;

pub const GHOST_KOGAN: Color = Color::new(0.88, 0.52, 0.22, 1.0);
pub const GHOST_RAYA: Color = Color::new(0.74, 0.96, 1.0, 1.0);
pub const HURT_TINT: Color = Color::new(1.0, 0.85, 0.80, 1.0);

fn sub(v: i32) -> f32 {
    v as f32 / SUB as f32
}

fn ease_in(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t
}

fn ease_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t) * (1.0 - t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

#[derive(Clone, Copy, Debug)]
pub struct Snapshot {
    pub frame: u32,
    pub x: f32,
    pub y: f32,
    pub facing_right: bool,
    pub cell: Cell,
    ground: crate::sequences::GroundContext,
}

/// Winner drawing time starts only after the body has recovered. It advances
/// with the world, freezes at MatchOver and clears at the next round/rematch.
#[derive(Default)]
pub struct VictoryClock {
    winner: Option<usize>,
    last_frame: u32,
    age: u32,
}

impl VictoryClock {
    pub fn update(&mut self, w: &World, phase: aeon_sim::Phase) {
        use aeon_sim::{Phase, RoundOutcome};
        let winner = match phase {
            Phase::RoundEnd { outcome: RoundOutcome::Winner(i), frame: 30.. } => Some(i),
            Phase::MatchOver { winner } => Some(winner),
            _ => None,
        }.filter(|&i| victory_at_rest(&w.fighters[i]));
        let Some(winner) = winner else { *self = Self::default(); return; };
        if self.winner == Some(winner) && w.frame >= self.last_frame {
            self.age = self.age.saturating_add(w.frame - self.last_frame);
        } else {
            // A direct loaded end screen has no preceding gesture to play.
            self.age = if matches!(phase, Phase::MatchOver { .. }) { 70 } else { 0 };
        }
        self.winner = Some(winner);
        self.last_frame = w.frame;
    }

    pub fn age(&self, i: usize) -> Option<u32> { (self.winner == Some(i)).then_some(self.age) }
    pub fn winner(&self) -> Option<usize> { self.winner }
}

pub fn victory_at_rest(f: &Fighter) -> bool {
    !f.airborne && matches!(f.action, Action::Stand | Action::Crouch | Action::Walk { .. })
}

/// What each body showed over the last few simulation frames.
#[derive(Default)]
pub struct History {
    trail: [VecDeque<Snapshot>; 2],
}

impl History {

    fn victory_ready(&self, w: &World, i: usize) -> bool {
        let f = &w.fighters[i];
        if !victory_at_rest(f) { return false; }
        let c = self.ground_context(w,i);
        !(f.id == CharacterId::Kogan && matches!(f.action,Action::Stand)
            && c.from == crate::sequences::GroundState::Crouch && c.age < 2)
    }


    /// Keep the attacking gesture in front. At equal grounded height, a
    /// crouching body must remain visible below the opponent's standing torso.
    /// Its two-frame rise keeps that order until the standing drawing returns.
    pub fn draw_order(&self, w: &World) -> [usize; 2] {
        // Judgment's low arm and the legacy low kick's cape overlap the receiver.
        // Keep the grounded low consequence visible through that cloth.
        for attacker in [0, 1] {
            let defender = 1 - attacker;
            let a = &w.fighters[attacker];
            let d = &w.fighters[defender];
            if a.id == CharacterId::Kogan && !a.airborne && !d.airborne
                && matches!(a.action, Action::Attack { move_id: MoveId::Super | MoveId::CrK, .. })
                && (matches!(d.action, Action::Crouch | Action::Block { crouching: true, .. })
                    || (matches!(d.action, Action::Hit { .. }) && d.input().down())) {
                return [attacker, defender];
            }
        }
        let attacking = w.fighters.each_ref().map(|f| f.action.attacking().is_some());
        match attacking {
            [true, false] => return [1, 0],
            [false, true] => return [0, 1],
            _ => {}
        }
        if w.fighters[0].pos.y != w.fighters[1].pos.y {
            return if w.fighters[0].pos.y > w.fighters[1].pos.y { [1, 0] } else { [0, 1] };
        }
        let low = [0, 1].map(|i| {
            let f = &w.fighters[i];
            let c = self.ground_context(w, i);
            matches!(f.action, Action::Crouch)
                || (c.state == crate::sequences::GroundState::Stand
                    && c.from == crate::sequences::GroundState::Crouch && c.age < 2)
        });
        if low == [true, false] { [1, 0] } else { [0, 1] }
    }

    /// Record once per simulation tick, after the world stepped. Frozen ticks
    /// (hitstop, RC) overwrite the same frame; a reset or replay that moves
    /// the frame counter backwards clears the trail.
    pub fn record(&mut self, w: &World, cells: [Cell; 2]) {
        for (i, cell) in cells.into_iter().enumerate() {
            let f = &w.fighters[i];
            let snap = Snapshot {
                frame: w.frame,
                x: sub(f.pos.x),
                y: sub(f.pos.y),
                facing_right: f.facing_right,
                cell,
                ground: self.ground_context(w, i),
            };
            let t = &mut self.trail[i];
            match t.back().map(|s| s.frame) {
                Some(last) if last == w.frame => {
                    *t.back_mut().unwrap() = snap;
                }
                Some(last) if last + 1 == w.frame => t.push_back(snap),
                Some(_) => {
                    t.clear();
                    t.push_back(snap);
                }
                None => t.push_back(snap),
            }
            while t.len() > TRAIL {
                t.pop_front();
            }
        }
    }

    pub fn ground_context(&self, w: &World, i: usize) -> crate::sequences::GroundContext {
        use crate::sequences::{GroundContext, GroundState};
        let state = GroundState::of(&w.fighters[i].action);
        if let Some(previous) = self.trail[i].back().filter(|s| {
            s.frame == w.frame || s.frame.checked_add(1) == Some(w.frame)
        }) {
            if previous.ground.state == state {
                return GroundContext { age: previous.ground.age.saturating_add(u32::from(previous.frame != w.frame)),
                    ..previous.ground };
            }
            // Low attack and recoil releases are already crouched. Replaying
            // the standing entry would bob the head upward before lowering it.
            let was_low = matches!(previous.cell, Cell::CrouchSaber(_) | Cell::CrouchLights(_) | Cell::Recoil(2 | 3 | 6 | 7))
                || (w.fighters[i].id == CharacterId::Kogan
                    && matches!(previous.cell, Cell::CrouchSaber(_) | Cell::CrouchPunch(_)));
            if was_low && matches!(state, GroundState::Crouch | GroundState::Stand) {
                return GroundContext { state, from: GroundState::Crouch,
                    age: if state == GroundState::Crouch { 2 } else { 0 } };
            }
            return GroundContext { state, from: previous.ground.state, age: 0 };
        }
        GroundContext { state, ..Default::default() }
    }

    // A feint can expire one tick before reaching the floor. Keep its drawn
    // withdrawal across that legal Jump state, using adjacent drawing history.
    fn feint_descent(&self, w: &World, i: usize) -> bool {
        let f = &w.fighters[i];
        f.id == CharacterId::Kogan && f.airborne && matches!(f.action, Action::Jump { .. })
            && f.last_move.and_then(|id| f.data().move_def(id)).is_some_and(|m| m.feintable)
            && self.trail[i].back().is_some_and(|s| {
                (s.frame == w.frame || s.frame.checked_add(1) == Some(w.frame))
                    && matches!(s.cell, Cell::AirSaber(4 | 5))
            })
    }

    fn saber_landing(&self, w: &World, i: usize) -> bool {
        let f = &w.fighters[i];
        if f.id != CharacterId::Kogan || f.airborne { return false; }
        let Action::Landing { frame, total: 2 } = f.action else { return false; };
        w.frame.checked_sub(frame as u32 + 1).and_then(|tick| self.at(i, tick))
            .is_some_and(|s| matches!(s.cell, Cell::AirSaber(_)))
    }

    fn air_recovery_landing(&self, w: &World, i: usize) -> Option<Cell> {
        let f = &w.fighters[i];
        if f.airborne { return None; }
        let Action::Landing { frame, total: 2 } = f.action else { return None; };
        let previous = w.frame.checked_sub(u32::from(frame) + 1).and_then(|tick| self.at(i, tick))?;
        matches!(previous.cell, Cell::AirRecovery(_))
            .then_some(if frame == 0 { Cell::AirRecovery(3) } else { Cell::Reaction(10) })
    }

    pub fn cell_for(&self, w: &World, i: usize, sprites: &SpriteSet) -> Cell {
        if let Some(cell) = self.air_recovery_landing(w, i).filter(|&c| sprites.frame(c).is_some()) {
            return cell;
        }
        if self.feint_descent(w, i) && sprites.frame(Cell::AirSaber(5)).is_some() {
            return Cell::AirSaber(5);
        }
        // Keep the blade in front through the existing full-jump landing.
        // A recorded air cut is required; last_move alone would affect later jumps.
        if self.saber_landing(w, i) && sprites.frame(Cell::Reaction(8)).is_some() {
            return Cell::Reaction(8);
        }
        sprites.cell_for_with_ground(&w.fighters[i], w.frame, self.ground_context(w, i))
    }

    pub fn reset(&mut self) {
        for t in &mut self.trail {
            t.clear();
        }
    }

    fn at(&self, i: usize, frame: u32) -> Option<Snapshot> {
        self.trail[i].iter().rev().find(|s| s.frame == frame).copied()
    }

    /// The most recent picture that differs from `current`.
    fn previous_cell(&self, i: usize, current: Cell) -> Option<Snapshot> {
        self.trail[i].iter().rev().find(|s| s.cell != current).copied()
    }
}

/// One drawn picture. `x`, `y` are the feet in world pixels; `rot` is
/// radians, positive leaning toward the body's facing; scale is about the
/// feet. `flash` mixes the sprite toward `flash_color` before tinting.
#[derive(Clone, Copy, Debug)]
pub struct Layer {
    pub cell: Cell,
    pub x: f32,
    pub y: f32,
    pub facing_right: bool,
    pub rot: f32,
    pub sx: f32,
    pub sy: f32,
    pub alpha: f32,
    pub flash: f32,
    pub flash_color: Color,
    pub tint: Color,
}

pub struct LayerOpts {
    /// Elapsed winner drawing ticks, present only after the body has recovered.
    pub win: Option<u32>,
    /// Zero-health consequence selected by the match presentation clock.
    pub defeat: Option<Cell>,
    /// Body flash from the effects layer: strength and colour.
    pub flash: (f32, Color),
}

#[derive(Clone, Copy)]
struct Motion {
    dx: f32,
    dy: f32,
    rot: f32,
    sx: f32,
    sy: f32,
    alpha: f32,
    /// Afterimages: colour, count, frames between them.
    ghosts: Option<(Color, u32, u32)>,
}

impl Motion {
    const REST: Self = Self {
        dx: 0.0,
        dy: 0.0,
        rot: 0.0,
        sx: 1.0,
        sy: 1.0,
        alpha: 1.0,
        ghosts: None,
    };
}

pub fn ghost_color(id: CharacterId) -> Color {
    match id {
        CharacterId::Kogan => GHOST_KOGAN,
        CharacterId::Raya => GHOST_RAYA,
    }
}

/// Everything to draw for fighter `i`, back to front: afterimages, the
/// fading previous picture, then the body itself.
pub fn layers(
    w: &World,
    i: usize,
    sprites: &SpriteSet,
    history: &History,
    opts: &LayerOpts,
) -> Vec<Layer> {
    let f = &w.fighters[i];
    let mut cell = history.cell_for(w, i, sprites);
    if let Some(age) = opts.win.filter(|_| history.victory_ready(w, i)) {
        cell = crate::sequences::victory_cell(f, age)
            .filter(|&cell| sprites.frame(cell).is_some()).unwrap_or(Cell::Pose(Pose::Win));
    }
    if let Some(down) = opts.defeat {
        cell = if sprites.frame(down).is_some() { down } else {
            Cell::Pose(match down {
                Cell::Floor(0) | Cell::Reaction(4) => Pose::Down,
                Cell::Floor(_) | Cell::Reaction(5..=7) => Pose::Getup,
                _ => Pose::Hurt,
            })
        };
    }
    let mut m = if opts.defeat.is_some() { Motion::REST } else { motion(f, w) };
    respect_authored_drawing(f.id, cell, &mut m);

    // Hitstop: the struck body shudders in place, the striker leans on
    // the hit. Amplitude follows the weight of the contact.
    if w.hitstop > 0 {
        if matches!(f.action, Action::Hit { .. } | Action::Block { .. }) {
            let amp = if w.hitstop >= 8 { 2.2 } else { 1.3 };
            m.dx += if w.hitstop.is_multiple_of(2) { amp } else { -amp };
        } else if f.action.attacking().is_some_and(|(_, _, c)| c != aeon_sim::Connect::None) {
            m.dx += 1.0;
        }
    }

    let facing = if f.facing_right { 1.0 } else { -1.0 };
    let x = sub(f.pos.x) + m.dx * facing;
    let y = sub(f.pos.y) + m.dy;
    let mut out = Vec::with_capacity(6);

    if let Some((color, count, spacing)) = m.ghosts {
        let count = if f.id == CharacterId::Kogan || matches!(cell, Cell::Ground(_)) { count.min(2) } else { count };
        for k in (1..=count).rev() {
            let Some(snap) = history.at(i, w.frame.saturating_sub(k * spacing)) else {
                continue;
            };
            // A changing step silhouette must not trail an old stance or
            // put a previous leaning head ahead of the braking body.
            if (matches!(cell, Cell::Uppercut(_) | Cell::UppercutCompact(_) | Cell::AirLights(_) | Cell::CrouchSaber(_) | Cell::Flash(_) | Cell::Chant(_) | Cell::Signature(_) | Cell::StandingLights(_) | Cell::CrouchLights(_) | Cell::Utility(_) | Cell::AirRecovery(_) | Cell::Recoil(_) | Cell::Ground(_)) || kogan_combat_cell(f.id, cell)) && snap.cell != cell { continue; }
            // A body that has not moved leaves no trail.
            if (snap.x - sub(f.pos.x)).abs() + (snap.y - sub(f.pos.y)).abs() < 1.0 {
                continue;
            }
            out.push(Layer {
                cell: snap.cell,
                x: snap.x,
                y: snap.y,
                facing_right: snap.facing_right,
                rot: if authored_drawing(f.id, snap.cell) { 0.0 } else { m.rot * 0.5 },
                sx: 1.0,
                sy: 1.0,
                alpha: 0.17 * (1.0 - (k - 1) as f32 / count as f32),
                flash: 0.85,
                flash_color: color,
                tint: WHITE,
            });
        }
    }

    // The hurt tint yields to a flash so a clean hit reads white, not pink.
    let settling = matches!(opts.defeat,Some(Cell::Floor(_) | Cell::Reaction(4..=7)));
    let tint = match f.action {
        Action::Hit { .. } | Action::Thrown { .. } if !settling && opts.flash.0 < 0.3 => HURT_TINT,
        _ => WHITE,
    };

    // A tumbling or floored body cuts rather than fades: the previous
    // upright picture has no honest place over a body lying flat.
    let cuts = opts.defeat.is_some() || matches!(f.action, Action::Knockdown { .. }) || (f.airborne && f.action.in_hitstun());
    // Authored movement/weapon phases already describe the transition. Overlaying
    // old silhouettes creates duplicate limbs and weapons through these cuts.
    if let Some(prev) = history.previous_cell(i, cell).filter(|prev| {
        !(cuts || [cell, prev.cell].into_iter().any(|c| {
            matches!(c, Cell::Uppercut(_) | Cell::UppercutCompact(_) | Cell::Movement(_) | Cell::Ranged(_) | Cell::AirLights(_) | Cell::CrouchSaber(_) | Cell::Flash(_) | Cell::Chant(_) | Cell::Signature(_) | Cell::StandingLights(_) | Cell::CrouchLights(_) | Cell::Utility(_) | Cell::AirRecovery(_) | Cell::Recoil(_) | Cell::Ground(_) | Cell::Atlas(0..=3))
                || kogan_combat_cell(f.id, c)
                || matches!((f.id, c), (CharacterId::Raya, Cell::Reaction(_)))
        }))
    }) {
        let age = w.frame.saturating_sub(prev.frame);
        if (1..=CROSSFADE).contains(&age) {
            out.push(Layer {
                cell: prev.cell,
                x,
                y,
                facing_right: f.facing_right,
                rot: m.rot,
                sx: m.sx,
                sy: m.sy,
                alpha: m.alpha * 0.55 * (1.0 - (age - 1) as f32 / CROSSFADE as f32),
                flash: 0.0,
                flash_color: WHITE,
                tint,
            });
        }
    }

    out.push(Layer {
        cell,
        x,
        y,
        facing_right: f.facing_right,
        rot: m.rot,
        sx: m.sx,
        sy: m.sy,
        alpha: m.alpha,
        flash: opts.flash.0,
        flash_color: opts.flash.1,
        tint,
    });
    out
}

/// Dedicated drawings already contain the bend/tumble. Rotating them again
/// around their feet puts the body below the floor and distorts sword arcs.
fn kogan_combat_cell(id: CharacterId, cell: Cell) -> bool {
    id == CharacterId::Kogan && matches!(cell, Cell::Atlas(0..=15) | Cell::Ground(_) | Cell::Disc(_) | Cell::Poke(_) | Cell::Thrust(_) | Cell::Uppercut(_) | Cell::UppercutCompact(_) | Cell::Reaction(_) | Cell::Recoil(_) | Cell::AirRecovery(_) | Cell::Floor(_) | Cell::Judgment(_) | Cell::AirShot(_) | Cell::AirSaber(_) | Cell::AirLights(_) | Cell::Flash(_) | Cell::CrouchSaber(_) | Cell::CrouchPunch(_) | Cell::Overhead(_) | Cell::ThrowTech(_) | Cell::Victory(_))
}

fn authored_drawing(id: CharacterId, cell: Cell) -> bool {
    matches!(cell, Cell::Reaction(_) | Cell::Uppercut(_) | Cell::UppercutCompact(_) | Cell::Movement(_) | Cell::Ranged(_) | Cell::AirLights(_) | Cell::CrouchSaber(_) | Cell::Flash(_) | Cell::Chant(_) | Cell::Signature(_) | Cell::StandingLights(_) | Cell::CrouchLights(_) | Cell::Utility(_) | Cell::AirRecovery(_) | Cell::Recoil(_) | Cell::Ground(_) | Cell::Atlas(0..=3))
        || kogan_combat_cell(id, cell)
}

fn respect_authored_drawing(id: CharacterId, cell: Cell, m: &mut Motion) {
    if matches!(cell, Cell::AirLights(_) | Cell::CrouchSaber(_) | Cell::Flash(_) | Cell::Chant(_) | Cell::Signature(_) | Cell::StandingLights(_) | Cell::CrouchLights(_)) || id == CharacterId::Kogan && matches!(cell, Cell::AirShot(_) | Cell::AirSaber(_) | Cell::AirLights(_) | Cell::Flash(_) | Cell::CrouchSaber(_) | Cell::CrouchPunch(_) | Cell::Overhead(_) | Cell::ThrowTech(_) | Cell::Victory(_)) {
        // Commitment is drawn; extra shifts detach the weapon from its contact line.
        m.dx = 0.0;
        m.dy = 0.0;
    }
    if authored_drawing(id, cell) {
        m.rot = 0.0;
        m.sx = 1.0;
        m.sy = 1.0;
    }
}

fn motion(f: &Fighter, w: &World) -> Motion {
    let mut m = Motion::REST;
    let d = f.data();
    let ghost = ghost_color(f.id);
    match &f.action {
        Action::Stand => {
            let breath = (w.frame as f32 * 0.055).sin();
            m.sy = 1.0 + 0.004 * breath;
            m.sx = 1.0 - 0.002 * breath;
        }
        Action::Crouch => {}
        Action::Walk { .. } => {
            m.dy = 0.8 * (w.frame as f32 * 0.26).sin().abs();
        }
        Action::Run => {
            // A glide: the body leans and the copper streams behind it.
            m.rot = 0.14;
            m.sy = 0.98;
            m.sx = 1.02;
            m.ghosts = Some((ghost, 3, 2));
        }
        Action::BackDash { frame } => {
            let t = *frame as f32 / 14.0;
            m.rot = -0.10 * (1.0 - t);
            m.alpha = if *frame < 8 { 0.9 } else { 1.0 };
            if *frame < 8 {
                m.ghosts = Some((ghost, 3, 2));
            }
        }
        Action::Prejump { frame, .. } => {
            let t = *frame as f32 / aeon_sim::PREJUMP as f32;
            m.sx = 1.08 - 0.08 * t;
            m.sy = 0.90 + 0.10 * t;
        }
        Action::Jump { hop, .. } => {
            let jump_y = if *hop { d.hop_y } else { d.jump_y } as f32;
            let rise = (f.vel.y as f32 / jump_y).clamp(-1.0, 1.0);
            if rise > 0.0 {
                m.sy = 1.0 + 0.05 * rise;
                m.sx = 1.0 - 0.03 * rise;
            }
            let forward = f.vel.x.signum() * if f.facing_right { 1 } else { -1 };
            m.rot = match forward {
                1 => 0.05,
                -1 => -0.03,
                _ => 0.0,
            };
        }
        Action::Landing { frame, total } => {
            let t = (*frame as f32 + 1.0) / (*total as f32).max(1.0);
            m.sx = lerp(1.07, 1.0, ease_out(t));
            m.sy = lerp(0.91, 1.0, ease_out(t));
        }
        Action::Attack {
            move_id, frame, ..
        } => {
            if let Some(mv) = d.move_def(*move_id) {
                attack(f, mv, *frame, &mut m, ghost);
            }
        }
        Action::Feint { frame } => {
            if f.id != CharacterId::Kogan {
                m.alpha = if frame % 2 == 0 { 0.6 } else { 1.0 };
                m.rot = -0.06 * (1.0 - *frame as f32 / 8.0);
            }
        }
        Action::Block { stun, .. } => {
            let k = (*stun as f32 / 12.0).min(1.0);
            m.dx = -2.0 * k;
            m.rot = -0.05 * k;
        }
        Action::Hit { stun, knockdown } => {
            if f.airborne {
                let lift = (sub(f.pos.y) / 70.0).clamp(0.0, 1.0);
                m.rot = if *knockdown {
                    -(0.35 + 1.0 * lift)
                } else {
                    -(0.2 + 0.4 * lift)
                };
                if *knockdown && f.vel.y < 0 {
                    // Falling to the floor: nearly flat, so the lying pose
                    // that follows is a continuation, not a cut.
                    m.rot = m.rot.min(-1.25);
                }
            } else {
                let k = (*stun as f32 / 16.0).min(1.0);
                m.rot = -0.14 * k;
                m.dx = -2.0 * k;
                m.sx = 1.0 - 0.03 * k;
            }
        }
        Action::Knockdown { frame } => {
            if *frame < 8 {
                let t = *frame as f32 / 8.0;
                m.dy = 6.0 * (t * PI).sin() * (1.0 - t);
                m.sx = 1.0 + 0.04 * (1.0 - t);
            }
        }
        Action::Getup { .. } => {}
        Action::Thrown { .. } => {
            m.rot = -0.12;
        }
        Action::ThrowTech { frame } => {
            let t = *frame as f32 / 16.0;
            m.rot = -0.08 * (1.0 - t);
        }
    }
    m
}

/// Anticipation, contact, recovery for every attack in both kits.
fn attack(f: &Fighter, mv: &MoveDef, frame: u16, m: &mut Motion, ghost: Color) {
    let s = mv.first_active() as f32;
    let a = mv.last_active() as f32;
    let tot = mv.total_frames() as f32;
    let fr = frame as f32;
    let rec = (tot - a).max(1.0);
    let kogan = f.id == CharacterId::Kogan;
    match mv.id {
        MoveId::Throw | MoveId::CommandGrab => {
            // Reach in, seize, hold; the recovery is the whiff.
            if fr < s {
                m.rot = 0.10 * ease_in(fr / s);
                m.sx = 1.0 + 0.04 * (fr / s);
            } else if fr < a {
                m.rot = 0.10;
                m.sx = 1.05;
            } else {
                m.rot = 0.10 * (1.0 - ease_out((fr - a) / rec));
            }
        }
        MoveId::Uppercut => {
            if f.vel.y > 0 {
                m.rot = 0.18;
                m.sy = 1.04;
                m.sx = 0.97;
                m.ghosts = Some((ghost, 3, 2));
            } else {
                m.rot = 0.08 * (sub(f.pos.y) / 60.0).clamp(0.0, 1.0);
            }
        }
        MoveId::SpecialOverhead => {
            m.rot = if f.vel.y > 0 { 0.10 } else { 0.26 };
            m.ghosts = Some((ghost, 2, 2));
        }
        MoveId::Super => {
            m.ghosts = Some((ghost, 4, 1));
            m.rot = if fr < a { 0.12 } else { 0.12 * (1.0 - ease_out((fr - a) / rec)) };
        }
        MoveId::CommandDash => {
            let t = fr / tot.max(1.0);
            if kogan {
                m.rot = 0.16 * (1.0 - t * t);
                m.ghosts = Some((ghost, 3, 2));
            } else {
                // The processional passes through the body: she is written
                // light for its duration, and floats.
                m.alpha = 0.55;
                m.dy = 2.0 * (t * PI).sin();
                m.rot = 0.06;
                m.ghosts = Some((ghost, 4, 2));
            }
        }
        MoveId::Rekka1 | MoveId::Rekka2 | MoveId::Rekka3 => {
            if !f.airborne && fr < mv.vel_frames as f32 {
                m.ghosts = Some((ghost, 2, 3));
            }
            phase(mv, frame, m, false);
        }
        MoveId::ExA if kogan => {
            if fr < mv.vel_frames as f32 {
                m.ghosts = Some((ghost, 3, 2));
            }
            phase(mv, frame, m, false);
        }
        MoveId::ShotA | MoveId::ExB | MoveId::AirShot if kogan => {
            // The revolver kicks after the shot leaves.
            if fr >= s {
                let t = ((fr - s) / 8.0).min(1.0);
                m.dx = -4.0 * (1.0 - ease_out(t));
                m.rot = -0.06 * (1.0 - t);
            } else {
                m.rot = 0.04 * (fr / s.max(1.0));
            }
        }
        MoveId::ShotA | MoveId::ExB => {
            // The crystal is tossed: a small underhand swing.
            if fr < s {
                m.rot = lerp(-0.04, 0.06, ease_in(fr / s.max(1.0)));
            } else {
                m.rot = 0.06 * (1.0 - ease_out((fr - s) / (tot - s).max(1.0)));
            }
        }
        MoveId::ShotB | MoveId::ExA => {
            // Voice and wave: gather, release, settle.
            if fr < s {
                m.rot = lerp(-0.05, 0.12, ease_in(fr / s.max(1.0)));
            } else {
                let t = (fr - s) / (tot - s).max(1.0);
                m.rot = 0.07 * (1.0 - ease_out(t));
                m.dx = 2.0 * (1.0 - t);
            }
        }
        MoveId::Charge => {
            let t = f.channel_frames as f32;
            m.sy = 1.0 + 0.012 * (t * 0.35).sin();
            m.dy = (t * 0.2).sin();
        }
        MoveId::Guard => {
            // Plants against the shot: weight back, disc forward.
            if fr < s {
                m.rot = -0.08 * ease_in(fr / s.max(1.0));
                m.sx = 0.96;
            } else if fr < a {
                m.rot = -0.10;
                m.sx = 1.02;
            } else {
                m.rot = -0.10 * (1.0 - ease_out((fr - a) / rec));
            }
        }
        MoveId::Detonate => {
            m.sx = 1.0 + 0.05 * (1.0 - fr / tot.max(1.0));
        }
        id if id.is_crouching() => phase(mv, frame, m, true),
        _ => phase(mv, frame, m, false),
    }
    if f.airborne && mv.id.is_normal() {
        m.rot *= 0.6;
        m.dx = 0.0;
    }
}

/// Settle back through the wind-up, drive forward on contact, ease home.
fn phase(mv: &MoveDef, frame: u16, m: &mut Motion, crouching: bool) {
    let s = mv.first_active() as f32;
    let a = mv.last_active() as f32;
    let tot = mv.total_frames() as f32;
    let fr = frame as f32;
    let lean = if crouching { 0.05 } else { 0.09 };
    if fr < s {
        let t = fr / s.max(1.0);
        m.rot = lerp(-0.04, lean, ease_in(t));
        m.sx = 1.0 - 0.02 * (1.0 - t);
    } else if fr < a {
        let t = (fr - s) / (a - s).max(1.0);
        m.rot = lean - 0.03 * t;
        let first = frame == mv.first_active();
        m.sx = if first { 1.06 } else { 1.03 };
        m.dx = if first { 2.0 } else { 1.0 };
    } else {
        let t = (fr - a) / (tot - a).max(1.0);
        m.rot = (lean - 0.03) * (1.0 - ease_out(t));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::px;

    fn set(id: CharacterId) -> SpriteSet {
        SpriteSet::empty(id)
    }

    #[test]
    fn every_state_of_both_kits_produces_a_finite_layer() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            let sprites = set(id);
            let history = History::default();
            let opts = LayerOpts {
                win: None, defeat: None,
                flash: (0.0, WHITE),
            };
            for move_id in MoveId::ALL {
                let Some(mv) = id.data().move_def(move_id) else { continue };
                for frame in 0..mv.total_frames() {
                    let mut w = World::new(id, id);
                    w.fighters[0].start_move(move_id);
                    w.fighters[0].action = Action::Attack {
                        move_id,
                        frame,
                        connected: aeon_sim::Connect::None,
                    };
                    let out = layers(&w, 0, &sprites, &history, &opts);
                    let body = out.last().unwrap();
                    assert!(body.rot.is_finite() && body.sx.is_finite() && body.sy.is_finite());
                    assert!(body.sx > 0.5 && body.sy > 0.5 && body.alpha > 0.0);
                    assert!(body.rot.abs() < 0.5, "{id:?} {move_id:?} f{frame}");
                }
            }
            for action in [
                Action::Run,
                Action::BackDash { frame: 3 },
                Action::Prejump { frame: 1, dir_x: 1, hop: true },
                Action::Landing { frame: 0, total: 2 },
                Action::Feint { frame: 2 },
                Action::Block { crouching: true, stun: 6 },
                Action::Hit { stun: 10, knockdown: false },
                Action::Knockdown { frame: 2 },
                Action::Getup { frame: 9 },
                Action::Thrown { frame: 1, techable: true, damage: 1, meter: 0 },
                Action::ThrowTech { frame: 3 },
            ] {
                let mut w = World::new(id, id);
                w.fighters[0].action = action;
                let out = layers(&w, 0, &sprites, &history, &opts);
                assert!(out.last().unwrap().rot.is_finite());
            }
        }
    }

    #[test]
    fn authored_falling_and_reversal_drawings_are_not_rotated_twice() {
        let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
        w.fighters[0].airborne = true;
        w.fighters[0].vel.y = -px(3);
        w.fighters[0].pos.y = px(30);
        w.fighters[0].action = Action::Hit { stun: 4, knockdown: true };
        let mut m = motion(&w.fighters[0], &w);
        assert!(m.rot < -1.0);
        respect_authored_drawing(CharacterId::Kogan, Cell::Reaction(3), &mut m);
        assert_eq!((m.rot, m.sx, m.sy), (0.0, 1.0, 1.0));
        w.fighters[0].start_move(MoveId::Uppercut);
        w.fighters[0].vel.y = px(6);
        let mut m = motion(&w.fighters[0], &w);
        respect_authored_drawing(CharacterId::Kogan, Cell::Uppercut(1), &mut m);
        assert_eq!((m.rot, m.sx, m.sy), (0.0, 1.0, 1.0));
    }

    #[test]
    fn a_launched_body_tumbles_and_lands_flat() {
        let mut w = World::new(CharacterId::Kogan, CharacterId::Kogan);
        w.fighters[0].pos.y = px(60);
        w.fighters[0].airborne = true;
        w.fighters[0].vel.y = -px(3);
        w.fighters[0].action = Action::Hit { stun: 10, knockdown: true };
        let out = layers(
            &w,
            0,
            &set(CharacterId::Kogan),
            &History::default(),
            &LayerOpts { win: None, defeat: None, flash: (0.0, WHITE) },
        );
        assert!(out.last().unwrap().rot <= -1.25);
    }

    #[test]
    fn afterimages_only_trail_a_moving_body_and_hold_through_hitstop() {
        let sprites = set(CharacterId::Kogan);
        let mut history = History::default();
        let mut w = World::new(CharacterId::Kogan, CharacterId::Kogan);
        let opts = LayerOpts { win: None, defeat: None, flash: (0.0, WHITE) };
        w.fighters[0].action = Action::Run;
        for _ in 0..6 {
            w.frame += 1;
            history.record(&w, [Cell::Pose(Pose::Run), Cell::Pose(Pose::Idle)]);
        }
        assert_eq!(layers(&w, 0, &sprites, &history, &opts).len(), 1, "no motion, no trail");
        for _ in 0..6 {
            w.frame += 1;
            w.fighters[0].pos.x += px(6);
            history.record(&w, [Cell::Pose(Pose::Run), Cell::Pose(Pose::Idle)]);
        }
        let out = layers(&w, 0, &sprites, &history, &opts);
        assert_eq!(out.len(), 3, "two light ghosts and the body");
        assert!(out[0].alpha < out[1].alpha && out[1].alpha <= 0.17);
        // A frozen tick overwrites the same frame instead of growing the trail.
        history.record(&w, [Cell::Pose(Pose::Run), Cell::Pose(Pose::Idle)]);
        assert_eq!(layers(&w, 0, &sprites, &history, &opts).len(), 3);
    }

    #[test]
    fn a_changed_picture_crossfades_for_two_frames_only() {
        let sprites = set(CharacterId::Raya);
        let mut history = History::default();
        let mut w = World::new(CharacterId::Raya, CharacterId::Raya);
        let opts = LayerOpts { win: None, defeat: None, flash: (0.0, WHITE) };
        history.record(&w, [Cell::Pose(Pose::Idle); 2]);
        w.frame += 1;
        w.fighters[0].action = Action::Attack {
            move_id: MoveId::StP,
            frame: 4,
            connected: aeon_sim::Connect::None,
        };
        let cell = sprites.cell_for(&w.fighters[0], w.frame);
        assert_ne!(cell, Cell::Pose(Pose::Idle));
        let out = layers(&w, 0, &sprites, &history, &opts);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].cell, Cell::Pose(Pose::Idle));
        history.record(&w, [cell, Cell::Pose(Pose::Idle)]);
        w.frame += 1;
        history.record(&w, [cell, Cell::Pose(Pose::Idle)]);
        w.frame += 1;
        history.record(&w, [cell, Cell::Pose(Pose::Idle)]);
        assert_eq!(layers(&w, 0, &sprites, &history, &opts).len(), 1);
    }

    #[test]
    fn movement_landing_does_not_leave_an_airborne_ghost() {
        let sprites = set(CharacterId::Kogan);
        let mut history = History::default();
        let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
        history.record(&w, [Cell::Movement(6), Cell::Pose(Pose::Idle)]);
        w.frame += 1;
        w.fighters[0].action = Action::Landing { frame: 1, total: 2 };
        let opts = LayerOpts { win: None, defeat: None, flash: (0.0, WHITE) };
        let out = layers(&w, 0, &sprites, &history, &opts);
        assert_eq!(out.len(), 1, "landing must not overlay an airborne body");
        assert!(!matches!(out[0].cell, Cell::Movement(_)));
    }

    #[test]
    fn crouching_attacks_return_low_and_standing_release_can_rise() {
        use crate::sequences::{ground_cell, GroundState};
        for facing in [false, true] {
            for (body, ending) in [Cell::CrouchPunch(3), Cell::CrouchSaber(3), Cell::CrouchSaber(7), Cell::CrouchSaber(11), Cell::CrouchSaber(15)].into_iter().map(|c| (CharacterId::Kogan, c))
                .chain([Cell::CrouchSaber(3), Cell::CrouchSaber(7), Cell::CrouchSaber(11), Cell::CrouchSaber(15), Cell::CrouchLights(3), Cell::CrouchLights(7)].into_iter().map(|c| (CharacterId::Raya, c))) {
                let mut w = World::new(body, CharacterId::Kogan);
                w.fighters[0].facing_right = facing;
                w.fighters[0].start_move(MoveId::CrP);
                let mut history = History::default();
                history.record(&w, [ending, Cell::Pose(Pose::Idle)]);
                w.frame += 1;
                w.fighters[0].action = Action::Crouch;
                let c = history.ground_context(&w, 0);
                assert_eq!(ground_cell(&w.fighters[0], c), Some(Cell::Ground(3)));
                history.record(&w, [Cell::Ground(3), Cell::Pose(Pose::Idle)]);
                assert_eq!(history.ground_context(&w, 0), c, "pause preserves the grounded return");
                w.frame += 1;
                w.fighters[0].action = Action::Stand;
                let c = history.ground_context(&w, 0);
                assert_eq!(c.from, GroundState::Crouch);
                assert_eq!(ground_cell(&w.fighters[0], c), Some(Cell::Ground(2)));
                w.fighters[0].start_move(MoveId::StP);
                assert_eq!(ground_cell(&w.fighters[0], history.ground_context(&w, 0)), None);
                history.reset();
                w.fighters[0].action = Action::Crouch;
                assert_eq!(ground_cell(&w.fighters[0], history.ground_context(&w, 0)), Some(Cell::Ground(2)),
                    "reset must not inherit an old low return");
            }
        }
    }

    #[test]
    fn low_recoil_returns_stay_low_and_yield_to_control_for_both_bodies() {
        use crate::sequences::{ground_cell, GroundState};
        for body in [CharacterId::Kogan, CharacterId::Raya] {
            for facing in [false, true] {
                for ending in [Cell::Recoil(2), Cell::Recoil(3), Cell::Recoil(6), Cell::Recoil(7)] {
                    for release in [Action::Crouch, Action::Stand] {
                        let mut w = World::new(body, body);
                        w.fighters[0].facing_right = facing;
                        w.fighters[0].action = Action::Block { crouching: true, stun: 0 };
                        let mut history = History::default();
                        history.record(&w, [ending, Cell::Pose(Pose::Idle)]);
                        w.frame += 1;
                        w.fighters[0].action = release;
                        let hash = w.state_hash();
                        let c = history.ground_context(&w, 0);
                        assert_eq!(c.from, GroundState::Crouch);
                        let cell = if matches!(w.fighters[0].action, Action::Crouch) { Cell::Ground(3) } else { Cell::Ground(2) };
                        assert_eq!(ground_cell(&w.fighters[0], c), Some(cell));
                        history.record(&w, [cell, Cell::Pose(Pose::Idle)]);
                        assert_eq!(history.ground_context(&w, 0), c, "pause retains the return");
                        assert_eq!(w.state_hash(), hash, "presentation cannot change control");
                        w.fighters[0].start_move(MoveId::StP);
                        assert_eq!(ground_cell(&w.fighters[0], history.ground_context(&w, 0)), None);
                        history.reset();
                        w.fighters[0].action = Action::Crouch;
                        assert_eq!(ground_cell(&w.fighters[0], history.ground_context(&w, 0)), Some(Cell::Ground(2)));
                    }
                }
            }
        }
    }

    #[test]
    fn ground_phase_clock_freezes_resets_and_yields_to_new_actions() {
        use crate::sequences::{ground_cell, GroundState};
        use aeon_sim::{Btn, InputFrame};
        for body in [CharacterId::Kogan, CharacterId::Raya] {
            let mut w = World::new(body, CharacterId::Kogan);
            let mut history = History::default();
            let cells = [Cell::Pose(Pose::Idle); 2];
            history.record(&w, cells);
            for dir in [6, 5, 6] {
                w.tick(InputFrame::dir(dir), InputFrame::dir(5));
                history.record(&w, cells);
            }
            let context = history.ground_context(&w, 0);
            assert_eq!((context.state, context.age), (GroundState::Run, 0));
            assert_eq!(ground_cell(&w.fighters[0], context), Some(if body == CharacterId::Kogan { Cell::Utility(4) } else { Cell::Ground(6) }));
            for _ in 0..12 {
                w.tick(InputFrame::dir(6), InputFrame::dir(5));
                history.record(&w, cells);
            }
            let before = history.ground_context(&w, 0);
            assert_eq!(ground_cell(&w.fighters[0], before), Some(Cell::Ground(1)));
            w.hitstop = 3;
            for _ in 0..3 {
                let hash = w.state_hash();
                for _ in 0..5 {
                    assert_eq!(history.ground_context(&w, 0), before);
                    history.record(&w, cells);
                }
                assert_eq!(hash, w.state_hash(), "presentation queries are read only");
                w.tick(InputFrame::dir(6), InputFrame::dir(5));
                history.record(&w, cells);
                assert_eq!(history.ground_context(&w, 0), before, "frozen ticks hold the drawing clock");
            }
            w.tick(InputFrame::dir(5), InputFrame::dir(5));
            history.record(&w, cells);
            assert_eq!(ground_cell(&w.fighters[0], history.ground_context(&w, 0)), Some(if body == CharacterId::Kogan { Cell::Utility(6) } else { Cell::Ground(6) }));
            w.tick(InputFrame::press(Btn::S), InputFrame::dir(5));
            history.record(&w, cells);
            assert!(w.fighters[0].action.attacking().is_some());
            assert_eq!(ground_cell(&w.fighters[0], history.ground_context(&w, 0)), None,
                "the visible brake never masks an immediate attack");
            w = World::new(body, CharacterId::Kogan);
            history.record(&w, cells);
            let reset = history.ground_context(&w, 0);
            assert_eq!((reset.age, reset.from), (0, GroundState::Other), "replay reset clears old movement");
            w.tick(InputFrame::dir(2), InputFrame::dir(5));
            history.record(&w, cells);
            assert_eq!(ground_cell(&w.fighters[0], history.ground_context(&w, 0)), Some(Cell::Ground(2)));
            for _ in 0..2 {
                w.tick(InputFrame::dir(2), InputFrame::dir(5));
                history.record(&w, cells);
            }
            assert_eq!(ground_cell(&w.fighters[0], history.ground_context(&w, 0)), Some(Cell::Ground(3)));
            w.tick(InputFrame::dir(5), InputFrame::dir(5));
            history.record(&w, cells);
            assert_eq!(ground_cell(&w.fighters[0], history.ground_context(&w, 0)), Some(Cell::Ground(2)));
            history.reset();
            assert_eq!(ground_cell(&w.fighters[0], history.ground_context(&w, 0)), None,
                "training reset discards a pending rise");
        }
    }

    #[test]
    fn raya_movement_return_cuts_previous_bodies_and_leaves_immediate_control() {
        let sprites = set(CharacterId::Raya);
        let opts = LayerOpts { win: None, defeat: None, flash: (0.0, WHITE) };
        for previous in [Cell::AirLights(0), Cell::AirLights(1), Cell::AirLights(2), Cell::AirLights(3), Cell::AirLights(4), Cell::AirLights(5), Cell::CrouchSaber(0), Cell::CrouchSaber(1), Cell::CrouchSaber(2), Cell::CrouchSaber(3), Cell::CrouchSaber(4), Cell::CrouchSaber(5), Cell::CrouchSaber(6), Cell::CrouchSaber(7), Cell::CrouchSaber(8), Cell::CrouchSaber(9), Cell::CrouchSaber(10), Cell::CrouchSaber(11), Cell::CrouchSaber(12), Cell::CrouchSaber(13), Cell::CrouchSaber(14), Cell::CrouchSaber(15), Cell::Uppercut(0), Cell::Uppercut(1), Cell::Uppercut(2), Cell::Uppercut(3), Cell::UppercutCompact(0), Cell::UppercutCompact(1), Cell::Chant(0), Cell::Chant(1), Cell::Chant(2), Cell::Chant(3), Cell::Chant(4), Cell::Chant(5), Cell::Chant(6), Cell::Chant(7), Cell::Signature(0), Cell::Signature(1), Cell::Signature(2), Cell::Signature(3), Cell::Signature(4), Cell::Signature(5), Cell::Signature(6), Cell::Signature(7), Cell::Signature(8), Cell::Signature(9), Cell::Signature(10), Cell::Signature(11), Cell::Flash(0), Cell::Flash(1), Cell::Flash(2), Cell::Flash(3), Cell::Flash(4), Cell::Flash(5), Cell::Flash(6), Cell::Flash(7), Cell::CrouchLights(0), Cell::CrouchLights(1), Cell::CrouchLights(2), Cell::CrouchLights(3), Cell::CrouchLights(4), Cell::CrouchLights(5), Cell::CrouchLights(6), Cell::CrouchLights(7), Cell::StandingLights(0), Cell::StandingLights(1), Cell::StandingLights(2), Cell::StandingLights(3), Cell::StandingLights(4), Cell::StandingLights(5), Cell::StandingLights(6), Cell::StandingLights(7), Cell::Recoil(0), Cell::Recoil(1), Cell::Recoil(2), Cell::Recoil(3), Cell::Recoil(4), Cell::Recoil(5), Cell::Recoil(6), Cell::Recoil(7), Cell::Reaction(0), Cell::Reaction(1), Cell::Reaction(2), Cell::Reaction(3), Cell::Reaction(4), Cell::Reaction(5), Cell::Reaction(6), Cell::Reaction(7), Cell::Ground(0), Cell::Ground(1), Cell::Ground(2), Cell::Ground(3),
            Cell::Ground(4), Cell::Ground(5), Cell::Ground(6), Cell::Ground(7), Cell::Atlas(0),
            Cell::Movement(3), Cell::Movement(6), Cell::Movement(7),
            Cell::Reaction(8), Cell::Reaction(9), Cell::Reaction(10), Cell::Reaction(11),
            Cell::AirRecovery(0), Cell::AirRecovery(1), Cell::AirRecovery(2), Cell::AirRecovery(3)] {
            let mut w = World::new(CharacterId::Raya, CharacterId::Kogan);
            let mut h = History::default(); h.record(&w, [previous, Cell::Pose(Pose::Idle)]);
            w.frame += 1; w.fighters[0].action = Action::Stand;
            let hash = w.state_hash();
            let body = layers(&w, 0, &sprites, &h, &opts);
            assert_eq!(body.len(), 1, "{previous:?}: no previous body remains over idle");
            assert_eq!(body[0].cell, Cell::Pose(Pose::Idle));
            assert_eq!(w.state_hash(), hash);
            w.fighters[0].start_move(MoveId::StP);
            assert_eq!(crate::sequences::movement_cell(&w.fighters[0]), None);
            h.reset();
            assert_eq!(layers(&w, 0, &sprites, &h, &opts).len(), 1);
        }
    }

    #[test]
    fn air_recovery_landing_requires_adjacent_recoil_for_both_bodies() {
        for body in [CharacterId::Kogan, CharacterId::Raya] {
            let mut w = World::new(body, if body == CharacterId::Kogan { CharacterId::Raya } else { CharacterId::Kogan });
            let mut h = History::default(); w.frame = 40; w.fighters[0].airborne = true;
            h.record(&w, [Cell::AirRecovery(2), Cell::Pose(Pose::Idle)]);
            w.fighters[0].airborne = false;
            for frame in 0..2 {
                w.frame += 1; w.fighters[0].action = Action::Landing { frame, total: 2 };
                let cell = if frame == 0 { Cell::AirRecovery(3) } else { Cell::Reaction(10) };
                assert_eq!(h.air_recovery_landing(&w, 0), Some(cell));
                h.record(&w, [cell, Cell::Pose(Pose::Idle)]);
                assert_eq!(h.air_recovery_landing(&w, 0), Some(cell), "same-tick redraw freezes the pose");
            }
            for action in [Action::Stand, Action::Crouch, Action::Hit { stun: 8, knockdown: false },
                Action::Landing { frame: 0, total: 8 }] {
                w.fighters[0].action = action; assert_eq!(h.air_recovery_landing(&w, 0), None);
            }
            w.frame += 10; w.fighters[0].action = Action::Landing { frame: 0, total: 2 };
            assert_eq!(h.air_recovery_landing(&w, 0), None, "stale history cannot affect another jump");
            h.reset(); assert_eq!(h.air_recovery_landing(&w, 0), None);
        }
    }

    #[test]
    fn saber_landing_uses_recent_air_history_and_clears_on_new_jump_or_reset() {
        let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
        let mut history = History::default();
        w.frame = 40;
        w.fighters[0].airborne = true;
        history.record(&w, [Cell::AirSaber(1), Cell::Pose(Pose::Idle)]);
        w.fighters[0].airborne = false;
        w.frame += 1;
        w.fighters[0].action = Action::Landing { frame: 0, total: 8 };
        assert!(!history.saber_landing(&w, 0), "the falling saber uses all four landing drawings");
        w.frame -= 1;
        for frame in 0..2 {
            w.frame += 1;
            w.fighters[0].action = Action::Landing { frame, total: 2 };
            assert!(history.saber_landing(&w, 0));
            history.record(&w, [Cell::Reaction(8), Cell::Pose(Pose::Idle)]);
        }
        w.fighters[0].action = Action::Stand;
        assert!(!history.saber_landing(&w, 0));
        w.frame += 10;
        w.fighters[0].action = Action::Landing { frame: 0, total: 2 };
        assert!(!history.saber_landing(&w, 0), "old cuts do not change a later jump");
        history.reset();
        assert!(!history.saber_landing(&w, 0));
    }

    #[test]
    fn feint_descent_keeps_adjacent_withdrawal_and_yields_on_landing_reset_or_new_action() {
        let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
        let mut history = History::default();
        w.frame = 23;
        w.fighters[0].airborne = true;
        w.fighters[0].last_move = Some(MoveId::Uppercut);
        w.fighters[0].action = Action::Feint { frame: 7 };
        history.record(&w, [Cell::AirSaber(5), Cell::Pose(Pose::Idle)]);
        w.frame += 1;
        w.fighters[0].action = Action::Jump { air_ok: true, hop: false };
        assert!(history.feint_descent(&w, 0));
        history.record(&w, [Cell::AirSaber(5), Cell::Pose(Pose::Idle)]);
        assert!(history.feint_descent(&w, 0), "same-frame freeze retains descent");
        for action in [Action::Hit { stun: 8, knockdown: false }, Action::Attack {
            move_id: MoveId::JS, frame: 0, connected: aeon_sim::Connect::None }, Action::Stand] {
            w.fighters[0].action = action;
            assert!(!history.feint_descent(&w, 0), "a new action owns its drawing");
        }
        w.frame += 1;
        w.fighters[0].airborne = false;
        w.fighters[0].action = Action::Landing { frame: 0, total: 2 };
        assert!(!history.feint_descent(&w, 0));
        assert!(history.saber_landing(&w, 0), "the supported front-blade landing follows");
        w.frame += 10;
        w.fighters[0].airborne = true;
        w.fighters[0].action = Action::Jump { air_ok: true, hop: false };
        assert!(!history.feint_descent(&w, 0), "stale last_move cannot change a later jump");
        history.reset();
        assert!(!history.feint_descent(&w, 0));
    }

    #[test]
    fn kogan_feint_reuse_is_opaque_and_untransformed() {
        let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
        for mv in CharacterId::Kogan.data().moves.iter().filter(|m| m.feintable) {
            for airborne in [false, true] {
                for frame in 0..aeon_sim::fighter::FEINT_RECOVERY {
                    w.fighters[0].last_move = Some(mv.id);
                    w.fighters[0].action = Action::Feint { frame };
                    w.fighters[0].airborne = airborne;
                    let cell = crate::sequences::feint_cell(&w.fighters[0]).unwrap();
                    let mut m = motion(&w.fighters[0], &w);
                    respect_authored_drawing(CharacterId::Kogan, cell, &mut m);
                    assert_eq!((m.dx, m.dy, m.rot, m.sx, m.sy, m.alpha), (0.0, 0.0, 0.0, 1.0, 1.0, 1.0));
                }
            }
        }
    }

    #[test]
    fn authored_kogan_return_has_one_body_and_no_extra_blade_rotation() {
        let sprites = set(CharacterId::Kogan);
        let opts = LayerOpts { win: None, defeat: None, flash: (0.0, WHITE) };
        for previous in [Cell::AirRecovery(0), Cell::AirRecovery(1), Cell::AirRecovery(2), Cell::AirRecovery(3), Cell::CrouchPunch(0), Cell::CrouchPunch(1), Cell::CrouchPunch(2), Cell::CrouchPunch(3), Cell::Victory(0), Cell::Victory(1), Cell::Victory(2), Cell::Victory(3), Cell::ThrowTech(0), Cell::ThrowTech(1), Cell::Utility(0), Cell::Utility(1), Cell::Utility(2), Cell::Utility(3), Cell::Overhead(0), Cell::Overhead(1), Cell::Overhead(2), Cell::Overhead(3), Cell::CrouchSaber(8), Cell::CrouchSaber(9), Cell::CrouchSaber(10), Cell::CrouchSaber(11), Cell::CrouchSaber(12), Cell::CrouchSaber(13), Cell::CrouchSaber(14), Cell::CrouchSaber(15), Cell::CrouchSaber(0), Cell::CrouchSaber(1), Cell::CrouchSaber(2), Cell::CrouchSaber(3), Cell::CrouchSaber(4), Cell::CrouchSaber(5), Cell::CrouchSaber(6), Cell::CrouchSaber(7), Cell::Flash(0), Cell::Flash(1), Cell::Flash(2), Cell::Flash(3), Cell::Flash(4), Cell::Flash(5), Cell::Flash(6), Cell::Flash(7), Cell::AirLights(0), Cell::AirLights(1), Cell::AirLights(2), Cell::AirLights(3), Cell::AirLights(4), Cell::AirLights(5), Cell::AirSaber(0), Cell::AirSaber(1), Cell::AirSaber(2), Cell::AirSaber(3), Cell::AirSaber(4), Cell::AirSaber(5), Cell::AirShot(0), Cell::AirShot(1), Cell::AirShot(2), Cell::AirShot(3), Cell::Judgment(0), Cell::Judgment(1), Cell::Judgment(2), Cell::Judgment(3), Cell::Floor(0), Cell::Floor(1), Cell::Floor(2), Cell::Floor(3), Cell::Recoil(1), Cell::Recoil(3), Cell::Recoil(5), Cell::Recoil(7), Cell::Reaction(0), Cell::Reaction(4), Cell::Reaction(5), Cell::Reaction(6), Cell::Reaction(7), Cell::Ground(2), Cell::Ground(5), Cell::Atlas(2), Cell::Disc(2), Cell::Poke(2), Cell::Atlas(6), Cell::Atlas(10), Cell::Thrust(2), Cell::Uppercut(3), Cell::UppercutCompact(1), Cell::Reaction(8)] {
            let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
            let mut history = History::default();
            history.record(&w, [previous, Cell::Pose(Pose::Idle)]);
            w.frame += 1;
            let out = layers(&w, 0, &sprites, &history, &opts);
            assert_eq!(out.len(), 1, "withdrawal cannot retain a second saber: {previous:?}");
            let mut m = Motion::REST;
            m.rot = 0.1; m.sx = 1.06; m.sy = 0.96;
            respect_authored_drawing(CharacterId::Kogan, previous, &mut m);
            assert_eq!((m.rot, m.sx, m.sy), (0.0, 1.0, 1.0));
        }
        assert!(!kogan_combat_cell(CharacterId::Raya, Cell::Atlas(6)), "Raya has separate review coverage");
    }

    #[test]
    fn judgment_and_low_kick_keep_a_crouched_receiver_visible_for_either_player() {
        for attacker in [0, 1] {
            let mut w = if attacker == 0 { World::new(CharacterId::Kogan, CharacterId::Raya) }
                else { World::new(CharacterId::Raya, CharacterId::Kogan) };
            let defender = 1 - attacker;
            let history = History::default();
            for move_id in [MoveId::Super, MoveId::CrK] {
                w.fighters[attacker].start_move(move_id);
                for action in [Action::Crouch, Action::Block { crouching: true, stun: 8 }] {
                    w.fighters[defender].action = action;
                    assert_eq!(history.draw_order(&w), [attacker, defender]);
                }
                w.fighters[defender].action = Action::Block { crouching: false, stun: 8 };
                assert_eq!(history.draw_order(&w), [defender, attacker], "standing guard leaves the weapon in front");
            }
            w.fighters[defender].action = Action::Crouch;
            w.fighters[attacker].start_move(MoveId::StS);
            assert_eq!(history.draw_order(&w), [defender, attacker], "other reviewed attacks retain their ordering");
        }
    }

    #[test]
    fn close_crouch_and_rise_stay_visible_without_hiding_attacks() {
        use aeon_sim::{Btn, InputFrame};
        for body in [CharacterId::Kogan, CharacterId::Raya] {
            for croucher in 0..2 {
                let mut w = World::new(body, body);
                let mut history = History::default();
                let cells = [Cell::Pose(Pose::Idle); 2];
                let mut inputs = [InputFrame::dir(5); 2];
                inputs[croucher] = InputFrame::dir(2);
                w.tick(inputs[0], inputs[1]);
                history.record(&w, cells);
                assert_eq!(history.draw_order(&w)[1], croucher);
                w.tick(InputFrame::dir(5), InputFrame::dir(5));
                history.record(&w, cells);
                assert_eq!(history.draw_order(&w)[1], croucher, "the half-rise stays visible");
                for _ in 0..2 {
                    w.tick(InputFrame::dir(5), InputFrame::dir(5));
                    history.record(&w, cells);
                }
                assert_eq!(history.draw_order(&w), [0, 1], "rest restores stable ordering");
                inputs[1 - croucher] = InputFrame::press(Btn::S);
                w.tick(inputs[0], inputs[1]);
                history.record(&w, cells);
                let hash = w.state_hash();
                assert_eq!(history.draw_order(&w)[1], 1 - croucher, "attacking hands retain priority");
                assert_eq!(w.state_hash(), hash);
            }
        }
    }

    #[test]
    fn victory_clock_waits_for_recovery_freezes_and_resets() {
        use aeon_sim::{Phase, RoundOutcome};
        let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
        let mut clock = VictoryClock::default();
        let end = |frame| Phase::RoundEnd { outcome: RoundOutcome::Winner(0), frame };
        w.frame = 50; clock.update(&w, end(29)); assert_eq!(clock.age(0), None);
        w.fighters[0].airborne = true;
        w.frame += 1; clock.update(&w, end(30)); assert_eq!(clock.age(0), None);
        w.fighters[0].airborne = false;
        w.fighters[0].action = Action::Landing { frame: 1, total: 2 };
        w.frame += 1; clock.update(&w, end(31)); assert_eq!(clock.age(0), None);
        w.fighters[0].action = Action::Stand;
        w.frame += 1; clock.update(&w, end(32)); assert_eq!(clock.age(0), Some(0));
        assert_eq!(clock.age(1), None);
        clock.update(&w, end(32)); assert_eq!(clock.age(0), Some(0));
        w.frame += 8; clock.update(&w, end(40)); assert_eq!(clock.age(0), Some(8));
        clock.update(&w, Phase::MatchOver { winner: 0 });
        clock.update(&w, Phase::MatchOver { winner: 0 });assert_eq!(clock.age(0),Some(8));
        w = World::new(CharacterId::Kogan,CharacterId::Raya);
        clock.update(&w,Phase::Intro { frame:0 });assert_eq!(clock.age(0),None);
        clock.update(&w,Phase::MatchOver { winner:0 });assert_eq!(clock.age(0),Some(70));
        clock.update(&w,Phase::RoundEnd { outcome:RoundOutcome::Draw,frame:50 });assert_eq!(clock.winner(),None);
        for action in [Action::Hit { stun:3,knockdown:false },Action::Knockdown { frame:3 },Action::ThrowTech { frame:3 }] {
            w.fighters[0].action=action;clock.update(&w,end(50));assert_eq!(clock.winner(),None);
        }
    }

    #[test]
    fn win_pose_only_replaces_a_body_at_rest() {
        let sprites = set(CharacterId::Kogan);
        let history = History::default();
        let opts = LayerOpts { win: Some(0), defeat: None, flash: (0.0, WHITE) };
        let mut w = World::new(CharacterId::Kogan, CharacterId::Kogan);
        assert_eq!(layers(&w, 0, &sprites, &history, &opts)[0].cell, Cell::Pose(Pose::Win));
        w.fighters[0].action = Action::Knockdown { frame: 3 };
        assert_eq!(layers(&w, 0, &sprites, &history, &opts)[0].cell, Cell::Pose(Pose::Down));
    }
    #[test]
    fn winner_gesture_keeps_the_existing_two_tick_crouch_rise() {
        let mut w=World::new(CharacterId::Kogan,CharacterId::Raya);
        let mut history=History::default();
        w.frame=51;
        w.fighters[0].action=Action::Attack { move_id:MoveId::CrHS,frame:28,connected:aeon_sim::Connect::Hit };
        history.record(&w,[Cell::CrouchSaber(7),Cell::Pose(Pose::Idle)]);
        w.fighters[0].action=Action::Stand;
        for tick in [52,53] {
            w.frame=tick;
            assert!(!history.victory_ready(&w,0),"the legal standing state still draws its short rise");
            assert_eq!(crate::sequences::ground_cell(&w.fighters[0],history.ground_context(&w,0)),Some(Cell::Ground(2)));
            history.record(&w,[Cell::Ground(2),Cell::Pose(Pose::Idle)]);
            assert!(!history.victory_ready(&w,0),"freeze holds the rise");
        }
        w.frame=54;assert!(history.victory_ready(&w,0));
        w.fighters[0].action=Action::Jump { air_ok:true,hop:false };
        assert!(!history.victory_ready(&w,0),"new actions retain their drawing");
        history.reset();w.fighters[0].action=Action::Stand;
        assert!(history.victory_ready(&w,0),"no stale rise after reset");
    }

    #[test]
    fn defeat_draws_one_opaque_supported_body_despite_underlying_getup() {
        for id in [CharacterId::Kogan,CharacterId::Raya] {
            let sprites=set(id);let mut w=World::new(id,id);let mut history=History::default();
            history.record(&w,[Cell::Pose(Pose::Idle);2]);w.frame+=1;
            w.fighters[1].health=0;w.fighters[1].action=Action::Getup {frame:23};
            let cell=if id==CharacterId::Kogan { Cell::Floor(0) } else { Cell::Reaction(4) };
            let opts=LayerOpts {win:None,defeat:Some(cell),flash:(0.0,WHITE)};
            let hash=w.state_hash();let out=layers(&w,1,&sprites,&history,&opts);
            assert_eq!(out.len(),1);assert_eq!(out[0].cell,Cell::Pose(Pose::Down));
            assert_eq!((out[0].rot,out[0].sx,out[0].sy,out[0].alpha),(0.0,1.0,1.0,1.0));
            assert_eq!(out[0].y,0.0);assert_eq!(w.state_hash(),hash);
        }
    }

}
