# Aeon — design (as implemented, 2026-09-05)

A 1v1 2D fighter whose game is the space between two bodies, and whose soul is the tension when that space collapses. Super Turbo footsies at the base; Samurai Shodown's tax on the heavy buttons; then a measured layer of Guilty Gear (Roman Cancel, Flash/Style), King of Fighters (hop, run) and Fatal Fury (feint).

One sentence: **footsies earns the knockdown; the knockdown opens the pressure web; the uppercut punishes arrogance; RC is how meter refuses the tax once.**

Not an anime fighter. Combos are typically 2–3 hits; natural 3–5-hit routes are acceptable. Narrow links are intentional, rewarding execution without making combos necessary to win. No air dash, no double jump, no same-frame 50/50, no fullscreen fireball war. The vault copy of this law (`Aeon/DESIGN.md`) is canon; this file is the crate's summary of what the code enforces. Numbers: `docs/FRAME-DATA.md`. Grading: `docs/QA.md`.

## The loop

Two states — **patient** (midrange, walk-speed thought, fishing for a poke into a confirm) and **explosive** (a whiff punished, a confirm into a rekka, a knockdown, then the pressure web). The law of the loop: **there is no safe posture.**

| Attacker ↓ / Defender → | Blocks | Uppercuts | Presses a button |
|---|---|---|---|
| **Command grab** | wins | loses | loses (hit beats throw) |
| **Frame trap / strike** | safe, plus | loses | wins |
| **Bait (hold block)** | loses tempo | wins big | loses tempo |

The metered trump: uppercut on hit → RC → damage and a hard knockdown. Every cell of that table is a test in `crates/sim/tests/trials.rs` (`trial_06_oki_triangle`).

## The six buttons

```
  P     S     HS
  K     FL    ST
```

| Button | As a normal | As a special |
|---|---|---|
| **P** | Ticks, links, interrupts, beats throws. | — |
| **K** | The fastest low (2K). Conditions the crouch. | — |
| **S** | Mid-long poke. Cancellable on hit/block. | Rekka (236), uppercut (623), shot A (214), super (236236). |
| **HS** | The SamSho button: longest, minus, death on whiff. | Uppercut (623), shot B (236), Kogan disc (214). |
| **FL** | Short-mid frame trap, fat hitstun, link-outs. | Command grab (63214), command dash (236), Raya consecrate (hold 214), Kogan air gun (j.FL). |
| **ST** | Same family. 2ST is the sweep. | Kogan special overhead (236). |

P and K carry no specials. Chords are adjacent pairs:

| Chord | Verb | Cost | Law |
|---|---|---|---|
| **P+K** | Normal throw | — | Beats both blocks. 7f tech. A strike that lands first wins. |
| **S+FL** | Roman Cancel | 250 bar | From any own attack frame on hit, block or whiff. 6f freeze. Never from hitstun or blockstun. |
| **FL+ST** | Feint | — | Cancels a feintable special's startup into 8f of nothing. Normals, throws, supers cannot be feinted. |
| **HS+ST** | Standing overhead | — | Universal. High. Slow. |
| **S+HS** | EX | character gauge | Motion + chord. Two per body. Without gauge, nothing comes out. |

Chord window is 3f. A normal that started inside the window is kara-cancelled into the chord and its cost is refunded.

## Movement

Walk, backdash (14f, shared, punishable), dash, **run** (66 then hold; a glide in the client, no leg cycle), **hop** (tap up: release within the 4f prejump), jump (hold up), feint. Hop is a lower, shorter, faster arc that shares the jump normals; hop-in normals are High.

Run immediately stops, blocks, crouches, jumps or attacks. Hop landing adds 0f recovery; full-jump landing adds 2f. Move-specific landing taxes remain. Air normals keep horizontal travel and hop identity; touchdown does not erase hitstun. The first free frame accepts its input, without buffering an early normal through recovery.

Blocking does not automatically end the attacker's pressure; gaps, recovery, reversals and baits still decide turns.

## Rekkas

Every body's signature grammar. Part one engages (safest), part two confirms, part three is the tax (knockdown) or the reset. Follow-ups are legal on hit, block and whiff inside a press window per part; any part is RC-able; stopping after part one is a different frame situation than finishing. Special cancels from normals open on the first active frame and close 2f after the last active frame.

- Kogan **saber string** (236+S → S → S): cut, backcut, thrust. Advancing.
- Raya **chant** (236+S → S → S): chant I, II, III. Advancing more slowly, longer reach.

## Projectiles

Pressure verbs, not neutral. One live shot per owner per type; same clash class cancels (Light: revolver, air gun, glyph; Heavy: wave, crystal); Heavy beats Light.

- Kogan **revolver** (214+S) spends a chamber, travels fast. **Energy wave** (236+HS) is slow and short-lived. **Air gun** (j.FL) is a different button from the air saber. **Disc-shield** (214+HS) destroys any shot its active frames touch and is +3 on block up close.
- Raya **voice glyph** (236+HS) **hangs** where it is cast. **Crystal** (214+S) arcs, lands, **arms**, then **detonates** on contact or on 214+S (shatter); unarmed it is harmless. **Consecrate** (hold 214+FL) fills the crystal gauge; every 50 is a buff tier that makes shots arm faster, live longer and hit harder. Charge is a buff, never a stored attack.

## Throws

- **Normal throw (P+K).** Beats stand and crouch block. Techable (P+K within 7f). Jabbable. Modest whiff.
- **Command grab (63214+FL).** Beats both blocks. Untechable. Loses to invulnerability (the uppercut). Whiff is max punish (38–40f recovery). Kogan's is the cape-snare — the box is his arms, never the cape. Raya's is the rite.

Throws whiff on airborne, stunned and downed bodies.

## The uppercut

623 on the slash buttons. Full invuln on frames 2–7. On hit: launch, hard knockdown, RC-able. Blocked or baited: landing recovery on top of a long whiff — max punish and a hard knockdown for the defender to eat. A launched body has no air recovery: its hitstun rides the arc and ends on the floor, in the knockdown, and the combo counter holds through the fall.

## Meter

- **Bar** 0–1000, shared law. RC 250. Super 1000 (236236+S).
- **Character gauge.** Kogan **firearm**: 6 chambers; revolver and air gun spend 1, EX spends 2; reload +1 per 60f after 90f without firing. Raya **crystal**: 0–100, filled only by consecrate, spent by EX (50), buffs in tiers of 50.

## Damage and knockdown

Scaling 100 / 80 / 60 / 45 then 35. Natural five-hit routes are legal; evaluate execution, commitment and payoff. Kogan jab → jab xx full rekka is tested at 170 damage and knockdown. Hard knockdown is 32f down + 24f getup; downed bodies are strike-invulnerable (no OTG). Pushback is a one-shot shove on contact; what a cornered defender cannot absorb moves the attacker instead, so the corner still costs spacing (shots do not shove their owner). Sweep, command grab, uppercut, rekka 3, crystal blast and super all knock down. Knockdown is the currency; oki is where rounds turn.

## Sim contract

`aeon-sim` is a pure 60 Hz integer function: `World::tick(&mut self, p1: InputFrame, p2: InputFrame)`. 256 subpixels per pixel. No floats in `World`, no clock, no filesystem, no renderer, input or audio crates (`crates/sim/tests/purity.rs`). Facing-relative numpad input, 16f buffer, 12f motion window (16f for 63214), charge 45f. Back = stand block (High/Mid), down-back = crouch block (Low/Mid), airborne cannot block. Advantage = `hitstun − (active − 1) − recovery` on first-active contact. Kogan's aura is a render-only box and never a hurtbox. `state_hash()` exists for replays and a future rollback layer; the client is macroquad and replaceable.

## Characters

Two bodies of the Sanctum. They share the system and none of the kit. No other bodies are seated in this crate; the armored man on Raya's plate is an NPC and is not kitted.

**Kogan — the duelist.** Saber, revolver, disc-shield. Copper hood-cape is aura. Saber string, energy wave, revolver, disc-shield, sunward cut (623), falling saber (236+ST, High), cape-snare (63214+FL), threshold-step (236+FL), air saber and air gun on different buttons, EX saber cut, EX revolver, judgment (super).

**Raya — the officiant.** Voice and crystals. Chant, voice glyph, crystal (plant/arm/detonate), shatter, consecrate, ascension (623), the rite (63214+FL), processional (236+FL, passes through the body), EX glyph, EX crystal, convergence (super). She footsies with 5P / 2K / 5S / 2HS; setplay is oppressive, not a zoner caste.

## Deliberately missing

Air dash, double jump, burst, parry, weapon clash, netcode, audio, invented bodies.
