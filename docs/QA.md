---
file: QA.md
layer: atrium-project
domain: aeon
nature: "QA rubric — Grok / GLM / Kimi fail the Fable pass against this file"
operator: Grok — interview lock under Sovereign Evan; Fable 5.1 bound each gate to its evidence 2026-09-02
created: 2026-09-02
last_verified: 2026-09-05
hal_version: "1.1"
authority: "Sovereign-direct 2026-09-02 — interview lock; Fable 5.1 implements"
---
<!-- hal:authoritative:yaml -->

# AEON — QA

Current direction: [[Aeon/notes/2026-09-05-consultation]]. The September 2 rubric remains the baseline with the explicit September 5 rule changes below. Evidence from earlier passes is historical until checked against the current tree.

This is a fail-closed rubric. A gate is **pass** only when the evidence column has been inspected against the *current* tree on **citadel** (`~/hedronite_repos/elio/aeon`). Intent, chat memory, and "the test file exists" are not evidence. Uncertain, indirect, or missing evidence = **not done**.

Run, on citadel, after `source ~/.cargo/env` and `export PATH="/opt/homebrew/bin:$PATH"`:

```
cd ~/hedronite_repos/elio/aeon
rustc --version              # 1.96.0 via rust-toolchain.toml
cargo test --workspace      # simulation plus client timing/animation checks
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p aeon            # title → versus
cargo run -p aeon -- --smoke # headless-ish launch evidence: writes shots/smoke-*.png and exits
```

Versus must launch from `cargo run -p aeon`. Training is a separate mode from the title, not the only window.

Test files: `crates/sim/tests/combat.rs` (law), `crates/sim/tests/trials.rs` (scripted trials 1–10), `crates/sim/tests/frame_data_doc.rs` (DOC1), `crates/sim/tests/purity.rs` (S1), unit tests in `crates/sim/src/input.rs`.

## Gates

| ID | Gate | Evidence that proves it |
|---|---|---|
| S1 | Sim purity | `purity.rs`: `sim_has_no_dependencies`, `sim_sources_have_no_floats_clock_fs_or_randomness`. `crates/sim/Cargo.toml` has an empty `[dependencies]`. `sub_to_px` (f32) lives in the client (`crates/client/src/render.rs`). |
| S2 | Toolchain | `rust-toolchain.toml` = `1.96.0`. `rustc --version` on citadel prints 1.96.0. Tests were run **on citadel** (see the implementation note for the run). |
| S3 | Determinism | `World::tick(&mut self, p1: InputFrame, p2: InputFrame)`; `same_inputs_replay_to_the_same_state` replays an input log and compares `state_hash()`. Client replays (`F9` save / `F11` play in training) reuse the same log format. |
| B1 | Buttons rebadged | `Btn` / `Buttons` are P/K/S/HS/FL/ST (`crates/sim/src/input.rs`). HUD input display and help panel print those names. No LP/MP/HP anywhere in `crates/`. |
| B2 | Chords | `roman_cancel_spends_250_and_cancels_a_whiff`, `roman_cancel_works_after_hit_or_block`, `roman_cancel_requires_meter`, `roman_cancel_cannot_burst_from_hitstun_or_blockstun`, `feint_cancels_special_startup_to_nothing`, `overhead_chord_is_high`, `ex_chord_spends_character_gauge_not_the_bar`, `ex_without_gauge_does_not_come_out`, `ex_chord_can_land_a_frame_apart`, `throw_chord_is_a_throw`, `chord_same_frame_and_within_window`. |
| M1 | Walk / dash / backdash | `backdash_is_punishable`. Walk speeds per body in `FRAME-DATA.md`. |
| M2 | Hop | `hop_is_a_lower_shorter_arc_than_jump`, `hop_overhead_beats_crouch_block_and_loses_to_stand_block`. Tap/hold split: release up within prejump = hop. |
| M3 | Run-glide | `run_is_universal_and_faster_than_walk`. Client draws `run.png` (a single glide pose, no leg cycle) for `Action::Run`. |
| M4 | Feint | `feint_cancels_special_startup_to_nothing`, `feint_does_not_apply_to_normals_or_after_active`, `feinted_dp_is_punishable_before_a_baited_dp_recovers`. |
| R1 | Rekka machinery | `rekka_parts_follow_and_stopping_early_is_a_different_situation`, `rekka_part_is_roman_cancellable`, `special_cancel_window_is_tight`. |
| R2 | Kogan rekka | saber cut → backcut → thrust on 236+S, thrust knocks down. `FRAME-DATA.md` KOGAN specials. |
| R3 | Raya rekka | chant I → II → III on 236+S; chant III knocks down. Her 5P/2K/5S/2HS footsie normals are in the normals table; `both_bodies_have_shared_combat_jobs`. |
| T1 | Command grab | `command_grab_beats_crouch_block_and_stand_block`, `command_grab_is_untechable`, `command_grab_loses_to_uppercut`, `command_grab_whiff_is_a_long_recovery`. |
| T2 | Normal throw | `normal_throw_beats_crouch_block_now`, `normal_throw_is_techable`, `simultaneous_throws_tech`, `jab_on_the_throw_frame_beats_the_throw`, `throws_whiff_on_stunned_and_airborne_bodies`. The prototype "throw loses to crouch" test is deleted, not skipped. |
| T3 | Uppercut tax | `uppercut_is_invulnerable_early_and_taxed_on_whiff`, `uppercut_on_hit_is_rc_able`. |
| G1 | Super bar | `METER_MAX = 1000`, `RC_COST = 250`, super `meter_cost = 1000`; one field `Fighter::meter`. |
| G2 | Kogan firearm | `kogan_firearm_gauge_spends_and_cools`. `Fighter::gauge: i32`; `GaugeDef` in `chars/kogan.rs`. |
| G3 | Raya crystal | `raya_consecrate_fills_the_crystal_gauge_and_buffs_crystals`, `back_charge_release_is_not_a_stored_attack`. |
| G4 | EX | `ex_chord_spends_character_gauge_not_the_bar`, `ex_without_gauge_does_not_come_out`. |
| P1 | Projectile law | `one_shot_per_owner_per_type_but_types_coexist`, `same_class_shots_cancel`, `kogan_wave_is_short_lived`. |
| P2 | Placed shots | `raya_voice_is_a_hanging_glyph`, `raya_crystal_plants_arms_and_detonates_on_contact`, `raya_can_shatter_an_armed_crystal_early`. |
| P3 | Disc | `kogan_disc_is_plus_and_destroys_a_shot` (+3 on block, on **214+HS** — moved from 214+K by the 2026-09-02 consultation; P/K carry no specials). |
| A1 | Aura ≠ box | `kogan_aura_never_extends_hurtboxes`, `trial_10_cape_is_not_a_box`. Training `F2` outlines the aura in copper, hurtboxes in cyan. |
| D1 | Damage | `damage_scaling_is_100_80_60_45_35`, `no_normal_chains_from_frame_data`, `normals_do_not_cancel_into_normals`. |
| D2 | Knockdown currency | 2ST, command grab, uppercut, rekka 3, crystal blast, super all `knockdown`. `trial_04_sandwich`, `trial_06_oki_triangle` prove oki. |
| V1 | Versus shell | `versus_is_first_to_two_rounds`, `time_over_goes_to_the_healthier_body`. `cargo run -p aeon`: title → select (any pairing, mirrors included) → 3 rounds → KO → winner screen → P/Enter rematch, FL/Esc back to select. `--smoke` writes `shots/smoke-{title,select,versus-*,winner}.png`. |
| V2 | Training shell | Title → Training: F1 dummy (stand / crouch / block-all reads hit level / jump / wakeup DP / wakeup P / tech / CPU off), F2 boxes, F3/F4 swap bodies, F5 reset, Space pause, `.` frame-step, `=` fill meter/gauge, `-` heal, on-screen ADV, F8 remap, F9/F11 replay save/play, F12 screenshot. `dummy_block_all_reads_hit_level_from_move_data`, `measured_advantage_matches_frame_data_on_a_clean_hit`. |
| I1 | Stick | `crates/client/src/input.rs`: gilrs, P1 = first pad, 2×3 default map West/North/RT over South/East/RT2 (Mayflash F700, Android mode). All five chords reachable on the six face buttons. Startup prints `pad N = name (os name) uuid …` and the live map. F8 remap records raw HID codes to `~/.config/aeon/stick.cfg`. |
| I2 | Keyboard | P1 fallback WASD + Y/U/I over H/J/K (tap W = hop, hold = jump, 66 = run). P2 arrows + P/[/] over L/;/'. Same 2×3 shape as the stick. |
| ART1 | Both bodies on screen | `crates/client/assets/kogan/*.png` (38 poses) and `assets/raya/*.png` (36 poses), 800×800 keyed frames anchored at the feet, drawn at the game camera (`shots/smoke-versus-poke.png`). Kogan = hood + cape + saber; Raya = vertical linen + cloak. |
| ART2 | Provenance | Poses generated from the identity plates (`art/plates/01-kogan-hero.png`, `02-raya-congregation.png`) as references, one keyed pose per state, chroma-keyed by `tools/keyout.py`. Nothing from `fight-ready/05` or `06`. Plates unsliced. Vault copies in `Aeon/art/fight-ready/{kogan,raya,stage}/`. |
| ART3 | Stage / HUD | `assets/stage/sanctum.png` (honeycomb vault, threshold, two moons) with a procedural fallback. Firearm gauge is a six-chamber cylinder; crystal gauge is a shard cluster (`render.rs::draw_hud`). |
| DOC1 | FRAME-DATA.md | `frame_data_doc_matches_code` regenerates the tables from `chars/{kogan,raya}.rs` and fails on drift. |
| DOC2 | This file | Vault `Aeon/QA.md` and crate `docs/QA.md` list the same gates (disc on 214+HS in both). |
| DOC3 | Implementation note | `Aeon/notes/2026-09-02-fable-pass.md` (vault) describes what landed, what was authored, and what was verified on citadel. |

## September 5 polish checks

These are evidence requirements, not an assertion that the full game has passed manual QA.

| ID | Gate | Evidence |
|---|---|---|
| FLOW1 | Run responds immediately | `polish.rs::run_immediately_stops_blocks_crouches_attacks_or_jumps`, both bodies/facings. |
| FLOW2 | Hop/full-jump distinction survives attacks | `air_normals_preserve_travel_and_hop_identity_through_recovery`, `hop_landing_accepts_a_ground_button_and_full_jump_owes_recovery`, `uppercuts_keep_their_authored_landing_tax`, `landing_does_not_erase_air_hitstun`, `hop_touchdown_flash_is_a_ground_normal_without_spending_air_gun_gauge`. |
| FLOW3 | Strict legal input timing | `first_free_frame_accepts_a_link_but_an_early_press_is_not_buffered`, `wakeup_and_blockstun_expiry_accept_the_current_input`. No normal-chain rule changes. |
| FLOW4 | Render-rate-independent delivery | Client `timing.rs`: 30–240 Hz schedules, pause/suspension, brief button/up taps, chords/facing. Physical stick event delivery remains a manual check. |
| D3 | Natural five-hit routes permitted | `kogan_link_into_full_rekka_is_a_natural_five_hit_route`: jab link → full rekka, 170 damage, knockdown. The old automatic rejection by hit count is superseded; balance still needs play. |
| ART4 | Animation contact/continuity | Client `sprites.rs` phase tests; `--polish-preview --capture` produces movement/rekka frames and state trace. Inspect both bodies in motion, foot anchoring, mirrored framing, startup/contact/recovery and hitstop. Existing pose coverage is not complete multi-frame animation. |
| PLAY1 | Feel and competitive aspirations | Evan's stick play plus repeated versus sessions: input reliability, strict links, safe hops, full-jump tax, pressure/defense, whiff readability and animation quality. Record hardware and actual observations. Unperformed play stays pending. |
| LAUNCH1 | Launched bodies ride the arc | `combat.rs::a_launched_body_rides_the_arc_into_the_hard_knockdown` (both bodies: never actionable before `Knockdown`, combo holds), `a_juggled_body_whose_stun_ended_in_the_air_lands_like_a_jump`. Uppercut on hit is a knockdown, not a race. |
| CORNER1 | Corner pushback transfers | `combat.rs::cornered_pushback_moves_the_attacker_but_shots_do_not`; universal table row `pushback` in `FRAME-DATA.md`. |
| MOTION1 | Every state moves | Client `anim.rs` tests: every move of both kits and every non-attack state yields a finite, bounded layer; launched bodies tumble and land flat; afterimages only trail a moving body and hold through hitstop; a changed picture crossfades two frames; the win pose only replaces a body at rest. Inspect `--polish-preview --capture` for both bodies. |
| FX1 | Impact reads without the camera | `fx.rs`: sparks, dust, rings and flashes spawn from `World::events` and state transitions, live in simulation frames, hold through hitstop. Reaction scene test `preview.rs::reaction_review_launches_grabs_and_lands_the_super`. No camera shake, zoom or slowdown. |
| FX2 | Frozen effects and training resets | `fx.rs::tests::frozen_cast_and_landing_spawn_once` reproduces a freeze on a cast/landing frame; one release makes one effect. `roman_cancel_on_an_unchanged_world_frame_still_spawns_once` covers RC without advancing `World::frame`. F3/F4/F5 and successful F11 playback clear presentation history immediately, including while paused; inspect reset/frame-step in training. |

## September 5 ranged animation checks

| ID | Gate | Evidence |
|---|---|---|
| ART11 | Kogan grounded revolver, wave and EX release/return | Eight V5 drawings; `ranged_preview_exercises_release_guard_whiff_and_recovery` verifies 48 legal-input cases and all four phases per action. Full 120-second final viewed at 1×; normal/EX muzzle release, wave withdrawal and crouch guard frame-stepped. V3 muzzle mismatch corrected in art. |

Ranged batch: 120 workspace tests, clean clippy and locked/offline release. Focused 7,200-tick and retained 2,100-tick traces match their baselines byte for byte. Eight smoke images match the previously inspected batch by SHA-256. S2/A2 comparison and precise review ledger are in `docs/FULL-KIT-2026-09-05.md` and the vault full-kit pass. Air gun, victim reaction refinement and the remaining full-kit families stay open.

## September 5 full-kit movement checks

| ID | Gate | Evidence |
|---|---|---|
| ART8 | Kogan drawn prejump/hop/full-jump coverage | Eight movement cells; 24 directional/facing/position cases viewed at 1×, selected final landing frames stepped. `movement_preview_preserves_hop_and_jump_landings` verifies phases and 0f/2f durations. |
| ART9 | Clean movement transitions | `movement_landing_does_not_leave_an_airborne_ghost`; authored movement cuts instead of crossfading, with anatomical scale and projected air roots. Final frames at 0.633–0.683 and 12.933–12.983 seconds were inspected. |
| ART10 | Isolated hit/block/whiff coverage | `--kit-preview` and `lights_preview_exercises_legal_moves_and_guard_outcomes` exercise standing P/K and crouching K for both bodies. Dedicated Kogan CrK → crouched Raya baseline exposes the existing wrong punch drawing and overlap; attack/reaction acceptance remains open. |

Full-kit movement report: [[Aeon/notes/2026-09-05-full-kit-pass]] and crate `docs/FULL-KIT-2026-09-05.md`. Current batch: 119 tests, clean clippy, release build; focused 1,440-tick movement and retained 2,100-tick polish traces byte-identical to baseline. These checks accept standalone Kogan jumps, not air attacks or the complete kits. Four-game observations are recorded in [[Aeon/notes/2026-09-05-animation-references]].

## September 5 reaction animation checks

| ID | Gate | Evidence |
|---|---|---|
| ART5 | Authored reaction/recovery and reversal drawings | `sequences.rs`: 32 selected drawings across five assets, source regions and anatomical scales; `authored_regions_preserve_complete_silhouettes_and_effects` checks the actual PNG boundaries. Review stand/crouch recoil, rise/fall, floor/getup and landing for both bodies. Grounded recoil remains one drawing per stance; full-kit animation is still incomplete. |
| ART6 | Reversal continuity through the full airborne action | `reversal_art_follows_startup_rise_apex_and_descent_for_both_facings`, `an_uppercut_keeps_its_descent_drawing_after_attack_expiry`, and `anim.rs::authored_falling_and_reversal_drawings_are_not_rotated_twice`. Review the end of Kogan's attack frames through the remaining committed fall and landing. |
| ART7 | Floor contact, keying and corner visibility | `a_throw_reaction_reaches_the_floor_and_all_getup_drawings_without_changing_the_sim`; key/despill tests preserve cyan, linen, copper and existing alpha. `render.rs::both_stage_walls_leave_room_for_a_reaction_without_changing_zoom`. Inspect scenes 6/7 tick 150 and scene 7 tick 300; fixed wall margins and backdrop overscan leave room for silhouettes. No camera effects. |

Pass report: crate `docs/REACTIONS-2026-09-05.md` and vault [[Aeon/notes/2026-09-05-reaction-pass]]. These gates establish incremental coverage, not full animation or competitive acceptance. Legal 0f hop / 2f jump and move-specific landing recovery are unchanged.

Animation atlas provenance: `crates/client/assets/animation/PROMPTS.md` and vault `art/fight-ready/animation/`. Generated sheets are keyed once in the client; original plates remain unsliced. Pass evidence: [[Aeon/notes/2026-09-05-polish-pass]].

Latest independent review: crate `docs/QA-MOTION-2026-09-05.md` and vault [[Aeon/notes/2026-09-05-motion-qa]].

## Scripted trials

Each trial is a headless test in `crates/sim/tests/trials.rs` **and** a thing you can do in training. A trial fails if the described outcome does not happen.

1. **Jab link.** `trial_01_jab_link` — 5P into 5P on hit is two hits, not a chain.
2. **Whiff tax.** `trial_02_whiff_tax` — far 5HS whiffs, 5S punishes before recovery ends.
3. **Disc vs voice.** `trial_03_disc_vs_voice` — Raya glyph, Kogan 214+HS destroys it and keeps his turn.
4. **Sandwich.** `trial_04_sandwich` — Raya knockdown → crystal in front → 236+FL behind; defender wakes between body and armed crystal.
5. **Hop mix.** `trial_05_hop_mix` — hop j.HS hits crouch-block, is blocked standing; empty hop 2K hits stand-block.
6. **Oki triangle.** `trial_06_oki_triangle` — command grab beats a blocking dummy; uppercut beats the grab; meaty beats a wakeup button; blocked wakeup DP is punished.
7. **Uppercut xx RC.** `trial_07_uppercut_rc` — 623 hits, S+FL spends 250 and continues; without meter the tax lands.
8. **Feint DP.** `trial_08_feint_dp` — feinted rekka baits a DP; attacker punishes.
9. **Throw tech / jab.** `trial_09_throw_tech_and_jab` — P+K vs P+K techs; P on the throw frame wins.
10. **Cape.** `trial_10_cape_is_not_a_box` — a jab through cape-only space whiffs.

## Reviewer protocol

1. Pull or rsync the citadel tree (source only). Do not grade castle `target/` or chat.
2. Run S2 then S1 then `cargo test -p aeon-sim`. If tests are red, stop; the pass is failed.
3. Launch versus. Play trials 1–10 or run `cargo run -p aeon -- --smoke` and read `shots/`. Note which trials were not performed by hand.
4. Read `docs/FRAME-DATA.md` against `crates/sim/src/chars/{kogan,raya}.rs` (or trust `frame_data_doc_matches_code`, which does that read).
5. File findings as **blocker** (a gate fails), **tune** (law holds, numbers feel wrong — expected; we tune later unless a feel-target in `DESIGN.md` is violated), **nits**.
6. A feel-target violation (weapon-heavies plus on block, normal chains, cape in the hurtbox, Raya as a fullscreen zoner, hop indistinguishable from jump) is a **blocker**, not a tune.

Related: [[Aeon/HANDOFF]] · [[Aeon/DESIGN]] · [[Aeon/FRAME-DATA]] · [[Aeon/INFRA]] · [[Aeon/notes/2026-09-02-fable-pass]] · [[Aeon/notes/2026-09-03-grok-qa]]

## September 5 cape-snare / threshold-step checks

| ID | Check | Evidence |
|---|---|---|
| ART12 | Kogan cape capture/release and grounded step phases | Eight V3 drawings; `utility_preview_exercises_snare_evasion_and_step_without_new_mechanics` exercises 28 legal-input cases, guard capture, miss/jump escape, recovery and authored step travel. Entire 70-second final viewed at 1×, both facings and center/corners. Grab release/ready and step launch/brake/settle frame-stepped; old-cell trail and backward ending saber corrected. |

121 tests pass (89 sim + 32 client), clippy and locked/offline release pass. Full 4,200-tick utility and retained 2,100-tick polish traces match their baselines. A1/G2/S2 and G1 refraction, before/after footage and final diagnostics are recorded in the full-kit pass. This accepts two Kogan actions; Raya's older reaction/landing/getup crossfades, other families and physical stick play remain open.

## September 5 saber / rekka / reversal checks

| ID | Gate | Evidence |
|---|---|---|
| ART13 | Kogan saber phases, complete silhouettes and clean recovery | Four short-poke and two compact reversal drawings; measured cut regions and matching-cell trails. `saber_preview_reaches_normals_rekka_followups_ex_and_reversal_legally` verifies 128 cases. Complete final 328s viewed at 1×; selected poke, corner thrust and reversal phase boundaries stepped. Keyed-art tests protect source regions, HUD and corner clearance. |

125 tests pass (89 sim + 36 client), clippy and locked/offline release pass. All 19,680 final saber ticks and 2,100 retained polish ticks match baseline traces. The complete 35s integration preview and eight fresh smoke screenshots were inspected after fixed framing changed. S2/A1/G2 and S1/A2/K1 refraction, before/after defects and corrections are in `docs/FULL-KIT-2026-09-05.md` and the vault full-kit pass. Remaining Kogan/Raya families and physical stick play remain open.
