# AEON

Repository: [VirtualMachinist/Aeon](https://github.com/VirtualMachinist/Aeon). `main` contains the current integrated build.

A grounded 1v1 2D fighter in Rust. Two bodies of the Sanctum — **Kogan** (saber, revolver, disc-shield) and **Raya** (voice glyphs, crystals, the rite). Super Turbo footsies, Samurai Shodown's tax on the heavy buttons, then a measured layer of Roman Cancel, hop, run and feint.

Lights link. Weapon-heavies are minus. There are no normal chains. Two or three hits are typical; natural three-to-five-hit routes are allowed. Execution stays strict. The knockdown is the currency.

- Law: [DESIGN.md](DESIGN.md). Numbers: [FRAME-DATA.md](docs/FRAME-DATA.md), generated from code and checked by a test. Grading: [QA.md](docs/QA.md).
- Toolchain is pinned to **Rust 1.96.0** by `rust-toolchain.toml`.
- Verified platform: Apple Silicon macOS. Other platforms have not yet been validated.

## Get started

Install Rust through rustup and the platform C/linker toolchain, then clone and fetch dependencies once:

```sh
git clone https://github.com/VirtualMachinist/Aeon.git
cd Aeon
cargo fetch --locked
cargo run --release --locked -p aeon
```

On macOS, after dependencies are fetched, double-click `Play-Aeon.command` for subsequent optimized playtests. Its build cache lives under `~/Library/Caches/AeonBuild`, outside the source tree. The launcher uses offline mode; a fresh checkout needs the initial fetch above.

## Development commands

```sh
cargo run --release -p aeon     # title → versus / training / remap
cargo run -p aeon -- --smoke     # scripted launch; writes shots/smoke-*.png and exits
cargo test --workspace          # simulation law plus client timing/animation checks
cargo run --release -p aeon -- --polish-preview  # repeatable 35-second movement/rekka/whiff/reaction review
cargo run --release -p aeon -- --polish-preview --capture # 30 fps PNGs + trace in shots/polish
cargo run --release -p aeon -- --kit-preview --kit-movement # 24 isolated hop/jump cases
cargo run --release -p aeon -- --kit-preview --kit-ranged # 48 gun/wave/EX hit, guard and miss cases
cargo run --release -p aeon -- --kit-preview --kit-utility # 28 cape-snare / threshold-step cases
cargo run --release -p aeon -- --kit-preview --kit-saber # 128 saber / rekka / reversal cases
cargo run --release -p aeon -- --kit-preview --kit-disc # 20 defensive shield cases
cargo run --release -p aeon -- --kit-preview --kit-ground # 36 walk / glide / crouch / retreat cases
cargo run --release -p aeon -- --kit-preview --kit-reaction # 36 Kogan victim hit / guard / launch / floor cases
cargo run --release -p aeon -- --kit-preview --kit-reaction --kit-raya # 36 Raya victim cases
cargo run --release -p aeon -- --kit-preview --capture # 60 fps lights cases + trace in shots/kit
cargo clippy --workspace --all-targets -- -D warnings
```

## Layout

```text
crates/sim      aeon-sim: deterministic 60 Hz match. Integer subpixels. Zero dependencies.
                No floats in World, no clock, no filesystem, no renderer (tests/purity.rs).
crates/client   aeon: macroquad + gilrs client. Versus, training, stick remap, replays.
                sequences.rs selects authored reactions/reversals; anim.rs adds motion; fx.rs draws impact.
crates/client/assets/{kogan,raya}/*.png   one keyed 800×800 pose per state
crates/client/assets/animation/*.png     authored walk/attack cells, keyed at load
crates/client/assets/stage/sanctum.png    the Sanctum honeycomb vault
docs/           QA.md (fail-closed rubric), FRAME-DATA.md
tools/keyout.py chroma-keys a generated pose onto the sprite canvas
```

The sim is `World::tick(&mut self, p1: InputFrame, p2: InputFrame)`. Same inputs replay to the same `state_hash()`. That is the contract a later rollback layer (GGRS) consumes; the client is replaceable.

## Controls

Six buttons in a 2×3, the same shape on stick and keyboard:

```
  P     S     HS
  K     FL    ST
```

| | Stick (gilrs, P1) | Keyboard P1 (fallback) | Keyboard P2 |
|---|---|---|---|
| move | left stick / d-pad | `W A S D` | arrows |
| P S HS | West North RT | `Y U I` | `P [ ]` |
| K FL ST | South East RT2 | `H J K` | `L ; '` |

The default pad map is the Street-Fighter-on-Xbox convention, which is where a Mayflash F700 in Android mode lands. **F8** opens the in-game remap (records raw HID codes, saved to `~/.config/aeon/stick.cfg`). Startup prints every pad gilrs sees and the live map.

Tap up = **hop**, hold up = jump. `66` then hold = **run** (a glide). `44` = backdash. Run immediately stops, blocks, jumps or attacks. Hops add no landing recovery; full jumps add 2f, while committed moves keep their own landing tax.

| Chord | Verb |
|---|---|
| `P+K` | throw (techable, jabbable) |
| `S+FL` | Roman Cancel, 250 |
| `FL+ST` | feint a special's startup |
| `HS+ST` | standing overhead |
| `S+HS` | EX (motion + chord; spends character gauge) |

Specials: `236+S` rekka (press S again for parts 2, 3) · `623+S/HS` uppercut · `63214+FL` command grab · `236+FL` command dash · `214+S` shot A · `236+HS` shot B · `214+HS` Kogan disc / `214+FL` hold Raya consecrate · `236+ST` Kogan falling saber · `j.FL` Kogan air gun · `236236+S` super. The training help panel (`/`) lists both kits with their names.

## Training

`F1` dummy (stand · crouch · block-all · jump · wakeup DP · wakeup P · tech · CPU off) · `F2` boxes (push / hurt / hit, aura outlined separately) · `F3 F4` swap bodies · `F5` reset · `Space` pause · `.` frame-step · `=` fill meter and gauge · `-` heal · `F9` save replay · `F11` play latest replay · `F12` screenshot. Frame advantage after every exchange is measured from the sim and shown as `ADV`.

## Versus

Character select (any pairing, mirrors included) → best of three rounds, 99 s each → KO / time over / double KO → winner screen → rematch or back to select.

## Out of scope this pass

Netcode, audio, camera effects, other bodies, and anything that puts a float in the sim.

The current build has 169 passing tests and verified versus/training launches. [First polish pass](docs/POLISH-2026-09-05.md) and [motion pass](docs/MOTION-2026-09-05.md) notes record the changes and limits; the [motion QA review](docs/QA-MOTION-2026-09-05.md) records the preceding verification and follow-up fixes. Both kits are playable and every state of both bodies moves through anticipation, contact and recovery with impact effects; the [reaction iteration](docs/REACTIONS-2026-09-05.md) adds 32 selected drawings for reactions, uppercuts, floor recovery and landing. The [full-kit animation batches](docs/FULL-KIT-2026-09-05.md) add reviewed Kogan jump, ground movement, ranged, utility, saber/reversal, disc, recoil, floor recovery, Judgment, airborne revolver and distinct airborne saber/fist/boot/knee and standing Flash/Style and crouching saber and overhead/throw/tech phases plus focused comparison previews, informed by inspected fighting-game footage. Full-kit animation, stick feel and competitive balance remain ongoing work. Finish Kogan and Raya before expanding the roster.

[Development guide](docs/DEVELOPMENT.md) covers the repository workflow and checks. [Animation prompts](crates/client/assets/animation/PROMPTS.md) preserve the generated-art provenance.

Judgment comparison: `cargo run --release -p aeon -- --kit-preview --kit-super`. Sixteen legal super cases cover hit, standing/crouching guard and miss in both facings at center/corners; `--kit-response` filters before `--kit-case`.

Airborne comparison: `cargo run --release -p aeon -- --kit-preview --kit-air --kit-move=AirShot`. Thirty-two gun cases cover hop/full jump, hit/standing/low guard/miss, both facings at center/corners. `--kit-jump=hop|full`, move and response filters apply before `--kit-case`. JS/JHS/JST and JP/JK/JFL each have a reviewed 120-case matrix including early misses. Add --kit-air-early for legal apex-input misses that expose full-jump withdrawal.

Air-saber comparison: `cargo run --release -p aeon -- --kit-preview --kit-air --kit-air-early --kit-move=JHS --kit-jump=full`. Original timing and landing rules are preserved; the full-kit report records 300s of final playback and all three contact paths.

Air fist/boot/knee comparison: `cargo run --release -p aeon -- --kit-preview --kit-air --kit-move=JFL --kit-response=CrouchBlock`. Empty-cylinder JFL uses a bent knee; JP and JK retain distinct downward fist and boot contacts. The full-kit report records the corrected 300s final review and unchanged timing.

Crouching saber comparison: `cargo run --release -p aeon -- --kit-preview --kit-crouch`. Eighty legal cases cover CrS/CrHS/CrFL/CrST, hit/high and low guard/crouched hit/miss in both facings at center/corners. `--kit-move`, `--kit-response` and `--kit-case` isolate an exchange. The full-kit report records all 140s of final playback and the unchanged low-guard/knockdown rules.

Overhead comparison: `cargo run --release -p aeon -- --kit-preview --kit-overhead`. Forty legal cases cover standing and falling overheads against hit, standing guard, crouching guard, crouched hit and miss in both facings at center/corners. Move/response/case filters isolate exchanges. All 80s of final playback and the unchanged landing rules are documented in the full-kit report.

Throw comparison: `cargo run --release -p aeon -- --kit-preview --kit-throw`. Thirty-two legal cases cover hit, both guards, crouched hit, miss, jump escape and early/late throw tech in both facings at center/corners. Move/response/case filters isolate exchanges; all 80s of final playback and original timing are documented in the full-kit report.

Feint comparison: `cargo run --release -p aeon -- --kit-preview --kit-feint`. All eleven feintable Kogan commitments have early/late startup cases in both facings at center/corners (88 cases / 176s). `--kit-feint-timing=early|late` and move/response filters precede `--kit-case=N`. Original eight-frame cancel and legal landing behavior are retained.

Victory review: `cargo run --release -p aeon -- --kit-preview --kit-victory`. Sixteen real KO cases cover standing and airborne finishes, crouch recovery, next round and rematch in both facings at center/corners. `--kit-victory-state=Standing|Air|NextRound|Rematch` filters before `--kit-case=N`. Kogan winner art is reviewed; defeated-body continuity remains open.

KO review: `cargo run --release -p aeon -- --kit-preview --kit-ko`. Default victim is Kogan; `--kit-raya` selects Raya. `--kit-move=StP|CrK|Uppercut|CrST|Throw|CommandGrab` filters before `--kit-case=N`. Forty-eight real KO cases cover both facings/corners, grounded collapse, actual landing, persistent floor, next round and rematch. Both defeated bodies are reviewed; remaining kit work continues.

Crouching punch review: `cargo run --release -p aeon -- --kit-preview --kit-crp`. Twenty cases cover hit, standing/crouching guard, crouched hit and whiff, both facings at center/corners. Kogan CrP is reviewed with four drawn phases and unchanged 4/2/6; `--kit-raya` is available for future Raya review.

Airborne exchange review: `cargo run --release -p aeon -- --kit-preview --kit-air-exchange`. Kogan has16 CrHS anti-airs and24 uppercut/RC/normal juggles; `--kit-raya` selects24 Kogan-receiver juggles. Move and case filters isolate routes. Four defensive keys retain the forward saber through the existing landing.

Raya movement review: `cargo run --release -p aeon -- --kit-preview --kit-movement --kit-raya`. Twenty-four standalone jumps cover hop/full, neutral/forward/back and both facings at center/corners. Eight drawings preserve a compact hop and full-size jump with clean original 0f/2f returns. All cases, 64 shared airborne exchanges and retained integration are reviewed; broader Raya attacks/reactions remain open.

Raya airborne recovery review: `cargo run --release -p aeon -- --kit-preview --kit-air-exchange --kit-move=JST`. Four defensive keys recover diagonal recoil into gathered descent, feet and support within original stun and landing. Forty affected anti-air/juggle cases are reviewed; move/case filters isolate them. Broader Raya ground/knockdown and attack coverage remains open.

Raya ground review: `cargo run --release -p aeon -- --kit-preview --kit-ground --kit-raya`. Nine walk/crouch/run-exit/retreat states in both facings at center/corners; state/case filters isolate sequences. All36 cases/54 s reviewed; shallow run gather and supported retreat preserve immediate control and original timing.

Raya grounded hit/guard and retained floor recovery are reviewed across36 cases /90 s, with eight Kogan low-return regression cases and complete35 s integration. 171 workspace tests, clippy and release pass; simulation traces unchanged. Full-kit work continues; see [review evidence](docs/FULL-KIT-2026-09-05.md).

Raya standing palm and low kick now have four reviewed phases each, with corrected contact height and clean return. All40 cases /40 s and complete35 s integration reviewed;171 tests, clippy/release and unchanged simulation traces. Use `--kit-preview --kit-raya --kit-move=StP` or `StK`. See [full-kit evidence](docs/FULL-KIT-2026-09-05.md); broader goal continues.

Raya crouching palm and ankle kick now have four reviewed phases and clean low returns. All40 cases /40 s and complete35 s integration reviewed;171 tests, clippy/release and unchanged simulation traces. Use `--kit-preview --kit-raya --kit-crp` or `--kit-move=CrK`. [Full-kit evidence](docs/FULL-KIT-2026-09-05.md); broader goal continues.
