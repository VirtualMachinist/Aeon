# aeon-fighter

Deterministic 60 Hz fighting-game simulation for
[Aeon](https://github.com/VirtualMachinist/Aeon).

The crate is a pure function of `(World, InputFrame, InputFrame) → World`.
No rendering, no wall clock, no floats in the game state, no filesystem.
Same inputs replay to the same `state_hash()`. That is the contract a
rollback netcode layer consumes.

## Install

```toml
[dependencies]
aeon-fighter = "0.1"
```

## Tick

```rust
use aeon_fighter::{CharacterId, InputFrame, World};

let mut world = World::new(CharacterId::Kogan, CharacterId::Raya);
world.tick(InputFrame::default(), InputFrame::default());
let _hash = world.state_hash();
```

`InputFrame` is facing-relative numpad input plus the six-button set
(`P`, `K`, `S`, `HS`, `FL`, `ST`). The caller converts raw keys or stick
state; this crate never talks to a device.

Best-of-three match flow lives on `Match` / `Phase` in addition to the
single-round `World`.

## What this crate is not

It does not draw, play audio, read a clock, or talk to the network.
The published crate is the simulation library only. The playable
macroquad client lives in the same repository as the unpublished
`aeon` package.

## License

**DRAFT.** Copyright © Hedronite LLC. Source-available proprietary EULA — **not** open source, **not** MIT. See [EULA.md](../../EULA.md) in the repository root.

`aeon-fighter` **0.1.0** on [crates.io](https://crates.io/crates/aeon-fighter) was published under MIT in error. That crate release is unchanged from here. This tree follows the EULA.
