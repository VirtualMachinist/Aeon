//! Stick first. P1 is the first pad gilrs sees; P2 is the keyboard, and a
//! second pad if present. The layout is the authorized 2×3:
//!
//! ```text
//!   P     S     HS
//!   K     FL    ST
//! ```
//!
//! Default pad map is the SF-on-Xbox convention (top row West/North/RT, bottom
//! South/East/RT2), which is where a Mayflash F700 in Android mode lands.
//! F8 opens an in-game remap that records raw codes so any HID box works.

use std::collections::HashMap;
use std::path::PathBuf;

use aeon_sim::input::{stick_to_dir, Btn, Buttons, InputFrame};
use gilrs::{Axis, Button, Event, EventType, GamepadId, Gilrs};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bind {
    Logical(Button),
    Raw(u32),
}

impl Bind {
    fn label(self) -> String {
        match self {
            Bind::Logical(b) => format!("{b:?}"),
            Bind::Raw(c) => format!("#{c}"),
        }
    }

    fn parse(s: &str) -> Option<Bind> {
        if let Some(rest) = s.strip_prefix('#') {
            return rest.parse().ok().map(Bind::Raw);
        }
        let b = match s {
            "South" => Button::South,
            "East" => Button::East,
            "North" => Button::North,
            "West" => Button::West,
            "LeftTrigger" => Button::LeftTrigger,
            "LeftTrigger2" => Button::LeftTrigger2,
            "RightTrigger" => Button::RightTrigger,
            "RightTrigger2" => Button::RightTrigger2,
            "Select" => Button::Select,
            "Start" => Button::Start,
            "C" => Button::C,
            "Z" => Button::Z,
            _ => return None,
        };
        Some(Bind::Logical(b))
    }
}

#[derive(Clone, Debug)]
pub struct PadMap {
    pub binds: [Bind; 6],
}

impl Default for PadMap {
    fn default() -> Self {
        Self {
            binds: [
                Bind::Logical(Button::West),
                Bind::Logical(Button::South),
                Bind::Logical(Button::North),
                Bind::Logical(Button::East),
                Bind::Logical(Button::RightTrigger),
                Bind::Logical(Button::RightTrigger2),
            ],
        }
    }
}

impl PadMap {
    fn slot(b: Btn) -> usize {
        match b {
            Btn::P => 0,
            Btn::K => 1,
            Btn::S => 2,
            Btn::FL => 3,
            Btn::HS => 4,
            Btn::ST => 5,
        }
    }

    pub fn bind(&self, b: Btn) -> Bind {
        self.binds[Self::slot(b)]
    }

    pub fn set(&mut self, b: Btn, bind: Bind) {
        self.binds[Self::slot(b)] = bind;
    }

    fn path() -> Option<PathBuf> {
        let home = std::env::var_os("HOME")?;
        Some(PathBuf::from(home).join(".config").join("aeon").join("stick.cfg"))
    }

    pub fn load() -> Self {
        let mut m = Self::default();
        let Some(p) = Self::path() else { return m };
        let Ok(text) = std::fs::read_to_string(p) else { return m };
        for line in text.lines() {
            let Some((k, v)) = line.split_once('=') else { continue };
            let btn = match k.trim() {
                "P" => Btn::P,
                "K" => Btn::K,
                "S" => Btn::S,
                "HS" => Btn::HS,
                "FL" => Btn::FL,
                "ST" => Btn::ST,
                _ => continue,
            };
            if let Some(b) = Bind::parse(v.trim()) {
                m.set(btn, b);
            }
        }
        m
    }

    pub fn save(&self) {
        let Some(p) = Self::path() else { return };
        if let Some(dir) = p.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let mut out = String::new();
        for b in Btn::ALL {
            out.push_str(&format!("{}={}\n", b.label(), self.bind(b).label()));
        }
        let _ = std::fs::write(p, out);
    }

    pub fn describe(&self) -> String {
        Btn::ALL
            .iter()
            .map(|b| format!("{}:{}", b.label(), self.bind(*b).label()))
            .collect::<Vec<_>>()
            .join("  ")
    }
}

pub struct Pads {
    gilrs: Option<Gilrs>,
    /// Raw button codes tracked from events, per pad.
    raw: HashMap<GamepadId, HashMap<u32, bool>>,
    /// Most recent raw press (for remap capture).
    last_press: Option<(GamepadId, Button, u32)>,
    pub map: PadMap,
    pub announced: Vec<String>,
}

impl Pads {
    pub fn new() -> Self {
        let gilrs = match Gilrs::new() {
            Ok(g) => Some(g),
            Err(e) => {
                eprintln!("[aeon] gilrs unavailable: {e}. Keyboard only.");
                None
            }
        };
        let mut pads = Self {
            gilrs,
            raw: HashMap::new(),
            last_press: None,
            map: PadMap::load(),
            announced: Vec::new(),
        };
        pads.announce_all();
        pads
    }

    fn announce_all(&mut self) {
        let Some(g) = &self.gilrs else { return };
        let mut lines = Vec::new();
        for (i, (_, gp)) in g.gamepads().enumerate() {
            let uuid = gp
                .uuid()
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>();
            let line = format!(
                "pad {} = {} ({}) uuid {}",
                i + 1,
                gp.name(),
                gp.os_name(),
                uuid
            );
            eprintln!("[aeon] {line}");
            lines.push(line);
        }
        eprintln!("[aeon] stick map: {}", self.map.describe());
        self.announced = lines;
    }

    /// Drain events every frame so state stays current and raw codes are
    /// tracked for any box gilrs does not recognise.
    pub fn pump(&mut self) {
        let Some(g) = &mut self.gilrs else { return };
        let mut reannounce = false;
        while let Some(Event { id, event, .. }) = g.next_event() {
            match event {
                EventType::ButtonPressed(btn, code) => {
                    let c = code.into_u32();
                    self.raw.entry(id).or_default().insert(c, true);
                    self.last_press = Some((id, btn, c));
                }
                EventType::ButtonReleased(_, code) => {
                    self.raw.entry(id).or_default().insert(code.into_u32(), false);
                }
                EventType::Connected | EventType::Disconnected => reannounce = true,
                _ => {}
            }
        }
        if reannounce {
            self.announce_all();
        }
    }

    /// Advance gilrs' frame counter after all reads so `just pressed` edges
    /// are exactly one frame wide.
    pub fn end_frame(&mut self) {
        if let Some(g) = &mut self.gilrs {
            g.inc();
        }
    }

    pub fn count(&self) -> usize {
        self.gilrs.as_ref().map(|g| g.gamepads().count()).unwrap_or(0)
    }

    fn pad_id(&self, index: usize) -> Option<GamepadId> {
        self.gilrs
            .as_ref()?
            .gamepads()
            .nth(index)
            .map(|(id, _)| id)
    }

    /// Take the most recent raw press and clear it (remap capture).
    pub fn take_press(&mut self) -> Option<(Button, u32)> {
        self.last_press.take().map(|(_, b, c)| (b, c))
    }

    fn pressed(&self, id: GamepadId, bind: Bind) -> bool {
        let Some(g) = &self.gilrs else { return false };
        match bind {
            Bind::Logical(b) => g.gamepad(id).is_pressed(b),
            Bind::Raw(c) => self
                .raw
                .get(&id)
                .and_then(|m| m.get(&c))
                .copied()
                .unwrap_or(false),
        }
    }

    /// Facing-relative frame for pad `index` (0 = first pad), or None if no
    /// such pad exists.
    pub fn read(&self, index: usize, facing_right: bool) -> Option<InputFrame> {
        let id = self.pad_id(index)?;
        let g = self.gilrs.as_ref()?;
        let gp = g.gamepad(id);
        let mut sx = 0;
        let mut sy = 0;
        let lx = gp.value(Axis::LeftStickX);
        let ly = gp.value(Axis::LeftStickY);
        if lx > 0.5 {
            sx = 1;
        } else if lx < -0.5 {
            sx = -1;
        }
        if ly > 0.5 {
            sy = 1;
        } else if ly < -0.5 {
            sy = -1;
        }
        let dx = gp.value(Axis::DPadX);
        let dy = gp.value(Axis::DPadY);
        if dx > 0.5 || gp.is_pressed(Button::DPadRight) {
            sx = 1;
        } else if dx < -0.5 || gp.is_pressed(Button::DPadLeft) {
            sx = -1;
        }
        if dy > 0.5 || gp.is_pressed(Button::DPadUp) {
            sy = 1;
        } else if dy < -0.5 || gp.is_pressed(Button::DPadDown) {
            sy = -1;
        }
        let mut buttons = Buttons::default();
        for b in Btn::ALL {
            buttons.set(b, self.pressed(id, self.map.bind(b)));
        }
        Some(InputFrame {
            dir: stick_to_dir(sx, sy, facing_right),
            buttons,
        })
    }

    /// Menu navigation from the first pad: (up, down, left, right, confirm, back).
    pub fn menu_edges(&self) -> MenuEdges {
        let mut m = MenuEdges::default();
        let Some(id) = self.pad_id(0) else { return m };
        let Some(g) = &self.gilrs else { return m };
        let gp = g.gamepad(id);
        let just = |b: Button| gp.button_data(b).map(|d| d.is_pressed() && d.counter() == g.counter()).unwrap_or(false);
        m.up = just(Button::DPadUp);
        m.down = just(Button::DPadDown);
        m.left = just(Button::DPadLeft);
        m.right = just(Button::DPadRight);
        m.confirm = just(Button::South) || just(Button::West) || just(Button::Start);
        m.back = just(Button::East) || just(Button::Select);
        m
    }

    pub fn start_pressed(&self) -> bool {
        let Some(id) = self.pad_id(0) else { return false };
        let Some(g) = &self.gilrs else { return false };
        let gp = g.gamepad(id);
        gp.button_data(Button::Start)
            .map(|d| d.is_pressed() && d.counter() == g.counter())
            .unwrap_or(false)
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct MenuEdges {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub confirm: bool,
    pub back: bool,
}

/// Keyboard, same 2×3 as the stick. P1: Y/U/I over H/J/K, WASD.
/// P2: P/[/] over L/;/', arrows.
pub fn keyboard(player: usize, facing_right: bool) -> InputFrame {
    let (sx, sy, buttons) = if player == 0 {
        (
            key_axis(KeyCode::D, KeyCode::A),
            key_axis(KeyCode::W, KeyCode::S),
            Buttons {
                p: is_key_down(KeyCode::Y),
                s: is_key_down(KeyCode::U),
                hs: is_key_down(KeyCode::I),
                k: is_key_down(KeyCode::H),
                fl: is_key_down(KeyCode::J),
                st: is_key_down(KeyCode::K),
            },
        )
    } else {
        (
            key_axis(KeyCode::Right, KeyCode::Left),
            key_axis(KeyCode::Up, KeyCode::Down),
            Buttons {
                p: is_key_down(KeyCode::P),
                s: is_key_down(KeyCode::LeftBracket),
                hs: is_key_down(KeyCode::RightBracket),
                k: is_key_down(KeyCode::L),
                fl: is_key_down(KeyCode::Semicolon),
                st: is_key_down(KeyCode::Apostrophe),
            },
        )
    };
    InputFrame {
        dir: stick_to_dir(sx, sy, facing_right),
        buttons,
    }
}

fn key_axis(pos: KeyCode, neg: KeyCode) -> i32 {
    is_key_down(pos) as i32 - is_key_down(neg) as i32
}

pub fn merge(a: InputFrame, b: InputFrame) -> InputFrame {
    InputFrame {
        dir: if a.dir != 5 { a.dir } else { b.dir },
        buttons: a.buttons.or(b.buttons),
    }
}
