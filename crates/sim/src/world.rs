//! Match world: two fighters, projectiles, hitstop, events. One round.
//!
//! `World::tick(p1, p2)` is the whole sim: a pure function of the previous
//! state and two input frames. No floats, no clock, no I/O.

use std::hash::{Hash, Hasher};

use crate::chars::CharacterId;
use crate::collision::{projectile_hits, resolve_push, scale_damage, strike_hits, HitResult};
use crate::fighter::{
    Action, Connect, Fighter, COMMAND_GRAB_HOLD, GETUP_FRAMES, RC_FREEZE_FRAMES, THROW_TECH_WINDOW,
};
use crate::geom::{px, Aabb, Vec2i};
use crate::input::{Btn, Buttons, InputFrame};
use crate::moves::{HitLevel, MoveId, ProjectileDef, ProjectileKind, ShotBehavior, ThrowKind};

pub const STAGE_W: i32 = px(760);
pub const ROUND_TIME: u32 = 99 * 60;
pub const P1_START_X: i32 = px(260);
pub const P2_START_X: i32 = px(500);
/// Frames a detonation's blast stays live.
pub const DETONATE_FRAMES: u16 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShotState {
    Flying,
    Hanging,
    Planted { armed: bool, timer: u16 },
    Detonating { frame: u16 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Projectile {
    pub owner: usize,
    pub pos: Vec2i,
    pub vel: Vec2i,
    pub facing_right: bool,
    pub ttl: u16,
    pub def: ProjectileDef,
    pub state: ShotState,
    /// Buffed values resolved at spawn.
    pub damage: i32,
    pub arm_after: u16,
    pub armed_life: u16,
}

impl Projectile {
    pub fn hitbox(&self) -> Aabb {
        match self.state {
            ShotState::Detonating { .. } => self
                .def
                .blast
                .unwrap_or(self.def.hitbox)
                .to_world(self.pos, self.facing_right),
            _ => self.def.hitbox.to_world(self.pos, self.facing_right),
        }
    }

    /// Can this shot hurt someone right now? A tossed crystal is inert until
    /// it has planted and armed.
    pub fn live(&self) -> bool {
        match self.state {
            ShotState::Flying => !matches!(self.def.behavior, ShotBehavior::Plant { .. }),
            ShotState::Hanging => true,
            ShotState::Planted { armed, .. } => armed,
            ShotState::Detonating { .. } => true,
        }
    }

    pub fn planted(&self) -> bool {
        matches!(self.state, ShotState::Planted { .. })
    }

    pub fn armed(&self) -> bool {
        matches!(self.state, ShotState::Planted { armed: true, .. })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventKind {
    Hit,
    Block,
    Punish,
    /// Grab connected; damage lands when the hold resolves.
    Grab,
    Throw,
    ThrowTech,
    Knockdown,
    RomanCancel,
    Feint,
    ProjectileGuard,
    Clash,
    Plant,
    Armed,
    Detonate,
    KO,
    TimeOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CombatEvent {
    pub kind: EventKind,
    pub attacker: usize,
    pub move_id: Option<MoveId>,
    pub damage: i32,
    pub advantage: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RoundOutcome {
    Winner(usize),
    Draw,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DummyMode {
    Stand,
    Crouch,
    BlockAll,
    Jump,
    /// 623+S on the first actionable frame after a knockdown.
    WakeupDp,
    /// P on the first actionable frame after a knockdown (the button press
    /// a meaty strike is meant to beat).
    WakeupP,
    /// Mash P+K while grabbed.
    Tech,
    CpuOff,
}

impl DummyMode {
    pub const ALL: [DummyMode; 8] = [
        Self::Stand,
        Self::Crouch,
        Self::BlockAll,
        Self::Jump,
        Self::WakeupDp,
        Self::WakeupP,
        Self::Tech,
        Self::CpuOff,
    ];

    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|m| *m == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Stand => "STAND",
            Self::Crouch => "CROUCH",
            Self::BlockAll => "BLOCK",
            Self::Jump => "JUMP",
            Self::WakeupDp => "WAKEUP 623",
            Self::WakeupP => "WAKEUP P",
            Self::Tech => "TECH",
            Self::CpuOff => "P2",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct World {
    pub fighters: [Fighter; 2],
    pub projectiles: Vec<Projectile>,
    pub frame: u32,
    pub hitstop: u8,
    pub rc_freeze: u8,
    pub time_left: u32,
    pub events: Vec<CombatEvent>,
    /// Measured frame advantage of the last exchange, from `adv_owner`'s
    /// point of view (positive = the attacker acts first).
    pub last_advantage: i32,
    pub adv_owner: usize,
    adv_pending: Option<(usize, Option<u32>, Option<u32>)>,
    pub dummy: DummyMode,
    dummy_wake: bool,
    pub p1_char: CharacterId,
    pub p2_char: CharacterId,
    pub outcome: Option<RoundOutcome>,
}

impl World {
    pub fn new(p1: CharacterId, p2: CharacterId) -> Self {
        Self {
            fighters: [
                Fighter::spawn(p1, P1_START_X, true),
                Fighter::spawn(p2, P2_START_X, false),
            ],
            projectiles: Vec::new(),
            frame: 0,
            hitstop: 0,
            rc_freeze: 0,
            time_left: ROUND_TIME,
            events: Vec::new(),
            last_advantage: 0,
            adv_owner: 0,
            adv_pending: None,
            dummy: DummyMode::CpuOff,
            dummy_wake: false,
            p1_char: p1,
            p2_char: p2,
            outcome: None,
        }
    }

    pub fn training(p1: CharacterId, p2: CharacterId) -> Self {
        let mut w = Self::new(p1, p2);
        w.dummy = DummyMode::Stand;
        w
    }

    pub fn reset(&mut self) {
        let dummy = self.dummy;
        *self = Self::new(self.p1_char, self.p2_char);
        self.dummy = dummy;
    }

    pub fn swap_p1(&mut self) {
        self.p1_char = self.p1_char.next();
        self.reset();
    }

    pub fn swap_p2(&mut self) {
        self.p2_char = self.p2_char.next();
        self.reset();
    }

    /// FNV-1a over the whole state. Same inputs → same hash. This is what a
    /// rollback layer would compare for desync detection.
    pub fn state_hash(&self) -> u64 {
        let mut h = Fnv1a(0xcbf2_9ce4_8422_2325);
        self.hash(&mut h);
        h.0
    }

    pub fn tick(&mut self, p1: InputFrame, p2_raw: InputFrame) {
        self.events.clear();
        let p2 = self.filter_dummy(p2_raw);

        self.fighters[0].buffer_input(p1);
        self.fighters[1].buffer_input(p2);

        if self.hitstop > 0 {
            self.hitstop -= 1;
            for f in &mut self.fighters {
                f.accumulate_frozen();
            }
            return;
        }

        if self.rc_freeze > 0 {
            self.rc_freeze -= 1;
            for f in &mut self.fighters {
                f.accumulate_frozen();
            }
            return;
        }

        let rc0 = self.fighters[0].try_roman_cancel();
        let rc1 = self.fighters[1].try_roman_cancel();
        if rc0 || rc1 {
            self.rc_freeze = RC_FREEZE_FRAMES;
            for (i, used) in [rc0, rc1].into_iter().enumerate() {
                if used {
                    self.push_event(EventKind::RomanCancel, i, None, 0);
                }
            }
            return;
        }

        self.frame += 1;
        if self.time_left > 0 && self.outcome.is_none() {
            self.time_left -= 1;
        }

        let x0 = self.fighters[0].pos.x;
        let x1 = self.fighters[1].pos.x;
        let dist = (x0 - x1).abs();
        for i in 0..2 {
            self.fighters[i].last_distance = dist;
            self.fighters[i].has_planted =
                self.projectiles.iter().any(|p| p.owner == i && p.planted());
        }
        self.fighters[0].face_toward(x1);
        self.fighters[1].face_toward(x0);

        self.step_thrown();

        let before = [
            self.fighters[0].action.clone(),
            self.fighters[1].action.clone(),
        ];
        self.fighters[0].tick_pre_collision(STAGE_W);
        self.fighters[1].tick_pre_collision(STAGE_W);
        for (i, prev) in before.iter().enumerate() {
            if matches!(self.fighters[i].action, Action::Feint { frame: 0 })
                && !matches!(prev, Action::Feint { .. })
            {
                let mv = prev.attacking().map(|(id, _, _)| id);
                self.push_event(EventKind::Feint, i, mv, 0);
            }
        }

        self.spawn_projectiles();
        self.trigger_detonations();
        self.step_projectiles();
        {
            let (a, rest) = self.fighters.split_at_mut(1);
            resolve_push(&mut a[0], &mut rest[0], STAGE_W);
        }
        self.resolve_strikes();
        self.decay_combos();
        self.measure_advantage();
        self.check_round_end();
    }

    fn push_event(
        &mut self,
        kind: EventKind,
        attacker: usize,
        move_id: Option<MoveId>,
        damage: i32,
    ) {
        self.events.push(CombatEvent {
            kind,
            attacker,
            move_id,
            damage,
            advantage: self.last_advantage,
        });
    }

    fn filter_dummy(&mut self, human: InputFrame) -> InputFrame {
        match self.dummy {
            DummyMode::CpuOff => human,
            DummyMode::Stand => InputFrame::default(),
            DummyMode::Crouch => InputFrame::dir(2),
            DummyMode::Jump => InputFrame::dir(8),
            DummyMode::BlockAll => {
                // Reacts rather than pre-holds, so it does not walk out of
                // range during a long startup. Input is facing-relative:
                // 4 = back, 1 = down-back. Crouch block unless the incoming
                // attack is High.
                let f0 = &self.fighters[0];
                let incoming = f0
                    .current_move()
                    .zip(f0.action.attacking())
                    .filter(|(m, (_, frame, connected))| {
                        *connected == Connect::None
                            && !m.is_throw()
                            && *frame + 6 >= m.first_active()
                            && *frame < m.last_active()
                    })
                    .map(|(m, _)| m.level);
                let shot = self
                    .projectiles
                    .iter()
                    .find(|p| {
                        p.owner == 0
                            && p.live()
                            && (p.pos.x - self.fighters[1].pos.x).abs() < px(120)
                    })
                    .map(|p| p.def.level);
                let dir = match (incoming, shot, self.fighters[1].action.in_blockstun()) {
                    (Some(HitLevel::High), _, _) | (None, Some(HitLevel::High), _) => 4,
                    (Some(_), _, _) | (None, Some(_), _) => 1,
                    (None, None, true) => 1,
                    (None, None, false) => 5,
                };
                InputFrame::dir(dir)
            }
            DummyMode::WakeupDp => {
                let f = &self.fighters[1];
                match f.action {
                    Action::Getup { frame } if frame + 3 == GETUP_FRAMES => {
                        self.dummy_wake = true;
                        InputFrame::dir(6)
                    }
                    Action::Getup { frame } if frame + 2 == GETUP_FRAMES => InputFrame::dir(2),
                    Action::Getup { frame } if frame + 1 == GETUP_FRAMES => InputFrame::dir(3),
                    Action::Stand | Action::Crouch | Action::Walk { .. } if self.dummy_wake => {
                        self.dummy_wake = false;
                        InputFrame::dir_press(3, Btn::S)
                    }
                    _ => InputFrame::default(),
                }
            }
            DummyMode::WakeupP => {
                let f = &self.fighters[1];
                match f.action {
                    Action::Getup { frame } if frame + 1 == GETUP_FRAMES => {
                        self.dummy_wake = true;
                        InputFrame::default()
                    }
                    Action::Stand | Action::Crouch | Action::Walk { .. } if self.dummy_wake => {
                        self.dummy_wake = false;
                        InputFrame::press(Btn::P)
                    }
                    _ => InputFrame::default(),
                }
            }
            DummyMode::Tech => match self.fighters[1].action {
                Action::Thrown { .. } => InputFrame {
                    dir: 5,
                    buttons: Buttons::two(Btn::P, Btn::K),
                },
                _ => InputFrame::default(),
            },
        }
    }

    /// Resolve grabs: tech window for normal throws, hold for command grabs.
    fn step_thrown(&mut self) {
        for def in 0..2 {
            let atk = 1 - def;
            let Action::Thrown {
                frame,
                techable,
                damage,
                meter,
            } = self.fighters[def].action.clone()
            else {
                continue;
            };
            let hold = if techable {
                THROW_TECH_WINDOW
            } else {
                COMMAND_GRAB_HOLD
            };
            if techable {
                let recent = self.fighters[def].buffer.pressed_recently(hold + 1);
                if recent.p && recent.k {
                    self.fighters[atk].action = Action::ThrowTech { frame: 0 };
                    self.fighters[def].action = Action::ThrowTech { frame: 0 };
                    self.push_event(EventKind::ThrowTech, def, Some(MoveId::Throw), 0);
                    continue;
                }
            }
            if frame + 1 >= hold {
                let from_right = self.fighters[atk].pos.x > self.fighters[def].pos.x;
                let dmg = scale_damage(damage, self.fighters[def].combo);
                self.fighters[def].health -= dmg;
                self.fighters[def].apply_hit(20, true, px(7), px(24), from_right);
                self.fighters[atk].add_meter(meter);
                self.fighters[def].add_meter(meter / 4);
                let mv = self.fighters[atk].action.attacking().map(|(id, _, _)| id);
                self.push_event(EventKind::Throw, atk, mv, dmg);
                self.check_ko(atk, def);
            } else {
                self.fighters[def].action = Action::Thrown {
                    frame: frame + 1,
                    techable,
                    damage,
                    meter,
                };
            }
        }
    }

    fn spawn_projectiles(&mut self) {
        for i in 0..2 {
            let f = &self.fighters[i];
            let Some((id, frame, Connect::None)) = f.action.attacking() else {
                continue;
            };
            let Some(mv) = f.data().move_def(id) else {
                continue;
            };
            let Some(def) = mv.projectile else {
                continue;
            };
            if frame != mv.startup as u16 {
                continue;
            }
            // One shot per owner per type.
            if self
                .projectiles
                .iter()
                .any(|p| p.owner == i && p.def.kind == def.kind)
            {
                continue;
            }
            let dir = if f.facing_right { 1 } else { -1 };
            let spawn = def.spawn.to_world(f.pos, f.facing_right);
            let tier = f.buff_tier();
            let (arm_after, armed_life) = match def.behavior {
                ShotBehavior::Plant {
                    arm_after,
                    armed_life,
                } => (
                    (arm_after as i32 - 6 * tier).max(4) as u16,
                    armed_life + 30 * tier as u16,
                ),
                _ => (0, 0),
            };
            let damage = def.damage * (100 + 20 * tier) / 100;
            let state = match def.behavior {
                ShotBehavior::Travel | ShotBehavior::Plant { .. } => ShotState::Flying,
                ShotBehavior::Hang => ShotState::Hanging,
            };
            let lifetime = match def.behavior {
                ShotBehavior::Hang => def.lifetime + 20 * tier as u16,
                _ => def.lifetime,
            };
            self.projectiles.push(Projectile {
                owner: i,
                pos: Vec2i::new(spawn.left, spawn.bottom),
                vel: Vec2i::new(dir * def.vel_x, def.vel_y),
                facing_right: f.facing_right,
                ttl: lifetime,
                def,
                state,
                damage,
                arm_after,
                armed_life,
            });
            if matches!(def.behavior, ShotBehavior::Plant { .. }) {
                self.push_event(EventKind::Plant, i, Some(id), 0);
            }
        }
    }

    /// Raya's 214+S on a planted, armed crystal blows it early.
    fn trigger_detonations(&mut self) {
        for i in 0..2 {
            let f = &self.fighters[i];
            let Some((MoveId::Detonate, frame, _)) = f.action.attacking() else {
                continue;
            };
            let Some(mv) = f.data().move_def(MoveId::Detonate) else {
                continue;
            };
            if frame != mv.startup as u16 {
                continue;
            }
            let mut fired = false;
            for p in &mut self.projectiles {
                if p.owner == i && p.armed() {
                    p.state = ShotState::Detonating { frame: 0 };
                    fired = true;
                }
            }
            if fired {
                self.push_event(EventKind::Detonate, i, Some(MoveId::Detonate), 0);
            }
        }
    }

    fn step_projectiles(&mut self) {
        let mut hits: Vec<(usize, HitResult)> = Vec::new();
        let mut consumed = Vec::new();

        // Motion and lifetime.
        let mut newly_armed = Vec::new();
        for (pi, proj) in self.projectiles.iter_mut().enumerate() {
            match proj.state {
                ShotState::Flying => {
                    proj.pos.x += proj.vel.x;
                    proj.pos.y += proj.vel.y;
                    proj.vel.y -= proj.def.gravity;
                    if proj.pos.y <= 0 {
                        proj.pos.y = 0;
                        match proj.def.behavior {
                            ShotBehavior::Plant { .. } => {
                                proj.vel = Vec2i::ZERO;
                                proj.state = ShotState::Planted {
                                    armed: false,
                                    timer: 0,
                                };
                            }
                            _ if proj.def.gravity != 0 || proj.def.vel_y != 0 => {
                                consumed.push(pi);
                                continue;
                            }
                            _ => {}
                        }
                    }
                    if proj.ttl == 0 || proj.pos.x < 0 || proj.pos.x > STAGE_W {
                        consumed.push(pi);
                        continue;
                    }
                    proj.ttl -= 1;
                }
                ShotState::Hanging => {
                    if proj.ttl == 0 {
                        consumed.push(pi);
                        continue;
                    }
                    proj.ttl -= 1;
                }
                ShotState::Planted { armed, timer } => {
                    let t = timer + 1;
                    if !armed && t >= proj.arm_after {
                        proj.state = ShotState::Planted {
                            armed: true,
                            timer: 0,
                        };
                        newly_armed.push(proj.owner);
                    } else if armed && t >= proj.armed_life {
                        consumed.push(pi);
                        continue;
                    } else {
                        proj.state = ShotState::Planted { armed, timer: t };
                    }
                }
                ShotState::Detonating { frame } => {
                    if frame + 1 >= DETONATE_FRAMES {
                        consumed.push(pi);
                        continue;
                    }
                    proj.state = ShotState::Detonating { frame: frame + 1 };
                }
            }
        }
        for owner in newly_armed {
            self.push_event(EventKind::Armed, owner, None, 0);
        }

        // Shot vs shot: same class, both die.
        for i in 0..self.projectiles.len() {
            for j in (i + 1)..self.projectiles.len() {
                let (a, b) = (&self.projectiles[i], &self.projectiles[j]);
                if a.owner != b.owner
                    && a.live()
                    && b.live()
                    && a.def.kind.clash_class() == b.def.kind.clash_class()
                    && a.hitbox().overlaps(b.hitbox())
                {
                    consumed.push(i);
                    consumed.push(j);
                    self.events.push(CombatEvent {
                        kind: EventKind::Clash,
                        attacker: a.owner,
                        move_id: None,
                        damage: 0,
                        advantage: self.last_advantage,
                    });
                }
            }
        }

        // A guard active this frame consumes shots before fighter hit tests.
        for fighter_index in 0..2 {
            let Some(mv) = self.fighters[fighter_index].current_move() else {
                continue;
            };
            if !mv.projectile_guard {
                continue;
            }
            let guards = self.fighters[fighter_index].hitboxes();
            if guards.is_empty() {
                continue;
            }
            for (projectile_index, projectile) in self.projectiles.iter().enumerate() {
                if projectile.owner != fighter_index
                    && guards
                        .iter()
                        .any(|guard| guard.overlaps(projectile.hitbox()))
                {
                    consumed.push(projectile_index);
                    self.events.push(CombatEvent {
                        kind: EventKind::ProjectileGuard,
                        attacker: fighter_index,
                        move_id: Some(mv.id),
                        damage: 0,
                        advantage: self.last_advantage,
                    });
                }
            }
        }

        // Hit tests.
        let snaps: Vec<(usize, usize, Aabb, ProjectileDef, i32, ShotState)> = self
            .projectiles
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.owner, p.hitbox(), p.def, p.damage, p.state))
            .collect();
        for (pi, owner, hitbox, def, damage, state) in snaps {
            if consumed.contains(&pi) {
                continue;
            }
            if !self.projectiles[pi].live() {
                continue;
            }
            let defn = 1 - owner;
            if let Some(mut hit) =
                projectile_hits(owner, hitbox, &def, damage, &self.fighters[defn])
            {
                hit.attacker = owner;
                hit.defender = defn;
                hits.push((pi, hit));
                match state {
                    // An armed crystal touched: it detonates and the blast
                    // is what hits. Keep it for its blast frames.
                    ShotState::Planted { armed: true, .. } => {
                        self.projectiles[pi].state = ShotState::Detonating { frame: 0 };
                        self.events.push(CombatEvent {
                            kind: EventKind::Detonate,
                            attacker: owner,
                            move_id: None,
                            damage: 0,
                            advantage: self.last_advantage,
                        });
                    }
                    // A blast hits once.
                    ShotState::Detonating { .. } => consumed.push(pi),
                    _ => consumed.push(pi),
                }
            }
        }
        consumed.sort_unstable();
        consumed.dedup();
        for i in consumed.into_iter().rev() {
            if i < self.projectiles.len() {
                self.projectiles.remove(i);
            }
        }
        for (_, hit) in hits {
            self.apply_hit(hit);
        }
    }

    fn resolve_strikes(&mut self) {
        let h01 = strike_hits(&self.fighters[0], &self.fighters[1]).map(|mut h| {
            h.attacker = 0;
            h.defender = 1;
            h
        });
        let h10 = strike_hits(&self.fighters[1], &self.fighters[0]).map(|mut h| {
            h.attacker = 1;
            h.defender = 0;
            h
        });
        match (h01, h10) {
            (Some(a), Some(b)) => {
                let a_throw = a.throw != ThrowKind::None;
                let b_throw = b.throw != ThrowKind::None;
                if a_throw && b_throw {
                    // Simultaneous grabs tech.
                    self.fighters[0].action = Action::ThrowTech { frame: 0 };
                    self.fighters[1].action = Action::ThrowTech { frame: 0 };
                    self.push_event(EventKind::ThrowTech, 0, Some(MoveId::Throw), 0);
                } else if a_throw {
                    // Hit beats throw.
                    self.apply_hit(b);
                } else if b_throw {
                    self.apply_hit(a);
                } else {
                    self.apply_hit(a);
                    self.apply_hit(b);
                }
            }
            (Some(h), None) | (None, Some(h)) => self.apply_hit(h),
            (None, None) => {}
        }
    }

    fn apply_hit(&mut self, hit: HitResult) {
        let atk = hit.attacker;
        let def = hit.defender;
        let from_right = self.fighters[atk].pos.x > self.fighters[def].pos.x;
        let crouching = self.fighters[def].input().down();

        if hit.throw != ThrowKind::None {
            // No hitstop on the grab itself: the hold is the beat, and the
            // tech window counts from the frame the hands land.
            self.fighters[atk].mark_connected(true);
            self.fighters[def].apply_grab(hit.throw, hit.damage, hit.meter);
            self.push_event(EventKind::Grab, atk, hit.move_id, 0);
            return;
        }

        let punish = matches!(
            self.fighters[def].action,
            Action::Attack { .. }
                | Action::Landing { .. }
                | Action::BackDash { .. }
                | Action::Feint { .. }
        ) && !hit.blocked;

        self.fighters[atk].mark_connected(!hit.blocked);
        self.adv_pending = Some((atk, None, None));

        if hit.blocked {
            let chip = hit.chip;
            self.fighters[def].health = (self.fighters[def].health - chip).max(1);
            self.fighters[def].apply_block(hit.blockstun, hit.push_block, from_right, crouching);
            self.fighters[atk].add_meter(hit.meter / 2);
            self.fighters[def].add_meter(hit.meter / 3);
            self.hitstop = hit.blockstop;
            self.push_event(EventKind::Block, atk, hit.move_id, chip);
        } else {
            let already = self.fighters[def].combo;
            let dmg = scale_damage(hit.damage, already);
            // Subsequent combo hits shove less so a 2f link doesn't walk itself
            // out of range — same reason ST jabs stay glued at the corner.
            let push = if already > 0 {
                hit.push_hit / 2
            } else {
                hit.push_hit
            };
            self.fighters[def].health -= dmg;
            self.fighters[def].apply_hit(hit.hitstun, hit.knockdown, hit.launch, push, from_right);
            self.fighters[atk].add_meter(hit.meter);
            self.fighters[def].add_meter(hit.meter / 4);
            self.hitstop = hit.hitstop;
            let kind = if punish {
                EventKind::Punish
            } else if hit.knockdown {
                EventKind::Knockdown
            } else {
                EventKind::Hit
            };
            self.push_event(kind, atk, hit.move_id, dmg);
            self.check_ko(atk, def);
        }
    }

    fn check_ko(&mut self, atk: usize, def: usize) {
        if self.fighters[def].health <= 0 {
            self.fighters[def].health = 0;
            self.push_event(EventKind::KO, atk, None, 0);
        }
    }

    fn decay_combos(&mut self) {
        for i in 0..2 {
            let f = &self.fighters[i];
            let stunned = matches!(
                f.action,
                Action::Hit { .. }
                    | Action::Knockdown { .. }
                    | Action::Getup { .. }
                    | Action::Thrown { .. }
            );
            if !stunned && f.combo > 0 {
                self.fighters[i].reset_combo();
            }
        }
    }

    /// Count frames until each body is actionable after a contact; the
    /// difference is the true advantage, whatever the frame data claims.
    fn measure_advantage(&mut self) {
        let Some((atk, mut a_free, mut d_free)) = self.adv_pending else {
            return;
        };
        let def = 1 - atk;
        if a_free.is_none() && self.fighters[atk].action.actionable() {
            a_free = Some(self.frame);
        }
        if d_free.is_none() && self.fighters[def].action.actionable() {
            d_free = Some(self.frame);
        }
        match (a_free, d_free) {
            (Some(a), Some(d)) => {
                self.last_advantage = d as i32 - a as i32;
                self.adv_owner = atk;
                self.adv_pending = None;
            }
            _ => self.adv_pending = Some((atk, a_free, d_free)),
        }
    }

    /// Advantage from P1's point of view for the HUD.
    pub fn advantage_p1(&self) -> i32 {
        if self.adv_owner == 0 {
            self.last_advantage
        } else {
            -self.last_advantage
        }
    }

    fn check_round_end(&mut self) {
        if self.outcome.is_some() {
            return;
        }
        let h0 = self.fighters[0].health;
        let h1 = self.fighters[1].health;
        if h0 <= 0 && h1 <= 0 {
            self.outcome = Some(RoundOutcome::Draw);
        } else if h1 <= 0 {
            self.outcome = Some(RoundOutcome::Winner(0));
        } else if h0 <= 0 {
            self.outcome = Some(RoundOutcome::Winner(1));
        } else if self.time_left == 0 {
            let p0 = h0 * 1000 / self.fighters[0].data().max_health;
            let p1 = h1 * 1000 / self.fighters[1].data().max_health;
            self.outcome = Some(match p0.cmp(&p1) {
                std::cmp::Ordering::Greater => RoundOutcome::Winner(0),
                std::cmp::Ordering::Less => RoundOutcome::Winner(1),
                std::cmp::Ordering::Equal => RoundOutcome::Draw,
            });
            self.push_event(EventKind::TimeOver, 0, None, 0);
        }
    }

    pub fn camera_x(&self) -> i32 {
        (self.fighters[0].pos.x + self.fighters[1].pos.x) / 2
    }

    pub fn debug_boxes(&self, who: usize) -> DebugBoxes {
        let f = &self.fighters[who];
        DebugBoxes {
            push: f.pushbox(),
            hurt: f.hurtboxes(),
            hit: f.hitboxes(),
            aura: f.visual_aura_box(),
        }
    }

    pub fn projectile_boxes(&self) -> Vec<(Aabb, ProjectileKind, bool)> {
        self.projectiles
            .iter()
            .map(|p| (p.hitbox(), p.def.kind, p.live()))
            .collect()
    }
}

pub struct DebugBoxes {
    pub push: Aabb,
    pub hurt: Vec<Aabb>,
    pub hit: Vec<Aabb>,
    pub aura: Option<Aabb>,
}

struct Fnv1a(u64);

impl Hasher for Fnv1a {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 ^= *b as u64;
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}
