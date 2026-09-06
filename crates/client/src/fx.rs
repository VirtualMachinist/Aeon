//! Impact presentation: sparks, flashes, dust, rings. Everything lives in
//! simulation frames, so hitstop holds a spark on the point of contact and
//! pause or frame-step freeze it. No camera work: the stage never moves.

use aeon_sim::{Aabb, Action, CharacterId, EventKind, MoveId, ShotState, World, SUB};
use macroquad::prelude::*;

use crate::render::{View, COPPER, CYAN, LINEN, VH, VW};

fn sub(v: i32) -> f32 {
    v as f32 / SUB as f32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FxKind {
    /// Strike landed. `heavy` scales it; `body` picks the attacker's matter.
    HitSpark { heavy: bool, body: CharacterId },
    BlockSpark,
    ThrowImpact,
    Tech,
    Dust { big: bool },
    RcRing,
    Feint,
    ExRing,
    Muzzle,
    AirMuzzle,
    Cast,
    DiscGuard,
    Clash,
    Armed,
    Blast,
    Ko,
}

#[derive(Clone, Copy, Debug)]
pub struct Fx {
    pub kind: FxKind,
    /// World pixels.
    pub x: f32,
    pub y: f32,
    pub born: u32,
    pub facing_right: bool,
}

impl Fx {
    fn life(&self) -> u32 {
        match self.kind {
            FxKind::HitSpark { heavy: true, .. } => 18,
            FxKind::HitSpark { .. } => 13,
            FxKind::BlockSpark => 10,
            FxKind::ThrowImpact => 16,
            FxKind::Tech => 12,
            FxKind::Dust { big: true } => 18,
            FxKind::Dust { .. } => 10,
            FxKind::RcRing => 14,
            FxKind::Feint => 9,
            FxKind::ExRing => 9,
            FxKind::Muzzle | FxKind::AirMuzzle => 4,
            FxKind::Cast => 9,
            FxKind::DiscGuard => 10,
            FxKind::Clash => 10,
            FxKind::Armed => 12,
            FxKind::Blast => 12,
            FxKind::Ko => 36,
        }
    }
}

#[derive(Default)]
pub struct Effects {
    fx: Vec<Fx>,
    /// Body flash per fighter: strength, colour, ticks left.
    flash: [(f32, Color, u8); 2],
    /// A super's start dims the vault behind the bodies.
    wash: Option<u32>,
    prev: [Option<Action>; 2],
    last_frame: Option<u32>,
}

impl Effects {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Current body flash for the sprite layer.
    pub fn flash(&self, i: usize) -> (f32, Color) {
        let (s, c, n) = self.flash[i];
        if n > 0 {
            (s, c)
        } else {
            (0.0, WHITE)
        }
    }

    fn set_flash(&mut self, i: usize, strength: f32, color: Color, ticks: u8) {
        self.flash[i] = (strength, color, ticks);
    }

    fn spawn(&mut self, w: &World, kind: FxKind, x: f32, y: f32, facing_right: bool) {
        self.fx.push(Fx {
            kind,
            x,
            y,
            born: w.frame,
            facing_right,
        });
    }

    /// Call once after every simulation tick, frozen ticks included.
    pub fn after_tick(&mut self, w: &World) {
        // A reset, replay or new round moves the counter backwards.
        if self.last_frame.is_some_and(|f| w.frame < f) {
            self.reset();
        }
        let advanced = self.last_frame != Some(w.frame);
        self.last_frame = Some(w.frame);
        // Flashes share the drawing/effect clock. Hitstop and RC consume
        // input ticks without advancing that clock; retain the contact flash.
        // Event processing below still runs so RC can begin on the same frame.
        if advanced {
            for (_, _, n) in &mut self.flash {
                *n = n.saturating_sub(1);
            }
        }
        self.fx.retain(|fx| w.frame.saturating_sub(fx.born) <= fx.life());

        for ev in &w.events {
            let atk = ev.attacker;
            let def = 1 - atk;
            match ev.kind {
                EventKind::Hit | EventKind::Punish | EventKind::Knockdown => {
                    let (x, y) = contact_point(w, atk, def, ev.move_id);
                    let heavy = ev.damage >= 85 || ev.kind == EventKind::Knockdown;
                    let body = w.fighters[atk].id;
                    let facing_right = w.fighters[atk].facing_right;
                    self.spawn(w, FxKind::HitSpark { heavy, body }, x, y, facing_right);
                    self.set_flash(def, 0.62, WHITE, 2);
                }
                EventKind::Block => {
                    let (x, y) = contact_point(w, atk, def, ev.move_id);
                    let facing_right = w.fighters[atk].facing_right;
                    self.spawn(w, FxKind::BlockSpark, x, y, facing_right);
                    self.set_flash(def, 0.30, LINEN, 1);
                }
                EventKind::Throw => {
                    let (x, y) = feet(w, def);
                    let facing_right = w.fighters[atk].facing_right;
                    self.spawn(w, FxKind::ThrowImpact, x, y, facing_right);
                    self.set_flash(def, 0.55, WHITE, 2);
                }
                EventKind::ThrowTech => {
                    let (ax, ay) = chest(w, 0);
                    let (bx, _) = chest(w, 1);
                    self.spawn(w, FxKind::Tech, (ax + bx) / 2.0, ay, true);
                }
                EventKind::RomanCancel => {
                    let (x, y) = chest(w, atk);
                    self.spawn(w, FxKind::RcRing, x, y, w.fighters[atk].facing_right);
                    self.set_flash(atk, 0.55, CYAN, 4);
                }
                EventKind::Feint => {
                    let (x, y) = chest(w, atk);
                    self.spawn(w, FxKind::Feint, x, y, w.fighters[atk].facing_right);
                }
                EventKind::ProjectileGuard => {
                    let f = &w.fighters[atk];
                    let (x, y) = f
                        .current_move()
                        .and_then(|mv| {
                            let (_, frame, _) = f.action.attacking()?;
                            mv.hitboxes_on(frame).next()
                        })
                        .map(|b| {
                            let b = b.to_world(f.pos, f.facing_right);
                            (sub(b.center_x()), sub((b.bottom + b.top) / 2))
                        })
                        .unwrap_or_else(|| chest(w, atk));
                    self.spawn(w, FxKind::DiscGuard, x, y, f.facing_right);
                }
                EventKind::Clash => {
                    let (ax, ay) = chest(w, 0);
                    let (bx, _) = chest(w, 1);
                    self.spawn(w, FxKind::Clash, (ax + bx) / 2.0, ay, true);
                }
                EventKind::Armed => {
                    if let Some(p) = w
                        .projectiles
                        .iter()
                        .find(|p| p.owner == atk && p.armed())
                    {
                        let b = p.hitbox();
                        self.spawn(
                            w,
                            FxKind::Armed,
                            sub(b.center_x()),
                            sub((b.bottom + b.top) / 2),
                            p.facing_right,
                        );
                    }
                }
                EventKind::Detonate => {
                    if let Some(p) = w.projectiles.iter().find(|p| {
                        p.owner == atk && matches!(p.state, ShotState::Detonating { .. })
                    }) {
                        let b = p.hitbox();
                        self.spawn(
                            w,
                            FxKind::Blast,
                            sub(b.center_x()),
                            sub((b.bottom + b.top) / 2),
                            p.facing_right,
                        );
                    }
                }
                EventKind::KO => {
                    let (x, y) = chest(w, def);
                    self.spawn(w, FxKind::Ko, x, y, true);
                    self.set_flash(def, 0.9, LINEN, 6);
                }
                EventKind::Grab | EventKind::Plant | EventKind::TimeOver => {}
            }
        }

        // State transitions the events do not name.
        for i in 0..2 {
            let f = &w.fighters[i];
            let prev = self.prev[i].take();
            match &f.action {
                Action::Knockdown { frame: 0 }
                    if !matches!(prev, Some(Action::Knockdown { .. })) =>
                {
                    let (x, y) = feet(w, i);
                    self.spawn(w, FxKind::Dust { big: true }, x, y, f.facing_right);
                }
                Action::Landing { frame: 0, total } if advanced && *total >= 2 => {
                    let (x, y) = feet(w, i);
                    self.spawn(w, FxKind::Dust { big: false }, x, y, f.facing_right);
                }
                Action::Attack {
                    move_id, frame, ..
                } => {
                    let started = !matches!(
                        prev,
                        Some(Action::Attack { move_id: pm, .. }) if pm == *move_id
                    );
                    if started && *move_id == MoveId::Super {
                        self.wash = Some(w.frame);
                        self.set_flash(i, 0.9, WHITE, 3);
                        let (x, y) = chest(w, i);
                        self.spawn(w, FxKind::ExRing, x, y, f.facing_right);
                    } else if started && move_id.is_ex() {
                        self.set_flash(i, 0.7, CYAN, 3);
                        let (x, y) = chest(w, i);
                        self.spawn(w, FxKind::ExRing, x, y, f.facing_right);
                    }
                    if let Some(mv) = f.data().move_def(*move_id) {
                        if let Some(def) = mv.projectile {
                            if advanced && *frame == mv.startup as u16 {
                                let spawn = def.spawn.to_world(f.pos, f.facing_right);
                                let kind = if f.id == CharacterId::Kogan && *move_id == MoveId::AirShot {
                                    FxKind::AirMuzzle
                                } else if f.id == CharacterId::Kogan
                                    && matches!(
                                        move_id,
                                        MoveId::ShotA | MoveId::ExB
                                    ) {
                                    FxKind::Muzzle
                                } else {
                                    FxKind::Cast
                                };
                                self.spawn(
                                    w,
                                    kind,
                                    sub(spawn.left),
                                    sub(spawn.bottom) + 8.0,
                                    f.facing_right,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
            self.prev[i] = Some(f.action.clone());
        }
    }

    /// Drawn over the stage, under the bodies.
    pub fn draw_behind(&self, v: &View, w: &World) {
        if let Some(born) = self.wash {
            let age = w.frame.saturating_sub(born);
            if age < 14 {
                let a = 0.55 * (1.0 - age as f32 / 14.0);
                v.rect(0.0, 0.0, VW, VH, Color::new(0.0, 0.0, 0.02, a));
            }
        }
    }

    /// Drawn over the bodies.
    pub fn draw(&self, v: &View, w: &World) {
        for fx in &self.fx {
            let age = w.frame.saturating_sub(fx.born);
            let life = fx.life();
            if age > life {
                continue;
            }
            let t = age as f32 / life as f32;
            draw_fx(v, fx, t);
        }
    }
}

fn feet(w: &World, i: usize) -> (f32, f32) {
    let f = &w.fighters[i];
    (sub(f.pos.x), sub(f.pos.y))
}

fn chest(w: &World, i: usize) -> (f32, f32) {
    let f = &w.fighters[i];
    (sub(f.pos.x), sub(f.pos.y) + sub(f.data().stand_h) * 0.6)
}

/// Where a strike met the body: the overlap of the live hitbox and the
/// hurtbox, else the hurtbox edge facing the attacker (shots).
fn contact_point(w: &World, atk: usize, def: usize, move_id: Option<MoveId>) -> (f32, f32) {
    let a = &w.fighters[atk];
    let d = &w.fighters[def];
    let hurt: Vec<Aabb> = d.hurtboxes();
    let hurt = hurt.first().copied().unwrap_or(d.pushbox());
    let hit = move_id
        .and_then(|id| a.data().move_def(id))
        .zip(a.action.attacking())
        .and_then(|(mv, (_, frame, _))| mv.hitboxes_on(frame).next())
        .map(|b| b.to_world(a.pos, a.facing_right));
    if let Some(h) = hit {
        let left = h.left.max(hurt.left);
        let right = h.right.min(hurt.right);
        let bottom = h.bottom.max(hurt.bottom);
        let top = h.top.min(hurt.top);
        if left < right && bottom < top {
            return (sub((left + right) / 2), sub((bottom + top) / 2));
        }
        let x = if a.facing_right { h.right.min(hurt.right) } else { h.left.max(hurt.left) };
        return (sub(x), sub((h.bottom + h.top) / 2));
    }
    let x = if a.pos.x < d.pos.x { hurt.left } else { hurt.right };
    (sub(x) , sub(hurt.bottom + hurt.height() * 6 / 10))
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

fn draw_fx(v: &View, fx: &Fx, t: f32) {
    let c = v.world(fx.x, fx.y);
    let fade = 1.0 - t;
    let dir = if fx.facing_right { 1.0 } else { -1.0 };
    match fx.kind {
        FxKind::HitSpark { heavy, body } => {
            let (rays, matter, writing) = match body {
                CharacterId::Kogan => (10, COPPER, CYAN),
                CharacterId::Raya => (8, CYAN, LINEN),
            };
            let scale = if heavy { 1.5 } else { 1.0 };
            // Rays of the attacker's matter.
            for k in 0..rays {
                let ang = k as f32 / rays as f32 * std::f32::consts::TAU + 0.3 * dir;
                let r0 = 4.0 * scale + 22.0 * scale * ease_out(t);
                let r1 = r0 + (14.0 * scale) * fade;
                v.line(
                    c.x + ang.cos() * r0,
                    c.y + ang.sin() * r0,
                    c.x + ang.cos() * r1,
                    c.y + ang.sin() * r1,
                    (3.0 - 2.0 * t) * scale.max(1.0),
                    Color { a: fade, ..matter },
                );
            }
            // Written light: a hexagon opening.
            v.poly_lines(c.x, c.y, 6, (6.0 + 34.0 * ease_out(t)) * scale, 30.0, 2.0, Color { a: 0.8 * fade, ..writing });
            // Core.
            v.circle(c.x, c.y, (10.0 * scale) * fade, Color { a: fade, ..LINEN });
            if heavy {
                // The cut: a streak through the point of contact.
                let ang = -0.55 * dir;
                let len = 26.0 + 30.0 * ease_out(t);
                v.line(
                    c.x - ang.cos() * len,
                    c.y - ang.sin() * len,
                    c.x + ang.cos() * len,
                    c.y + ang.sin() * len,
                    5.0 * fade,
                    Color { a: fade, ..WHITE },
                );
            }
        }
        FxKind::BlockSpark => {
            v.poly_lines(c.x, c.y, 6, 5.0 + 18.0 * ease_out(t), 0.0, 2.5, Color { a: fade, ..COPPER });
            for k in 0..6 {
                let ang = k as f32 / 6.0 * std::f32::consts::TAU;
                let r = 8.0 + 12.0 * ease_out(t);
                v.line(c.x + ang.cos() * r, c.y + ang.sin() * r, c.x + ang.cos() * (r + 5.0), c.y + ang.sin() * (r + 5.0), 2.0, Color { a: fade, ..LINEN });
            }
            v.circle(c.x, c.y, 5.0 * fade, Color { a: fade, ..LINEN });
        }
        FxKind::ThrowImpact => {
            dust(v, c, t, true);
            v.poly_lines(c.x, c.y - 30.0, 6, 10.0 + 40.0 * ease_out(t), 0.0, 3.0, Color { a: fade, ..COPPER });
        }
        FxKind::Tech => {
            v.circle_lines(c.x, c.y, 6.0 + 30.0 * ease_out(t), 2.0, Color { a: fade, ..LINEN });
            v.circle(c.x, c.y, 6.0 * fade, Color { a: fade, ..LINEN });
        }
        FxKind::Dust { big } => dust(v, c, t, big),
        FxKind::RcRing => {
            v.poly_lines(c.x, c.y, 6, 10.0 + 70.0 * ease_out(t), 30.0, 3.0, Color { a: fade, ..CYAN });
            v.poly_lines(c.x, c.y, 6, 4.0 + 40.0 * ease_out(t), 0.0, 2.0, Color { a: fade, ..LINEN });
            for k in 0..6 {
                let ang = k as f32 / 6.0 * std::f32::consts::TAU + 0.5;
                let r = 20.0 + 60.0 * ease_out(t);
                v.poly(c.x + ang.cos() * r, c.y + ang.sin() * r, 6, 5.0 * fade, 0.0, Color { a: fade, ..CYAN });
            }
            v.circle(c.x, c.y, 14.0 * fade, Color { a: 0.8 * fade, ..WHITE });
        }
        FxKind::Feint => {
            for k in 0..5 {
                let ang = -1.2 + k as f32 * 0.6;
                let r = 12.0 + 24.0 * ease_out(t);
                v.poly(c.x + ang.cos() * r * dir, c.y - ang.sin().abs() * r, 6, 3.5 * fade, 0.0, Color { a: fade, ..COPPER });
            }
        }
        FxKind::ExRing => {
            v.poly_lines(c.x, c.y, 6, 8.0 + 44.0 * ease_out(t), 30.0, 2.5, Color { a: fade, ..CYAN });
            v.circle(c.x, c.y, 10.0 * fade, Color { a: 0.7 * fade, ..CYAN });
        }
        FxKind::Muzzle | FxKind::AirMuzzle => {
            v.circle(c.x, c.y, 12.0 * fade + 4.0, Color { a: fade, ..LINEN });
            for k in 0..3 {
                let ang = (k as f32 - 1.0) * 0.35
                    + if fx.kind == FxKind::AirMuzzle { std::f32::consts::FRAC_PI_4 } else { 0.0 };
                let len = 26.0 + 14.0 * t;
                v.line(c.x, c.y, c.x + dir * ang.cos() * len, c.y + ang.sin() * len, 3.0, Color { a: fade, ..CYAN });
            }
        }
        FxKind::Cast => {
            v.poly_lines(c.x, c.y, 6, 4.0 + 26.0 * ease_out(t), t * 60.0, 2.0, Color { a: fade, ..CYAN });
            v.poly_lines(c.x, c.y, 3, 3.0 + 14.0 * ease_out(t), -t * 90.0, 1.5, Color { a: fade, ..LINEN });
        }
        FxKind::DiscGuard => {
            let orange = Color::new(1.0, 0.62, 0.22, 1.0);
            v.circle_lines(c.x, c.y, 18.0 + 36.0 * ease_out(t), 3.0, Color { a: fade, ..orange });
            v.circle(c.x, c.y, 16.0 * fade, Color { a: 0.6 * fade, ..LINEN });
        }
        FxKind::Clash => {
            for k in 0..8 {
                let ang = k as f32 / 8.0 * std::f32::consts::TAU;
                let r = 6.0 + 28.0 * ease_out(t);
                v.line(c.x, c.y, c.x + ang.cos() * r, c.y + ang.sin() * r, 2.0, Color { a: fade, ..WHITE });
            }
            v.circle(c.x, c.y, 10.0 * fade, Color { a: fade, ..CYAN });
        }
        FxKind::Armed => {
            v.poly_lines(c.x, c.y, 6, 8.0 + 26.0 * ease_out(t), 0.0, 2.0, Color { a: fade, ..CYAN });
        }
        FxKind::Blast => {
            v.circle(c.x, c.y, 10.0 + 50.0 * ease_out(t), Color { a: 0.35 * fade, ..CYAN });
            v.circle_lines(c.x, c.y, 12.0 + 60.0 * ease_out(t), 3.0, Color { a: fade, ..WHITE });
            for k in 0..7 {
                let ang = k as f32 / 7.0 * std::f32::consts::TAU + 0.2;
                let r = 20.0 + 50.0 * ease_out(t);
                v.poly(c.x + ang.cos() * r, c.y + ang.sin() * r, 3, 6.0 * fade, ang.to_degrees(), Color { a: fade, ..CYAN });
            }
        }
        FxKind::Ko => {
            v.circle_lines(c.x, c.y, 20.0 + 160.0 * ease_out(t), 4.0 * fade + 1.0, Color { a: fade, ..LINEN });
            v.poly_lines(c.x, c.y, 6, 10.0 + 90.0 * ease_out(t), 30.0, 3.0, Color { a: fade, ..CYAN });
        }
    }
}

fn dust(v: &View, c: Vec2, t: f32, big: bool) {
    let fade = 1.0 - t;
    let n = if big { 6 } else { 3 };
    let spread = if big { 34.0 } else { 16.0 };
    for k in 0..n {
        let side = if k % 2 == 0 { 1.0 } else { -1.0 };
        let lane = (k / 2) as f32 + 1.0;
        let x = c.x + side * (6.0 + spread * ease_out(t)) * lane / n as f32 * 2.0;
        let y = c.y - (3.0 + 12.0 * ease_out(t)) * lane / n as f32 * 2.0;
        let r = (if big { 9.0 } else { 5.0 }) * (0.4 + 0.6 * fade);
        v.circle(x, y, r, Color::new(0.80, 0.72, 0.60, 0.35 * fade));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::{Chord, InputFrame};

    #[test]
    fn body_flash_holds_through_hitstop_then_expires_on_world_ticks() {
        for body in [CharacterId::Kogan, CharacterId::Raya] {
            for blocked in [false, true] {
                let opponent = if body == CharacterId::Kogan { CharacterId::Raya } else { CharacterId::Kogan };
                let mut w = World::new(body, opponent);
                w.fighters[0].pos.x = aeon_sim::px(300);
                w.fighters[1].pos.x = aeon_sim::px(340);
                let mut effects = Effects::default();
                for tick in 0..12 {
                    let a = if tick == 0 { InputFrame::press(aeon_sim::Btn::P) } else { InputFrame::default() };
                    let b = InputFrame::dir(if blocked { 4 } else { 5 });
                    w.tick(a, b);
                    effects.after_tick(&w);
                    if w.hitstop > 0 { break; }
                }
                assert!(w.hitstop > 0, "legal jab must connect");
                let held = effects.flash[1];
                assert!(held.2 > 0);
                let frozen_frame = w.frame;
                for _ in 0..w.hitstop {
                    w.tick(InputFrame::default(), InputFrame::default());
                    effects.after_tick(&w);
                    assert_eq!(w.frame, frozen_frame);
                    assert_eq!(effects.flash[1], held, "{body:?} block={blocked}: frozen flash must hold");
                }
                for elapsed in 1..=held.2 {
                    w.tick(InputFrame::default(), InputFrame::default());
                    effects.after_tick(&w);
                    assert_eq!(effects.flash[1].2, held.2 - elapsed);
                }
                assert_eq!(effects.flash(1).0, 0.0);
                effects.reset();
                assert!(effects.fx.is_empty());
                assert_eq!(effects.flash(0).0, 0.0);
                assert_eq!(effects.flash(1).0, 0.0);
            }
        }
    }

    #[test]
    fn frozen_cast_and_landing_spawn_once() {
        for body in [CharacterId::Kogan, CharacterId::Raya] {
            let mut w = World::new(body, body);
            let mut effects = Effects::default();
            w.fighters[0].start_move(MoveId::ShotA);
            let startup = body.data().move_def(MoveId::ShotA).unwrap().startup as u16;
            for _ in 0..startup {
                w.tick(InputFrame::default(), InputFrame::default());
                effects.after_tick(&w);
            }
            assert!(matches!(w.fighters[0].action, Action::Attack { frame, .. } if frame == startup));
            let kind = if body == CharacterId::Kogan { FxKind::Muzzle } else { FxKind::Cast };
            assert_eq!(effects.fx.iter().filter(|fx| fx.kind == kind).count(), 1);
            let frozen_frame = w.frame;
            w.hitstop = 6;
            for _ in 0..6 {
                w.tick(InputFrame::default(), InputFrame::default());
                effects.after_tick(&w);
            }
            assert_eq!(w.frame, frozen_frame);
            assert_eq!(effects.fx.iter().filter(|fx| fx.kind == kind).count(), 1,
                "{body:?}: a frozen release must not emit another cast");
        }

        let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
        let mut effects = Effects::default();
        w.fighters[0].action = Action::Landing { frame: 0, total: 2 };
        effects.after_tick(&w);
        w.hitstop = 6;
        for _ in 0..6 {
            w.tick(InputFrame::default(), InputFrame::default());
            effects.after_tick(&w);
        }
        assert_eq!(effects.fx.iter().filter(|fx| matches!(fx.kind, FxKind::Dust { .. })).count(), 1);
    }

    #[test]
    fn air_muzzle_uses_the_existing_spawn_and_freezes_for_both_facings() {
        for facing in [false, true] {
            let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
            let f = &mut w.fighters[0];
            f.facing_right = facing;
            f.airborne = true;
            f.pos.y = aeon_sim::px(200);
            f.start_move(MoveId::AirShot);
            let mut effects = Effects::default();
            for _ in 0..8 {
                w.tick(InputFrame::default(), InputFrame::default());
                effects.after_tick(&w);
            }
            let f = &w.fighters[0];
            let muzzle = effects.fx.iter().find(|fx| fx.kind == FxKind::AirMuzzle).unwrap();
            assert_eq!(muzzle.facing_right, facing);
            assert_eq!(muzzle.x, sub(f.pos.x) + if facing { 20.0 } else { -20.0 });
            assert_eq!(muzzle.y, sub(f.pos.y) + 48.0);
            w.hitstop = 6;
            for _ in 0..6 {
                w.tick(InputFrame::default(), InputFrame::default());
                effects.after_tick(&w);
            }
            assert_eq!(effects.fx.iter().filter(|fx| fx.kind == FxKind::AirMuzzle).count(), 1);
        }
    }

    #[test]
    fn roman_cancel_on_an_unchanged_world_frame_still_spawns_once() {
        let mut w = World::new(CharacterId::Kogan, CharacterId::Raya);
        let mut effects = Effects::default();
        w.fighters[0].meter = 1000;
        w.fighters[0].start_move(MoveId::StHS);
        effects.after_tick(&w);
        let frame = w.frame;
        w.tick(InputFrame::chord(Chord::RomanCancel), InputFrame::default());
        effects.after_tick(&w);
        assert_eq!(w.frame, frame);
        assert!(w.rc_freeze > 0);
        let held = effects.flash[0];
        assert!(held.2 > 0);
        for _ in 0..w.rc_freeze {
            w.tick(InputFrame::default(), InputFrame::default());
            effects.after_tick(&w);
        }
        assert_eq!(effects.flash[0], held, "RC body flash must hold with its ring");
        assert_eq!(effects.fx.iter().filter(|fx| fx.kind == FxKind::RcRing).count(), 1);
    }
}
