# Aeon — frame data

Law: `DESIGN.md`. Advantage on first-active contact = `hitstun − (active − 1) − recovery` (same with blockstun).

These are the authored numbers from the September 2 implementation, updated for the September 5 consultation. The tables below the marker are **generated from the move data** in `crates/sim/src/chars/{kogan,raya}.rs` by `crates/sim/tests/frame_data_doc.rs`, and `cargo test -p aeon-sim` fails if they drift from the code. Retune in the code, then:

```
AEON_REGEN_DOCS=1 cargo test -p aeon-sim --test frame_data_doc
```

Feel-targets these numbers must not violate:

- Lights link (tight 1–2f windows are correct). No normal-to-normal chains.
- Weapon-heavies (far S / far HS) stay **minus** on hit and on block. A missed heavy is a lost turn.
- Disc close is plus on block (+3) so it frame-traps.
- Two or three hits are typical; natural three-to-five-hit routes are allowed. Scaling 100 / 80 / 60 / 45 / 35.
- Command grab whiff is max-punish. Uppercut blocked or baited is max-punish + hard knockdown.
- Hop is a distinct, shorter, faster arc than jump. Run is a glide.

## September 5 flow pass

- Landing recovery: full jump **2f**, hop **0f**. Move-specific landing recovery is preserved.
- Run transitions are immediate; current inputs are accepted on the first free recovery frame. Early normal presses are not buffered through recovery.
- Air normals retain horizontal travel and hop identity. Landing does not erase remaining air hitstun.
- No move damage, startup, active, recovery, cancel windows or gauge costs were retuned. The tested natural Kogan jab → jab xx full rekka is **five hits / 170 damage / knockdown**.
- Airtime tables now follow the simulation's gravity-before-position integration. This corrects the documentation by two frames; it does not retune the arcs.

## September 2 implementation (changes from the August 13 prototype)

| Verb | Decision |
|---|---|
| Buttons | LP/MP/HP/LK/MK/HK → **P / K / S / HS / FL / ST**. FL and ST are the fat-hitstun trap buttons; 2ST is the sweep. |
| Chords | S+FL Roman Cancel (250, 6f freeze). FL+ST feint (8f recovery). HS+ST standing overhead (High). S+HS EX (character gauge). P+K throw. Chord window 3f; a normal that started inside the window is kara-cancelled into the chord and its cost refunded. |
| Hop | Tap up (released within prejump) = hop, hold = jump. Hop shares jump normals; hop overhead is High. |
| Run | 66 then hold forward. Both bodies. Client draws a glide. |
| Feint | FL+ST during a feintable special's startup → 8f neutral recovery. Normals, throws and supers are not feintable. |
| Rekka | Parts 1–3 with a press window per part; follow-ups are legal on hit, block and whiff. Any part is RC-able. Kogan: advancing saber string on 236+S. Raya: chant on 236+S. |
| Throw law | Command grab 63214+FL beats both blocks, untechable, loses to invuln (uppercut). Normal throw P+K beats both blocks, 7f tech window, loses to a strike that lands first. "Throw loses to crouch" is gone. |
| Gauges | Kogan **firearm** (CYL): 6 chambers, revolver and air gun spend 1, EX spends 2; reload +1 per 60f after 90f without firing. Raya **crystal**: 0–100, filled only by consecrate (hold 214+FL, +2/f), spent by EX (50); every 50 is a buff tier that makes crystals and glyphs arm faster, live longer and hit harder. Charge is a buff, never a stored attack. |
| Placed shots | Raya voice glyph **hangs** where cast. Crystal is thrown in an arc, lands, **arms**, then **detonates** on contact or 214+S; unarmed it is harmless. One live shot per owner per type. |
| Disc | Moved from 214+K to **214+HS** (P/K carry no specials). Destroys any shot its active frames touch. |
| Cancel window | Special cancel from first active to last active + 2f (was 8f of recovery). |
| Motion table | Full table per body: 236+S rekka, 623+S/HS uppercut, 63214+FL command grab, 236+FL command dash, 214+S shot A, 236+HS shot B, 214+HS Kogan disc / Raya consecrate on 214+FL, 236+ST Kogan special overhead, j.FL Kogan air gun, 236+S+HS EX A, 214+S+HS EX B, 236236+S super. |
| Knockdown | Hard knockdown 32f down + 24f getup (56f). Downed bodies are strike-invulnerable — no OTG. Sweep, command grab, uppercut, rekka 3, crystal blast and super knock down. |

<!-- generated:begin -->
_Generated from `crates/sim/src/chars` by `tests/frame_data_doc.rs`. Do not edit by hand; retune the code and run `AEON_REGEN_DOCS=1 cargo test -p aeon-sim --test frame_data_doc`._

## Universal

| Law | value |
|---|---|
| tick | 60 Hz, 256 subpixels per pixel |
| stage width | 760 px |
| round | 99 s, first to 2, intro 60f, round-end 100f |
| super bar | 0–1000; RC 250; super 1000 |
| Roman Cancel (S+FL) | 250 meter, 6f freeze, from any own attack frame on hit/block/whiff; never from hitstun or blockstun |
| feint (FL+ST) | cancels a feintable special's startup to 8f of recovery |
| chord window | 3f; a normal started inside the window kara-cancels into the chord |
| special cancel window | from first active frame to last active + 2f |
| motion buffer | 12f for 236/214/623; 16f for 63214; charge 45f |
| prejump / landing | 4f prejump; full jump 2f landing, hop 0f; move-specific landing recovery still applies |
| backdash | 14f, punishable |
| normal throw (P+K) | tech window 7f after the grab connects; tech = both pushed apart, 16f each |
| command grab (63214+FL) | untechable; 4f hold then the throw resolves; whiff recovery is the move's own |
| hard knockdown | 32f down + 24f getup; downed body is strike-invulnerable (no OTG) |
| crystal detonate | 6f blast |
| combo scaling | 100 / 80 / 60 / 45, floor 35 |
| projectiles | one live shot per owner per type; same clash class cancel; Heavy beats Light |

### KOGAN

| Body | value |
|---|---|
| health | 1000 |
| walk fwd / back | 3 / 2 px/f |
| run (glide) | 6 px/f |
| jump x / y | 4 / 13 px/f |
| hop x / y | 4 / 8 px/f |
| gravity | 0.625 px/f² |
| jump airtime / hop airtime | 41f / 25f (+4f prejump; full jump landing 2f, hop landing 0f) |
| pushbox w, stand h, crouch h | 32 / 96 / 62 px |
| throw range / close range | 36 / 52 px |
| gauge (CYL) | max 6, starts 6, +1 every 60f after 90f idle |
| aura (render only) | 128×112 px at (-54, 4) — never a hurtbox |

Hurtbox standing 40×96 px, crouching 40×62 px.

**Normals.** Advantage on first-active contact. Lights link; weapon-heavies are minus.

| Normal | s/a/r | dmg | hitstun / blockstun | on hit / block | level | cancel |
|---|---|---|---|---|---|---|
| 5P | 4/2/6 | 40 | 12 / 8 | +5 / +1 | Mid | hit+block |
| 5K | 5/3/8 | 45 | 12 / 8 | +2 / -2 | Mid | hit+block |
| 5S | 7/3/14 | 70 | 15 / 11 | -1 / -5 | Mid | hit+block |
| 5HS | 9/3/20 | 110 | 16 / 12 | -6 / -10 | Mid | hit |
| c.HS | 7/2/14 | 105 | 18 / 13 | +3 / -2 | Mid | hit |
| 5FL | 7/3/12 | 75 | 19 / 13 | +5 / -1 | Mid | hit |
| 5ST | 9/3/14 | 90 | 20 / 14 | +4 / -2 | Mid | hit |
| 2P | 4/2/6 | 35 | 11 / 7 | +4 / +0 | Mid | hit+block |
| 2K | 4/2/8 | 40 | 11 / 7 | +2 / -2 | **Low** | hit+block |
| 2S | 6/3/12 | 65 | 15 / 11 | +1 / -3 | Mid | hit+block |
| 2HS | 8/3/18 | 100 | 16 / 12 | -4 / -8 | Mid | hit |
| 2FL | 6/3/12 | 70 | 18 / 12 | +4 / -2 | Mid | hit |
| 2ST | 8/3/22 | 90 | 20 / 12 | KD / -12 | **Low** | never |
| j.P | 4/8/4 | 40 | 14 / 8 | +3 / -3 | High | never |
| j.K | 4/8/4 | 40 | 14 / 8 | +3 / -3 | High | never |
| j.S | 5/6/6 | 65 | 14 / 8 | +3 / -3 | High | never |
| j.HS | 6/5/8 | 90 | 14 / 8 | +2 / -4 | High | never |
| j.FL | 5/6/6 | 55 | 14 / 8 | +3 / -3 | High | never |
| j.ST | 6/5/8 | 90 | 14 / 8 | +2 / -4 | High | never |
| overhead | 22/3/16 | 95 | 22 / 12 | +4 / -6 | High | never |
| throw (techable, loses to a strike) | 2/1/20 | 140 | 0 / 0 | throw | Mid | never |

**Specials.** Costs are deducted on start. EX = motion + S+HS.

| Special | input | s/a/r | dmg | hitstun / blockstun | on hit / block | level | notes |
|---|---|---|---|---|---|---|---|
| saber cut | 236+S | 8/3/14 | 60 | 16 / 12 | +0 / -4 | Mid | moves 5 px/f for 8f; feintable; S on f8–22 → backcut |
| backcut | — | 7/3/14 | 70 | 17 / 11 | +1 / -5 | Mid | moves 4 px/f for 6f; feintable; S on f7–21 → thrust |
| thrust | — | 10/3/22 | 90 | 24 / 12 | KD / -12 | Mid | moves 6 px/f for 8f; feintable |
| energy wave | 236+HS | 14/2/22 | shot | 0 / 0 | — (38f total) | Mid | feintable; wave shot: travels 3 px/f, life 40f, dmg 60, stun 20/18, Mid |
| revolver | 214+S | 11/1/24 | shot | 0 / 0 | — (36f total) | Mid | CYL 1; feintable; revolver shot: travels 9 px/f, life 90f, dmg 70, stun 16/14, Mid |
| disc-shield | 214+HS | 8/5/12 | 75 | 17 / 19 | +1 / +3 | Mid | destroys shots; feintable |
| sunward cut | 623+S/HS | 3/10/24 | 140 | 20 / 12 | KD / -21 | Mid | full invuln 2–7; launch 10; land +12f; moves 3 px/f; rises 12 px/f; feintable |
| cape-snare | 63214+FL | 6/2/38 | 160 | 0 / 0 | throw | Mid | feintable; untechable, beats both blocks, loses to invuln |
| threshold-step | 236+FL | 0/0/16 | — | 0 / 0 | — (16f total) | Mid | moves 5 px/f for 12f |
| falling saber | 236+ST | 18/4/14 | 100 | 22 / 14 | KD / -3 | High | land +8f; moves 4 px/f; rises 7 px/f; feintable |
| air gun | FL | 8/1/12 | shot | 0 / 0 | — (21f total) | Mid | CYL 1; air gun shot: travels 6 px/f, life 60f, dmg 50, stun 14/12, Mid |
| EX saber cut | 236+S+HS | 6/3/12 | 80 | 20 / 16 | +6 / +2 | Mid | CYL 2; moves 6 px/f for 8f; feintable; S on f6–20 → backcut |
| EX revolver | 214+S+HS | 9/1/22 | shot | 0 / 0 | — (32f total) | Mid | CYL 2; feintable; revolver shot: travels 12 px/f, life 90f, dmg 110, stun 24/16, Mid, KD |
| judgment | 236236+S | 4/8/26 | 280 | 18 / 14 | KD / -19 | Mid | full invuln 1–4; launch 4; meter 1000; moves 8 px/f |

### RAYA

| Body | value |
|---|---|
| health | 950 |
| walk fwd / back | 2.500 / 1.750 px/f |
| run (glide) | 5 px/f |
| jump x / y | 3 / 12 px/f |
| hop x / y | 3 / 7 px/f |
| gravity | 0.688 px/f² |
| jump airtime / hop airtime | 34f / 20f (+4f prejump; full jump landing 2f, hop landing 0f) |
| pushbox w, stand h, crouch h | 30 / 98 / 60 px |
| throw range / close range | 34 / 48 px |
| gauge (CRYSTAL) | max 100, starts 0, no regen; buff tier every 50 gauge |

Hurtbox standing 38×98 px, crouching 38×60 px.

**Normals.** Advantage on first-active contact. Lights link; weapon-heavies are minus.

| Normal | s/a/r | dmg | hitstun / blockstun | on hit / block | level | cancel |
|---|---|---|---|---|---|---|
| 5P | 5/2/8 | 55 | 14 / 9 | +5 / +0 | Mid | hit+block |
| 5K | 5/3/9 | 50 | 12 / 8 | +1 / -3 | Mid | hit+block |
| 5S | 8/3/14 | 90 | 16 / 12 | +0 / -4 | Mid | hit+block |
| 5HS | 11/3/22 | 150 | 18 / 13 | -6 / -11 | Mid | never |
| c.HS | 7/2/16 | 130 | 20 / 14 | +3 / -3 | Mid | hit |
| 5FL | 7/3/13 | 80 | 20 / 14 | +5 / -1 | Mid | hit |
| 5ST | 10/3/16 | 100 | 22 / 15 | +4 / -3 | Mid | hit |
| 2P | 5/2/8 | 50 | 12 / 8 | +3 / -1 | Mid | hit+block |
| 2K | 5/2/9 | 50 | 12 / 8 | +2 / -2 | **Low** | hit+block |
| 2S | 8/3/14 | 90 | 16 / 12 | +0 / -4 | Mid | hit |
| 2HS | 10/3/20 | 140 | 18 / 13 | -4 / -9 | Mid | hit |
| 2FL | 7/3/13 | 75 | 19 / 13 | +4 / -2 | Mid | hit |
| 2ST | 9/3/22 | 110 | 22 / 13 | KD / -11 | **Low** | never |
| j.P | 5/8/6 | 50 | 14 / 8 | +1 / -5 | High | never |
| j.K | 5/8/6 | 50 | 14 / 8 | +1 / -5 | High | never |
| j.S | 6/6/6 | 80 | 14 / 8 | +3 / -3 | High | never |
| j.HS | 7/5/6 | 120 | 14 / 8 | +4 / -2 | High | never |
| j.FL | 6/6/6 | 70 | 14 / 8 | +3 / -3 | High | never |
| j.ST | 7/5/6 | 110 | 14 / 8 | +4 / -2 | High | never |
| overhead | 24/3/16 | 100 | 23 / 13 | +5 / -5 | High | never |
| throw (techable, loses to a strike) | 2/1/22 | 170 | 0 / 0 | throw | Mid | never |

**Specials.** Costs are deducted on start. EX = motion + S+HS.

| Special | input | s/a/r | dmg | hitstun / blockstun | on hit / block | level | notes |
|---|---|---|---|---|---|---|---|
| chant I | 236+S | 9/3/14 | 65 | 16 / 12 | +0 / -4 | Mid | moves 3 px/f for 8f; feintable; S on f9–23 → chant II |
| chant II | — | 8/3/14 | 70 | 17 / 11 | +1 / -5 | Mid | moves 3 px/f for 6f; feintable; S on f8–22 → chant III |
| chant III | — | 11/3/24 | 100 | 24 / 12 | KD / -14 | Mid | moves 4 px/f for 8f; feintable |
| voice glyph | 236+HS | 13/1/22 | shot | 0 / 0 | — (36f total) | Mid | feintable; glyph shot: hangs 60f, dmg 70, stun 18/16, Mid |
| crystal | 214+S | 15/1/16 | shot | 0 / 0 | — (32f total) | Mid | feintable; crystal shot: plant: arc 2/4 px/f, arms 20f after landing, armed 90f, dmg 90, stun 22/16, Mid, KD |
| shatter | 214+S (crystal planted) | 6/1/12 | — | 0 / 0 | — (19f total) | Mid |  |
| processional | 236+FL | 0/0/18 | — | 0 / 0 | — (18f total) | Mid | passes through; moves 8 px/f for 14f |
| the rite | 63214+FL | 7/2/40 | 180 | 0 / 0 | throw | Mid | feintable; untechable, beats both blocks, loses to invuln |
| consecrate | 214+FL | 10/1/8 | — | 0 / 0 | — (19f total) | Mid | feintable; hold FL up to 60f, +2 gauge/f |
| ascension | 623+S/HS | 4/8/28 | 130 | 19 / 11 | KD / -24 | Mid | full invuln 2–7; launch 9; land +12f; moves 2 px/f; rises 11 px/f; feintable |
| EX glyph | 236+S+HS | 11/1/20 | shot | 0 / 0 | — (32f total) | Mid | CRYSTAL 50; feintable; glyph shot: hangs 90f, dmg 90, stun 22/18, Mid |
| EX crystal | 214+S+HS | 13/1/18 | shot | 0 / 0 | — (32f total) | Mid | CRYSTAL 50; feintable; crystal shot: plant: arc 5/6 px/f, arms 1f after landing, armed 120f, dmg 110, stun 22/16, Mid, KD |
| convergence | 236236+S | 6/6/30 | 340 | 20 / 12 | KD / -23 | Mid | full invuln 1–8; launch 6; meter 1000; moves 14 px/f |

<!-- generated:end -->
