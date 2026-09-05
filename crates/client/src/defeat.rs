//! Presentation consequence of zero health. Match/simulation state stays untouched.
use aeon_sim::{Action, CharacterId, Phase, World};
use crate::sprites::Cell;

#[derive(Clone, Copy)]
enum Stage { Reaction, Collapse { start: u32 }, Down }

#[derive(Clone, Copy)]
struct Track { last_frame: u32, crouching: bool, stage: Stage }

#[derive(Default)]
pub struct Clock { tracks: [Option<Track>; 2] }

impl Clock {
    pub fn update(&mut self, w: &World, phase: Phase) {
        let ending = matches!(phase, Phase::RoundEnd { .. } | Phase::MatchOver { .. });
        for (i, slot) in self.tracks.iter_mut().enumerate() {
            let f = &w.fighters[i];
            if !ending || f.health > 0 { *slot = None; continue; }
            if slot.is_some_and(|t| w.frame < t.last_frame) { *slot = None; }
            let fresh = slot.is_none();
            let t = slot.get_or_insert(Track { last_frame: w.frame,
                crouching: f.input().down(), stage: Stage::Reaction });
            t.last_frame = w.frame;
            if f.airborne || matches!(f.action, Action::Thrown { .. }) {
                t.stage = Stage::Reaction;
            } else if (matches!(f.action, Action::Knockdown { .. } | Action::Getup { .. })
                && !matches!(t.stage, Stage::Collapse { .. }))
                || (fresh && matches!(phase, Phase::MatchOver { .. })) {
                t.stage = Stage::Down;
            } else {
                t.stage = match t.stage {
                    Stage::Down => Stage::Down,
                    Stage::Collapse { start } if w.frame.saturating_sub(start) >= 12 => Stage::Down,
                    Stage::Collapse { start } => Stage::Collapse { start },
                    Stage::Reaction if matches!(f.action,
                        Action::Hit { stun: 4.., .. }) => Stage::Reaction,
                    Stage::Reaction => Stage::Collapse { start: w.frame },
                };
            }
        }
    }

    pub fn cell(&self, w: &World, i: usize) -> Option<Cell> {
        let t = self.tracks[i]?;
        let f = &w.fighters[i];
        let floor = |n| if f.id == CharacterId::Kogan { Cell::Floor(n) } else { Cell::Reaction(4 + n) };
        Some(match t.stage {
            Stage::Down => floor(0),
            Stage::Collapse { start } => {
                let phase = (w.frame.saturating_sub(start) / 4).min(3) as usize;
                floor((if t.crouching { 2usize } else { 3usize }).saturating_sub(phase))
            }
            Stage::Reaction if f.airborne => Cell::Reaction(if f.vel.y > 0 { 2 } else { 3 }),
            Stage::Reaction if f.id == CharacterId::Kogan => Cell::Recoil(usize::from(t.crouching) * 2),
            Stage::Reaction => Cell::Reaction(usize::from(t.crouching)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::{InputFrame, RoundOutcome};
    fn end() -> Phase { Phase::RoundEnd { outcome: RoundOutcome::Winner(0), frame: 20 } }

    #[test]
    fn defeat_collapse_freezes_stays_down_and_clears_on_reset() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            let mut w = World::new(id,id);
            let mut clock = Clock::default();
            w.frame=40;w.fighters[1].health=0;
            w.fighters[1].action=Action::Hit { stun:8,knockdown:false };
            clock.update(&w,end());assert_eq!(clock.cell(&w,0),None);
            let hit=clock.cell(&w,1);
            w.frame+=1;w.fighters[1].action=Action::Hit { stun:3,knockdown:false };
            clock.update(&w,end());let first=clock.cell(&w,1);assert_ne!(first,hit);
            for _ in 0..10 { clock.update(&w,end());assert_eq!(clock.cell(&w,1),first); }
            w.frame+=12;w.fighters[1].action=Action::Stand;
            clock.update(&w,end());let floor=clock.cell(&w,1);
            assert_eq!(floor,Some(if id==CharacterId::Kogan { Cell::Floor(0) } else { Cell::Reaction(4) }));
            for action in [Action::Getup {frame:23},Action::Stand] {
                w.frame+=1;w.fighters[1].action=action;clock.update(&w,end());assert_eq!(clock.cell(&w,1),floor);
            }
            clock.update(&w,Phase::MatchOver {winner:0});assert_eq!(clock.cell(&w,1),floor);
            w=World::new(id,id);clock.update(&w,Phase::Intro {frame:0});assert_eq!(clock.cell(&w,1),None);
            w.fighters[1].health=0;clock.update(&w,Phase::MatchOver {winner:0});assert_eq!(clock.cell(&w,1),floor);
        }
    }

    #[test]
    fn defeat_preserves_airborne_consequence_crouch_and_live_timeout_bodies() {
        let mut w=World::new(CharacterId::Kogan,CharacterId::Raya);
        let mut clock=Clock::default();
        clock.update(&w,end());assert_eq!(clock.cell(&w,0),None);assert_eq!(clock.cell(&w,1),None);
        w.fighters[0].health=0;w.fighters[0].buffer_input(InputFrame::dir(2));
        w.fighters[0].action=Action::Hit {stun:8,knockdown:false};clock.update(&w,end());
        w.fighters[0].buffer_input(InputFrame::default());w.frame+=1;clock.update(&w,end());
        assert_eq!(clock.cell(&w,0),Some(Cell::Recoil(2)),"round-end neutral input cannot stand the crouched victim up");
        w.fighters[0].airborne=true;w.fighters[0].vel.y=1;w.frame+=1;clock.update(&w,end());
        assert_eq!(clock.cell(&w,0),Some(Cell::Reaction(2)));
        w.fighters[0].vel.y = -1;w.frame+=1;clock.update(&w,end());assert_eq!(clock.cell(&w,0),Some(Cell::Reaction(3)));
        w.fighters[0].airborne=false;w.fighters[0].action=Action::Knockdown {frame:0};w.frame+=1;
        clock.update(&w,end());assert_eq!(clock.cell(&w,0),Some(Cell::Floor(0)));
        w.fighters[1].health=0;w.fighters[1].action=Action::Knockdown {frame:0};
        clock.update(&w,Phase::RoundEnd {outcome:RoundOutcome::Draw,frame:30});
        assert!(clock.cell(&w,0).is_some() && clock.cell(&w,1).is_some(),"double KO covers both bodies");
        clock.update(&w,Phase::Fight);assert_eq!(clock.cell(&w,0),None);assert_eq!(clock.cell(&w,1),None);
    }
    #[test]
    fn grounded_sweep_keeps_drawn_collapse_through_knockdown_entry() {
        for id in [CharacterId::Kogan, CharacterId::Raya] {
            let mut w = World::new(id, id);
            let mut clock = Clock::default();
            w.frame = 40;
            w.fighters[1].health = 0;
            w.fighters[1].action = Action::Hit { stun: 4, knockdown: true };
            clock.update(&w, end());
            let recoil = clock.cell(&w, 1);
            w.frame += 1;
            w.fighters[1].action = Action::Hit { stun: 3, knockdown: true };
            clock.update(&w, end());
            assert_ne!(clock.cell(&w, 1), recoil);
            w.frame += 4;
            w.fighters[1].action = Action::Knockdown { frame: 0 };
            clock.update(&w, end());
            assert_eq!(clock.cell(&w, 1), Some(if id == CharacterId::Kogan {
                Cell::Floor(2)
            } else { Cell::Reaction(6) }));
            w.frame += 8;
            clock.update(&w, end());
            assert_eq!(clock.cell(&w, 1), Some(if id == CharacterId::Kogan {
                Cell::Floor(0)
            } else { Cell::Reaction(4) }));
        }
    }

}
