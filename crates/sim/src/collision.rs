//! Pushboxes, hit/hurt tests, projectile collisions, throw law.

use crate::fighter::Fighter;
use crate::geom::Aabb;
use crate::moves::{HitLevel, MoveId, ProjectileDef, ThrowKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HitResult {
    pub attacker: usize,
    pub defender: usize,
    pub move_id: Option<MoveId>,
    pub damage: i32,
    pub chip: i32,
    pub hitstun: u8,
    pub blockstun: u8,
    pub hitstop: u8,
    pub blockstop: u8,
    pub push_hit: i32,
    pub push_block: i32,
    pub level: HitLevel,
    pub knockdown: bool,
    pub launch: i32,
    pub blocked: bool,
    pub throw: ThrowKind,
    pub meter: i32,
}

pub fn any_overlap(a: &[Aabb], b: &[Aabb]) -> bool {
    a.iter().any(|x| b.iter().any(|y| x.overlaps(*y)))
}

/// Separate two pushboxes so they don't occupy the same space. A cornered
/// defender transfers leftover push onto the attacker. A pass-through
/// movement special (Raya's glide) skips separation entirely.
pub fn resolve_push(a: &mut Fighter, b: &mut Fighter, stage_w: i32) {
    if a.passing_through() || b.passing_through() {
        return;
    }
    let pa = a.pushbox();
    let pb = b.pushbox();
    if !pa.overlaps(pb) {
        return;
    }
    let overlap = if pa.center_x() < pb.center_x() {
        pa.right - pb.left
    } else {
        pb.right - pa.left
    };
    if overlap <= 0 {
        return;
    }
    let half = overlap / 2;
    if pa.center_x() < pb.center_x() {
        a.pos.x -= half;
        b.pos.x += overlap - half;
    } else {
        b.pos.x -= half;
        a.pos.x += overlap - half;
    }
    clamp_stage(a, stage_w);
    clamp_stage(b, stage_w);

    // If still overlapping because one is in the corner, shove the other.
    let pa = a.pushbox();
    let pb = b.pushbox();
    if !pa.overlaps(pb) {
        return;
    }
    let overlap = (pa.right.min(pb.right) - pa.left.max(pb.left)).max(0);
    if a.pos.x <= a.data().push_w / 2 + 1 {
        b.pos.x += overlap;
    } else if a.pos.x >= stage_w - a.data().push_w / 2 - 1 {
        b.pos.x -= overlap;
    } else if b.pos.x <= b.data().push_w / 2 + 1 {
        a.pos.x += overlap;
    } else if b.pos.x >= stage_w - b.data().push_w / 2 - 1 {
        a.pos.x -= overlap;
    }
    clamp_stage(a, stage_w);
    clamp_stage(b, stage_w);
}

fn clamp_stage(f: &mut Fighter, stage_w: i32) {
    let half = f.data().push_w / 2;
    f.pos.x = f.pos.x.clamp(half, stage_w - half);
}

pub fn strike_hits(attacker: &Fighter, defender: &Fighter) -> Option<HitResult> {
    let hitboxes = attacker.hitboxes();
    if hitboxes.is_empty() {
        return None;
    }
    let mv = attacker.current_move()?;
    if mv.is_throw() {
        // Throw law: both throws beat stand and crouch block. Neither grabs
        // an airborne, stunned, downed, or already-grabbed body. Uppercut
        // invulnerability beats both.
        if defender.airborne || defender.action.throw_protected() || defender.throw_invuln() {
            return None;
        }
        // Grab range is measured against the pushbox, not the hurtbox.
        if !hitboxes.iter().any(|h| h.overlaps(defender.pushbox())) {
            return None;
        }
        return Some(HitResult {
            attacker: 0,
            defender: 1,
            move_id: Some(mv.id),
            damage: mv.damage,
            chip: 0,
            hitstun: mv.hitstun,
            blockstun: 0,
            hitstop: mv.hitstop,
            blockstop: 0,
            push_hit: mv.pushback_hit,
            push_block: 0,
            level: HitLevel::Mid,
            knockdown: true,
            launch: 0,
            blocked: false,
            throw: mv.throw,
            meter: mv.meter_on_hit,
        });
    }
    if defender.strike_invuln() {
        return None;
    }
    if !any_overlap(&hitboxes, &defender.hurtboxes()) {
        return None;
    }
    let blocked = defender.would_block(mv.level);
    Some(HitResult {
        attacker: 0,
        defender: 1,
        move_id: Some(mv.id),
        damage: mv.damage,
        chip: mv.chip,
        hitstun: mv.hitstun,
        blockstun: mv.blockstun,
        hitstop: mv.hitstop,
        blockstop: mv.blockstop,
        push_hit: mv.pushback_hit,
        push_block: mv.pushback_block,
        level: mv.level,
        knockdown: mv.knockdown,
        launch: mv.launch,
        blocked,
        throw: ThrowKind::None,
        meter: mv.meter_on_hit,
    })
}

pub fn projectile_hits(
    owner: usize,
    hitbox: Aabb,
    def: &ProjectileDef,
    damage: i32,
    defender: &Fighter,
) -> Option<HitResult> {
    if defender.strike_invuln() {
        return None;
    }
    if !defender.hurtboxes().iter().any(|h| h.overlaps(hitbox)) {
        return None;
    }
    let blocked = defender.would_block(def.level);
    Some(HitResult {
        attacker: owner,
        defender: 1 - owner,
        move_id: None,
        damage,
        chip: def.chip,
        hitstun: def.hitstun,
        blockstun: def.blockstun,
        hitstop: def.hitstop,
        blockstop: def.hitstop.saturating_sub(2),
        push_hit: def.pushback,
        push_block: def.pushback * 3 / 4,
        level: def.level,
        knockdown: def.knockdown,
        launch: 0,
        blocked,
        throw: ThrowKind::None,
        meter: damage / 10,
    })
}

/// Combo scaling: 100 / 80 / 60 / 45 then floors at 35. Short routes stay fat.
pub fn scale_damage(base: i32, combo_hits_already: u8) -> i32 {
    let pct = match combo_hits_already {
        0 => 100,
        1 => 80,
        2 => 60,
        3 => 45,
        _ => 35,
    };
    (base * pct / 100).max(1)
}
