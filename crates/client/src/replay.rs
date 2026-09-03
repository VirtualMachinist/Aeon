//! Input-log replays. The sim is deterministic, so a per-frame log of both
//! inputs *is* a recording. Client-side only; the sim never touches a file.
//!
//! Format (text): header lines `p1=KOGAN`, `p2=RAYA`, `dummy=STAND`, then
//! one line per tick `dir:bits dir:bits` where bits is P K S HS FL ST.

use std::path::PathBuf;

use aeon_sim::input::{Btn, Buttons, InputFrame};
use aeon_sim::{CharacterId, DummyMode};

#[derive(Clone, Debug, Default)]
pub struct Replay {
    pub p1: Option<CharacterId>,
    pub p2: Option<CharacterId>,
    pub dummy: Option<DummyMode>,
    pub frames: Vec<(InputFrame, InputFrame)>,
}

fn encode(f: InputFrame) -> String {
    let bits: String = Btn::ALL
        .iter()
        .map(|b| if f.buttons.get(*b) { '1' } else { '0' })
        .collect();
    format!("{}:{}", f.dir, bits)
}

fn decode(s: &str) -> Option<InputFrame> {
    let (d, bits) = s.split_once(':')?;
    let dir: u8 = d.parse().ok()?;
    let mut buttons = Buttons::default();
    for (b, c) in Btn::ALL.iter().zip(bits.chars()) {
        buttons.set(*b, c == '1');
    }
    Some(InputFrame { dir, buttons })
}

fn char_name(c: CharacterId) -> &'static str {
    c.name()
}

fn parse_char(s: &str) -> Option<CharacterId> {
    match s {
        "KOGAN" => Some(CharacterId::Kogan),
        "RAYA" => Some(CharacterId::Raya),
        _ => None,
    }
}

fn parse_dummy(s: &str) -> Option<DummyMode> {
    DummyMode::ALL.iter().copied().find(|m| m.label() == s)
}

impl Replay {
    pub fn start(p1: CharacterId, p2: CharacterId, dummy: DummyMode) -> Self {
        Self {
            p1: Some(p1),
            p2: Some(p2),
            dummy: Some(dummy),
            frames: Vec::new(),
        }
    }

    pub fn push(&mut self, a: InputFrame, b: InputFrame) {
        self.frames.push((a, b));
    }

    pub fn dir() -> PathBuf {
        PathBuf::from("replays")
    }

    pub fn save(&self) -> std::io::Result<PathBuf> {
        std::fs::create_dir_all(Self::dir())?;
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let path = Self::dir().join(format!("{stamp}.aeonrep"));
        let mut out = String::new();
        if let Some(c) = self.p1 {
            out.push_str(&format!("p1={}\n", char_name(c)));
        }
        if let Some(c) = self.p2 {
            out.push_str(&format!("p2={}\n", char_name(c)));
        }
        if let Some(d) = self.dummy {
            out.push_str(&format!("dummy={}\n", d.label()));
        }
        for (a, b) in &self.frames {
            out.push_str(&encode(*a));
            out.push(' ');
            out.push_str(&encode(*b));
            out.push('\n');
        }
        std::fs::write(&path, out)?;
        Ok(path)
    }

    /// Newest replay on disk.
    pub fn load_latest() -> Option<Self> {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(Self::dir())
            .ok()?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().map(|e| e == "aeonrep").unwrap_or(false))
            .collect();
        entries.sort();
        let path = entries.pop()?;
        Self::load(&path)
    }

    pub fn load(path: &std::path::Path) -> Option<Self> {
        let text = std::fs::read_to_string(path).ok()?;
        let mut r = Self::default();
        for line in text.lines() {
            if let Some(v) = line.strip_prefix("p1=") {
                r.p1 = parse_char(v);
            } else if let Some(v) = line.strip_prefix("p2=") {
                r.p2 = parse_char(v);
            } else if let Some(v) = line.strip_prefix("dummy=") {
                r.dummy = parse_dummy(v);
            } else if let Some((a, b)) = line.split_once(' ') {
                if let (Some(a), Some(b)) = (decode(a), decode(b)) {
                    r.frames.push((a, b));
                }
            }
        }
        Some(r)
    }
}
