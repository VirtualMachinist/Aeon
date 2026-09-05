//! Render-rate independent tick scheduling and input delivery.
//! Strict link windows live in the sim; a fast display must not lose a tap.
use aeon_sim::{Btn, Buttons, InputFrame, TICK_HZ};

const DT: f64 = 1.0 / TICK_HZ as f64;

#[derive(Default)]
pub struct FixedClock {
    remainder: f64,
}

impl FixedClock {
    pub fn advance(&mut self, elapsed: f64) -> usize {
        // A suspended window resumes at the present instead of replaying a
        // backlog of seconds. Ordinary 30/60/120/144 Hz frames lose no ticks.
        self.remainder += elapsed.clamp(0.0, 0.1);
        let ticks = ((self.remainder + 1e-9) / DT).floor() as usize;
        self.remainder = (self.remainder - ticks as f64 * DT).max(0.0);
        ticks
    }

    pub fn reset(&mut self) {
        self.remainder = 0.0;
    }
}

#[derive(Default)]
pub struct InputLatch {
    held: InputFrame,
    pending: Buttons,
    press_dir: Option<u8>,
    jump_dir: Option<u8>,
}

impl InputLatch {
    /// Raw direction is world-relative. Convert facing when a sim tick
    /// consumes it, including when a cross-up occurs between render frames.
    pub fn sample(&mut self, raw: InputFrame) {
        if raw.up() && !self.held.up() {
            self.jump_dir = Some(raw.dir);
        }
        let mut edge = false;
        for b in Btn::ALL {
            if raw.buttons.get(b) && !self.held.buttons.get(b) {
                self.pending.set(b, true);
                edge = true;
            }
        }
        if edge {
            self.press_dir = Some(raw.dir);
        }
        self.held = raw;
    }

    pub fn take(&mut self, facing_right: bool) -> InputFrame {
        let mut input = InputFrame {
            dir: self
                .press_dir
                .take()
                .or(self.jump_dir.take())
                .unwrap_or(self.held.dir),
            buttons: self.held.buttons.or(self.pending),
        };
        self.pending = Buttons::default();
        if !facing_right {
            input.dir = match input.dir {
                1 => 3,
                3 => 1,
                4 => 6,
                6 => 4,
                7 => 9,
                9 => 7,
                d => d,
            };
        }
        input
    }

    pub fn discard_edges(&mut self) {
        self.pending = Buttons::default();
        self.press_dir = None;
        self.jump_dir = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_display_rate_runs_sixty_sim_ticks_per_second() {
        for rate in [30, 60, 75, 120, 144, 165, 240] {
            let mut clock = FixedClock::default();
            let ticks: usize = (0..rate * 10)
                .map(|_| clock.advance(1.0 / rate as f64))
                .sum();
            assert_eq!(ticks, 600, "{rate} Hz");
        }
    }

    #[test]
    fn pause_and_window_suspension_do_not_leave_a_catchup_backlog() {
        let mut clock = FixedClock::default();
        clock.advance(DT / 2.0);
        clock.reset();
        assert_eq!(clock.advance(DT / 2.0), 0);
        clock.reset();
        assert_eq!(clock.advance(3.0), 6);
        assert_eq!(clock.advance(DT), 1);
    }

    #[test]
    fn a_tap_between_ticks_arrives_once_with_its_motion_direction() {
        let mut input = InputLatch::default();
        input.sample(InputFrame::dir_press(6, Btn::S));
        input.sample(InputFrame::default());
        assert_eq!(input.take(true), InputFrame::dir_press(6, Btn::S));
        assert_eq!(input.take(true), InputFrame::default());
    }

    #[test]
    fn brief_up_between_ticks_still_starts_a_hop() {
        let mut input = InputLatch::default();
        input.sample(InputFrame::dir(9));
        input.sample(InputFrame::default());
        assert_eq!(input.take(false), InputFrame::dir(7));
        assert_eq!(input.take(false), InputFrame::default());
        input.sample(InputFrame::dir(8));
        input.sample(InputFrame::default());
        input.discard_edges();
        assert_eq!(input.take(true), InputFrame::default());
    }

    #[test]
    fn chord_edges_coexist_and_facing_is_converted_at_consumption() {
        let mut input = InputLatch::default();
        input.sample(InputFrame::dir_press(3, Btn::S));
        input.sample(InputFrame::dir_press(3, Btn::HS));
        let frame = input.take(false);
        assert_eq!(frame.dir, 1);
        assert_eq!(frame.buttons, Buttons::two(Btn::S, Btn::HS));
        assert_eq!(input.take(false).buttons, Buttons::one(Btn::HS));
    }

    #[test]
    fn pausing_discards_taps_without_discarding_a_held_direction() {
        let mut input = InputLatch::default();
        input.sample(InputFrame::dir_press(4, Btn::P));
        input.sample(InputFrame::dir(4));
        input.discard_edges();
        assert_eq!(input.take(true), InputFrame::dir(4));
    }
}
