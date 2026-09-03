//! Per-tick input, a short ring buffer, and motion detection.
//!
//! Directions are numpad notation relative to the fighter's facing
//! (6 = toward opponent, 4 = away). The client is responsible for
//! converting raw keys / stick into a facing-relative [`InputFrame`].
//!
//! Buttons are the six of the law: P K S HS FL ST. Chords are adjacent
//! pairs on the authorized stick layout (`P S HS` over `K FL ST`):
//! P+K throw, S+FL Roman Cancel, S+HS EX, FL+ST feint, HS+ST overhead.

pub const BUFFER_LEN: usize = 16;
pub const MOTION_WINDOW: usize = 12;
pub const HCB_WINDOW: usize = 16;
pub const CHARGE_FRAMES: u16 = 45;
pub const CHARGE_RELEASE_WINDOW: u8 = 5;
/// A chord tolerates its two buttons landing this many frames apart.
pub const CHORD_WINDOW: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Btn {
    P,
    K,
    S,
    HS,
    FL,
    ST,
}

impl Btn {
    pub const ALL: [Btn; 6] = [Btn::P, Btn::K, Btn::S, Btn::HS, Btn::FL, Btn::ST];

    pub fn label(self) -> &'static str {
        match self {
            Btn::P => "P",
            Btn::K => "K",
            Btn::S => "S",
            Btn::HS => "HS",
            Btn::FL => "FL",
            Btn::ST => "ST",
        }
    }

    /// Slash family carries specials, uppercuts, projectiles.
    pub fn is_slash(self) -> bool {
        matches!(self, Btn::S | Btn::HS)
    }

    /// Flash / Style family carries grabs, dashes, charge, overheads.
    pub fn is_flash(self) -> bool {
        matches!(self, Btn::FL | Btn::ST)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Chord {
    /// P+K
    Throw,
    /// S+FL, 250 bar
    RomanCancel,
    /// S+HS, character gauge
    Ex,
    /// FL+ST
    Feint,
    /// HS+ST, universal standing overhead
    Overhead,
}

impl Chord {
    pub fn pair(self) -> (Btn, Btn) {
        match self {
            Chord::Throw => (Btn::P, Btn::K),
            Chord::RomanCancel => (Btn::S, Btn::FL),
            Chord::Ex => (Btn::S, Btn::HS),
            Chord::Feint => (Btn::FL, Btn::ST),
            Chord::Overhead => (Btn::HS, Btn::ST),
        }
    }

    pub const ALL: [Chord; 5] = [
        Chord::Throw,
        Chord::RomanCancel,
        Chord::Ex,
        Chord::Feint,
        Chord::Overhead,
    ];
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Buttons {
    pub p: bool,
    pub k: bool,
    pub s: bool,
    pub hs: bool,
    pub fl: bool,
    pub st: bool,
}

impl Buttons {
    pub const NONE: Buttons = Buttons {
        p: false,
        k: false,
        s: false,
        hs: false,
        fl: false,
        st: false,
    };

    pub fn one(b: Btn) -> Self {
        let mut out = Self::default();
        out.set(b, true);
        out
    }

    pub fn two(a: Btn, b: Btn) -> Self {
        let mut out = Self::one(a);
        out.set(b, true);
        out
    }

    pub fn chord(c: Chord) -> Self {
        let (a, b) = c.pair();
        Self::two(a, b)
    }

    pub fn get(self, b: Btn) -> bool {
        match b {
            Btn::P => self.p,
            Btn::K => self.k,
            Btn::S => self.s,
            Btn::HS => self.hs,
            Btn::FL => self.fl,
            Btn::ST => self.st,
        }
    }

    pub fn set(&mut self, b: Btn, v: bool) {
        match b {
            Btn::P => self.p = v,
            Btn::K => self.k = v,
            Btn::S => self.s = v,
            Btn::HS => self.hs = v,
            Btn::FL => self.fl = v,
            Btn::ST => self.st = v,
        }
    }

    pub fn any(self) -> bool {
        self.p || self.k || self.s || self.hs || self.fl || self.st
    }

    pub fn count(self) -> u8 {
        Btn::ALL.iter().filter(|b| self.get(**b)).count() as u8
    }

    pub fn iter(self) -> impl Iterator<Item = Btn> {
        Btn::ALL.into_iter().filter(move |b| self.get(*b))
    }

    /// The single button, if exactly one is set.
    pub fn single(self) -> Option<Btn> {
        let mut it = self.iter();
        let first = it.next()?;
        if it.next().is_some() {
            None
        } else {
            Some(first)
        }
    }

    pub fn has_chord(self, c: Chord) -> bool {
        let (a, b) = c.pair();
        self.get(a) && self.get(b)
    }

    pub fn just_pressed(self, prev: Self) -> Self {
        Self {
            p: self.p && !prev.p,
            k: self.k && !prev.k,
            s: self.s && !prev.s,
            hs: self.hs && !prev.hs,
            fl: self.fl && !prev.fl,
            st: self.st && !prev.st,
        }
    }

    pub fn or(self, other: Self) -> Self {
        Self {
            p: self.p || other.p,
            k: self.k || other.k,
            s: self.s || other.s,
            hs: self.hs || other.hs,
            fl: self.fl || other.fl,
            st: self.st || other.st,
        }
    }
}

/// One tick of facing-relative input.
/// `dir` is numpad 1..=9 (5 = neutral).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputFrame {
    pub dir: u8,
    pub buttons: Buttons,
}

impl Default for InputFrame {
    fn default() -> Self {
        Self {
            dir: 5,
            buttons: Buttons::default(),
        }
    }
}

impl InputFrame {
    pub fn dir(d: u8) -> Self {
        Self {
            dir: d,
            buttons: Buttons::default(),
        }
    }

    pub fn press(b: Btn) -> Self {
        Self {
            dir: 5,
            buttons: Buttons::one(b),
        }
    }

    pub fn dir_press(d: u8, b: Btn) -> Self {
        Self {
            dir: d,
            buttons: Buttons::one(b),
        }
    }

    pub fn chord(c: Chord) -> Self {
        Self {
            dir: 5,
            buttons: Buttons::chord(c),
        }
    }

    pub fn down(self) -> bool {
        matches!(self.dir, 1..=3)
    }
    pub fn up(self) -> bool {
        matches!(self.dir, 7..=9)
    }
    pub fn forward(self) -> bool {
        matches!(self.dir, 3 | 6 | 9)
    }
    pub fn back(self) -> bool {
        matches!(self.dir, 1 | 4 | 7)
    }
    pub fn down_back(self) -> bool {
        self.dir == 1
    }
}

/// Convert a world-space stick (x: -1/0/1, y: -1/0/1, y+ = up) into a
/// facing-relative numpad direction.
pub fn stick_to_dir(sx: i32, sy: i32, facing_right: bool) -> u8 {
    let fx = if facing_right { sx } else { -sx };
    match (fx, sy) {
        (-1, -1) => 1,
        (0, -1) => 2,
        (1, -1) => 3,
        (-1, 0) => 4,
        (0, 0) => 5,
        (1, 0) => 6,
        (-1, 1) => 7,
        (0, 1) => 8,
        (1, 1) => 9,
        _ => 5,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InputBuffer {
    frames: [InputFrame; BUFFER_LEN],
    head: usize,
    filled: usize,
    back_charge: u16,
    down_charge: u16,
    back_release: u8,
    down_release: u8,
    /// Frames since each button was pressed (edge), saturating. Used for
    /// the chord tolerance window.
    press_age: [u8; 6],
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            frames: [InputFrame::default(); BUFFER_LEN],
            head: 0,
            filled: 0,
            back_charge: 0,
            down_charge: 0,
            back_release: 0,
            down_release: 0,
            press_age: [u8::MAX; 6],
        }
    }
}

impl InputBuffer {
    pub fn push(&mut self, frame: InputFrame) {
        let prev = self.latest();
        let was_back = prev.back();
        let was_down = prev.down();

        if frame.back() {
            self.back_charge = self.back_charge.saturating_add(1);
            self.back_release = 0;
        } else {
            if was_back && self.back_charge >= CHARGE_FRAMES {
                self.back_release = CHARGE_RELEASE_WINDOW;
            } else {
                self.back_release = self.back_release.saturating_sub(1);
            }
            self.back_charge = 0;
        }

        if frame.down() {
            self.down_charge = self.down_charge.saturating_add(1);
            self.down_release = 0;
        } else {
            if was_down && self.down_charge >= CHARGE_FRAMES {
                self.down_release = CHARGE_RELEASE_WINDOW;
            } else {
                self.down_release = self.down_release.saturating_sub(1);
            }
            self.down_charge = 0;
        }

        let edges = frame.buttons.just_pressed(prev.buttons);
        for (i, b) in Btn::ALL.iter().enumerate() {
            if edges.get(*b) {
                self.press_age[i] = 0;
            } else {
                self.press_age[i] = self.press_age[i].saturating_add(1);
            }
        }

        self.frames[self.head] = frame;
        self.head = (self.head + 1) % BUFFER_LEN;
        self.filled = (self.filled + 1).min(BUFFER_LEN);
    }

    pub fn latest(&self) -> InputFrame {
        if self.filled == 0 {
            return InputFrame::default();
        }
        let i = (self.head + BUFFER_LEN - 1) % BUFFER_LEN;
        self.frames[i]
    }

    pub fn prev(&self) -> InputFrame {
        if self.filled < 2 {
            return InputFrame::default();
        }
        let i = (self.head + BUFFER_LEN - 2) % BUFFER_LEN;
        self.frames[i]
    }

    /// Buttons whose press edge happened this frame.
    pub fn pressed(&self) -> Buttons {
        self.latest().buttons.just_pressed(self.prev().buttons)
    }

    /// Buttons pressed within the last `n` frames (inclusive of this one)
    /// and still held.
    pub fn pressed_within(&self, n: u8) -> Buttons {
        let held = self.latest().buttons;
        let mut out = Buttons::default();
        for (i, b) in Btn::ALL.iter().enumerate() {
            if held.get(*b) && self.press_age[i] < n {
                out.set(*b, true);
            }
        }
        out
    }

    /// Buttons whose press edge happened within the last `n` frames, held or
    /// not. Used for the throw-tech window.
    pub fn pressed_recently(&self, n: u8) -> Buttons {
        let mut out = Buttons::default();
        for (i, b) in Btn::ALL.iter().enumerate() {
            if self.press_age[i] < n {
                out.set(*b, true);
            }
        }
        out
    }

    /// A chord fires the frame its second button lands, provided the first
    /// landed within [`CHORD_WINDOW`]. Returns at most one chord; if two
    /// are simultaneously completable the law's priority is RC, EX, feint,
    /// overhead, throw.
    pub fn chord(&self) -> Option<Chord> {
        let edge = self.pressed();
        if !edge.any() {
            return None;
        }
        let recent = self.pressed_within(CHORD_WINDOW);
        for c in [
            Chord::RomanCancel,
            Chord::Ex,
            Chord::Feint,
            Chord::Overhead,
            Chord::Throw,
        ] {
            let (a, b) = c.pair();
            let completes_now = (edge.get(a) && recent.get(b)) || (edge.get(b) && recent.get(a));
            if completes_now {
                return Some(c);
            }
        }
        None
    }

    /// Iterate from oldest-in-window to newest.
    pub fn window(&self, n: usize) -> impl Iterator<Item = InputFrame> + '_ {
        let n = n.min(self.filled);
        (0..n).map(move |k| {
            let i = (self.head + BUFFER_LEN - n + k) % BUFFER_LEN;
            self.frames[i]
        })
    }

    pub fn motion(&self, motion: Motion) -> bool {
        match motion {
            Motion::None => true,
            Motion::Qcf => self.sequence(&[2, 3, 6], MOTION_WINDOW),
            Motion::Qcb => self.sequence(&[2, 1, 4], MOTION_WINDOW),
            Motion::Dp => self.sequence(&[6, 2, 3], MOTION_WINDOW),
            Motion::Hcb => self.sequence(&[6, 2, 4], HCB_WINDOW) && self.hcb_passes_down(),
            Motion::SuperQcf => self.sequence(&[2, 3, 6, 2, 3, 6], BUFFER_LEN),
            Motion::ForwardDash => self.dash(6),
            Motion::BackDash => self.dash(4),
            Motion::ChargeBackForward => self.back_release > 0 && self.latest().forward(),
            Motion::ChargeDownUp => self.down_release > 0 && self.latest().up(),
        }
    }

    pub fn back_charge(&self) -> u16 {
        self.back_charge
    }

    pub fn down_charge(&self) -> u16 {
        self.down_charge
    }

    /// 63214 must actually pass through 3 or 1 as well as 2, so a sloppy
    /// 6-2-4 (down, then back) still reads but a 6-4 does not.
    fn hcb_passes_down(&self) -> bool {
        self.window(HCB_WINDOW).any(|f| matches!(f.dir, 1..=3))
    }

    /// Loose sequential match: each required direction (or a diagonal that
    /// contains it) must appear in order within the window.
    fn sequence(&self, parts: &[u8], window: usize) -> bool {
        let frames: Vec<u8> = self.window(window).map(|f| f.dir).collect();
        let mut at = 0;
        for &need in parts {
            let mut found = false;
            while at < frames.len() {
                if dir_matches(frames[at], need) {
                    found = true;
                    at += 1;
                    break;
                }
                at += 1;
            }
            if !found {
                return false;
            }
        }
        true
    }

    /// Two taps of `dir` (6 or 4) within 10f, released to neutral, up, or the
    /// opposite side in between. Passing through down does not count as a
    /// release, so 214 214 does not backdash and 236 236 does not run.
    fn dash(&self, dir: u8) -> bool {
        let opposite = if dir == 6 { 4 } else { 6 };
        let frames: Vec<InputFrame> = self.window(10).collect();
        let mut taps = 0;
        let mut released = true;
        for f in frames {
            let match_dir = dir_matches(f.dir, dir);
            if match_dir && released {
                taps += 1;
                released = false;
            } else if f.dir == 5 || f.dir == 8 || dir_matches(f.dir, opposite) {
                released = true;
            }
        }
        taps >= 2
    }
}

fn dir_matches(got: u8, need: u8) -> bool {
    if got == need {
        return true;
    }
    match need {
        2 => matches!(got, 1..=3),
        4 => matches!(got, 1 | 4 | 7),
        6 => matches!(got, 3 | 6 | 9),
        8 => matches!(got, 7..=9),
        _ => false,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Motion {
    /// Plain button (used for chord-only routes such as the overhead).
    None,
    Qcf,      // 236
    Qcb,      // 214
    Dp,       // 623
    Hcb,      // 63214
    SuperQcf, // 236236
    ForwardDash,
    BackDash,
    ChargeBackForward,
    ChargeDownUp,
}

impl Motion {
    /// Longer motions win when two routes are simultaneously satisfied so
    /// that 236236+S doesn't degrade to 236+S and 63214+FL to 214+FL.
    pub fn rank(self) -> u8 {
        match self {
            Motion::SuperQcf => 4,
            Motion::Hcb => 3,
            Motion::Dp | Motion::ChargeBackForward | Motion::ChargeDownUp => 2,
            Motion::Qcf | Motion::Qcb | Motion::ForwardDash | Motion::BackDash => 1,
            Motion::None => 0,
        }
    }

    pub fn notation(self) -> &'static str {
        match self {
            Motion::None => "",
            Motion::Qcf => "236",
            Motion::Qcb => "214",
            Motion::Dp => "623",
            Motion::Hcb => "63214",
            Motion::SuperQcf => "236236",
            Motion::ForwardDash => "66",
            Motion::BackDash => "44",
            Motion::ChargeBackForward => "[4]6",
            Motion::ChargeDownUp => "[2]8",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir_only(d: u8) -> InputFrame {
        InputFrame::dir(d)
    }

    #[test]
    fn qcf_reads_236() {
        let mut b = InputBuffer::default();
        for d in [5, 2, 3, 6, 5] {
            b.push(dir_only(d));
        }
        assert!(b.motion(Motion::Qcf));
        assert!(!b.motion(Motion::Dp));
    }

    #[test]
    fn qcf_accepts_diagonals() {
        let mut b = InputBuffer::default();
        for d in [2, 3, 6] {
            b.push(dir_only(d));
        }
        assert!(b.motion(Motion::Qcf));
    }

    #[test]
    fn hcb_reads_63214_and_not_64() {
        let mut b = InputBuffer::default();
        for d in [6, 3, 2, 1, 4] {
            b.push(dir_only(d));
        }
        assert!(b.motion(Motion::Hcb));
        let mut b = InputBuffer::default();
        for d in [6, 5, 4] {
            b.push(dir_only(d));
        }
        assert!(!b.motion(Motion::Hcb));
    }

    #[test]
    fn charge_release_is_first_class() {
        let mut b = InputBuffer::default();
        for _ in 0..CHARGE_FRAMES {
            b.push(dir_only(4));
        }
        assert_eq!(b.back_charge(), CHARGE_FRAMES);
        b.push(dir_only(6));
        assert!(b.motion(Motion::ChargeBackForward));
    }

    #[test]
    fn short_charge_does_not_release() {
        let mut b = InputBuffer::default();
        for _ in 0..(CHARGE_FRAMES - 1) {
            b.push(dir_only(4));
        }
        b.push(dir_only(6));
        assert!(!b.motion(Motion::ChargeBackForward));
    }

    #[test]
    fn chord_same_frame_and_within_window() {
        let mut b = InputBuffer::default();
        b.push(InputFrame::chord(Chord::RomanCancel));
        assert_eq!(b.chord(), Some(Chord::RomanCancel));
        b.push(InputFrame::chord(Chord::RomanCancel));
        assert_eq!(b.chord(), None, "held chord does not re-fire");

        let mut b = InputBuffer::default();
        b.push(InputFrame::press(Btn::FL));
        b.push(InputFrame::press(Btn::FL));
        b.push(InputFrame {
            dir: 5,
            buttons: Buttons::two(Btn::FL, Btn::ST),
        });
        assert_eq!(b.chord(), Some(Chord::Feint));

        let mut b = InputBuffer::default();
        for _ in 0..CHORD_WINDOW + 1 {
            b.push(InputFrame::press(Btn::HS));
        }
        b.push(InputFrame {
            dir: 5,
            buttons: Buttons::two(Btn::HS, Btn::ST),
        });
        assert_eq!(b.chord(), None, "too late to be a chord");
    }
}
