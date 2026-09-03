//! `docs/FRAME-DATA.md` must match the move data in `crates/sim/src/chars`.
//!
//! The section between `<!-- generated:begin -->` and `<!-- generated:end -->`
//! is produced from the live `Character` tables. This test regenerates it and
//! fails if the file on disk differs, so QA gate DOC1 ("FRAME-DATA matches
//! code") is checked by `cargo test`, not by eye.
//!
//! Regenerate after a retune:
//!
//! ```text
//! AEON_REGEN_DOCS=1 cargo test -p aeon-sim --test frame_data_doc
//! ```

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use aeon_sim::fighter::{
    BACKDASH_FRAMES, COMMAND_GRAB_HOLD, LANDING_RECOVERY, THROW_TECH_FRAMES,
};
use aeon_sim::input::{CHARGE_FRAMES, CHORD_WINDOW, HCB_WINDOW, MOTION_WINDOW};
use aeon_sim::moves::CancelRule;
use aeon_sim::versus::{INTRO_FRAMES, ROUND_END_FRAMES};
use aeon_sim::world::DETONATE_FRAMES;
use aeon_sim::{
    Character, CharacterId, HitLevel, MoveDef, MoveId, ShotBehavior, ThrowKind,
    CANCEL_LATE_FRAMES, FEINT_RECOVERY, GETUP_FRAMES, KNOCKDOWN_FRAMES, METER_MAX, PREJUMP,
    RC_COST, RC_FREEZE_FRAMES, ROUNDS_TO_WIN, ROUND_TIME, STAGE_W, SUB, THROW_TECH_WINDOW,
};

const BEGIN: &str = "<!-- generated:begin -->";
const END: &str = "<!-- generated:end -->";

fn doc_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("docs")
        .join("FRAME-DATA.md")
}

/// Subpixels → pixels, exact where possible.
fn pxf(v: i32) -> String {
    if v % SUB == 0 {
        format!("{}", v / SUB)
    } else {
        format!("{:.3}", v as f64 / SUB as f64)
    }
}

fn level(l: HitLevel) -> &'static str {
    match l {
        HitLevel::High => "High",
        HitLevel::Mid => "Mid",
        HitLevel::Low => "**Low**",
    }
}

fn cancel(c: CancelRule) -> &'static str {
    match c {
        CancelRule::Never => "never",
        CancelRule::OnHit => "hit",
        CancelRule::OnHitOrBlock => "hit+block",
    }
}

fn sar(m: &MoveDef) -> String {
    format!("{}/{}/{}", m.startup, m.active, m.recovery)
}

fn adv(m: &MoveDef) -> String {
    if m.is_throw() {
        return "throw".into();
    }
    if m.damage == 0 && m.hitboxes.is_empty() {
        // Shots, dashes, channels: the frame situation is the shot's, not
        // the body's. Total frames are what matter.
        return format!("— ({}f total)", m.total_frames());
    }
    if m.knockdown {
        format!("KD / {:+}", m.advantage_on_block())
    } else {
        format!("{:+} / {:+}", m.advantage_on_hit(), m.advantage_on_block())
    }
}

fn notes(c: &Character, m: &MoveDef) -> String {
    let mut n = Vec::new();
    if m.invuln.strike || m.invuln.throw {
        let kind = match (m.invuln.strike, m.invuln.throw) {
            (true, true) => "full",
            (true, false) => "strike",
            _ => "throw",
        };
        n.push(format!("{kind} invuln {}–{}", m.invuln.start + 1, m.invuln.end));
    }
    if m.launch > 0 {
        n.push(format!("launch {}", pxf(m.launch)));
    }
    if m.land_recovery > 0 {
        n.push(format!("land +{}f", m.land_recovery));
    }
    if m.meter_cost > 0 {
        n.push(format!("meter {}", m.meter_cost));
    }
    if m.gauge_cost > 0 {
        n.push(format!("{} {}", c.gauge.label, m.gauge_cost));
    }
    if m.projectile_guard {
        n.push("destroys shots".into());
    }
    if m.pass_through {
        n.push("passes through".into());
    }
    if m.vel_x != 0 {
        let span = if m.vel_frames > 0 {
            format!(" for {}f", m.vel_frames)
        } else {
            String::new()
        };
        n.push(format!("moves {} px/f{span}", pxf(m.vel_x)));
    }
    if m.vel_y != 0 {
        n.push(format!("rises {} px/f", pxf(m.vel_y)));
    }
    if m.feintable && m.id.is_special() {
        n.push("feintable".into());
    }
    if let Some(ch) = m.channel {
        n.push(format!(
            "hold {} up to {}f, +{} gauge/f",
            ch.button.label(),
            ch.max_frames,
            ch.gauge_per_frame
        ));
    }
    for f in m.followups {
        n.push(format!(
            "{} on f{}–{} → {}",
            f.button.label(),
            f.from,
            f.to,
            c.move_name(f.next)
        ));
    }
    match m.throw {
        ThrowKind::Normal => n.push("techable, loses to a strike".into()),
        ThrowKind::Command => n.push("untechable, beats both blocks, loses to invuln".into()),
        ThrowKind::None => {}
    }
    if let Some(p) = &m.projectile {
        let beh = match p.behavior {
            ShotBehavior::Travel => format!("travels {} px/f, life {}f", pxf(p.vel_x), p.lifetime),
            ShotBehavior::Hang => format!("hangs {}f", p.lifetime),
            ShotBehavior::Plant {
                arm_after,
                armed_life,
            } => format!(
                "plant: arc {}/{} px/f, arms {}f after landing, armed {}f",
                pxf(p.vel_x),
                pxf(p.vel_y),
                arm_after,
                armed_life
            ),
        };
        n.push(format!(
            "{} shot: {beh}, dmg {}, stun {}/{}, {}{}",
            p.kind.name(),
            p.damage,
            p.hitstun,
            p.blockstun,
            level(p.level),
            if p.knockdown { ", KD" } else { "" }
        ));
    }
    n.join("; ")
}

fn normals_table(out: &mut String, c: &Character) {
    writeln!(out, "| Normal | s/a/r | dmg | hitstun / blockstun | on hit / block | level | cancel |").unwrap();
    writeln!(out, "|---|---|---|---|---|---|---|").unwrap();
    for m in c.moves.iter().filter(|m| m.id.is_normal() || m.id == MoveId::Throw) {
        let extra = notes(c, m);
        let extra = if extra.is_empty() {
            String::new()
        } else {
            format!(" ({extra})")
        };
        writeln!(
            out,
            "| {}{extra} | {} | {} | {} / {} | {} | {} | {} |",
            c.move_name(m.id),
            sar(m),
            m.damage,
            m.hitstun,
            m.blockstun,
            adv(m),
            level(m.level),
            cancel(m.cancel),
        )
        .unwrap();
    }
}

fn specials_table(out: &mut String, c: &Character) {
    writeln!(out, "| Special | input | s/a/r | dmg | hitstun / blockstun | on hit / block | level | notes |").unwrap();
    writeln!(out, "|---|---|---|---|---|---|---|---|").unwrap();
    for m in c.moves.iter().filter(|m| m.id.is_special()) {
        let input = c.input_for(m.id).unwrap_or_else(|| match m.id {
            MoveId::Detonate => "214+S (crystal planted)".into(),
            _ => "—".into(),
        });
        let dmg = if m.damage > 0 {
            m.damage.to_string()
        } else if m.projectile.is_some() {
            "shot".into()
        } else {
            "—".into()
        };
        writeln!(
            out,
            "| {} | {input} | {} | {dmg} | {} / {} | {} | {} | {} |",
            c.move_name(m.id),
            sar(m),
            m.hitstun,
            m.blockstun,
            adv(m),
            level(m.level),
            notes(c, m),
        )
        .unwrap();
    }
}

fn body(out: &mut String, id: CharacterId) {
    let c = id.data();
    writeln!(out, "### {}\n", id.name()).unwrap();
    writeln!(out, "| Body | value |").unwrap();
    writeln!(out, "|---|---|").unwrap();
    writeln!(out, "| health | {} |", c.max_health).unwrap();
    writeln!(out, "| walk fwd / back | {} / {} px/f |", pxf(c.walk_fwd), pxf(c.walk_back)).unwrap();
    writeln!(out, "| run (glide) | {} px/f |", pxf(c.run_speed)).unwrap();
    writeln!(out, "| jump x / y | {} / {} px/f |", pxf(c.jump_x), pxf(c.jump_y)).unwrap();
    writeln!(out, "| hop x / y | {} / {} px/f |", pxf(c.hop_x), pxf(c.hop_y)).unwrap();
    writeln!(out, "| gravity | {} px/f² |", pxf(c.gravity)).unwrap();
    let air = |vy: i32| {
        // Frames until y returns to 0 under constant gravity, in subpixels.
        let mut y = 0i64;
        let mut v = vy as i64;
        let mut f = 0;
        loop {
            y += v;
            v -= c.gravity as i64;
            f += 1;
            if y <= 0 {
                break;
            }
        }
        f
    };
    writeln!(out, "| jump airtime / hop airtime | {}f / {}f (+{PREJUMP}f prejump, +{LANDING_RECOVERY}f landing) |", air(c.jump_y), air(c.hop_y)).unwrap();
    writeln!(out, "| pushbox w, stand h, crouch h | {} / {} / {} px |", pxf(c.push_w), pxf(c.stand_h), pxf(c.crouch_h)).unwrap();
    writeln!(out, "| throw range / close range | {} / {} px |", pxf(c.throw_range), pxf(c.close_range)).unwrap();
    let g = c.gauge;
    let regen = if g.regen_amount > 0 {
        format!(
            "+{} every {}f after {}f idle",
            g.regen_amount, g.regen_every, g.regen_delay
        )
    } else {
        "no regen".into()
    };
    let buff = if g.buff_step > 0 {
        format!("; buff tier every {} gauge", g.buff_step)
    } else {
        String::new()
    };
    writeln!(out, "| gauge ({}) | max {}, starts {}, {regen}{buff} |", g.label, g.max, g.start).unwrap();
    if let Some(a) = c.aura {
        writeln!(out, "| aura (render only) | {}×{} px at ({}, {}) — never a hurtbox |", pxf(a.w), pxf(a.h), pxf(a.x), pxf(a.y)).unwrap();
    }
    writeln!(out).unwrap();
    writeln!(out, "Hurtbox standing {}×{} px, crouching {}×{} px.\n", pxf(c.hurt_stand().w), pxf(c.hurt_stand().h), pxf(c.hurt_crouch().w), pxf(c.hurt_crouch().h)).unwrap();
    writeln!(out, "**Normals.** Advantage on first-active contact. Lights link; weapon-heavies are minus.\n").unwrap();
    normals_table(out, c);
    writeln!(out).unwrap();
    writeln!(out, "**Specials.** Costs are deducted on start. EX = motion + S+HS.\n").unwrap();
    specials_table(out, c);
    writeln!(out).unwrap();
}

fn generate() -> String {
    let mut out = String::new();
    writeln!(out, "{BEGIN}").unwrap();
    writeln!(out, "_Generated from `crates/sim/src/chars` by `tests/frame_data_doc.rs`. Do not edit by hand; retune the code and run `AEON_REGEN_DOCS=1 cargo test -p aeon-sim --test frame_data_doc`._\n").unwrap();
    writeln!(out, "## Universal\n").unwrap();
    writeln!(out, "| Law | value |").unwrap();
    writeln!(out, "|---|---|").unwrap();
    writeln!(out, "| tick | 60 Hz, {SUB} subpixels per pixel |").unwrap();
    writeln!(out, "| stage width | {} px |", pxf(STAGE_W)).unwrap();
    writeln!(out, "| round | {} s, first to {ROUNDS_TO_WIN}, intro {INTRO_FRAMES}f, round-end {ROUND_END_FRAMES}f |", ROUND_TIME / 60).unwrap();
    writeln!(out, "| super bar | 0–{METER_MAX}; RC {RC_COST}; super 1000 |").unwrap();
    writeln!(out, "| Roman Cancel (S+FL) | {RC_COST} meter, {RC_FREEZE_FRAMES}f freeze, from any own attack frame on hit/block/whiff; never from hitstun or blockstun |").unwrap();
    writeln!(out, "| feint (FL+ST) | cancels a feintable special's startup to {FEINT_RECOVERY}f of recovery |").unwrap();
    writeln!(out, "| chord window | {CHORD_WINDOW}f; a normal started inside the window kara-cancels into the chord |").unwrap();
    writeln!(out, "| special cancel window | from first active frame to last active + {CANCEL_LATE_FRAMES}f |").unwrap();
    writeln!(out, "| motion buffer | {MOTION_WINDOW}f for 236/214/623; {HCB_WINDOW}f for 63214; charge {CHARGE_FRAMES}f |").unwrap();
    writeln!(out, "| prejump / landing | {PREJUMP}f / {LANDING_RECOVERY}f; tap up = hop, hold up = jump |").unwrap();
    writeln!(out, "| backdash | {BACKDASH_FRAMES}f, punishable |").unwrap();
    writeln!(out, "| normal throw (P+K) | tech window {THROW_TECH_WINDOW}f after the grab connects; tech = both pushed apart, {THROW_TECH_FRAMES}f each |").unwrap();
    writeln!(out, "| command grab (63214+FL) | untechable; {COMMAND_GRAB_HOLD}f hold then the throw resolves; whiff recovery is the move's own |").unwrap();
    writeln!(out, "| hard knockdown | {KNOCKDOWN_FRAMES}f down + {GETUP_FRAMES}f getup; downed body is strike-invulnerable (no OTG) |").unwrap();
    writeln!(out, "| crystal detonate | {DETONATE_FRAMES}f blast |").unwrap();
    writeln!(out, "| combo scaling | 100 / 80 / 60 / 45, floor 35 |").unwrap();
    writeln!(out, "| projectiles | one live shot per owner per type; same clash class cancel; Heavy beats Light |").unwrap();
    writeln!(out).unwrap();
    body(&mut out, CharacterId::Kogan);
    body(&mut out, CharacterId::Raya);
    write!(out, "{END}").unwrap();
    out
}

#[test]
fn frame_data_doc_matches_code() {
    let path = doc_path();
    let generated = generate();
    let regen = std::env::var_os("AEON_REGEN_DOCS").is_some();
    let existing = fs::read_to_string(&path).unwrap_or_default();
    let (head, tail) = match (existing.find(BEGIN), existing.find(END)) {
        (Some(b), Some(e)) if e > b => (&existing[..b], &existing[e + END.len()..]),
        _ => (existing.as_str(), ""),
    };
    let wanted = format!("{head}{generated}{tail}");
    if regen {
        fs::write(&path, &wanted).expect("write docs/FRAME-DATA.md");
        return;
    }
    assert!(
        existing == wanted,
        "docs/FRAME-DATA.md is stale. Run: AEON_REGEN_DOCS=1 cargo test -p aeon-sim --test frame_data_doc"
    );
}
