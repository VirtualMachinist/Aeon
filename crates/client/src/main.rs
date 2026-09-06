//! AEON client: versus and training shells around the pure sim.
//! The sim is sacred; this crate is a window, a stick, and a sprite table.

mod anim;
mod fx;
mod input;
mod preview;
mod kit_preview;
mod render;
mod replay;
mod replay_preview;
mod sprites;
mod sequences;
mod defeat;
mod timing;

use aeon_sim::input::{Btn, Chord};
use aeon_sim::{Action, CharacterId, EventKind, InputFrame, Match, Phase, World};
use macroquad::prelude::*;

use anim::{History, LayerOpts};
use fx::Effects;
use input::{keyboard, merge, Bind, MenuEdges, Pads};
use render::{
    draw_fighter, draw_hud, draw_input, draw_match_overlay, draw_projectiles, FighterDraw, Flash,
    HudOpts, Stage, View, COPPER, COPPER_DIM, CYAN, INK, LINEN, VH, VW,
};
use replay::Replay;
use sprites::SpriteSet;
use timing::{FixedClock, InputLatch};

fn window_conf() -> Conf {
    Conf {
        window_title: "AEON".to_owned(),
        window_width: 1280,
        window_height: 800,
        high_dpi: !std::env::args().any(|a| a == "--capture-1x"),
        window_resizable: true,
        ..Default::default()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Menu {
    Versus,
    Training,
    Remap,
}

enum Mode {
    Title { cursor: Menu },
    Select { for_training: bool, p1: CharacterId, p2: CharacterId, locked: [bool; 2] },
    Versus { m: Match },
    Training(Training),
    Remap { step: usize, back_to: Box<Mode>, captured: Vec<Bind> },
}

struct Training {
    world: World,
    show_boxes: bool,
    show_help: bool,
    paused: bool,
    step_once: bool,
    log: Vec<String>,
    flash: Option<(String, Color, u8)>,
    recording: Replay,
    playback: Option<(Replay, usize)>,
    /// Frames until the dummy reads a wakeup ("delayed" oki practice).
    last_hash: u64,
}

impl Training {
    fn new(p1: CharacterId, p2: CharacterId) -> Self {
        let world = World::training(p1, p2);
        Self {
            recording: Replay::start(p1, p2, world.dummy),
            world,
            show_boxes: true,
            show_help: true,
            paused: false,
            step_once: false,
            log: Vec::new(),
            flash: None,
            playback: None,
            last_hash: 0,
        }
    }

    fn reset(&mut self) {
        self.world.reset();
        self.log.clear();
        self.recording = Replay::start(self.world.p1_char, self.world.p2_char, self.world.dummy);
        self.playback = None;
    }
}

struct Assets {
    kogan: SpriteSet,
    raya: SpriteSet,
    stage: Stage,
    portraits: [Option<Texture2D>; 2],
    flash: Flash,
}

/// Render-only state that rides along with a world: what was drawn lately
/// (for crossfades and afterimages), impact effects, and the round's winner.
#[derive(Default)]
struct Presentation {
    history: History,
    effects: Effects,
    victory: anim::VictoryClock,
    defeat: defeat::Clock,
}

impl Presentation {
    fn reset(&mut self) {
        self.history.reset();
        self.effects.reset();
        self.victory = anim::VictoryClock::default();
        self.defeat = defeat::Clock::default();
    }

    /// Once after every simulation tick, frozen ticks included.
    fn after_tick(&mut self, assets: &Assets, w: &World) {
        self.effects.after_tick(w);
        let cells = [0, 1].map(|i| {
            let f = &w.fighters[i];
            self.history.cell_for(w, i, assets.sprites(f.id))
        });
        self.history.record(w, cells);
    }

    fn draw(&self, view: &View, assets: &Assets, w: &World, boxes: bool) {
        self.effects.draw_behind(view, w);
        let order = self.victory.winner()
            .map(|i| [1-i,i]).unwrap_or_else(|| self.history.draw_order(w));
        for i in order {
            let sprites = assets.sprites(w.fighters[i].id);
            let layers = anim::layers(
                w,
                i,
                sprites,
                &self.history,
                &LayerOpts {
                    win: self.victory.age(i),
                    defeat: self.defeat.cell(w,i),
                    flash: self.effects.flash(i),
                },
            );
            let fd = FighterDraw {
                sprites: Some(sprites),
                layers: &layers,
                show_boxes: boxes,
                flash: &assets.flash,
            };
            draw_fighter(view, w, i, &fd);
        }
        draw_projectiles(view, w, boxes, w.frame);
        self.effects.draw(view, w);
    }
}

impl Assets {
    fn sprites(&self, id: CharacterId) -> &SpriteSet {
        match id {
            CharacterId::Kogan => &self.kogan,
            CharacterId::Raya => &self.raya,
        }
    }
}

/// Find the directory that holds `assets/`, so `cargo run -p aeon` from the
/// workspace root, running from `crates/client`, and a shipped binary next
/// to its assets all load the same files.
fn locate_assets() {
    let mut candidates: Vec<std::path::PathBuf> = vec![".".into(), "crates/client".into()];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.to_path_buf());
        }
    }
    candidates.push(env!("CARGO_MANIFEST_DIR").into());
    for c in candidates {
        if c.join("assets").join("select").is_dir() {
            eprintln!("[aeon] assets: {}", c.display());
            set_pc_assets_folder(&c.to_string_lossy());
            return;
        }
    }
    eprintln!("[aeon] assets: none found; drawing box bodies");
}

#[macroquad::main(window_conf)]
async fn main() {
    locate_assets();
    let assets = Assets {
        kogan: SpriteSet::load(CharacterId::Kogan).await,
        raya: SpriteSet::load(CharacterId::Raya).await,
        stage: Stage::load().await,
        portraits: [
            load_texture("assets/select/kogan.png").await.ok(),
            load_texture("assets/select/raya.png").await.ok(),
        ],
        flash: Flash::load(),
    };
    if std::env::args().any(|a| a == "--polish-preview") {
        preview::run(&assets).await;
        return;
    }
    if std::env::args().any(|a| a == "--kit-preview") {
        kit_preview::run(&assets).await;
        return;
    }
    if std::env::args().any(|a| a == "--replay-review") {
        replay_preview::run(&assets).await;
        return;
    }
    let mut pads = Pads::new();
    let smoke = std::env::args().any(|a| a == "--smoke");
    let mut mode = if smoke {
        Mode::Versus { m: Match::new(CharacterId::Kogan, CharacterId::Raya) }
    } else {
        Mode::Title { cursor: Menu::Versus }
    };
    let mut clock = FixedClock::default();
    let mut pres = Presentation::default();
    let mut inputs = [InputLatch::default(), InputLatch::default()];
    let mut frame: u32 = 0;
    let mut toast: Option<(String, u16)> = None;
    let mut smoke_script: Option<Smoke> = smoke.then(Smoke::default);
    // Screenshots are taken after drawing, right before the swap.
    let mut shot_request: Option<String> = None;

    loop {
        pads.pump();
        let mut view = View::fit();
        frame = frame.wrapping_add(1);

        if let Some(s) = &mut smoke_script {
            if s.done && shot_request.is_none() {
                eprintln!("[aeon] smoke complete: {} screenshots in ./shots", s.shots);
                std::process::exit(if s.shots == 8 { 0 } else { 1 });
            }
            if let Some(next) = s.step(&mut mode, frame, &mut shot_request) {
                mode = next;
            }
        }

        if is_key_pressed(KeyCode::F12) {
            let path = format!("shots/aeon-{frame}.png");
            toast = Some((format!("screenshot {path}"), 120));
            shot_request = Some(path);
        }
        if is_key_pressed(KeyCode::F8) && !matches!(mode, Mode::Remap { .. }) {
            let back = std::mem::replace(&mut mode, Mode::Title { cursor: Menu::Remap });
            mode = Mode::Remap { step: 0, back_to: Box::new(back), captured: Vec::new() };
        }

        let edges = menu_edges(&pads);
        inputs[0].sample(read_p1(&pads, true));
        inputs[1].sample(read_p2(&pads, true));
        if !matches!(mode, Mode::Versus { .. } | Mode::Training(_)) {
            clock.reset();
            pres.reset();
            for input in &mut inputs { input.discard_edges(); }
        }

        match &mut mode {
            Mode::Title { cursor } => {
                if edges.up {
                    *cursor = match cursor {
                        Menu::Versus => Menu::Remap,
                        Menu::Training => Menu::Versus,
                        Menu::Remap => Menu::Training,
                    };
                }
                if edges.down {
                    *cursor = match cursor {
                        Menu::Versus => Menu::Training,
                        Menu::Training => Menu::Remap,
                        Menu::Remap => Menu::Versus,
                    };
                }
                let chosen = if edges.confirm { Some(*cursor) } else { None };
                draw_title(&view, *cursor, &pads, frame, &assets);
                if let Some(c) = chosen {
                    mode = match c {
                        Menu::Versus | Menu::Training => Mode::Select {
                            for_training: c == Menu::Training,
                            p1: CharacterId::Kogan,
                            p2: CharacterId::Raya,
                            locked: [false, false],
                        },
                        Menu::Remap => Mode::Remap {
                            step: 0,
                            back_to: Box::new(Mode::Title { cursor: Menu::Versus }),
                            captured: Vec::new(),
                        },
                    };
                }
            }
            Mode::Select { for_training, p1, p2, locked } => {
                // P1 on stick/keyboard-1, P2 on keyboard-2 (or second pad).
                let e1 = edges;
                let e2 = p2_edges(&pads);
                if !locked[0] && (e1.left || e1.right) {
                    *p1 = p1.next();
                }
                if !locked[1] && (e2.left || e2.right) {
                    *p2 = p2.next();
                }
                if e1.confirm {
                    if !locked[0] {
                        locked[0] = true;
                    } else if *for_training || locked[1] {
                        // fallthrough to start below
                    }
                }
                if e2.confirm && !locked[1] {
                    locked[1] = true;
                }
                // In training, P1's confirm also locks P2 (the dummy).
                if *for_training && locked[0] {
                    locked[1] = true;
                }
                if e1.back || e2.back {
                    if locked[1] && !*for_training {
                        locked[1] = false;
                    } else if locked[0] {
                        locked[0] = false;
                        locked[1] = false;
                    } else {
                        mode = Mode::Title { cursor: if *for_training { Menu::Training } else { Menu::Versus } };
                        next_frame().await;
                        continue;
                    }
                }
                draw_select(&view, &assets, *p1, *p2, *locked, *for_training, frame);
                if locked[0] && locked[1] {
                    // Hold a beat so the lock reads, then go.
                    let (a, b, t) = (*p1, *p2, *for_training);
                    mode = if t {
                        Mode::Training(Training::new(a, b))
                    } else {
                        Mode::Versus { m: Match::new(a, b) }
                    };
                }
            }
            Mode::Versus { m } => {
                let ticks = frame_ticks(&mut clock, get_frame_time() as f64, smoke);
                for _ in 0..ticks {
                    let (p1, p2) = match &smoke_script {
                        Some(s) => s.inputs(m.world.frame),
                        None => (
                            inputs[0].take(m.world.fighters[0].facing_right),
                            inputs[1].take(m.world.fighters[1].facing_right),
                        ),
                    };
                    let prior = m.world.frame;
                    m.tick(p1, p2);
                    if m.world.frame < prior { pres.reset(); }
                    pres.victory.update(&m.world, m.phase);
                    pres.defeat.update(&m.world, m.phase);
                    pres.after_tick(&assets, &m.world);
                }
                pres.victory.update(&m.world, m.phase);
                pres.defeat.update(&m.world, m.phase);
                view.follow(&m.world);
                assets.stage.draw(&view, m.world.frame);
                pres.draw(&view, &assets, &m.world, false);
                draw_hud(&view, &m.world, &HudOpts { wins: Some(m.wins), round: Some(m.round) });
                draw_match_overlay(&view, m, m.world.frame);
                if let Phase::MatchOver { .. } = m.phase {
                    if edges.confirm || is_key_pressed(KeyCode::Enter) {
                        m.rematch();
                        pres.reset();
                    } else if edges.back || is_key_pressed(KeyCode::Escape) {
                        let (a, b) = (m.world.p1_char, m.world.p2_char);
                        mode = Mode::Select { for_training: false, p1: a, p2: b, locked: [false, false] };
                    }
                } else if is_key_pressed(KeyCode::Escape) || pads.start_pressed() && is_key_down(KeyCode::LeftShift) {
                    mode = Mode::Title { cursor: Menu::Versus };
                }
            }
            Mode::Training(t) => {
                training_keys(t, &mut toast, &mut pres);
                let ticks = if t.paused {
                    clock.reset();
                    usize::from(t.step_once)
                } else {
                    frame_ticks(&mut clock, get_frame_time() as f64, smoke)
                };
                for _ in 0..ticks {
                    let (p1, p2) = if let Some((rep, i)) = &mut t.playback {
                        if *i < rep.frames.len() {
                            let f = rep.frames[*i];
                            *i += 1;
                            f
                        } else {
                            t.playback = None;
                            (InputFrame::default(), InputFrame::default())
                        }
                    } else if let Some(s) = &smoke_script {
                        s.inputs(t.world.frame)
                    } else {
                        (
                            inputs[0].take(t.world.fighters[0].facing_right),
                            inputs[1].take(t.world.fighters[1].facing_right),
                        )
                    };
                    t.world.tick(p1, p2);
                    pres.after_tick(&assets, &t.world);
                    if t.playback.is_none() {
                        t.recording.push(p1, p2);
                    }
                    t.last_hash = t.world.state_hash();
                    log_events(t);
                    if t.step_once {
                        t.step_once = false;
                    }
                }
                if t.paused {
                    for input in &mut inputs { input.discard_edges(); }
                }
                if let Some((_, _, n)) = &mut t.flash {
                    *n = n.saturating_sub(ticks as u8);
                    if *n == 0 {
                        t.flash = None;
                    }
                }
                view.follow(&t.world);
                assets.stage.draw(&view, t.world.frame);
                pres.draw(&view, &assets, &t.world, t.show_boxes);
                draw_hud(&view, &t.world, &HudOpts { wins: None, round: None });
                draw_training_hud(&view, t, &pads);
                if is_key_pressed(KeyCode::Escape) {
                    mode = Mode::Title { cursor: Menu::Training };
                }
            }
            Mode::Remap { step, back_to, captured } => {
                let order = Btn::ALL;
                if *step < order.len() {
                    if let Some((btn, code)) = pads.take_press() {
                        // Prefer the logical name when gilrs knows it, so the
                        // config stays readable; fall back to the raw code.
                        let bind = if btn == gilrs::Button::Unknown { Bind::Raw(code) } else { Bind::Logical(btn) };
                        captured.push(bind);
                        *step += 1;
                    }
                } else {
                    for (b, bind) in order.iter().zip(captured.iter()) {
                        pads.map.set(*b, *bind);
                    }
                    pads.map.save();
                    toast = Some((format!("stick map saved: {}", pads.map.describe()), 240));
                    let back = std::mem::replace(back_to.as_mut(), Mode::Title { cursor: Menu::Versus });
                    mode = back;
                    next_frame().await;
                    continue;
                }
                if is_key_pressed(KeyCode::Escape) {
                    let back = std::mem::replace(back_to.as_mut(), Mode::Title { cursor: Menu::Versus });
                    mode = back;
                    next_frame().await;
                    continue;
                }
                draw_remap(&view, *step, &pads);
            }
        }

        if let Some((text, n)) = &mut toast {
            *n = n.saturating_sub(1);
            view.text(text, 24.0, VH - 12.0, 16.0, INK);
            if *n == 0 {
                toast = None;
            }
        }
        view.bars();
        pads.end_frame();
        if let Some(path) = shot_request.take() {
            screenshot(&path);
        }
        next_frame().await;
    }
}

fn read_p1(pads: &Pads, facing_right: bool) -> InputFrame {
    let kb = keyboard(0, facing_right);
    match pads.read(0, facing_right) {
        Some(pad) => merge(pad, kb),
        None => kb,
    }
}

fn read_p2(pads: &Pads, facing_right: bool) -> InputFrame {
    let kb = keyboard(1, facing_right);
    match pads.read(1, facing_right) {
        Some(pad) => merge(pad, kb),
        None => kb,
    }
}

fn menu_edges(pads: &Pads) -> MenuEdges {
    let mut e = pads.menu_edges();
    e.up |= is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up);
    e.down |= is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down);
    e.left |= is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left);
    e.right |= is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right);
    e.confirm |= is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Y) || is_key_pressed(KeyCode::Space);
    e.back |= is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::J);
    e
}

fn p2_edges(pads: &Pads) -> MenuEdges {
    let mut e = MenuEdges {
        left: is_key_pressed(KeyCode::Left),
        right: is_key_pressed(KeyCode::Right),
        confirm: is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::KpEnter),
        back: is_key_pressed(KeyCode::Semicolon),
        ..Default::default()
    };
    // Second pad, if any: read its buttons as a frame and treat P as confirm.
    if let Some(f) = pads.read(1, true) {
        e.confirm |= f.buttons.p;
        e.back |= f.buttons.fl;
    }
    e
}

fn training_keys(t: &mut Training, toast: &mut Option<(String, u16)>, pres: &mut Presentation) {
    if is_key_pressed(KeyCode::F1) {
        t.world.dummy = t.world.dummy.next();
        t.recording.dummy = Some(t.world.dummy);
    }
    if is_key_pressed(KeyCode::F2) {
        t.show_boxes = !t.show_boxes;
    }
    if is_key_pressed(KeyCode::F3) {
        t.world.swap_p1();
        t.reset();
        pres.reset();
    }
    if is_key_pressed(KeyCode::F4) {
        t.world.swap_p2();
        t.reset();
        pres.reset();
    }
    if is_key_pressed(KeyCode::F5) {
        t.reset();
        pres.reset();
    }
    if is_key_pressed(KeyCode::Space) {
        t.paused = !t.paused;
    }
    if is_key_pressed(KeyCode::Period) {
        t.paused = true;
        t.step_once = true;
    }
    if is_key_down(KeyCode::Comma) {
        t.paused = true;
        t.step_once = true;
    }
    if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
        for f in &mut t.world.fighters {
            f.meter = 1000;
            f.gauge = f.data().gauge.max;
        }
    }
    if is_key_pressed(KeyCode::Minus) {
        for f in &mut t.world.fighters {
            f.health = f.data().max_health;
        }
    }
    if is_key_pressed(KeyCode::Slash) || is_key_pressed(KeyCode::F10) {
        t.show_help = !t.show_help;
    }
    if is_key_pressed(KeyCode::F9) {
        match t.recording.save() {
            Ok(p) => *toast = Some((format!("replay saved {}", p.display()), 180)),
            Err(e) => *toast = Some((format!("replay save failed: {e}"), 180)),
        }
    }
    if is_key_pressed(KeyCode::F11) {
        match Replay::load_latest() {
            Some(rep) => {
                let p1 = rep.p1.unwrap_or(t.world.p1_char);
                let p2 = rep.p2.unwrap_or(t.world.p2_char);
                t.world = World::training(p1, p2);
                pres.reset();
                if let Some(d) = rep.dummy {
                    t.world.dummy = d;
                }
                t.log.clear();
                let n = rep.frames.len();
                t.playback = Some((rep, 0));
                t.paused = false;
                *toast = Some((format!("replaying {n} frames"), 180));
            }
            None => *toast = Some(("no replay in ./replays".to_string(), 180)),
        }
    }
}

fn log_events(t: &mut Training) {
    let w = &t.world;
    for ev in &w.events {
        let who = if ev.attacker == 0 { "P1" } else { "P2" };
        let name = ev
            .move_id
            .map(|m| w.fighters[ev.attacker].data().move_name(m))
            .unwrap_or("-");
        let line = match ev.kind {
            EventKind::Hit => format!("{who} HIT {name}  {}", ev.damage),
            EventKind::Block => format!("{who} BLOCKED {name}"),
            EventKind::Punish => format!("{who} PUNISH {name}  {}", ev.damage),
            EventKind::Grab => format!("{who} GRAB {name}"),
            EventKind::Throw => format!("{who} THROW {name}  {}", ev.damage),
            EventKind::ThrowTech => format!("{who} TECH"),
            EventKind::Knockdown => format!("{who} KD {name}  {}", ev.damage),
            EventKind::RomanCancel => format!("{who} ROMAN CANCEL  -250"),
            EventKind::Feint => format!("{who} FEINT {name}"),
            EventKind::ProjectileGuard => format!("{who} DISC GUARD"),
            EventKind::Clash => "SHOTS CLASH".to_string(),
            EventKind::Plant => format!("{who} PLANT"),
            EventKind::Armed => format!("{who} CRYSTAL ARMED"),
            EventKind::Detonate => format!("{who} SHATTER"),
            EventKind::KO => format!("{who} K.O."),
            EventKind::TimeOver => "TIME".to_string(),
        };
        let flash = match ev.kind {
            EventKind::Punish => Some((GOLD, 50)),
            EventKind::RomanCancel => Some((CYAN, 38)),
            EventKind::KO => Some((RED, 80)),
            EventKind::Feint => Some((LINEN, 30)),
            EventKind::Hit | EventKind::Knockdown | EventKind::Throw => Some((Color::new(0.94, 0.78, 0.55, 1.0), 28)),
            _ => None,
        };
        if let Some((c, n)) = flash {
            t.flash = Some((line.clone(), c, n));
        }
        t.log.push(line);
        if t.log.len() > 8 {
            t.log.remove(0);
        }
    }
}

fn draw_training_hud(v: &View, t: &Training, pads: &Pads) {
    let w = &t.world;
    // Measured advantage, P1's point of view.
    let adv = w.advantage_p1();
    let col = if adv > 0 { Color::new(0.45, 0.85, 0.55, 1.0) } else if adv < 0 { Color::new(0.9, 0.45, 0.45, 1.0) } else { INK };
    v.text_center(&format!("ADV {adv:+}"), VW / 2.0, 140.0, 26.0, col);
    let status = format!(
        "dummy {}   boxes {}   f {}   {}{}",
        w.dummy.label(),
        if t.show_boxes { "ON" } else { "OFF" },
        w.frame,
        if t.paused { "PAUSED  (. step  , hold)" } else { "" },
        if t.playback.is_some() { "  REPLAY" } else { "" }
    );
    v.text(&status, 24.0, VH - 92.0, 16.0, INK);
    v.text(&format!("hash {:016x}", t.last_hash), VW - 260.0, VH - 92.0, 14.0, COPPER_DIM);

    // State labels over the heads.
    for i in 0..2 {
        let f = &w.fighters[i];
        let p = v.world(render::sub_to_px(f.pos.x), render::sub_to_px(f.pos.y) + render::sub_to_px(f.data().stand_h) + 24.0);
        let label = match &f.action {
            Action::Attack { move_id, frame, .. } => format!("{} f{}", f.data().move_name(*move_id), frame),
            a => a.name().to_string(),
        };
        v.text_center(&label, p.x, p.y, 15.0, INK);
    }

    draw_input(v, 62.0, 178.0, w.fighters[0].input(), false);
    draw_input(v, VW - 62.0, 178.0, w.fighters[1].input(), true);

    let mut y = 240.0;
    for line in &t.log {
        v.text(line, 24.0, y, 17.0, Color::new(0.78, 0.74, 0.66, 0.9));
        y += 18.0;
    }
    if let Some((text, color, n)) = &t.flash {
        let a = (*n as f32 / 40.0).clamp(0.0, 1.0);
        v.text_center(text, VW / 2.0, 200.0, 34.0, Color { a, ..*color });
    }

    if t.show_help {
        let p1 = w.fighters[0].data();
        let mut lines = vec![
            "STICK  P S HS / K FL ST     P+K throw   S+FL RC (250)   FL+ST feint   HS+ST overhead   S+HS EX (gauge)".to_string(),
            "KEYS   P1 WASD + Y U I / H J K      P2 arrows + P [ ] / L ; '      tap up = hop, hold = jump, 66 = run".to_string(),
        ];
        for (i, l) in special_list(p1).iter().enumerate() {
            lines.push(format!("{:<6} {}", if i == 0 { p1.id.name() } else { "" }, l));
        }
        let p2 = w.fighters[1].data();
        if p2.id != p1.id {
            for (i, l) in special_list(p2).iter().enumerate() {
                lines.push(format!("{:<6} {}", if i == 0 { p2.id.name() } else { "" }, l));
            }
        }
        lines.push("F1 dummy  F2 boxes  F3/F4 swap  F5 reset  SPACE pause  . step  = fill  - heal  F8 remap  F9 save replay  F11 play  F12 shot  / help".to_string());
        if !pads.announced.is_empty() {
            lines.push(format!("{}   map {}", pads.announced.join(" | "), pads.map.describe()));
        } else {
            lines.push("no pad detected · keyboard P1".to_string());
        }
        let h = 18.0 * lines.len() as f32 + 16.0;
        v.rect(16.0, VH - 92.0 - h - 8.0, VW - 32.0, h, Color::new(0.02, 0.02, 0.03, 0.82));
        let mut y = VH - 92.0 - h + 8.0;
        for l in lines {
            v.text(&l, 24.0, y, 15.0, Color::new(0.80, 0.76, 0.70, 0.95));
            y += 18.0;
        }
    }
}

fn special_list(c: &aeon_sim::Character) -> Vec<String> {
    let mut parts = Vec::new();
    for r in &c.specials {
        if r.move_id.is_rekka() && r.move_id != aeon_sim::MoveId::Rekka1 {
            continue;
        }
        if let Some(input) = c.input_for(r.move_id) {
            parts.push(format!("{} {}", input, c.move_name(r.move_id)));
        }
    }
    parts
        .chunks(6)
        .map(|ch| ch.join("   "))
        .collect()
}

fn draw_title(v: &View, cursor: Menu, pads: &Pads, frame: u32, assets: &Assets) {
    clear_background(BLACK);
    v.rect(0.0, 0.0, VW, VH, render::VAULT);
    // Honeycomb wash.
    let r = 60.0;
    let dx = r * 1.732;
    let mut row = 0;
    let mut y = 0.0;
    while y < VH + r {
        let mut x = if row % 2 == 0 { 0.0 } else { dx / 2.0 };
        while x < VW + dx {
            let a = 0.04 + 0.03 * ((frame as f32 * 0.01 + x * 0.002 + y * 0.003).sin() + 1.0);
            v.poly_lines(x, y, 6, r - 4.0, 30.0, 1.5, Color::new(0.72, 0.45, 0.2, a));
            x += dx;
        }
        y += r * 1.5;
        row += 1;
    }
    v.text_center("AEON", VW / 2.0, 300.0, 140.0, LINEN);
    v.rect(VW / 2.0 - 220.0, 318.0, 440.0, 2.0, COPPER);
    v.text_center("the space between two bodies", VW / 2.0, 352.0, 22.0, INK);
    let items = [(Menu::Versus, "VERSUS"), (Menu::Training, "TRAINING"), (Menu::Remap, "STICK REMAP")];
    for (i, (m, label)) in items.iter().enumerate() {
        let y = 440.0 + i as f32 * 48.0;
        let sel = *m == cursor;
        if sel {
            v.poly(VW / 2.0 - 120.0, y - 12.0, 6, 9.0, 0.0, CYAN);
        }
        v.text_center(label, VW / 2.0, y, 32.0, if sel { LINEN } else { INK });
    }
    let pad = if pads.count() > 0 { pads.announced.join("  |  ") } else { "no pad · keyboard P1 (WASD, Y U I / H J K)".to_string() };
    v.text_center(&pad, VW / 2.0, VH - 60.0, 15.0, COPPER_DIM);
    v.text_center("up/down · P or ENTER confirm · F8 remap · F12 screenshot", VW / 2.0, VH - 36.0, 15.0, COPPER_DIM);
    let sprites = format!(
        "{} {} poses · {} {} poses · stage {}",
        assets.kogan.body().name(),
        assets.kogan.count(),
        assets.raya.body().name(),
        assets.raya.count(),
        if assets.stage.backdrop.is_some() { "sanctum" } else { "procedural" }
    );
    v.text(&sprites, 24.0, VH - 12.0, 13.0, COPPER_DIM);
}

fn draw_select(v: &View, assets: &Assets, p1: CharacterId, p2: CharacterId, locked: [bool; 2], training: bool, frame: u32) {
    clear_background(BLACK);
    v.rect(0.0, 0.0, VW, VH, render::VAULT);
    let cards = [(CharacterId::Kogan, 0usize), (CharacterId::Raya, 1usize)];
    let (y, w, h) = (110.0, 440.0, 500.0);
    let card_x = |i: usize| 160.0 + i as f32 * 520.0;
    // Pass 1: plates, cover-fit by center-cropping the *source* rect so the
    // draw never spills past its card (the plates are wider than the card gap).
    for (i, (_, pi)) in cards.iter().enumerate() {
        let x = card_x(i);
        v.rect(x, y, w, h, Color::new(0.08, 0.07, 0.09, 1.0));
        if let Some(t) = &assets.portraits[*pi] {
            let (tw, th) = (t.width(), t.height());
            let (sw, sh) = if tw / th > w / h { (th * w / h, th) } else { (tw, tw * h / w) };
            let src = Rect::new((tw - sw) / 2.0, (th - sh) / 2.0, sw, sh);
            draw_texture_ex(
                t,
                v.sx(x),
                v.sy(y),
                WHITE,
                DrawTextureParams { dest_size: Some(vec2(w * v.scale, h * v.scale)), source: Some(src), ..Default::default() },
            );
        }
    }
    v.text_center(if training { "TRAINING · choose" } else { "VERSUS · choose" }, VW / 2.0, 60.0, 30.0, LINEN);
    // Pass 2: frames and labels over every plate.
    for (i, (id, _)) in cards.iter().enumerate() {
        let x = card_x(i);
        let p1_here = p1 == *id;
        let p2_here = p2 == *id;
        let border = if p1_here && p2_here { LINEN } else if p1_here { CYAN } else if p2_here { COPPER } else { COPPER_DIM };
        v.rect_lines(x, y, w, h, 3.0, border);
        v.text_center(id.name(), x + w / 2.0, y + h + 40.0, 36.0, LINEN);
        let sub = match id {
            CharacterId::Kogan => "the duelist · saber · revolver · disc",
            CharacterId::Raya => "the officiant · voice · crystal · rite",
        };
        v.text_center(sub, x + w / 2.0, y + h + 66.0, 16.0, INK);
        let pulse = 0.6 + 0.4 * (frame as f32 * 0.1).sin();
        if p1_here {
            v.text(if locked[0] { "P1 LOCKED" } else { "P1" }, x + 12.0, y + 30.0, 24.0, Color { a: if locked[0] { 1.0 } else { pulse }, ..CYAN });
        }
        if p2_here {
            v.text_right(if locked[1] { "P2 LOCKED" } else { if training { "DUMMY" } else { "P2" } }, x + w - 12.0, y + 30.0, 24.0, Color { a: if locked[1] { 1.0 } else { pulse }, ..COPPER });
        }
    }
    v.text_center("P1 left/right · P confirm · FL back        P2 arrows · P confirm", VW / 2.0, VH - 36.0, 15.0, COPPER_DIM);
}

fn draw_remap(v: &View, step: usize, pads: &Pads) {
    clear_background(BLACK);
    v.rect(0.0, 0.0, VW, VH, render::VAULT);
    v.text_center("STICK REMAP", VW / 2.0, 120.0, 40.0, LINEN);
    v.text_center("Layout:   P  S  HS   over   K  FL  ST", VW / 2.0, 170.0, 22.0, INK);
    let order = Btn::ALL;
    for (i, b) in order.iter().enumerate() {
        let (col, r) = (i % 3, i / 3);
        // Draw the 2×3 in the stick's order: P S HS / K FL ST.
        let grid = [[Btn::P, Btn::S, Btn::HS], [Btn::K, Btn::FL, Btn::ST]];
        let _ = (col, r);
        let (gr, gc) = grid
            .iter()
            .enumerate()
            .find_map(|(gr, row)| row.iter().position(|x| x == b).map(|gc| (gr, gc)))
            .unwrap();
        let x = VW / 2.0 - 200.0 + gc as f32 * 140.0;
        let y = 260.0 + gr as f32 * 110.0;
        let current = i == step;
        let done = i < step;
        v.rect(x, y, 120.0, 80.0, if current { COPPER } else if done { Color::new(0.16, 0.40, 0.48, 1.0) } else { Color::new(0.10, 0.09, 0.10, 1.0) });
        v.text_center(b.label(), x + 60.0, y + 50.0, 36.0, if current { BLACK } else { LINEN });
    }
    let msg = if step < order.len() {
        format!("press the stick button for  {}", order[step].label())
    } else {
        "saved".to_string()
    };
    v.text_center(&msg, VW / 2.0, 540.0, 28.0, LINEN);
    v.text_center(&format!("current: {}", pads.map.describe()), VW / 2.0, 600.0, 15.0, INK);
    v.text_center("ESC cancels", VW / 2.0, VH - 36.0, 15.0, COPPER_DIM);
}

fn screenshot(path: &str) {
    let _ = std::fs::create_dir_all("shots");
    get_screen_data().export_png(path);
}

// Smoke renders every simulation tick so screenshot triggers cannot be skipped
// by a slow display frame. Interactive play retains its fixed-step accumulator.
fn frame_ticks(clock: &mut FixedClock, elapsed: f64, smoke: bool) -> usize {
    if smoke { 1 } else { clock.advance(elapsed) }
}

/// `--smoke`: drive versus then training with scripted inputs, screenshot
/// each, and exit. Launch evidence for QA V1/V2 without a hand on the stick.
#[derive(Default)]
struct Smoke {
    stage: u8,
    stage_frame: u32,
    shots: u32,
    done: bool,
}

impl Smoke {
    fn inputs(&self, f: u32) -> (InputFrame, InputFrame) {
        // A little choreography: walk in, Kogan pokes, Raya blocks then
        // places a glyph, Kogan runs a rekka, a hop, a throw attempt.
        let p1 = match f {
            0..=40 => InputFrame::dir(6),
            41 => InputFrame::press(Btn::S),
            60..=62 => InputFrame::dir([2, 3, 6][(f - 60) as usize]),
            63 => InputFrame::dir_press(6, Btn::S),
            80 => InputFrame::press(Btn::S),
            100 => InputFrame::press(Btn::S),
            140 => InputFrame::dir(9),
            150 => InputFrame::press(Btn::HS),
            190 => InputFrame::chord(Chord::Throw),
            220..=222 => InputFrame::dir([2, 1, 4][(f - 220) as usize]),
            223 => InputFrame::dir_press(4, Btn::S),
            260..=264 => InputFrame::dir([6, 3, 2, 1, 4][(f - 260) as usize]),
            265 => InputFrame::dir_press(4, Btn::FL),
            _ => InputFrame::default(),
        };
        let p2 = match f {
            0..=30 => InputFrame::dir(6),
            35..=70 => InputFrame::dir(4),
            110..=112 => InputFrame::dir([2, 3, 6][(f - 110) as usize]),
            113 => InputFrame::dir_press(6, Btn::HS),
            170..=172 => InputFrame::dir([2, 1, 4][(f - 170) as usize]),
            173 => InputFrame::dir_press(4, Btn::S),
            240..=242 => InputFrame::dir([6, 2, 3][(f - 240) as usize]),
            243 => InputFrame::dir_press(3, Btn::S),
            _ => InputFrame::default(),
        };
        (p1, p2)
    }

    /// Returns a mode to switch into, if any. Screenshot requests are
    /// fulfilled by the main loop after drawing.
    fn step(&mut self, mode: &mut Mode, frame: u32, shot: &mut Option<String>) -> Option<Mode> {
        self.stage_frame += 1;
        let mut shot_at = |s: &mut Smoke, name: &str| {
            *shot = Some(format!("shots/smoke-{name}.png"));
            s.shots += 1;
        };
        match (self.stage, mode) {
            (0, Mode::Versus { m }) => {
                // Skip the intro quickly, then let the choreography run.
                if m.world.frame == 0 && frame > 5 && matches!(m.phase, Phase::Intro { .. }) {
                    m.phase = Phase::Fight;
                }
                if m.world.frame == 45 {
                    shot_at(self, "versus-poke");
                }
                if m.world.frame == 118 {
                    shot_at(self, "versus-glyph");
                }
                if m.world.frame == 200 {
                    shot_at(self, "versus-mid");
                }
                if m.world.frame >= 300 {
                    self.stage = 1;
                    let mut t = Training::new(CharacterId::Raya, CharacterId::Kogan);
                    t.world.dummy = aeon_sim::DummyMode::BlockAll;
                    return Some(Mode::Training(t));
                }
            }
            (1, Mode::Training(t)) => {
                if t.world.frame == 70 {
                    shot_at(self, "training-boxes");
                }
                if t.world.frame == 180 {
                    shot_at(self, "training-glyph");
                }
                if t.world.frame >= 300 {
                    self.stage = 2;
                    self.stage_frame = 0;
                    return Some(Mode::Title { cursor: Menu::Versus });
                }
            }
            (2, Mode::Title { .. }) => {
                if self.stage_frame == 30 {
                    shot_at(self, "title");
                }
                if self.stage_frame == 31 {
                    self.stage = 3;
                    self.stage_frame = 0;
                    return Some(Mode::Select {
                        for_training: false,
                        p1: CharacterId::Kogan,
                        p2: CharacterId::Raya,
                        locked: [true, false],
                    });
                }
            }
            (3, Mode::Select { .. }) => {
                if self.stage_frame == 30 {
                    shot_at(self, "select");
                }
                if self.stage_frame == 31 {
                    self.stage = 4;
                    self.stage_frame = 0;
                    let mut m = Match::new(CharacterId::Raya, CharacterId::Kogan);
                    m.phase = Phase::MatchOver { winner: 0 };
                    return Some(Mode::Versus { m });
                }
            }
            (4, Mode::Versus { .. }) if self.stage_frame == 30 => {
                shot_at(self, "winner");
                self.done = true;
            }
            _ => {}
        }
        None
    }
}

#[cfg(test)]
mod smoke_tests {
    use super::*;

    #[test]
    fn smoke_captures_all_eight_scenes_despite_display_jitter_and_start_offset() {
        let expected = ["versus-poke", "versus-glyph", "versus-mid", "training-boxes",
            "training-glyph", "title", "select", "winner"];
        for offset in 0..30 {
            let mut smoke = Smoke::default();
            let mut mode = Mode::Versus { m: Match::new(CharacterId::Kogan, CharacterId::Raya) };
            let mut clock = FixedClock::default();
            let mut shots = Vec::new();
            for frame in 1..1000 {
                let mut shot = None;
                if let Some(next) = smoke.step(&mut mode, frame + offset, &mut shot) { mode = next; }
                let elapsed = [0.0, 1.0 / 144.0, 0.3, 1.0 / 60.0, 0.004][frame as usize % 5];
                for _ in 0..frame_ticks(&mut clock, elapsed, true) {
                    match &mut mode {
                        Mode::Versus { m } => { let (a, b) = smoke.inputs(m.world.frame); m.tick(a, b); }
                        Mode::Training(t) => { let (a, b) = smoke.inputs(t.world.frame); t.world.tick(a, b); }
                        _ => {}
                    }
                }
                if let Some(path) = shot { shots.push(path); }
                if smoke.done { break; }
            }
            assert!(smoke.done, "offset {offset}");
            assert_eq!(smoke.shots, 8);
            assert_eq!(shots, expected.map(|name| format!("shots/smoke-{name}.png")), "offset {offset}");
        }
    }
}
