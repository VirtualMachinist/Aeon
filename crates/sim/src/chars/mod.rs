mod kogan;
mod raya;

pub use kogan::kogan;
pub use raya::raya;

use crate::geom::{px, LocalBox};
use crate::input::{Btn, InputBuffer, Motion};
use crate::moves::{MoveDef, MoveId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CharacterId {
    Kogan,
    Raya,
}

impl CharacterId {
    pub fn name(self) -> &'static str {
        match self {
            Self::Kogan => "KOGAN",
            Self::Raya => "RAYA",
        }
    }

    pub fn data(self) -> &'static Character {
        match self {
            Self::Kogan => kogan(),
            Self::Raya => raya(),
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Kogan => Self::Raya,
            Self::Raya => Self::Kogan,
        }
    }
}

/// A motion + button → move. `ex` routes require the S+HS chord instead of
/// a single button and spend the character gauge.
#[derive(Clone, Copy, Debug)]
pub struct SpecialRoute {
    pub motion: Motion,
    pub buttons: &'static [Btn],
    pub ex: bool,
    pub move_id: MoveId,
    pub air_ok: bool,
    pub ground_ok: bool,
}

impl SpecialRoute {
    pub const fn ground(motion: Motion, buttons: &'static [Btn], move_id: MoveId) -> Self {
        Self {
            motion,
            buttons,
            ex: false,
            move_id,
            air_ok: false,
            ground_ok: true,
        }
    }

    pub const fn air(motion: Motion, buttons: &'static [Btn], move_id: MoveId) -> Self {
        Self {
            motion,
            buttons,
            ex: false,
            move_id,
            air_ok: true,
            ground_ok: false,
        }
    }

    pub const fn ex(motion: Motion, move_id: MoveId) -> Self {
        Self {
            motion,
            buttons: &[],
            ex: true,
            move_id,
            air_ok: false,
            ground_ok: true,
        }
    }
}

/// Character gauge law. Kogan: chambers with cooldown reload. Raya: crystal
/// gauge filled by consecrate, no regen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GaugeDef {
    pub max: i32,
    pub start: i32,
    /// After `regen_delay` frames without spending, gain `regen_amount`
    /// every `regen_every` frames.
    pub regen_delay: u16,
    pub regen_every: u16,
    pub regen_amount: i32,
    /// Gauge per buff tier (Raya). 0 = this gauge does not buff.
    pub buff_step: i32,
    pub label: &'static str,
}

pub struct Character {
    pub id: CharacterId,
    pub walk_fwd: i32,
    pub walk_back: i32,
    pub run_speed: i32,
    pub jump_x: i32,
    pub jump_y: i32,
    pub hop_x: i32,
    pub hop_y: i32,
    pub gravity: i32,
    pub max_health: i32,
    pub push_w: i32,
    pub stand_h: i32,
    pub crouch_h: i32,
    pub throw_range: i32,
    pub close_range: i32,
    pub color: u32,
    /// Renderer-only silhouette extension. Never participates in collision.
    pub aura: Option<LocalBox>,
    pub gauge: GaugeDef,
    pub poke_heavy: MoveId,
    pub weapon_heavy: MoveId,
    pub space_controls: &'static [MoveId],
    pub reversal: MoveId,
    pub moves: Vec<MoveDef>,
    pub specials: Vec<SpecialRoute>,
    pub names: &'static [(MoveId, &'static str)],
}

impl Character {
    pub fn move_def(&self, id: MoveId) -> Option<&MoveDef> {
        self.moves.iter().find(|m| m.id == id)
    }

    pub fn has_move(&self, id: MoveId) -> bool {
        self.move_def(id).is_some()
    }

    pub fn move_name(&self, id: MoveId) -> &'static str {
        self.names
            .iter()
            .find(|(m, _)| *m == id)
            .map(|(_, n)| *n)
            .unwrap_or_else(|| id.slot_name())
    }

    pub fn hurt_stand(&self) -> LocalBox {
        LocalBox::new(-self.push_w / 2, 0, self.push_w + px(8), self.stand_h)
    }

    pub fn hurt_crouch(&self) -> LocalBox {
        LocalBox::new(-self.push_w / 2, 0, self.push_w + px(8), self.crouch_h)
    }

    /// Route a single button to a normal by stance and distance.
    pub fn select_normal(
        &self,
        crouching: bool,
        airborne: bool,
        button: Btn,
        distance: i32,
    ) -> Option<MoveId> {
        use MoveId::*;
        let close = distance <= self.close_range;
        let id = match (airborne, crouching, button) {
            (true, _, Btn::P) => JP,
            (true, _, Btn::K) => JK,
            (true, _, Btn::S) => JS,
            (true, _, Btn::HS) => JHS,
            (true, _, Btn::FL) => JFL,
            (true, _, Btn::ST) => JST,
            (_, true, Btn::P) => CrP,
            (_, true, Btn::K) => CrK,
            (_, true, Btn::S) => CrS,
            (_, true, Btn::HS) => CrHS,
            (_, true, Btn::FL) => CrFL,
            (_, true, Btn::ST) => CrST,
            (_, _, Btn::P) => StP,
            (_, _, Btn::K) => StK,
            (_, _, Btn::S) => StS,
            (_, _, Btn::HS) if close && self.has_move(StHSClose) => StHSClose,
            (_, _, Btn::HS) => StHS,
            (_, _, Btn::FL) => StFL,
            (_, _, Btn::ST) => StST,
        };
        self.move_def(id).map(|_| id)
    }

    /// Find the best special for the buttons just pressed. `ex` is true when
    /// the S+HS chord completed this frame; then only EX routes are eligible.
    pub fn match_special(
        &self,
        airborne: bool,
        pressed: crate::input::Buttons,
        ex: bool,
        meter: i32,
        gauge: i32,
        buffer: &InputBuffer,
    ) -> Option<MoveId> {
        let mut best: Option<(u8, MoveId)> = None;
        for route in &self.specials {
            if airborne && !route.air_ok {
                continue;
            }
            if !airborne && !route.ground_ok {
                continue;
            }
            if route.ex != ex {
                continue;
            }
            if !route.ex && !route.buttons.iter().any(|b| pressed.get(*b)) {
                continue;
            }
            let Some(mv) = self.move_def(route.move_id) else {
                continue;
            };
            if mv.meter_cost > 0 && meter < mv.meter_cost {
                continue;
            }
            if mv.gauge_cost > 0 && gauge < mv.gauge_cost {
                continue;
            }
            if buffer.motion(route.motion) {
                let rank = route.motion.rank();
                if best.map(|(r, _)| rank > r).unwrap_or(true) {
                    best = Some((rank, route.move_id));
                }
            }
        }
        best.map(|(_, id)| id)
    }

    /// Human-readable input for a special slot, for docs and the help panel.
    pub fn input_for(&self, id: MoveId) -> Option<String> {
        let route = self.specials.iter().find(|r| r.move_id == id)?;
        let btn = if route.ex {
            "S+HS".to_string()
        } else {
            route
                .buttons
                .iter()
                .map(|b| b.label())
                .collect::<Vec<_>>()
                .join("/")
        };
        let motion = route.motion.notation();
        Some(if motion.is_empty() {
            btn
        } else {
            format!("{motion}+{btn}")
        })
    }
}

pub fn all() -> [CharacterId; 2] {
    [CharacterId::Kogan, CharacterId::Raya]
}
