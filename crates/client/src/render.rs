//! Drawing. Everything goes through a virtual 1280×800 canvas letterboxed
//! into the window, so castle, a 1080p mini, and a 1280×800 Deck read the
//! same. The sim is in subpixels; `View` turns them into canvas pixels.

use aeon_sim::{
    Aabb, Action, CharacterId, Fighter, Match, Phase, ProjectileKind, ShotState, World, METER_MAX,
    STAGE_W, SUB,
};
use macroquad::prelude::*;

use crate::sprites::{pose_for, SpriteSet};

pub const VW: f32 = 1280.0;
pub const VH: f32 = 800.0;
pub const GROUND: f32 = 640.0;
/// World pixels → canvas pixels. Bodies land around a third of the canvas.
pub const WS: f32 = 2.2;

pub const COPPER: Color = Color::new(0.722, 0.451, 0.200, 1.0);
pub const COPPER_DIM: Color = Color::new(0.45, 0.28, 0.13, 1.0);
pub const CYAN: Color = Color::new(0.35, 0.90, 1.0, 1.0);
pub const CYAN_DIM: Color = Color::new(0.16, 0.40, 0.48, 1.0);
pub const LINEN: Color = Color::new(0.91, 0.86, 0.77, 1.0);
pub const VAULT: Color = Color::new(0.045, 0.040, 0.055, 1.0);
pub const INK: Color = Color::new(0.75, 0.72, 0.66, 1.0);

pub fn sub_to_px(v: i32) -> f32 {
    v as f32 / SUB as f32
}

pub struct View {
    pub scale: f32,
    pub ox: f32,
    pub oy: f32,
    /// Camera centre in world pixels.
    pub cam_x: f32,
}

impl View {
    pub fn fit() -> Self {
        let sw = screen_width();
        let sh = screen_height();
        let scale = (sw / VW).min(sh / VH);
        Self {
            scale,
            ox: (sw - VW * scale) / 2.0,
            oy: (sh - VH * scale) / 2.0,
            cam_x: sub_to_px(STAGE_W) / 2.0,
        }
    }

    pub fn follow(&mut self, w: &World) {
        let target = sub_to_px(w.camera_x());
        let half_view = VW / WS / 2.0;
        let stage = sub_to_px(STAGE_W);
        self.cam_x = target.clamp(half_view.min(stage / 2.0), (stage - half_view).max(stage / 2.0));
    }

    pub fn sx(&self, x: f32) -> f32 {
        self.ox + x * self.scale
    }

    pub fn sy(&self, y: f32) -> f32 {
        self.oy + y * self.scale
    }

    /// World pixel position → canvas position.
    pub fn world(&self, wx: f32, wy: f32) -> Vec2 {
        vec2(VW / 2.0 + (wx - self.cam_x) * WS, GROUND - wy * WS)
    }

    pub fn rect(&self, x: f32, y: f32, w: f32, h: f32, c: Color) {
        draw_rectangle(self.sx(x), self.sy(y), w * self.scale, h * self.scale, c);
    }

    pub fn rect_lines(&self, x: f32, y: f32, w: f32, h: f32, t: f32, c: Color) {
        draw_rectangle_lines(self.sx(x), self.sy(y), w * self.scale, h * self.scale, t * self.scale, c);
    }

    pub fn line(&self, x1: f32, y1: f32, x2: f32, y2: f32, t: f32, c: Color) {
        draw_line(self.sx(x1), self.sy(y1), self.sx(x2), self.sy(y2), t * self.scale, c);
    }

    pub fn circle(&self, x: f32, y: f32, r: f32, c: Color) {
        draw_circle(self.sx(x), self.sy(y), r * self.scale, c);
    }

    pub fn circle_lines(&self, x: f32, y: f32, r: f32, t: f32, c: Color) {
        draw_circle_lines(self.sx(x), self.sy(y), r * self.scale, t * self.scale, c);
    }

    pub fn poly(&self, x: f32, y: f32, sides: u8, r: f32, rot: f32, c: Color) {
        draw_poly(self.sx(x), self.sy(y), sides, r * self.scale, rot, c);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn poly_lines(&self, x: f32, y: f32, sides: u8, r: f32, rot: f32, t: f32, c: Color) {
        draw_poly_lines(self.sx(x), self.sy(y), sides, r * self.scale, rot, t * self.scale, c);
    }

    pub fn text(&self, s: &str, x: f32, y: f32, size: f32, c: Color) {
        draw_text(s, self.sx(x), self.sy(y), size * self.scale, c);
    }

    pub fn text_w(&self, s: &str, size: f32) -> f32 {
        measure_text(s, None, (size * self.scale) as u16, 1.0).width / self.scale
    }

    pub fn text_center(&self, s: &str, cx: f32, y: f32, size: f32, c: Color) {
        let w = self.text_w(s, size);
        self.text(s, cx - w / 2.0, y, size, c);
    }

    pub fn text_right(&self, s: &str, rx: f32, y: f32, size: f32, c: Color) {
        let w = self.text_w(s, size);
        self.text(s, rx - w, y, size, c);
    }

    pub fn aabb(&self, b: &Aabb, c: Color, fill: bool, t: f32) {
        let tl = self.world(sub_to_px(b.left), sub_to_px(b.top));
        let w = sub_to_px(b.right - b.left) * WS;
        let h = sub_to_px(b.top - b.bottom) * WS;
        if fill {
            self.rect(tl.x, tl.y, w, h, c);
        }
        self.rect_lines(tl.x, tl.y, w, h, t, Color { a: 1.0, ..c });
    }

    /// Letterbox bars over anything that spilled outside the canvas.
    pub fn bars(&self) {
        let sw = screen_width();
        let sh = screen_height();
        if self.ox > 0.0 {
            draw_rectangle(0.0, 0.0, self.ox, sh, BLACK);
            draw_rectangle(sw - self.ox, 0.0, self.ox, sh, BLACK);
        }
        if self.oy > 0.0 {
            draw_rectangle(0.0, 0.0, sw, self.oy, BLACK);
            draw_rectangle(0.0, sh - self.oy, sw, self.oy, BLACK);
        }
    }
}

pub struct Stage {
    pub backdrop: Option<Texture2D>,
}

impl Stage {
    pub async fn load() -> Self {
        let backdrop = load_texture("assets/stage/sanctum.png").await.ok();
        if let Some(t) = &backdrop {
            t.set_filter(FilterMode::Linear);
        }
        Self { backdrop }
    }

    pub fn draw(&self, v: &View, frame: u32) {
        clear_background(BLACK);
        v.rect(0.0, 0.0, VW, VH, VAULT);
        let stage_w = sub_to_px(STAGE_W);
        // Parallax: the vault drifts at a third of camera speed.
        let par = (v.cam_x - stage_w / 2.0) * 0.35 * WS;
        if let Some(t) = &self.backdrop {
            // Cover the canvas above the floor; overscan so parallax never
            // shows an edge.
            let h = GROUND + 40.0;
            let w = h * (t.width() / t.height()).max(VW / h);
            let x = (VW - w) / 2.0 - par;
            draw_texture_ex(
                t,
                v.sx(x),
                v.sy(-40.0),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(w * v.scale, h * v.scale)),
                    ..Default::default()
                },
            );
            v.rect(0.0, 0.0, VW, GROUND, Color::new(0.0, 0.0, 0.0, 0.28));
        } else {
            self.honeycomb(v, par, frame);
        }
        // Floor: near-black stone with a written light seam.
        v.rect(0.0, GROUND, VW, VH - GROUND, Color::new(0.06, 0.055, 0.065, 1.0));
        v.rect(0.0, GROUND, VW, 3.0, CYAN_DIM);
        v.rect(0.0, GROUND + 3.0, VW, 1.0, Color::new(0.35, 0.9, 1.0, 0.25));
        // Floor tiles in copper, marching with the camera.
        let tile = 64.0 * WS;
        let off = ((v.cam_x * WS) % tile + tile) % tile;
        let mut x = -off;
        while x < VW + tile {
            v.line(x, GROUND + 4.0, x, VH, 1.0, Color::new(0.72, 0.45, 0.2, 0.10));
            x += tile;
        }
        for i in 1..4 {
            let y = GROUND + 4.0 + i as f32 * 42.0;
            v.line(0.0, y, VW, y, 1.0, Color::new(0.72, 0.45, 0.2, 0.06));
        }
        // Stage posts: the threshold at either end.
        for wx in [0.0, stage_w] {
            let p = v.world(wx, 0.0);
            v.rect(p.x - 10.0, GROUND - 420.0, 20.0, 420.0, Color::new(0.13, 0.10, 0.09, 1.0));
            v.rect(p.x - 3.0, GROUND - 420.0, 6.0, 420.0, Color::new(0.35, 0.9, 1.0, 0.25));
        }
    }

    fn honeycomb(&self, v: &View, par: f32, frame: u32) {
        let r = 46.0;
        let dx = r * 1.732;
        let dy = r * 1.5;
        let pulse = 0.5 + 0.5 * ((frame as f32) * 0.02).sin();
        let mut row = 0;
        let mut y = -r;
        while y < GROUND {
            let mut x = if row % 2 == 0 { -par % dx } else { -par % dx + dx / 2.0 };
            x -= dx;
            while x < VW + dx {
                let depth = (y / GROUND).clamp(0.0, 1.0);
                let a = 0.05 + 0.12 * (1.0 - depth);
                v.poly_lines(x, y, 6, r - 3.0, 30.0, 1.5, Color::new(0.72, 0.45, 0.2, a));
                if (row * 7 + (x / dx) as i32) % 9 == 0 {
                    v.poly(x, y, 6, r * 0.35, 30.0, Color::new(0.35, 0.9, 1.0, 0.04 + 0.05 * pulse));
                }
                x += dx;
            }
            y += dy;
            row += 1;
        }
        // Two moons through the vault.
        v.circle(VW * 0.78 - par * 0.3, 120.0, 34.0, Color::new(0.85, 0.80, 0.72, 0.10));
        v.circle(VW * 0.86 - par * 0.3, 168.0, 18.0, Color::new(0.35, 0.9, 1.0, 0.10));
    }
}

pub struct FighterDraw<'a> {
    pub sprites: Option<&'a SpriteSet>,
    pub show_boxes: bool,
}

pub fn draw_fighter(v: &View, w: &World, i: usize, fd: &FighterDraw) {
    let f = &w.fighters[i];
    let d = f.data();
    let feet = v.world(sub_to_px(f.pos.x), sub_to_px(f.pos.y));
    let stand_h = sub_to_px(d.stand_h) * WS;
    let facing = if f.facing_right { 1.0 } else { -1.0 };

    // Shadow.
    let sh = v.world(sub_to_px(f.pos.x), 0.0);
    draw_ellipse(
        v.sx(sh.x),
        v.sy(sh.y + 4.0),
        44.0 * v.scale,
        9.0 * v.scale,
        0.0,
        Color::new(0.0, 0.0, 0.0, 0.5),
    );

    let hit_tint = match f.action {
        Action::Hit { .. } | Action::Thrown { .. } => Color::new(1.0, 0.85, 0.8, 1.0),
        _ => WHITE,
    };

    let sprite = fd.sprites.and_then(|s| s.get(pose_for(f)));
    match sprite {
        Some(tex) => {
            // Constant canvas: every pose is authored on the same frame with
            // feet at the bottom, so one size holds across states.
            let h = stand_h * 1.55;
            let wdt = h * (tex.width() / tex.height());
            let x = feet.x - wdt / 2.0;
            let y = feet.y - h * 0.94;
            // Smear on active frames / glide: a ghost trailing behind.
            let ghost = match &f.action {
                Action::Run => Some((-18.0 * facing, 0.30)),
                Action::Attack { move_id, frame, .. } => {
                    let mv = d.move_def(*move_id);
                    mv.filter(|m| m.is_active(*frame)).map(|_| (-10.0 * facing, 0.35))
                }
                Action::Jump { hop: true, .. } => Some((-8.0 * facing, 0.22)),
                _ => None,
            };
            if let Some((dx, a)) = ghost {
                draw_texture_ex(
                    tex,
                    v.sx(x + dx),
                    v.sy(y),
                    Color::new(0.72, 0.45, 0.2, a),
                    DrawTextureParams {
                        dest_size: Some(vec2(wdt * v.scale, h * v.scale)),
                        flip_x: !f.facing_right,
                        ..Default::default()
                    },
                );
            }
            draw_texture_ex(
                tex,
                v.sx(x),
                v.sy(y),
                hit_tint,
                DrawTextureParams {
                    dest_size: Some(vec2(wdt * v.scale, h * v.scale)),
                    flip_x: !f.facing_right,
                    ..Default::default()
                },
            );
        }
        None => {
            // Box body: identity colour, facing tick.
            let push = f.pushbox();
            let bw = sub_to_px(push.right - push.left) * WS;
            let bh = sub_to_px(push.top - push.bottom) * WS;
            let color = argb(d.color);
            v.rect(feet.x - bw / 2.0, feet.y - bh, bw, bh, Color { a: 0.9, ..color } );
            v.rect(feet.x + facing * (bw / 2.0 - 6.0) - 3.0, feet.y - bh + 14.0, 6.0, 12.0, LINEN);
        }
    }

    if fd.show_boxes {
        let boxes = w.debug_boxes(i);
        // Aura: copper, outline only, dashed feel via thin double line —
        // visibly not a collision box.
        if let Some(a) = boxes.aura {
            v.aabb(&a, Color::new(0.72, 0.45, 0.2, 0.0), false, 1.0);
            let tl = v.world(sub_to_px(a.left), sub_to_px(a.top));
            v.text("aura", tl.x + 4.0, tl.y + 14.0, 14.0, Color::new(0.72, 0.45, 0.2, 0.9));
        }
        v.aabb(&boxes.push, Color::new(0.3, 0.85, 0.35, 0.10), true, 1.0);
        for h in &boxes.hurt {
            v.aabb(h, Color::new(0.35, 0.8, 0.9, 0.18), true, 1.5);
        }
        for h in &boxes.hit {
            v.aabb(h, Color::new(0.95, 0.25, 0.25, 0.30), true, 1.5);
        }
        if f.throw_invuln() || f.strike_invuln() {
            let tl = v.world(sub_to_px(boxes.push.left), sub_to_px(boxes.push.top));
            v.text("INVULN", tl.x, tl.y - 4.0, 14.0, GOLD);
        }
    }

    // State label above the head (training) is drawn by the HUD.
}

pub fn draw_projectiles(v: &View, w: &World, show_boxes: bool, frame: u32) {
    for p in &w.projectiles {
        let b = p.hitbox();
        let c = v.world(sub_to_px((b.left + b.right) / 2), sub_to_px((b.bottom + b.top) / 2));
        let hw = sub_to_px(b.right - b.left) * WS / 2.0;
        let hh = sub_to_px(b.top - b.bottom) * WS / 2.0;
        let t = frame as f32 * 0.15;
        match p.def.kind {
            ProjectileKind::Revolver | ProjectileKind::AirShot => {
                v.circle(c.x, c.y, hh * 0.45, Color::new(1.0, 0.95, 0.7, 0.9));
                v.circle(c.x, c.y, hh * 0.9, Color::new(0.35, 0.9, 1.0, 0.35));
                let dir = if p.vel.x >= 0 { -1.0 } else { 1.0 };
                v.line(c.x, c.y, c.x + dir * hw * 2.5, c.y - p.vel.y as f32 / SUB as f32 * WS * 2.0, 3.0, Color::new(0.35, 0.9, 1.0, 0.5));
            }
            ProjectileKind::Wave => {
                let dir = if p.vel.x >= 0 { 1.0 } else { -1.0 };
                for k in 0..3 {
                    let a = 0.75 - k as f32 * 0.22;
                    let x = c.x - dir * k as f32 * 7.0;
                    v.line(x, c.y - hh, x + dir * 10.0, c.y, 4.0 - k as f32, Color::new(0.35, 0.9, 1.0, a));
                    v.line(x + dir * 10.0, c.y, x, c.y + hh, 4.0 - k as f32, Color::new(0.35, 0.9, 1.0, a));
                }
            }
            ProjectileKind::Glyph => {
                let pulse = 0.7 + 0.3 * (t * 2.0).sin();
                v.poly_lines(c.x, c.y, 6, hh * 0.9, t * 20.0, 2.5, Color::new(0.35, 0.9, 1.0, pulse));
                v.poly_lines(c.x, c.y, 3, hh * 0.5, -t * 30.0, 2.0, Color::new(0.9, 0.95, 1.0, pulse));
                v.circle(c.x, c.y, hh * 0.18, Color::new(0.9, 0.98, 1.0, pulse));
            }
            ProjectileKind::Crystal => {
                let (col, glow) = match p.state {
                    ShotState::Flying => (CYAN_DIM, 0.2),
                    ShotState::Planted { armed: false, .. } => (CYAN_DIM, 0.35),
                    ShotState::Planted { armed: true, .. } => (CYAN, 0.6 + 0.4 * (t * 3.0).sin().abs()),
                    ShotState::Detonating { .. } => (WHITE, 1.0),
                    ShotState::Hanging => (CYAN, 0.5),
                };
                if matches!(p.state, ShotState::Detonating { .. }) {
                    v.circle(c.x, c.y, hh, Color::new(0.35, 0.9, 1.0, 0.45));
                    v.circle_lines(c.x, c.y, hh * 1.1, 3.0, Color::new(0.9, 0.98, 1.0, 0.9));
                } else {
                    let base = v.world(sub_to_px(p.pos.x + (b.right - b.left) / 2), sub_to_px(p.pos.y));
                    // A shard: tall kite.
                    let hgt = hh * 2.0;
                    let wd = hw * 1.2;
                    draw_triangle(
                        vec2(v.sx(base.x), v.sy(base.y - hgt)),
                        vec2(v.sx(base.x - wd), v.sy(base.y - hgt * 0.35)),
                        vec2(v.sx(base.x + wd), v.sy(base.y - hgt * 0.35)),
                        Color { a: 0.9, ..col },
                    );
                    draw_triangle(
                        vec2(v.sx(base.x - wd), v.sy(base.y - hgt * 0.35)),
                        vec2(v.sx(base.x + wd), v.sy(base.y - hgt * 0.35)),
                        vec2(v.sx(base.x), v.sy(base.y)),
                        Color { a: 0.9, ..col },
                    );
                    v.circle(base.x, base.y - hgt * 0.4, wd * 0.5, Color::new(0.9, 0.98, 1.0, glow));
                }
            }
        }
        if show_boxes {
            let col = if p.live() {
                Color::new(0.95, 0.65, 0.25, 0.35)
            } else {
                Color::new(0.5, 0.5, 0.5, 0.15)
            };
            v.aabb(&b, col, true, 1.0);
        }
    }
}

pub fn argb(rgb: u32) -> Color {
    Color::from_rgba(((rgb >> 16) & 0xFF) as u8, ((rgb >> 8) & 0xFF) as u8, (rgb & 0xFF) as u8, 255)
}

// ------------------------------------------------------------------- HUD

pub struct HudOpts {
    pub wins: Option<[u8; 2]>,
    pub round: Option<u8>,
}

pub fn draw_hud(v: &View, w: &World, o: &HudOpts) {
    let p1 = &w.fighters[0];
    let p2 = &w.fighters[1];
    let bar_w = 470.0;
    let y = 42.0;
    health_bar(v, 60.0, y, bar_w, p1, false);
    health_bar(v, VW - 60.0 - bar_w, y, bar_w, p2, true);
    v.text(p1.id.name(), 62.0, y + 48.0, 24.0, LINEN);
    v.text_right(p2.id.name(), VW - 62.0, y + 48.0, 24.0, LINEN);

    // Round pips.
    if let Some(wins) = o.wins {
        for i in 0..2 {
            let lit = wins[0] > i as u8;
            let x = 62.0 + 130.0 + i as f32 * 20.0;
            pip(v, x, y + 40.0, lit);
            let lit = wins[1] > i as u8;
            let x = VW - 62.0 - 130.0 - i as f32 * 20.0;
            pip(v, x, y + 40.0, lit);
        }
    }

    // Timer in a hex.
    v.poly(VW / 2.0, y + 12.0, 6, 40.0, 0.0, Color::new(0.09, 0.08, 0.10, 1.0));
    v.poly_lines(VW / 2.0, y + 12.0, 6, 40.0, 0.0, 2.0, COPPER);
    let sec = w.time_left.div_ceil(60);
    v.text_center(&format!("{sec:02}"), VW / 2.0, y + 26.0, 40.0, LINEN);
    if let Some(r) = o.round {
        v.text_center(&format!("ROUND {r}"), VW / 2.0, y + 72.0, 16.0, INK);
    }

    // Super bars with the 250 ticks.
    super_bar(v, 60.0, VH - 54.0, 300.0, p1.meter, false);
    super_bar(v, VW - 60.0 - 300.0, VH - 54.0, 300.0, p2.meter, true);
    // Character gauges: cylinder / cluster.
    gauge(v, 60.0 + 300.0 + 40.0, VH - 46.0, p1, false);
    gauge(v, VW - 60.0 - 300.0 - 40.0, VH - 46.0, p2, true);

    if p1.combo >= 2 {
        v.text(&format!("{} HIT", p1.combo), 62.0, 150.0, 34.0, GOLD);
    }
    if p2.combo >= 2 {
        v.text_right(&format!("{} HIT", p2.combo), VW - 62.0, 150.0, 34.0, GOLD);
    }
}

fn pip(v: &View, x: f32, y: f32, lit: bool) {
    if lit {
        v.poly(x, y, 6, 7.0, 0.0, CYAN);
    } else {
        v.poly_lines(x, y, 6, 7.0, 0.0, 1.5, COPPER_DIM);
    }
}

fn health_bar(v: &View, x: f32, y: f32, w: f32, f: &Fighter, rtl: bool) {
    let h = 22.0;
    v.rect(x - 2.0, y - 2.0, w + 4.0, h + 4.0, COPPER_DIM);
    v.rect(x, y, w, h, Color::new(0.10, 0.06, 0.06, 1.0));
    let pct = (f.health as f32 / f.data().max_health as f32).clamp(0.0, 1.0);
    let fw = w * pct;
    let fx = if rtl { x + w - fw } else { x };
    let col = if pct > 0.3 { LINEN } else { Color::new(0.95, 0.55, 0.35, 1.0) };
    v.rect(fx, y, fw, h, col);
    v.rect(fx, y, fw, 3.0, Color::new(1.0, 1.0, 1.0, 0.35));
    // Copper end caps.
    v.rect(if rtl { x + w } else { x - 8.0 }, y - 6.0, 8.0, h + 12.0, COPPER);
}

fn super_bar(v: &View, x: f32, y: f32, w: f32, meter: i32, rtl: bool) {
    let h = 10.0;
    v.rect(x, y, w, h, Color::new(0.10, 0.09, 0.10, 1.0));
    let pct = meter as f32 / METER_MAX as f32;
    let fw = w * pct;
    let fx = if rtl { x + w - fw } else { x };
    let col = if meter >= METER_MAX { GOLD } else { Color::new(0.95, 0.80, 0.35, 1.0) };
    v.rect(fx, y, fw, h, col);
    for k in 1..4 {
        let tx = x + w * k as f32 / 4.0;
        v.line(tx, y - 2.0, tx, y + h + 2.0, 1.5, COPPER);
    }
    v.rect_lines(x, y, w, h, 1.0, COPPER_DIM);
    let label = if meter >= METER_MAX { "SUPER" } else if meter >= 250 { "RC" } else { "" };
    if rtl {
        v.text_right(label, x + w, y - 4.0, 14.0, GOLD);
    } else {
        v.text(label, x, y - 4.0, 14.0, GOLD);
    }
}

fn gauge(v: &View, x: f32, y: f32, f: &Fighter, rtl: bool) {
    let g = f.data().gauge;
    match f.id {
        CharacterId::Kogan => {
            // A cylinder: six chambers in a ring, lit while loaded.
            let cx = if rtl { x - 26.0 } else { x + 26.0 };
            v.circle_lines(cx, y, 26.0, 2.0, COPPER);
            v.circle(cx, y, 6.0, COPPER_DIM);
            for k in 0..g.max {
                let ang = k as f32 / g.max as f32 * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
                let px_ = cx + ang.cos() * 16.0;
                let py_ = y + ang.sin() * 16.0;
                if k < f.gauge {
                    v.circle(px_, py_, 5.0, Color::new(0.95, 0.85, 0.55, 1.0));
                } else {
                    v.circle_lines(px_, py_, 5.0, 1.5, COPPER_DIM);
                }
            }
            let label = format!("{}/{}", f.gauge, g.max);
            if rtl {
                v.text_right(&label, cx - 34.0, y + 6.0, 16.0, INK);
            } else {
                v.text(&label, cx + 34.0, y + 6.0, 16.0, INK);
            }
        }
        CharacterId::Raya => {
            // A cluster: five shards that brighten as she consecrates.
            let n = 5;
            let per = g.max / n;
            for k in 0..n {
                let lit = f.gauge >= (k + 1) * per;
                let partial = ((f.gauge - k * per).clamp(0, per)) as f32 / per as f32;
                let sx_ = if rtl { x - 8.0 - k as f32 * 20.0 } else { x + 8.0 + k as f32 * 20.0 };
                let hgt = 22.0 + (k % 2) as f32 * 10.0;
                let c = if lit { CYAN } else { Color::new(0.16, 0.40, 0.48, 0.4 + 0.6 * partial) };
                draw_triangle(
                    vec2(v.sx(sx_), v.sy(y - hgt)),
                    vec2(v.sx(sx_ - 7.0), v.sy(y + 6.0)),
                    vec2(v.sx(sx_ + 7.0), v.sy(y + 6.0)),
                    c,
                );
            }
            let tier = f.buff_tier();
            let label = if tier > 0 { format!("CONSECRATED {}", "+".repeat(tier as usize)) } else { format!("{}", f.gauge) };
            if rtl {
                v.text_right(&label, x - 8.0 - n as f32 * 20.0 - 6.0, y + 6.0, 16.0, if tier > 0 { CYAN } else { INK });
            } else {
                v.text(&label, x + 8.0 + n as f32 * 20.0 + 6.0, y + 6.0, 16.0, if tier > 0 { CYAN } else { INK });
            }
        }
    }
}

pub fn draw_input(v: &View, x: f32, y: f32, inp: aeon_sim::InputFrame, rtl: bool) {
    // Numpad direction glyph and the 2×3 button grid.
    let x0 = if rtl { x - 150.0 } else { x };
    v.text(&format!("{}", inp.dir), x0, y + 22.0, 26.0, INK);
    let labels = [["P", "S", "HS"], ["K", "FL", "ST"]];
    let on = [
        [inp.buttons.p, inp.buttons.s, inp.buttons.hs],
        [inp.buttons.k, inp.buttons.fl, inp.buttons.st],
    ];
    for (r, row) in labels.iter().enumerate() {
        for (c, l) in row.iter().enumerate() {
            let bx = x0 + 30.0 + c as f32 * 38.0;
            let by = y + r as f32 * 22.0;
            let lit = on[r][c];
            v.rect(bx, by, 34.0, 20.0, if lit { COPPER } else { Color::new(0.12, 0.10, 0.10, 1.0) });
            v.text(l, bx + 6.0, by + 15.0, 14.0, if lit { BLACK } else { INK });
        }
    }
}

pub fn draw_match_overlay(v: &View, m: &Match, frame: u32) {
    match m.phase {
        Phase::Intro { frame: f } => {
            let a = if f < 40 { 1.0 } else { (60 - f) as f32 / 20.0 };
            let text = if f < 40 { format!("ROUND {}", m.round) } else { "FIGHT".to_string() };
            v.text_center(&text, VW / 2.0, VH / 2.0, 64.0, Color { a, ..LINEN });
        }
        Phase::RoundEnd { outcome, frame: f } => {
            let a = (f as f32 / 15.0).min(1.0);
            let text = match outcome {
                aeon_sim::RoundOutcome::Winner(i) => format!("{} TAKES THE ROUND", m.world.fighters[i].id.name()),
                aeon_sim::RoundOutcome::Draw => "DOUBLE KO".to_string(),
            };
            let ko = m.world.time_left == 0 && !matches!(outcome, aeon_sim::RoundOutcome::Draw)
                && m.world.fighters.iter().all(|f| f.health > 0);
            v.text_center(if ko { "TIME" } else { "K.O." }, VW / 2.0, VH / 2.0 - 30.0, 80.0, Color { a, ..GOLD });
            v.text_center(&text, VW / 2.0, VH / 2.0 + 30.0, 30.0, Color { a, ..LINEN });
        }
        Phase::MatchOver { winner } => {
            v.rect(0.0, 0.0, VW, VH, Color::new(0.0, 0.0, 0.0, 0.55));
            let pulse = 0.85 + 0.15 * (frame as f32 * 0.08).sin();
            v.text_center(
                &format!("{} WINS", m.world.fighters[winner].id.name()),
                VW / 2.0,
                VH / 2.0 - 20.0,
                72.0,
                Color { a: pulse, ..LINEN },
            );
            v.text_center("P / ENTER  rematch      FL / ESC  character select", VW / 2.0, VH / 2.0 + 40.0, 22.0, INK);
        }
        Phase::Fight => {}
    }
}
