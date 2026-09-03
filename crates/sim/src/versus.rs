//! Versus match: intro → fight → round end → next round, first to two.
//! Pure and deterministic like the world it wraps.

use crate::chars::CharacterId;
use crate::input::InputFrame;
use crate::world::{RoundOutcome, World};

pub const INTRO_FRAMES: u16 = 60;
pub const ROUND_END_FRAMES: u16 = 100;
pub const ROUNDS_TO_WIN: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Phase {
    Intro { frame: u16 },
    Fight,
    RoundEnd { outcome: RoundOutcome, frame: u16 },
    MatchOver { winner: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Match {
    pub world: World,
    pub wins: [u8; 2],
    pub round: u8,
    pub phase: Phase,
}

impl Match {
    pub fn new(p1: CharacterId, p2: CharacterId) -> Self {
        Self {
            world: World::new(p1, p2),
            wins: [0, 0],
            round: 1,
            phase: Phase::Intro { frame: 0 },
        }
    }

    pub fn rematch(&mut self) {
        *self = Self::new(self.world.p1_char, self.world.p2_char);
    }

    pub fn fighting(&self) -> bool {
        self.phase == Phase::Fight
    }

    pub fn winner(&self) -> Option<usize> {
        match self.phase {
            Phase::MatchOver { winner } => Some(winner),
            _ => None,
        }
    }

    pub fn tick(&mut self, p1: InputFrame, p2: InputFrame) {
        match self.phase {
            Phase::Intro { frame } => {
                // Bodies stand; inputs are buffered but ignored so a held
                // direction at "fight" doesn't count as a charge.
                if frame + 1 >= INTRO_FRAMES {
                    self.phase = Phase::Fight;
                } else {
                    self.phase = Phase::Intro { frame: frame + 1 };
                }
            }
            Phase::Fight => {
                self.world.tick(p1, p2);
                if let Some(outcome) = self.world.outcome {
                    self.phase = Phase::RoundEnd { outcome, frame: 0 };
                }
            }
            Phase::RoundEnd { outcome, frame } => {
                // Let the KO'd body fall.
                self.world
                    .tick(InputFrame::default(), InputFrame::default());
                if frame + 1 >= ROUND_END_FRAMES {
                    match outcome {
                        RoundOutcome::Winner(w) => self.wins[w] += 1,
                        RoundOutcome::Draw => {
                            self.wins[0] += 1;
                            self.wins[1] += 1;
                        }
                    }
                    let done = self
                        .wins
                        .iter()
                        .enumerate()
                        .filter(|(_, w)| **w >= ROUNDS_TO_WIN)
                        .count();
                    if done == 1 {
                        let winner = if self.wins[0] >= ROUNDS_TO_WIN { 0 } else { 1 };
                        self.phase = Phase::MatchOver { winner };
                    } else if done == 2 {
                        // Both reached two on a draw round: sudden-death round.
                        self.wins = [ROUNDS_TO_WIN - 1, ROUNDS_TO_WIN - 1];
                        self.next_round();
                    } else {
                        self.next_round();
                    }
                } else {
                    self.phase = Phase::RoundEnd {
                        outcome,
                        frame: frame + 1,
                    };
                }
            }
            Phase::MatchOver { .. } => {}
        }
    }

    fn next_round(&mut self) {
        let (p1, p2) = (self.world.p1_char, self.world.p2_char);
        self.world = World::new(p1, p2);
        self.round += 1;
        self.phase = Phase::Intro { frame: 0 };
    }
}
