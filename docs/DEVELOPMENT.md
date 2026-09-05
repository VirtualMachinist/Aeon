# Developing Aeon

The primary repository is [VirtualMachinist/Aeon](https://github.com/VirtualMachinist/Aeon). `main` contains the integrated playable build. The September 5 first polish pass is `a9dec50`; its [report](POLISH-2026-09-05.md) separates verified behavior from remaining playtest and animation work.

## Project direction

Aeon is an ongoing project. Finish Kogan and Raya before expanding the roster. Gameplay comes first, with aesthetic close behind; camera effects, audio and netcode are deferred. Do not invent characters to fill roster slots.

Preserve narrow links and no normal chains. Two or three hits are typical; natural three-to-five-hit routes are permitted. Immediate run transitions, safe hop landings, a small full-jump landing tax, and retained attacker pressure are the current direction. Frame traps, reversals, baits and commitment still determine turns. The [design](../DESIGN.md) and [frame data](FRAME-DATA.md) define the implemented rules.

Animation aims for the flow of KOF XIII and Fatal Fury: City of the Wolves, with Samurai Shodown's heavy impact. Kogan's swordwork references Ukyo and Baiken. Raya remains composed, graceful, fluid, beautiful, deadly and terrifying. These are targets to judge in play, not claims that the animation is finished.

## Implementation constraints

- `crates/sim` stays a pure integer 60 Hz simulation, with 256 subpixels per pixel and zero dependencies. No renderer, filesystem, clock, randomness or floating-point state.
- `crates/client` owns presentation and physical input: macroquad, gilrs, menus, replays and training tools.
- Kogan's cape is an aura and never a hurtbox. Preserve both characters' established identity and the distinction between placed shots, gauges and command grabs.
- Keep Rust pinned to **1.96.0**. Fetch the lockfile's dependencies with `cargo fetch --locked` before using offline commands.
- Keep generated-art references and prompts. Identity plates are references, not animation frames. Consumed assets and their [prompt history](../crates/client/assets/animation/PROMPTS.md) live in the repository; private vault references are not required to run the game.

## Change and verify

Work on a focused branch from `main`. Record changes to authored numbers and behavior in the relevant design/frame-data documents. For move-data changes, regenerate the tables:

```sh
AEON_REGEN_DOCS=1 cargo test -p aeon-sim --test frame_data_doc --locked --offline
```

Run checks appropriate to the change before integrating:

```sh
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
git diff --check
```

For gameplay or rendering changes, launch the optimized client and inspect the affected exchanges:

```sh
cargo run --release --locked --offline -p aeon
cargo run --release --locked --offline -p aeon -- --smoke
cargo run --release --locked --offline -p aeon -- --polish-preview
```

`--polish-preview --capture` writes 720 PNGs at a capture rate of 30 fps plus a state trace for the 24-second review. Gameplay remains 60 Hz. Captures are supporting evidence; hardware latency, physical stick input and competitive feel require hands-on play. Use the [QA rubric](QA.md) and report unperformed checks explicitly.

Do not commit build outputs, screenshots, replays, local stick configuration or credentials. `target/`, `shots/` and `replays/` are ignored. Private build-host inventory and synchronization instructions remain outside this public repository.
