# First animation review milestone

Code ART67: `87359430c553ebe429a9d47460760238889fb346`. All69 implemented moves (35Kogan/34Raya) and16 Action variants have recorded visual acceptance.185 tests,clippy-Dwarnings and locked/offline release pass; the full simulation tree and generated frame-data document remain unchanged from `af92c10`. Physical stick play and competitive balance are subsequent acceptance.

- [Coverage matrix](ANIMATION-COVERAGE.md)
- [Four-game reference dossier](ANIMATION-REFERENCES.md)
- [Completion audit](ANIMATION-AUDIT.md)
- [Acceptance ledger](FULL-KIT-2026-09-05.md)
- [QA](QA.md), [authored frame data](FRAME-DATA.md), [build and controls](../README.md)

From the checkout, use `cargo run --release --locked --offline -p aeon` after the initial dependency fetch described in the README. Rust1.96.0 is pinned. Apple Silicon macOS is the verified platform. Training and Versus are selectable from the title. Space pauses,period steps,F5 resets,F9 saves andR loads replays;F11 remains bound but can be intercepted by macOS.

The local project review package contains13 labeled video selections,representative before/after footage,current integration,selected diagnostics and check logs. These bulk videos are not bundled in this Git repository. The ledger records exact batch names,acceptance observations and reproducible family selectors. `--polish-preview` runs the35-second integration; `--kit-preview --kit-move=StP` isolates Kogan's jab. Do not accumulate unbounded raw frame captures.

Minor source polish: fine yellow-green coloration on older cape curls and brighter flowing cape highlights. The long thrust has a narrow inspected intact corner margin. Physical play should assess light/link readability,heavy whiff commitment,hop/full-jump landings,immediate glide exits,ritual/effect hierarchy and mirrored corners. Controller mapping,feel and competitive balance remain unaccepted. Input-only replay omits manual fills/dummy changes; the HUD hash reflects the last completed tick after paused mutations. Sustained physical comma holding is not claimed.
