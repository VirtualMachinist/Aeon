# AEON

A grounded 1v1 2D fighter in Rust. Two bodies of the Sanctum — **Kogan** (saber, revolver, disc-shield) and **Raya** (voice glyphs, crystals, the rite). Super Turbo footsies, Samurai Shodown's tax on the heavy buttons, then a measured layer of Roman Cancel, hop, run and feint.

Lights link. Weapon-heavies are minus. There are no normal chains. Damage lives in 2–3 hits. The knockdown is the currency.

- Law: `DESIGN.md`. Numbers: `docs/FRAME-DATA.md` (generated from the code, checked by a test). Grading: `docs/QA.md`.
- Toolchain is pinned to **Rust 1.96.0** by `rust-toolchain.toml`.

```sh
cargo run -p aeon                # title → versus / training / remap
cargo run -p aeon -- --smoke     # scripted launch; writes shots/smoke-*.png and exits
cargo test -p aeon-sim           # the law, the ten QA trials, purity, frame-data doc
cargo clippy --workspace --all-targets
```

## Layout

```text
crates/sim      aeon-sim: deterministic 60 Hz match. Integer subpixels. Zero dependencies.
                No floats in World, no clock, no filesystem, no renderer (tests/purity.rs).
crates/client   aeon: macroquad + gilrs client. Versus, training, stick remap, replays.
crates/client/assets/{kogan,raya}/*.png   one keyed 800×800 pose per state
crates/client/assets/stage/sanctum.png    the Sanctum honeycomb vault
docs/           QA.md (fail-closed rubric), FRAME-DATA.md
tools/keyout.py chroma-keys a generated pose onto the sprite canvas
```

The sim is `World::step(&mut self, p1: InputFrame, p2: InputFrame)`. Same inputs replay to the same `state_hash()`. That is the contract a later rollback layer (GGRS) consumes; the client is replaceable.

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

Tap up = **hop**, hold up = jump. `66` then hold = **run** (a glide). `44` = backdash.

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

Netcode, audio, other bodies, and anything that puts a float in the sim.
