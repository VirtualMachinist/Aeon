# Contribute to Aeon

Aeon is a classical 1v1 2D fighter in Rust aimed at **competitive play**.

If you love **Samurai Shodown**, **Street Fighter II: Super Turbo**, **Third Strike**, **Street Fighter Alpha**, **The King of Fighters XIII**, **Guilty Gear Accent Core**, and **Garou: Mark of the Wolves** — and you want those instincts in a modern, deterministic Rust sim — you are invited in.

Help is especially welcome on **character design** and **character development** (moves, timing, identity, animation, feel), always judged against real play.

## What we are building

- Super Turbo footsies at the base; Samurai Shodown's tax on the heavy buttons.
- A measured layer of Roman Cancel / style tools, hop and run, and feint — not an anime fighter.
- Narrow links. No normal chains. Two or three hits are typical; natural three-to-five-hit routes are fine.
- Knockdown is the currency. Competitive balance and stick feel come after the animation milestone already in the README.

Canon: [DESIGN.md](DESIGN.md). Numbers: [docs/FRAME-DATA.md](docs/FRAME-DATA.md). Grading: [docs/QA.md](docs/QA.md). Dev workflow: [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md).

## Who this invite is for

- Rust game developers who enjoy hard real-time systems and clean crates.
- Fighting-game enthusiasts who can argue frame traps, oki, and whiff punish in plain language.
- Artists and designers who can keep Kogan and Raya's identities sharp while the roster stays small.

Finish **Kogan** and **Raya** before inventing new bodies. Gameplay first; aesthetics close behind. Camera FX, audio, and netcode are deferred.

## How to start

```sh
git clone https://github.com/VirtualMachinist/Aeon.git
cd Aeon
cargo fetch --locked
cargo test --workspace --locked
cargo run --release --locked -p aeon
```

On Apple Silicon macOS, after the first fetch, `Play-Aeon.command` is the usual playtest path. Other platforms are not yet validated — reports welcome.

## Good first contributions

- Character kit proposals grounded in [DESIGN.md](DESIGN.md) (normals, specials, rekkas, gauges) with expected frame situations — not wishlist supers alone.
- Balance notes from stick or pad play: what is plus, what is unsafe, what breaks the patient ↔ explosive loop.
- Animation and pose work that preserves identity plates and the [QA](docs/QA.md) / animation review path.
- Sim or client fixes that keep `crates/sim` pure (integer 60 Hz, no floats in `World`, zero dependencies) and regenerate frame-data docs when numbers change.

Open a focused branch from `main`, keep PRs small, and say what you played and what you checked. Prefer evidence: smoke shots, polish-preview traces, or a short written exchange.

## Hard edges (please respect)

- Do not invent roster filler characters.
- Do not reopen settled law for convenience (normal chains, floaty anime mobility, fullscreen fireball wars).
- Do not commit `target/`, `shots/`, `replays/`, local stick config, or credentials.
- Keep Rust pinned to **1.96.0** via `rust-toolchain.toml`.

## Talk to us

Use GitHub Issues for design questions and balance reports. Use Pull Requests for concrete changes. Link the design or frame-data section you are touching.

Competitive play is the goal. If your hands know those games listed above, pull up a chair.
