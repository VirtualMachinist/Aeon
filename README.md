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

The current build has 129 passing tests and verified versus/training launches. [First polish pass](docs/POLISH-2026-09-05.md) and [motion pass](docs/MOTION-2026-09-05.md) notes record the changes and limits; the [motion QA review](docs/QA-MOTION-2026-09-05.md) records the preceding verification and follow-up fixes. Both kits are playable and every state of both bodies moves through anticipation, contact and recovery with impact effects; the [reaction iteration](docs/REACTIONS-2026-09-05.md) adds 32 selected drawings for reactions, uppercuts, floor recovery and landing. The [full-kit animation batches](docs/FULL-KIT-2026-09-05.md) add reviewed Kogan jump, ground movement, ranged, utility, saber/reversal and disc phases plus focused comparison previews, informed by inspected fighting-game footage. Full-kit animation, stick feel and competitive balance remain ongoing work. Finish Kogan and Raya before expanding the roster.

[Development guide](docs/DEVELOPMENT.md) covers the repository workflow and checks. [Animation prompts](crates/client/assets/animation/PROMPTS.md) preserve the generated-art provenance.
