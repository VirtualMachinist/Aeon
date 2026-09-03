//! Shared drivers for headless proofs.
#![allow(dead_code)]

use aeon_sim::fighter::Action;
use aeon_sim::geom::px;
use aeon_sim::input::{Btn, Buttons, Chord, InputFrame};
use aeon_sim::moves::MoveId;
use aeon_sim::{CharacterId, DummyMode, EventKind, World};

pub fn idle() -> InputFrame {
    InputFrame::default()
}

pub fn dir(d: u8) -> InputFrame {
    InputFrame::dir(d)
}

pub fn press(b: Btn) -> InputFrame {
    InputFrame::press(b)
}

pub fn dir_press(d: u8, b: Btn) -> InputFrame {
    InputFrame::dir_press(d, b)
}

pub fn chord(c: Chord) -> InputFrame {
    InputFrame::chord(c)
}

pub fn back() -> InputFrame {
    dir(4)
}

pub fn down_back() -> InputFrame {
    dir(1)
}

pub fn two(a: Btn, b: Btn) -> InputFrame {
    InputFrame {
        dir: 5,
        buttons: Buttons::two(a, b),
    }
}

/// P1 acts, P2 idles (dummy filter still applies).
pub fn tap(w: &mut World, p1: InputFrame) {
    w.tick(p1, idle());
}

pub fn hold(w: &mut World, n: u32, p1: InputFrame) {
    for _ in 0..n {
        w.tick(p1, idle());
    }
}

pub fn hold2(w: &mut World, n: u32, p1: InputFrame, p2: InputFrame) {
    for _ in 0..n {
        w.tick(p1, p2);
    }
}

pub fn drain_hitstop(w: &mut World) {
    while w.hitstop > 0 || w.rc_freeze > 0 {
        w.tick(idle(), idle());
    }
}

/// Feed a motion (dirs) then the button with the final direction held.
pub fn motion(w: &mut World, dirs: &[u8], b: Btn) {
    for d in dirs {
        w.tick(dir(*d), idle());
    }
    let last = *dirs.last().unwrap_or(&5);
    w.tick(dir_press(last, b), idle());
}

/// Buffer a motion while P1 is locked (recovery, knockdown wait), then press
/// the button on the first actionable frame — how a player times oki.
pub fn buffered_special(w: &mut World, dirs: &[u8], b: Btn) {
    // Cycling the motion keeps an ordered copy inside the 12f window.
    let mut i = 0;
    while !w.fighters[0].action.actionable() || w.hitstop > 0 {
        w.tick(dir(dirs[i % dirs.len()]), idle());
        i += 1;
    }
    let last = *dirs.last().unwrap_or(&5);
    w.tick(dir_press(last, b), idle());
}

pub fn motion_p2(w: &mut World, dirs: &[u8], b: Btn) {
    for d in dirs {
        w.tick(idle(), dir(*d));
    }
    let last = *dirs.last().unwrap_or(&5);
    w.tick(idle(), dir_press(last, b));
}

pub fn motion_chord(w: &mut World, dirs: &[u8], c: Chord) {
    for d in dirs {
        w.tick(dir(*d), idle());
    }
    let last = *dirs.last().unwrap_or(&5);
    w.tick(
        InputFrame {
            dir: last,
            buttons: Buttons::chord(c),
        },
        idle(),
    );
}

pub fn close(p1: CharacterId, p2: CharacterId) -> World {
    let mut w = World::training(p1, p2);
    w.fighters[0].pos.x = px(370);
    w.fighters[1].pos.x = px(408);
    w.dummy = DummyMode::Stand;
    w
}

pub fn close_kogan() -> World {
    close(CharacterId::Kogan, CharacterId::Kogan)
}

pub fn free(p1: CharacterId, p2: CharacterId) -> World {
    let mut w = close(p1, p2);
    w.dummy = DummyMode::CpuOff;
    w
}

pub fn attacking(w: &World, who: usize, id: MoveId) -> bool {
    matches!(w.fighters[who].action, Action::Attack { move_id, .. } if move_id == id)
}

pub fn has_event(w: &World, kind: EventKind) -> bool {
    w.events.iter().any(|e| e.kind == kind)
}

/// Tick with the given inputs until `pred` or `max` frames; returns frames used.
pub fn run_until(
    w: &mut World,
    max: u32,
    p1: InputFrame,
    p2: InputFrame,
    pred: impl Fn(&World) -> bool,
) -> Option<u32> {
    for i in 0..max {
        w.tick(p1, p2);
        if pred(w) {
            return Some(i);
        }
    }
    None
}

/// Frames until fighter `who` is actionable again, feeding idle to both.
pub fn frames_until_actionable(w: &mut World, who: usize, max: u32) -> u32 {
    for i in 0..max {
        if w.fighters[who].action.actionable() && w.hitstop == 0 {
            return i;
        }
        w.tick(idle(), idle());
    }
    max
}

pub fn knock_down_p2(w: &mut World) {
    // Sweep from close range, then wait through hitstop.
    w.fighters[0].start_move(MoveId::CrST);
    hold(w, 12, dir(2));
    drain_hitstop(w);
    assert!(
        matches!(
            w.fighters[1].action,
            Action::Hit {
                knockdown: true,
                ..
            }
        ) || matches!(w.fighters[1].action, Action::Knockdown { .. }),
        "sweep should knock down, got {}",
        w.fighters[1].action.name()
    );
}
