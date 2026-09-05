# Reaction and reversal animation — September 5, 2026

This pass adds drawn reactions, floor recovery, landing and uppercut phases for Kogan and Raya. It builds on the reviewed motion pass `a9a2082`; the remaining full kits still need animation work.

## What changed

- Five new source assets provide 32 selected drawings: 12 reactions/recovery/landing cells and four reversal phases per body. Kogan's dedicated coil drawing replaces the first cell of his uppercut sheet. The loader reports 33 source cells because it retains that unused sheet cell.
- `sequences.rs` selects startup, rise, apex and descent from the existing action and vertical velocity. It keeps the descent drawing when an uppercut's attack expires before the body lands. Launched reactions lead into the floor and four getup drawings; landing samples four available drawings within its existing duration.
- Source rectangles follow measured green gaps rather than assuming a uniform grid. Per-cell anatomical scale and horizontal roots preserve body size; the lowest visible pixel supplies floor contact. Existing procedural tumble and stretch are disabled for these drawings because the art already contains the bend and rotation. This fixes the below-floor silhouettes found in the previous reaction review.
- Trails are lighter, with Kogan limited to two afterimages. Chroma-key cleanup now also processes the original transparent poses. Weak green spill is reduced; an edge-only warm correction reduces yellow fringes without recoloring interior ornament, cyan or linen.
- Fixed framing extends 96 world pixels beyond each collision wall, leaving room for fallen bodies and cape silhouettes at the corner. Backdrop overscan covers this range while preserving its aspect ratio. Fighter scale, collision walls and camera effects are unchanged.

Generated with the built-in image-generation tool from the approved identity plates and poses. Plates remain unsliced. Original generated variants are retained in the project art archive. Selected runtime PNGs and full prompts/selection decisions are in [`crates/client/assets/animation/REACTIONS-2026-09-05.md`](../crates/client/assets/animation/REACTIONS-2026-09-05.md).

## Verification

All builds and captures ran on the development host with pinned Rust 1.96.0:

| Check | Result |
|---|---|
| `cargo test --workspace --locked --offline` | 116 pass: 89 simulation, 27 client |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Clean |
| `cargo build -p aeon --release --locked --offline` | Pass |
| Release `--polish-preview --capture` | Eight scenes, 1,050 captured frames; 2,100 draws in 35.03 seconds, 60.0 draws/s |
| Simulation trace comparison | Entire 2,100-tick trace byte-identical to the baseline capture from `a9a2082` |
| Review video | 35 seconds, 1,280 × 800, 30 fps; encoded and decoded without errors |
| Release `--smoke` | Eight screenshots; both base pose sets and all five new assets loaded |
| Source scope | No changes to `crates/sim`, authored frame data, dependency lockfile or toolchain pin |

New checks cover actual PNG extraction boundaries, reversal phase selection for both bodies and facings, post-attack descent continuity, floor/getup selection without sim mutation, legal landing timing, double-rotation prevention, despill and fixed corner framing.

The eight-scene capture completed and selected exchange frames were inspected, including Kogan's rise and continued descent, both floor/getup sequences, the previously below-floor scene 6/7 tick-150 reactions, and the scene 7 tick-300 corner super. Final smoke winner, training boxes and versus contact screenshots were also inspected. Crouched recoil was reviewed in the source atlas; this preview does not provide a dedicated crouched-hit exchange. The complete video was encoded/decoded, not reviewed frame by frame.

## Remaining work

This is incremental art coverage. Grounded recoil remains one drawing per stance. Short 2f full-jump landings sample two of the four landing drawings; safe 0f hops add no recovery or forced animation. Move-specific landing taxes remain as authored.

Legacy poses and the newer sequences still show differences in rendering style and transition scale. Some older cape edges retain colored fringes. Raya's tall ascension crystal effects can overlap the timer area near the apex; effect silhouette/headroom remains a polish item. The visible body and inspected floor/corner reactions are improved, but this is not a declaration of complete visual QA across every state.

Continue with Kogan's lights, kicks and jump family, then cape-snare, revolver and wave; apply the same temporal coverage to Raya's footsies, glide, glyph and crystals. Recheck existing/new pose transitions during those passes. No roster expansion, audio, netcode or camera effects in this iteration. Physical stick enumeration, motion/chord reliability and repeated versus feel were not performed; stick tuning remains deferred until animation maturity.
