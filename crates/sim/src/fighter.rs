//! Per-fighter state machine.
//!
//! Input priority (ground, not locked):
//!   chord (throw / EX / overhead) → special / super → normal → jump / hop
//!   → backdash / run → walk / crouch / stand
//!
//! Locked states (hitstun, blockstun, attack, knockdown, prejump, thrown,
//! feint) eat the tick except for cancels, rekka follow-ups, feints, and
//! Roman Cancel.

use crate::chars::{Character, CharacterId};
use crate::geom::{px, Aabb, LocalBox, Vec2i};
use crate::input::{Btn, Buttons, Chord, InputBuffer, InputFrame, Motion, CHORD_WINDOW};
use crate::moves::{CancelRule, HitLevel, MoveDef, MoveId, ThrowKind};

pub const METER_MAX: i32 = 1000;
/// Frames of prejump. Releasing up before the last prejump frame hops.
pub const PREJUMP: u8 = 4;
pub const LANDING_RECOVERY: u8 = 2;
/// Hard knockdown: 32 down + 24 getup = 56 frames of oki. The currency.
pub const GETUP_FRAMES: u16 = 24;
pub const KNOCKDOWN_FRAMES: u16 = 32;
/// A grabbed defender may tech a normal throw for this many frames.
pub const THROW_TECH_WINDOW: u8 = 7;
/// A command grab holds this long before it resolves (no tech).
pub const COMMAND_GRAB_HOLD: u8 = 4;
pub const THROW_TECH_FRAMES: u16 = 16;
pub const BACKDASH_FRAMES: u16 = 14;
pub const RC_COST: i32 = 250;
pub const RC_FREEZE_FRAMES: u8 = 6;
/// Feint: special startup cancelled to nothing, then this many frames.
pub const FEINT_RECOVERY: u16 = 8;
/// Special cancels are legal from first active through this many frames
/// after the last active frame. Confirmed, not fished.
pub const CANCEL_LATE_FRAMES: u16 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Stance {
    Stand,
    Crouch,
    Air,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Stand,
    Crouch,
    Walk {
        forward: bool,
    },
    Run,
    BackDash {
        frame: u16,
    },
    Prejump {
        frame: u8,
        dir_x: i32,
        hop: bool,
    },
    Jump {
        air_ok: bool,
        hop: bool,
    },
    Attack {
        move_id: MoveId,
        frame: u16,
        connected: Connect,
    },
    /// FL+ST during a special's startup: the special never came.
    Feint {
        frame: u16,
    },
    Block {
        crouching: bool,
        stun: u8,
    },
    Hit {
        stun: u8,
        knockdown: bool,
    },
    Knockdown {
        frame: u16,
    },
    Getup {
        frame: u16,
    },
    /// Grabbed. Resolves into damage + knockdown unless teched in time.
    Thrown {
        frame: u8,
        techable: bool,
        damage: i32,
        meter: i32,
    },
    ThrowTech {
        frame: u16,
    },
    Landing {
        frame: u16,
        total: u16,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Connect {
    None,
    Hit,
    Block,
}

impl Action {
    pub fn attacking(&self) -> Option<(MoveId, u16, Connect)> {
        match *self {
            Action::Attack {
                move_id,
                frame,
                connected,
            } => Some((move_id, frame, connected)),
            _ => None,
        }
    }

    pub fn in_hitstun(&self) -> bool {
        matches!(self, Action::Hit { .. })
    }

    pub fn in_blockstun(&self) -> bool {
        matches!(self, Action::Block { .. })
    }

    pub fn actionable(&self) -> bool {
        matches!(
            self,
            Action::Stand
                | Action::Crouch
                | Action::Walk { .. }
                | Action::Run
                | Action::Jump { air_ok: true, .. }
        )
    }

    pub fn is_hop(&self) -> bool {
        matches!(
            self,
            Action::Jump { hop: true, .. } | Action::Prejump { hop: true, .. }
        )
    }

    /// Cannot be thrown: airborne handled separately; stunned, downed,
    /// already grabbed, or teching bodies are throw-protected.
    pub fn throw_protected(&self) -> bool {
        matches!(
            self,
            Action::Hit { .. }
                | Action::Block { .. }
                | Action::Knockdown { .. }
                | Action::Getup { .. }
                | Action::Thrown { .. }
                | Action::ThrowTech { .. }
        )
    }

    pub fn name(&self) -> &'static str {
        match self {
            Action::Stand => "stand",
            Action::Crouch => "crouch",
            Action::Walk { forward: true } => "walk",
            Action::Walk { forward: false } => "backwalk",
            Action::Run => "run",
            Action::BackDash { .. } => "backdash",
            Action::Prejump { hop: true, .. } => "prehop",
            Action::Prejump { .. } => "prejump",
            Action::Jump { hop: true, .. } => "hop",
            Action::Jump { .. } => "jump",
            Action::Attack { move_id, .. } => move_id.slot_name(),
            Action::Feint { .. } => "feint",
            Action::Block { .. } => "block",
            Action::Hit { .. } => "hitstun",
            Action::Knockdown { .. } => "down",
            Action::Getup { .. } => "getup",
            Action::Thrown { .. } => "thrown",
            Action::ThrowTech { .. } => "tech",
            Action::Landing { .. } => "land",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fighter {
    pub id: CharacterId,
    pub pos: Vec2i,
    pub vel: Vec2i,
    pub facing_right: bool,
    pub health: i32,
    pub meter: i32,
    /// Character gauge: Kogan chambers, Raya crystal charge.
    pub gauge: i32,
    /// Frames since the gauge was last spent (drives cooldown regen).
    pub gauge_idle: u16,
    pub action: Action,
    pub buffer: InputBuffer,
    pub airborne: bool,
    pub combo: u8,
    pub juggle: u8,
    pub last_move: Option<MoveId>,
    pub gravity_override: Option<i32>,
    /// Landing recovery owed by the current airborne move (uppercut tax).
    pub land_recovery: u8,
    /// Frames spent holding the current channel (consecrate).
    pub channel_frames: u16,
    /// Button edges and chords that landed during hitstop / RC freeze, so a
    /// cancel pressed inside the freeze is not lost.
    pub frozen_presses: Buttons,
    pub frozen_chord: Option<Chord>,
    /// World writes these before `tick_pre_collision`.
    pub last_distance: i32,
    pub has_planted: bool,
}

impl Fighter {
    pub fn spawn(id: CharacterId, x: i32, facing_right: bool) -> Self {
        let data = id.data();
        Self {
            id,
            pos: Vec2i::new(x, 0),
            vel: Vec2i::ZERO,
            facing_right,
            health: data.max_health,
            meter: 0,
            gauge: data.gauge.start,
            gauge_idle: 0,
            action: Action::Stand,
            buffer: InputBuffer::default(),
            airborne: false,
            combo: 0,
            juggle: 0,
            last_move: None,
            gravity_override: None,
            land_recovery: 0,
            channel_frames: 0,
            frozen_presses: Buttons::default(),
            frozen_chord: None,
            last_distance: px(200),
            has_planted: false,
        }
    }

    pub fn data(&self) -> &'static Character {
        self.id.data()
    }

    pub fn input(&self) -> InputFrame {
        self.buffer.latest()
    }

    /// Button edges this tick, including any that landed during a freeze.
    pub fn pressed_now(&self) -> Buttons {
        self.buffer.pressed().or(self.frozen_presses)
    }

    pub fn chord_now(&self) -> Option<Chord> {
        self.buffer.chord().or(self.frozen_chord)
    }

    /// Raya's buff tier from the crystal gauge (0 for bodies without one).
    pub fn buff_tier(&self) -> i32 {
        let step = self.data().gauge.buff_step;
        if step <= 0 {
            0
        } else {
            self.gauge / step
        }
    }

    pub fn pushbox(&self) -> Aabb {
        let d = self.data();
        let h = match self.hurt_stance() {
            Stance::Crouch => d.crouch_h,
            Stance::Air => d.stand_h * 4 / 5,
            Stance::Stand => d.stand_h,
        };
        Aabb::from_center_size(self.pos.x, self.pos.y, d.push_w, h)
    }

    pub fn hurtboxes(&self) -> Vec<Aabb> {
        let d = self.data();
        let local = match self.hurt_stance() {
            Stance::Crouch => d.hurt_crouch(),
            Stance::Air => {
                let h = if self.action.is_hop() {
                    d.stand_h * 7 / 10
                } else {
                    d.stand_h * 4 / 5
                };
                LocalBox::new(-d.push_w / 2, px(8), d.push_w + px(6), h)
            }
            Stance::Stand => d.hurt_stand(),
        };
        vec![local.to_world(self.pos, self.facing_right)]
    }

    /// Visual silhouette outside the hittable body. The renderer may draw
    /// this, but collision intentionally never reads it.
    pub fn visual_aura_box(&self) -> Option<Aabb> {
        self.data()
            .aura
            .map(|aura| aura.to_world(self.pos, self.facing_right))
    }

    pub fn hurt_stance(&self) -> Stance {
        if self.airborne {
            return Stance::Air;
        }
        match &self.action {
            Action::Crouch
            | Action::Block {
                crouching: true, ..
            } => Stance::Crouch,
            Action::Attack { move_id, .. } if move_id.is_crouching() => Stance::Crouch,
            Action::Knockdown { .. } | Action::Getup { .. } => Stance::Crouch,
            _ => Stance::Stand,
        }
    }

    pub fn current_move(&self) -> Option<&'static MoveDef> {
        self.action
            .attacking()
            .and_then(|(id, _, _)| self.data().move_def(id))
    }

    pub fn strike_invuln(&self) -> bool {
        if let Action::Attack { move_id, frame, .. } = self.action {
            if let Some(mv) = self.data().move_def(move_id) {
                return mv.invuln.strike_on(frame);
            }
        }
        // No OTG: a downed body is not hittable until it has risen.
        matches!(
            self.action,
            Action::Knockdown { .. } | Action::Getup { .. } | Action::ThrowTech { .. }
        )
    }

    pub fn throw_invuln(&self) -> bool {
        if let Action::Attack { move_id, frame, .. } = self.action {
            if let Some(mv) = self.data().move_def(move_id) {
                return mv.invuln.throw_on(frame);
            }
        }
        false
    }

    /// True while a pass-through movement special is running.
    pub fn passing_through(&self) -> bool {
        self.current_move().map(|m| m.pass_through).unwrap_or(false)
    }

    pub fn hitboxes(&self) -> Vec<Aabb> {
        let Some((id, frame, connected)) = self.action.attacking() else {
            return Vec::new();
        };
        if connected != Connect::None {
            // One hit per move.
            return Vec::new();
        }
        let Some(mv) = self.data().move_def(id) else {
            return Vec::new();
        };
        mv.hitboxes_on(frame)
            .map(|b| b.to_world(self.pos, self.facing_right))
            .collect()
    }

    pub fn would_block(&self, level: HitLevel) -> bool {
        if self.airborne {
            return false;
        }
        if !matches!(
            self.action,
            Action::Stand
                | Action::Crouch
                | Action::Walk { .. }
                | Action::Run
                | Action::Block { .. }
        ) {
            return false;
        }
        let inp = self.input();
        if !inp.back() {
            return false;
        }
        match level {
            HitLevel::Low => inp.down(),
            HitLevel::High => !inp.down(),
            HitLevel::Mid => true,
        }
    }

    pub fn add_meter(&mut self, amount: i32) {
        self.meter = (self.meter + amount).clamp(0, METER_MAX);
    }

    pub fn spend_meter(&mut self, amount: i32) -> bool {
        if self.meter >= amount {
            self.meter -= amount;
            true
        } else {
            false
        }
    }

    pub fn add_gauge(&mut self, amount: i32) {
        let max = self.data().gauge.max;
        self.gauge = (self.gauge + amount).clamp(0, max);
    }

    pub fn spend_gauge(&mut self, amount: i32) -> bool {
        if amount <= 0 {
            return true;
        }
        if self.gauge >= amount {
            self.gauge -= amount;
            self.gauge_idle = 0;
            true
        } else {
            false
        }
    }

    pub fn face_toward(&mut self, other_x: i32) {
        if matches!(
            self.action,
            Action::Attack { .. }
                | Action::Feint { .. }
                | Action::Hit { .. }
                | Action::Block { .. }
                | Action::Knockdown { .. }
                | Action::Getup { .. }
                | Action::Thrown { .. }
                | Action::ThrowTech { .. }
                | Action::Prejump { .. }
                | Action::BackDash { .. }
        ) {
            return;
        }
        if other_x != self.pos.x {
            self.facing_right = other_x > self.pos.x;
        }
    }

    pub fn buffer_input(&mut self, raw: InputFrame) {
        self.buffer.push(raw);
    }

    /// Called by World on frozen ticks so edges inside hitstop survive.
    pub fn accumulate_frozen(&mut self) {
        self.frozen_presses = self.frozen_presses.or(self.buffer.pressed());
        if let Some(c) = self.buffer.chord() {
            self.frozen_chord = Some(c);
        }
    }

    fn clear_frozen(&mut self) {
        self.frozen_presses = Buttons::default();
        self.frozen_chord = None;
    }

    /// RC cancels an attack after hit, block, or whiff. It does not burst out
    /// of defense: hitstun and blockstun remain binding.
    pub fn try_roman_cancel(&mut self) -> bool {
        if self.chord_now() != Some(Chord::RomanCancel) {
            return false;
        }
        let Action::Attack { move_id, .. } = self.action else {
            return false;
        };
        if move_id == MoveId::Throw || move_id == MoveId::CommandGrab {
            return false;
        }
        if !self.spend_meter(RC_COST) {
            return false;
        }
        self.clear_frozen();
        self.vel = Vec2i::ZERO;
        self.gravity_override = None;
        self.land_recovery = 0;
        self.action = if self.airborne {
            Action::Jump {
                air_ok: true,
                hop: false,
            }
        } else {
            Action::Stand
        };
        true
    }

    /// Step the state machine. Input was buffered by `World` first so global
    /// hitstop and Roman Cancel can inspect the same deterministic frame.
    pub fn tick_pre_collision(&mut self, stage_w: i32) {
        self.tick_gauge();
        self.step_action();
        self.integrate(stage_w);
        self.clear_frozen();
    }

    fn tick_gauge(&mut self) {
        let g = self.data().gauge;
        self.gauge_idle = self.gauge_idle.saturating_add(1);
        if g.regen_amount > 0
            && self.gauge < g.max
            && self.gauge_idle >= g.regen_delay
            && g.regen_every > 0
            && (self.gauge_idle - g.regen_delay).is_multiple_of(g.regen_every)
        {
            self.add_gauge(g.regen_amount);
        }
    }

    fn step_action(&mut self) {
        match self.action.clone() {
            Action::Hit { stun, knockdown } => self.step_hit(stun, knockdown),
            Action::Block { crouching, stun } => {
                if stun == 0 {
                    self.idle_from_input();
                } else {
                    self.action = Action::Block {
                        crouching,
                        stun: stun.saturating_sub(1),
                    };
                }
            }
            Action::Knockdown { frame } => {
                self.airborne = false;
                self.pos.y = 0;
                self.vel = Vec2i::ZERO;
                if frame + 1 >= KNOCKDOWN_FRAMES {
                    self.action = Action::Getup { frame: 0 };
                } else {
                    self.action = Action::Knockdown { frame: frame + 1 };
                }
            }
            Action::Getup { frame } => {
                if frame + 1 >= GETUP_FRAMES {
                    self.idle_from_input();
                } else {
                    self.action = Action::Getup { frame: frame + 1 };
                }
            }
            // Resolved by World (needs both fighters).
            Action::Thrown { .. } => {
                self.vel = Vec2i::ZERO;
            }
            Action::ThrowTech { frame } => {
                self.vel.x = if self.facing_right { -px(4) } else { px(4) };
                if frame + 1 >= THROW_TECH_FRAMES {
                    self.vel.x = 0;
                    self.idle_from_input();
                } else {
                    self.action = Action::ThrowTech { frame: frame + 1 };
                }
            }
            Action::Landing { frame, total } => {
                self.vel = Vec2i::ZERO;
                if frame + 1 >= total {
                    self.idle_from_input();
                } else {
                    self.action = Action::Landing {
                        frame: frame + 1,
                        total,
                    };
                }
            }
            Action::Feint { frame } => {
                self.vel.x = 0;
                if frame + 1 >= FEINT_RECOVERY {
                    self.idle_from_input();
                } else {
                    self.action = Action::Feint { frame: frame + 1 };
                }
            }
            Action::Prejump { frame, dir_x, hop } => {
                // Tap/hold: letting go of up before the jump leaves the
                // ground makes it a hop.
                let hop = hop || !self.input().up();
                if frame + 1 >= PREJUMP {
                    self.begin_jump(dir_x, hop);
                } else {
                    self.action = Action::Prejump {
                        frame: frame + 1,
                        dir_x,
                        hop,
                    };
                }
            }
            Action::BackDash { frame } => {
                if frame == 0 {
                    self.vel.x = if self.facing_right { -px(7) } else { px(7) };
                }
                if frame + 1 >= BACKDASH_FRAMES {
                    self.vel.x = 0;
                    self.idle_from_input();
                } else {
                    if frame > 8 {
                        self.vel.x = self.vel.x * 3 / 4;
                    }
                    self.action = Action::BackDash { frame: frame + 1 };
                }
            }
            Action::Attack {
                move_id,
                frame,
                connected,
            } => self.step_attack(move_id, frame, connected),
            Action::Jump { air_ok, hop } => {
                if air_ok {
                    self.try_air_offense(hop);
                }
            }
            Action::Run => {
                if !self.try_ground_offense() {
                    let inp = self.input();
                    if inp.up() {
                        self.action = Action::Prejump {
                            frame: 0,
                            dir_x: 1,
                            hop: false,
                        };
                    } else if !inp.forward() {
                        self.vel.x = 0;
                        self.idle_from_input();
                    } else {
                        let spd = self.data().run_speed;
                        self.vel.x = if self.facing_right { spd } else { -spd };
                        self.action = Action::Run;
                    }
                }
            }
            Action::Stand | Action::Crouch | Action::Walk { .. } => {
                if !self.try_ground_offense() {
                    self.ground_locomotion();
                }
            }
        }
    }

    fn step_hit(&mut self, stun: u8, knockdown: bool) {
        if stun == 0 {
            if knockdown && !self.airborne {
                self.action = Action::Knockdown { frame: 0 };
            } else if self.airborne {
                self.action = Action::Jump {
                    air_ok: false,
                    hop: false,
                };
            } else {
                self.idle_from_input();
            }
        } else {
            self.action = Action::Hit {
                stun: stun - 1,
                knockdown,
            };
        }
    }

    fn step_attack(&mut self, move_id: MoveId, frame: u16, connected: Connect) {
        let Some(mv) = self.data().move_def(move_id).cloned() else {
            self.idle_from_input();
            return;
        };

        if let Some(chord) = self.chord_now() {
            match chord {
                Chord::Feint if mv.feintable && mv.in_startup(frame) => {
                    self.vel = Vec2i::ZERO;
                    self.gravity_override = None;
                    self.land_recovery = 0;
                    self.channel_frames = 0;
                    self.action = Action::Feint { frame: 0 };
                    return;
                }
                // Kara: the first button of a chord started a move a frame or
                // two ago. The chord replaces it and refunds what it spent.
                Chord::Throw | Chord::Ex | Chord::Overhead
                    if !self.airborne
                        && connected == Connect::None
                        && frame < CHORD_WINDOW as u16
                        && !matches!(
                            mv.id,
                            MoveId::Throw
                                | MoveId::Overhead
                                | MoveId::ExA
                                | MoveId::ExB
                                | MoveId::Super
                        ) =>
                {
                    let refund_gauge = mv.gauge_cost;
                    let refund_meter = mv.meter_cost;
                    self.add_gauge(refund_gauge);
                    self.add_meter(refund_meter);
                    if self.try_chord(chord) {
                        return;
                    }
                    let _ = self.spend_gauge(refund_gauge);
                    let _ = self.spend_meter(refund_meter);
                }
                _ => {}
            }
        }

        let pressed = self.pressed_now();

        // Rekka follow-ups: legal on hit, block, or whiff inside the window.
        if pressed.any() && !mv.followups.is_empty() {
            for b in pressed.iter() {
                if let Some(next) = mv.followup_for(b, frame) {
                    self.start_move(next);
                    return;
                }
            }
        }

        if self.try_cancel(&mv, frame, connected, pressed) {
            return;
        }

        // Channel (consecrate): hold the button on the channel frame.
        if let Some(ch) = mv.channel {
            if frame == mv.first_active()
                && self.input().buttons.get(ch.button)
                && self.channel_frames < ch.max_frames
            {
                self.channel_frames += 1;
                self.add_gauge(ch.gauge_per_frame);
                return;
            }
        }

        let next = frame + 1;
        if mv.vel_frames > 0 && next >= mv.vel_frames as u16 && !self.airborne {
            self.vel.x = 0;
        }
        if mv.finished(next) {
            self.gravity_override = None;
            self.channel_frames = 0;
            if self.airborne {
                self.action = Action::Jump {
                    air_ok: false,
                    hop: false,
                };
            } else {
                self.vel.x = 0;
                self.idle_from_input();
            }
        } else {
            self.action = Action::Attack {
                move_id,
                frame: next,
                connected,
            };
        }
    }

    fn try_cancel(
        &mut self,
        mv: &MoveDef,
        frame: u16,
        connected: Connect,
        pressed: Buttons,
    ) -> bool {
        let can = match (mv.cancel, connected) {
            (CancelRule::Never, _) => false,
            (CancelRule::OnHit, Connect::Hit) => true,
            (CancelRule::OnHitOrBlock, Connect::Hit | Connect::Block) => true,
            _ => false,
        };
        if !can || frame < mv.first_active() || frame > mv.last_active() + CANCEL_LATE_FRAMES {
            return false;
        }
        let ex = self.chord_now() == Some(Chord::Ex);
        if !pressed.any() && !ex {
            return false;
        }
        if let Some(id) = self.data().match_special(
            self.airborne,
            pressed,
            ex,
            self.meter,
            self.gauge,
            &self.buffer,
        ) {
            if id.is_rekka() && id != MoveId::Rekka1 {
                return false;
            }
            self.start_move(id);
            return true;
        }
        false
    }

    /// Throw / EX / overhead from a neutral-ish state.
    fn try_chord(&mut self, chord: Chord) -> bool {
        match chord {
            Chord::Throw => {
                if self.data().has_move(MoveId::Throw) {
                    self.start_move(MoveId::Throw);
                    return true;
                }
            }
            Chord::Overhead => {
                if self.data().has_move(MoveId::Overhead) {
                    self.start_move(MoveId::Overhead);
                    return true;
                }
            }
            Chord::Ex => {
                if let Some(id) = self.data().match_special(
                    false,
                    Buttons::default(),
                    true,
                    self.meter,
                    self.gauge,
                    &self.buffer,
                ) {
                    self.start_move(id);
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    fn try_ground_offense(&mut self) -> bool {
        let inp = self.input();
        let pressed = self.pressed_now();

        if let Some(chord) = self.chord_now() {
            if self.try_chord(chord) {
                return true;
            }
        }

        if pressed.any() {
            let mut id = self.data().match_special(
                false,
                pressed,
                false,
                self.meter,
                self.gauge,
                &self.buffer,
            );
            // Plant-or-detonate: the shot slot detonates a planted crystal.
            if id == Some(MoveId::ShotA)
                && self.has_planted
                && self.data().has_move(MoveId::Detonate)
            {
                id = Some(MoveId::Detonate);
            }
            if let Some(id) = id {
                if id.is_rekka() && id != MoveId::Rekka1 {
                    // Parts 2–3 only follow part 1.
                } else {
                    self.start_move(id);
                    return true;
                }
            }
            if let Some(id) = self.pick_normal(pressed) {
                self.start_move(id);
                return true;
            }
        }

        if inp.up() {
            let dir_x = if inp.forward() {
                1
            } else if inp.back() {
                -1
            } else {
                0
            };
            self.action = Action::Prejump {
                frame: 0,
                dir_x,
                hop: false,
            };
            self.vel.x = 0;
            return true;
        }

        if self.buffer.motion(Motion::BackDash) && !self.airborne {
            self.action = Action::BackDash { frame: 0 };
            return true;
        }
        if self.buffer.motion(Motion::ForwardDash) && inp.forward() {
            self.action = Action::Run;
            let spd = self.data().run_speed;
            self.vel.x = if self.facing_right { spd } else { -spd };
            return true;
        }
        false
    }

    fn try_air_offense(&mut self, _hop: bool) {
        let pressed = self.pressed_now();
        if !pressed.any() {
            return;
        }
        if let Some(id) =
            self.data()
                .match_special(true, pressed, false, self.meter, self.gauge, &self.buffer)
        {
            self.start_move(id);
            return;
        }
        if let Some(id) = self.pick_normal(pressed) {
            self.start_move(id);
        }
    }

    fn pick_normal(&self, pressed: Buttons) -> Option<MoveId> {
        // Priority when two land the same frame: heavier family first, so a
        // sloppy chord still yields the bigger button.
        for b in [Btn::HS, Btn::ST, Btn::S, Btn::FL, Btn::K, Btn::P] {
            if !pressed.get(b) {
                continue;
            }
            if let Some(id) = self.data().select_normal(
                self.input().down() && !self.airborne,
                self.airborne,
                b,
                self.last_distance,
            ) {
                return Some(id);
            }
        }
        None
    }

    fn ground_locomotion(&mut self) {
        let inp = self.input();
        let d = self.data();
        if inp.down() {
            self.vel.x = 0;
            self.action = Action::Crouch;
            return;
        }
        if inp.forward() {
            self.vel.x = if self.facing_right {
                d.walk_fwd
            } else {
                -d.walk_fwd
            };
            self.action = Action::Walk { forward: true };
        } else if inp.back() {
            self.vel.x = if self.facing_right {
                -d.walk_back
            } else {
                d.walk_back
            };
            self.action = Action::Walk { forward: false };
        } else {
            self.vel.x = 0;
            self.action = Action::Stand;
        }
    }

    fn idle_from_input(&mut self) {
        self.gravity_override = None;
        if self.airborne {
            self.action = Action::Jump {
                air_ok: false,
                hop: false,
            };
        } else {
            self.ground_locomotion();
        }
    }

    fn begin_jump(&mut self, dir_x: i32, hop: bool) {
        let d = self.data();
        let signed = if self.facing_right { dir_x } else { -dir_x };
        if hop {
            self.vel.x = signed * d.hop_x;
            self.vel.y = d.hop_y;
        } else {
            self.vel.x = signed * d.jump_x;
            self.vel.y = d.jump_y;
        }
        self.airborne = true;
        self.action = Action::Jump { air_ok: true, hop };
    }

    pub fn start_move(&mut self, id: MoveId) {
        self.last_move = Some(id);
        self.channel_frames = 0;
        if let Some(mv) = self.data().move_def(id) {
            self.vel.x = if self.facing_right {
                mv.vel_x
            } else {
                -mv.vel_x
            };
            if mv.vel_y != 0 {
                self.vel.y = mv.vel_y;
                self.airborne = true;
                self.land_recovery = mv.land_recovery;
            }
            self.gravity_override = mv.gravity_override;
            if mv.meter_cost > 0 {
                let _ = self.spend_meter(mv.meter_cost);
            }
            if mv.gauge_cost > 0 {
                let _ = self.spend_gauge(mv.gauge_cost);
            }
        }
        self.action = Action::Attack {
            move_id: id,
            frame: 0,
            connected: Connect::None,
        };
    }

    fn integrate(&mut self, stage_w: i32) {
        if self.airborne {
            let g = self.gravity_override.unwrap_or(self.data().gravity);
            self.vel.y -= g;
        }
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        let half = self.data().push_w / 2;
        self.pos.x = self.pos.x.clamp(half, stage_w - half);

        if self.pos.y <= 0 {
            self.pos.y = 0;
            if self.airborne {
                self.airborne = false;
                self.vel = Vec2i::ZERO;
                self.gravity_override = None;
                if let Action::Hit {
                    knockdown: true, ..
                } = self.action
                {
                    self.action = Action::Knockdown { frame: 0 };
                } else {
                    let total = LANDING_RECOVERY.max(self.land_recovery) as u16;
                    self.action = Action::Landing { frame: 0, total };
                }
                self.land_recovery = 0;
            }
        }
    }

    pub fn mark_connected(&mut self, hit: bool) {
        if let Action::Attack { connected, .. } = &mut self.action {
            *connected = if hit { Connect::Hit } else { Connect::Block };
        }
    }

    pub fn apply_hit(
        &mut self,
        stun: u8,
        knockdown: bool,
        launch: i32,
        push: i32,
        from_right: bool,
    ) {
        self.combo = self.combo.saturating_add(1);
        let dir = if from_right { -1 } else { 1 };
        // Pushback is a one-shot displacement. Leaving it as velocity
        // would slide the defender out of link range over the whole stun.
        self.pos.x += dir * push;
        if launch > 0 {
            self.vel.x = dir * (push / 4);
            self.vel.y = launch;
            self.airborne = true;
        } else if !self.airborne {
            self.vel = Vec2i::ZERO;
        }
        self.action = Action::Hit { stun, knockdown };
        self.gravity_override = None;
        self.land_recovery = 0;
        self.channel_frames = 0;
    }

    pub fn apply_block(&mut self, stun: u8, push: i32, from_right: bool, crouching: bool) {
        self.combo = 0;
        let dir = if from_right { -1 } else { 1 };
        self.pos.x += dir * push;
        if !self.airborne {
            self.vel = Vec2i::ZERO;
        }
        self.action = Action::Block { crouching, stun };
    }

    pub fn apply_grab(&mut self, kind: ThrowKind, damage: i32, meter: i32) {
        self.vel = Vec2i::ZERO;
        self.gravity_override = None;
        self.channel_frames = 0;
        self.action = Action::Thrown {
            frame: 0,
            techable: kind == ThrowKind::Normal,
            damage,
            meter,
        };
    }

    pub fn reset_combo(&mut self) {
        self.combo = 0;
        self.juggle = 0;
    }
}
