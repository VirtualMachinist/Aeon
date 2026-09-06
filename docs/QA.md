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
| ART5 | Authored reaction/recovery and reversal drawings | `sequences.rs`: 32 selected drawings across five assets, source regions and anatomical scales; `authored_regions_preserve_complete_silhouettes_and_effects` checks the actual PNG boundaries. Review stand/crouch recoil, rise/fall, floor/getup and landing for both bodies. Raya grounded recoil remains one drawing per stance; Kogan follow-up coverage is recorded at ART16. Full-kit animation is still incomplete. |
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

## September 5 disc-shield checks

| ID | Gate | Evidence |
|---|---|---|
| ART14 | Kogan disc phases, open visibility and clean return | Four V2 drawings; `disc_preview_covers_close_contact_and_legal_projectile_absorption` exercises 20 legal cases and all phases. Complete before/final 30s viewed at 1×, both facings and center/corners; selected glyph absorption, recovery and crouch-block frames stepped. Source boundary and clean combat-transition regressions include disc cells. |

126 tests pass (89 sim + 37 client), clippy and locked/offline release pass. All 1,800 disc and 2,100 retained polish ticks match baseline. Complete polish video and eight fresh smoke images are byte-identical to the preceding fully inspected saber evidence. S2/A2 refraction and exact before/after review are in the full-kit report. Crouch-entry and Raya reaction/glyph ghosts, remaining families and physical stick play stay open.


## September 5 ground-movement checks

| ID | Gate | Evidence |
|---|---|---|
| ART15 | Kogan ground phases, immediate exits and visible crouch | Eight V4 drawings plus measured original walk cells; 36 legal-input cases verify two walk cycles, 14f backdash, immediate run exits, real block/hit and 2f full-jump landing. Complete before/final 54s viewed at 1×; run, retreat and close-crouch phase boundaries stepped. Client history tests protect freeze/reset/new-input precedence; draw-order test protects low-body visibility and attacker priority. |

129 tests pass (89 sim + 40 client), clippy and locked/offline release pass. All 3,240 ground and 2,100 retained integration ticks match baseline. The complete new 35s integration preview was viewed at 1×; eight fresh smoke images match the preceding inspected evidence byte for byte. G1/K1/S1/S2/A2 refraction, rejected candidates and final before/after evidence are in the full-kit report. Kogan attacks/reactions, Raya's kit and physical stick play remain open.


## September 5 Kogan recoil and floor review

| ID | Gate | Evidence |
|---|---|---|
| ART16 | Grounded impact/guard release, launches and supported floor recovery | All 36 before/final cases / 90s each viewed at 1×, both facings and center/corners; standing/crouched hit/guard and corner getup boundaries stepped. Eight recoil V2 and four floor V1 drawings retain scale/equipment, remove old-body ghosts and preserve immediate legal control. `reaction_preview_covers_grounded_guard_recoil_launch_and_floor_recovery` and `recoil_release_uses_four_remaining_frames_and_yields_to_legal_control` verify consequences, freeze, full recovery and four visible release ticks. |

131 tests pass (89 sim + 42 client), clippy and locked/offline release pass. All 5,400 reaction and 2,100 integration ticks match baseline. Full new 35s integration playback inspected; eight fresh smoke images match preceding inspected evidence byte for byte. G2/S2/A1/S1/A2 refraction, prompts and exact paused ticks are in the full-kit report. Air-normal juggles, tech, remaining Kogan attacks/feint/victory and Raya remain open.

## September 5 Kogan Judgment review

| ID | Check | Evidence |
|---|---|---|
| ART17 | Dual-weapon gather, commitment, withdrawal and reholster | Four V3 drawings; complete baseline/candidate/corrected 16-case matrices at 1× (40s each), both facings at center/corners. Standing and crouching guard plus holster boundaries stepped. Full weapons/cape, separated faces and clean legal return. `judgment_preview_uses_legal_metered_input_and_covers_hit_guard_and_miss` and `judgment_keeps_a_crouched_receiver_visible_for_either_player` verify legal input, consequences, active-only extension, freeze, recovery and ordering. |

133 tests pass (89 sim + 44 client), clippy and locked/offline release pass. All 2,400 final and 2,100 integration ticks match baseline. Complete new 35s integration playback reviewed; repeat smoke supplies all eight images, byte-identical to prior inspected evidence. S2/A2 refraction, rejected holster edits, exact paused ticks and archive paths are in the full-kit report. Remaining Kogan attacks/air/tech/feint/victory and Raya remain open.

## September 5 Kogan airborne revolver review

| ID | Check | Evidence |
|---|---|---|
| ART18 | Air gun intent, complete equipment, downward release and legal landing | Four selected V3/V1 drawings; all 32 before/final cases / 80s each viewed at 1×, hop/full, both facings at center/corners. Release, mirrored contact and exact landing ticks stepped. Air fixture tests all 224 legal loaded/empty air inputs; selector, muzzle-freeze, projected-tail and source-boundary regressions pass. Six air normals remain visually open. |

137 tests pass (89 sim + 48 client), clippy and locked/offline release pass. All 4,800 final, 300 focused and 2,100 integration ticks match baseline. Complete new 35s integration playback reviewed; eight fresh smoke images match preceding inspected Judgment evidence. S1/S2/A2/G1 refraction, excluded malformed cells, exact paused ticks and archive paths are in the full-kit report. Remaining Kogan attacks/tech/feint/victory/air juggles and Raya stay open.


## September 5 Kogan airborne saber review

| ID | Check | Evidence |
|---|---|---|
| ART19 | Distinct short/long/steep contact, withdrawal and legal landing | Six V2 drawings; all 120 final cases / 300s viewed at 1×, both facings at center/corners, hit/high guard/defeated low guard/miss and early recovery. Final JS 55–59, JHS 44/49/54/57 and JST 45/50/55/56/57 stepped. Recent history keeps the saber in front through the existing landing without affecting later jumps. Active-only selection, freeze, source regions, attack expiry, reset and clean transitions tested. |

140 tests pass (89 sim + 51 client), clippy and locked/offline release pass. All 18,000 final, 1,800 focused and 2,100 integration ticks match baseline. Complete new 35s integration playback reviewed; eight smoke hashes match preceding inspected AirShot evidence. S1/S2/A2/G1 refraction, exact baseline-review limits, prompts and archive paths are in the full-kit report. Kogan JP/JK/JFL, remaining grounded normals/throws/tech/feint/victory/air juggles and Raya stay open.


## September 5 Kogan airborne fist, boot and knee review

| ID | Check | Evidence |
|---|---|---|
| ART20 | Distinct downward fist/boot/bent-knee contact, gathered withdrawal and legal landing | Six selected V1/V4 drawings; all 120 corrected final cases / 300s viewed at 1×, both facings at center/corners, hop/full, hit/high guard/defeated low guard/miss and early recovery. Full early JP 39/43/51/55, JK 43/51 and JFL 39/44/49/55 stepped; focused JFL 56/64/65/67 verifies contact freeze and landing. V1 contacts rejected after actual contact review; lower V4 limbs meet both defender heights. Active-only selection, freeze, source regions, expiry/new-state isolation and clean transitions tested. |

142 tests pass (89 sim + 53 client), clippy and locked/offline release pass. All 18,000 corrected final, 900 focused contact and 2,100 integration ticks match baseline. Complete new 35s integration playback reviewed; eight smoke PNGs match preceding inspected airborne-saber evidence. K2/G2/G1 refraction, exact baseline and rejected-candidate review limits, prompts and archive paths are in the full-kit report. Remaining Kogan grounded normals/throws/tech/feint/victory/air juggles and Raya stay open; the original blocked grounded-light request remains un-retried.


## September 5 Kogan standing Flash and Style review

| ID | Check | Evidence |
|---|---|---|
| ART21 | Low pommel and waist-level saber contact, visible withdrawal and original return | Seven V2 drawings, four phases per move with shared withdrawal. All 40 before/final cases / 40s each viewed at 1× in 1280×720, both facings at center/corners, hit/high and low guard/crouched hit/miss. Focused StFL 14/20/27/33/35 and StST 17/22/28/37/39 stepped; standing contact paused in all four positions. Legal input/outcomes, active-only contact, freeze, new-state isolation, source boundaries and clean transitions tested. |

144 tests pass (89 sim + 55 client), clippy and locked/offline release pass. All 2,400 final, 240 focused and 2,100 integration ticks match baseline. Complete new 35s integration reviewed at 1×; eight smoke PNGs match preceding inspected air-light evidence. S2/G2 refraction, excluded blade-tip cells, exact prompts and archive paths are in the full-kit report. Remaining Kogan families and Raya stay open, including her legacy reaction ghosts and the broader old/new style transition.

## September 5 Kogan crouching saber review

| ID | Check | Evidence |
|---|---|---|
| ART22 | Four crouching weapon paths, supported withdrawal and clean low return | Sixteen V1/V3 drawings. All80 before/final cases/140s each reviewed at1× in1280×720, both facings at center/corners, hit/high and low guard/crouched hit/miss. Focused phase and contact steps, final mirrored/corner guards and sweep floor/getup steps. Legal move/outcome, active-only contact, source boundaries, new-state isolation, clean transitions and low-return history regressions pass. |

147 tests (89sim+58client), clippy with warnings denied and locked/offline release pass. All8,400 final,840 accepted focused and2,100 integration ticks match baseline; full new35s integration viewed. Accepted repeat smoke `crouch-smoke2/` has eight PNGs identical to preceding inspected Flash/Style evidence. Incomplete first smoke omitted versus-poke and is retained; three incidental low-focused clips from the preceding binary are labeled unaccepted. Exact S2/A1/G2 refraction, phase steps, prompt provenance, checks and archive paths are in the full-kit report. All sim values remain unchanged. Remaining Kogan families, CrHS airborne targets/air juggles, Raya and physical stick play remain open.


## September 5 Kogan overhead review

| ID | Check | Evidence |
|---|---|---|
| ART23 | Complete standing cut/withdrawal and falling saber/landing | Four new standing drawings and reviewed air-saber/landing reuse. All 40 before/final cases / 80s each viewed at 1× in 1280×720, both facings at center/corners, all five responses. Focused phases and final mirrored/corner contacts, eight landing ticks and defender getup stepped. Legal outcomes, active-only contact, freeze, state isolation, source boundaries and clean transitions pass. |

149 tests (89 sim + 60 client), clippy with warnings denied and locked/offline release pass. All 4,800 final, 480 focused and 2,100 integration ticks equal baseline; complete new 35s integration viewed. Accepted repeat smoke has all eight PNGs identical to the preceding inspected crouching-saber evidence. The incomplete first six-PNG smoke remains archived separately. Standing 22/3/16, falling 18/4/14, eight landing ticks and all simulation values are unchanged. Exact S2/A1/S1/A2/G1 refraction, review steps, prompts and archive paths are in the full-kit report. Remaining Kogan families, Raya and physical stick play stay open.

## September 5 Kogan normal throw and tech review

| Gate | Requirement | Evidence |
|---|---|---|
| ART24 | Deliberate throw recovery and supported tech separation | Four reviewed utility drawings reused at normal-throw timing; two new full-size separation/withdrawal drawings lead into ready. All 32 before/final cases / 80s each and focused 7.5s viewed at 1× in 1280×720; contact, tech and complete returns stepped at center/corners in both facings. |

151 tests (89 sim + 62 client), clippy with warnings denied and locked/offline release pass. All 4,800 final, 450 focused and 2,100 integration ticks match baseline. Complete new 35s integration viewed; eight smoke PNGs retained, seven byte-identical and changed versus-mid directly inspected as the intended normal-throw withdrawal. Original 2/1/20, damage, tech window and all simulation values remain unchanged. G2/A1/S2 refraction and exact evidence are in the full-kit report. Raya tech/guard/getup ghosts and brief pre-grab crouched overlap remain open; remaining kits and physical stick play are not accepted by this batch.

## September 5 Kogan feint review

| Gate | Requirement | Evidence |
|---|---|---|
| ART25 | Canceled equipment withdraws into ready and respects legal landing | Reviewed art reused for all eleven feintable moves. All 88 before/final cases / 176s each and focused 20s viewed at 1× in 1280×720. Selected weapon/guard phases and corrected late-uppercut Jump/landing stepped in both facings at center/corners. |

155 tests (89 sim + 66 client), clippy with warnings denied and locked/offline release pass. All 10,560 final, 1,200 focused and 2,100 integration ticks equal baseline; complete new 35s integration viewed. Eight smoke PNGs retained: four equal prior inspected throw evidence, four changed pairs directly reviewed as wall-clock/menu sampling differences. Exact changes, phase ticks and archived rejected Uppercut transition are in the full-kit report. The eight-frame feint and legal two-tick landings remain unchanged. G2/S2/G1 refraction supplies withdrawal and gathered descent; the cited excerpts are not feint demonstrations. Remaining kits and physical stick play stay open.

## September 5 Kogan victory review

| Gate | Requirement | Evidence |
|---|---|---|
| ART26 | Complete equipment and coherent supported entry into winner hold | Four new V1 drawings; all 16 before/final legal KO cases / 64s each viewed at 1× in 1280×720. Standing/airborne/crouched finishes, next round and rematch, both facings at center/corners. Corrected two-tick crouch rise and full gun phases stepped. |

159 tests (89 sim + 70 client), clippy with warnings denied and locked/offline release pass. All 3,840 final and 2,100 integration ticks equal baseline; full new 35s integration viewed. Eight smoke PNGs retained, four identical and four changed pairs directly reviewed as wall-clock/menu sampling. Final rise correction was recaptured in all four relevant cases; the retained integration and Raya-winner smoke do not enter that branch. S2/G2/A1 informs supported weapon return, not a claim of commercial victory behavior. This accepts Kogan's winner gesture; defeated-body idle/getup remains open and is the next presentation correction. Remaining kits and physical play stay open.

## September 5 both-body KO review

| Gate | Requirement | Evidence |
|---|---|---|
| ART27 | Defeated bodies retain consequence through grounded collapse, actual landing and match end | All 48 before/final real KO cases / 192s each viewed at 1× in 1280×720. Both bodies, facings and corners; standing/crouched hits, sweep, launch, throws, next round and rematch. Corrected eight-case grounded sweep / 32s recaptured and fully viewed. Support, landing and former getup boundaries stepped. |

165 tests (89 sim + 76 client), clippy with warnings denied and locked/offline release pass. All 11,520 final and 2,100 integration ticks equal baseline; full new 35s integration viewed. All eight smoke PNGs equal preceding inspected victory evidence. Integration/smoke do not enter the narrowly corrected grounded-sweep collapse branch; all affected cases were recaptured. Existing reaction/floor art is deliberately reused; A1/G2/S2 informs victim consequence, not a claim of commercial KO behavior. Legacy close low-attack art, Raya living-body trails/HUD overlap, remaining kits and physical play stay open. Full details: full-kit report and ko-verification.json.

## September 5 Kogan crouching punch review

| Gate | Requirement | Evidence |
|---|---|---|
| ART28 | Crouching punch has readable drawn contact/withdrawal and a coherent low return | Four complete V1 drawings. All 20 before/final cases/20 s each viewed at 1× in 1280×720; hit/both guards/crouched hit/whiff, both facings and corners. Selected phase/contact/support/return frames stepped. |

166 tests (89 sim +77 client), clean clippy and locked/offline release. All 1,200 final and 2,100 integration ticks equal baseline; complete new 35 s integration reviewed. Eight smoke images equal preceding inspected evidence. G2/S2 informs the arm's outbound/return and deliberate hold. Original 4/2/6 and all sim values fixed. Remaining grounded lights/kicks, airborne-target/juggle coverage, Raya and physical play stay open. Full-kit report and crp-verification.json hold exact evidence.

## September 5 Kogan airborne exchange review

| Gate | Requirement | Evidence |
|---|---|---|
| ART29 | Airborne contact and non-knockdown recovery retain readable bodies and legal support | All64 before/final cases /160 s each viewed at1× in1280×720: sixteen CrHS anti-airs,24 Kogan normal juggles and24 Kogan-receiver juggles. Four new V1 recoil/tuck/feet/compression drawings; selected contacts and complete landing stepped in both facings/corners. |

169 tests (89 sim +80 client), clippy and locked/offline release pass. All9,600 final and2,100 integration ticks equal baseline; complete new35 s integration reviewed. Fresh repeat smoke contains eight PNGs equal to inspected CrP evidence; first seven-PNG run retained as incomplete. G2/G1/S2/S1 informs consequence, gathered descent and intact weapon continuity. Original timing and all sim values remain fixed. Remaining grounded lights/kicks, Raya and physical play stay open. Exact review: full-kit report and air-recovery-verification.json.

## September 5 Raya movement review

| Gate | Requirement | Evidence |
|---|---|---|
| ART30 | Compact hop and full-size jump preserve support, identity and legal return | All 24 before/final jumps / 24 s each and 64 shared airborne exchanges / 160 s viewed at 1× in 1280×720; preparation, full arc, clean landing and mirrored/corner phases stepped. G1/K1/S1 refraction recorded. |

170 tests (89 sim + 81 client), clippy and locked/offline release pass. All 1,440 movement, 9,600 shared exchange and 2,100 integration ticks equal baseline. Complete new 35 s integration viewed, including stepped uppercut landing. Eight smoke PNGs: seven match prior inspected evidence, changed training tick pair directly inspected. Original failed candidate is retained as incomplete; fresh candidate2 is the unchanged accepted V1. Broader Raya attack/reaction families and physical play remain open. Exact evidence: full-kit report and raya-movement-verification.json.

## September 5 Raya airborne recovery review

| Gate | Requirement | Evidence |
|---|---|---|
| ART31 | Non-knockdown recoil regains supported form within existing control rules | All 40 affected before/final cases / 100 s each at 1× in 1280×720, candidate 10 s and selected contact/descent/corner landing steps; four V1 defensive drawings, G2/G1/S1. |

170 tests (89 sim + 81 client), clippy and locked/offline release pass. All 6,000 final, 600 candidate and 2,100 integration ticks equal baseline. New 35 s integration video is byte-identical to fully reviewed movement evidence, reused explicitly. Eight smoke PNGs: seven identical, changed training tick pair directly inspected. First old-binary capture labeled invalid; corrected candidate2/final accepted. Broader Raya grounded/knockdown reactions, attacks and physical play remain open. Exact evidence: full-kit report and raya-air-recovery-verification.json.
