# Raya crystal and voice glyph — 2026-09-06

ART47 reviewed September 6. Built-in imagegen V1 supplied eight crystal/glyph phases. Focused runtime frames27/31 (crystal) and25 (glyph) show complete hands/cloth, clean body withdrawal, and a readable voice gesture. V1 crystal release remained above the actual low projectile, so V2 lowers its first two poses through a coherent hip/knee bend. Both sources are preserved; V2 accepted after the full candidate1 review.

Original: `/Users/evanpincham/.codex/generated_images/01a07081-3028-76e3-8486-af4a0cdf6e0f/exec-ffb94d04-5b33-4bed-a809-2d1716dd63d8.png`.
Asset: `raya-ranged-v1-green.png`, 1536×1024.

V2 original: `/Users/evanpincham/.codex/generated_images/01a07081-3028-76e3-8486-af4a0cdf6e0f/exec-d631ea3a-7c2f-4609-bee5-cec99a8ebdae.png`.
V2 asset: `raya-ranged-v2-green.png`, 1536×1024. Source inspected: lower crystal gather/release, intact anatomy/feet/cloth and other six phases retained. Runtime calibration accepted with ready roots1290.

Exact V2 edit prompt (input: inspected V1 sheet):

Use case: precise-object-edit. Edit the supplied Aeon Raya eight-pose animation sheet. Change ONLY the first TWO figures in the top row: lower the tiny-crystal preparation and the empty-handed release. The released projectile appears below her knee in the game. In pose1, use a deeper but composed hip hinge and soft knee bend so the cupped hand and its ONE tiny cyan crystal sit just BELOW the forward knee. In pose2, extend the EMPTY forward hand outward just BELOW the forward knee, palm opening gently upward after a low underhand release. Keep the whole body anatomically coherent: bend at hip and knee, do not shorten arms or shrink her body. All other six drawings, cell layout, floor positions, anatomical scale, character identity, face, white linen hood/robes, copper cloak/armor/sandals, cyan jewels, paint texture, and full green gutters stay unchanged. Her face still looks right. Keep both sandals and all fingers and cloth fully inside each cell, with generous margins. No extra crystals after pose1, no effects or text. Flat chroma green background. This is a targeted low-hand correction, not a costume or scale redesign.

References: approved `raya-utility-v1-green.png` for identity/proportions/rendering, `../raya/shot_a.png` and `../raya/shot_b.png` for prior gesture intent. All three inspected before generation.

Exact prompt:

Use case: identity-preserve. Asset type: high-resolution painted 2D game animation sheet for Aeon. Reference image1 is approved Raya identity, proportions and rendering; references2/3 are older crystal and voice gesture intent only. Create eight complete right-facing Raya poses in exactly two rows of four, all full bodies at consistent anatomical scale, separated by wide flat green gutters. Preserve her calm warm face, white linen hood and layered robes, copper cloak/shoulder armor/sandals, cyan jeweled brow and bracers, fine painted shading. Grounded, composed, shoulders relaxed, complete feet and trailing cloth on each pose's floor. Top row is a LOW underhand crystal toss: 1 shallow knee bend with forward hand cupped beside the upper thigh holding ONE tiny cyan crystal, other hand at ribs; 2 low open underhand release just in front of the leading knee, fingertips pointing outward/upward, EMPTY hand, slight weight shift forward, no crystal drawn after release; 3 that empty forearm curls back toward hip with the cloth following softly; 4 upright familiar ready with one empty palm softly open at waist and the other hand near the chest. Bottom row is SPOKEN voice glyph: 5 quiet inhalation with one hand at collar and other palm cupped low near waist; 6 lips slightly parted speaking, one hand beside lower cheek without covering the face and other empty palm offered forward at lower waist level; 7 mouth closes and cheek hand folds down toward chest, low palm retracts; 8 composed upright ready with empty palm softly open at waist. Voice poses have NO drawn projectile, crystal, glyph or rays: the live game draws the placed glyph separately. Cloth remains copper and linen throughout, hanging naturally behind with subtle follow-through; no horizontal flight. No oversized effects, no extra characters, no weapons, no labels or text. Uniform flat chroma green RGB(0,210,0) background, no floor or cast shadow. Full complete head, separate fingers, cloak hems and both sandals in every cell with generous green margins. This is a new eight-pose sheet; preserve identity, not the older high pointing or held crystal gestures.

Accepted source specs (region, horizontal root, anatomical height):

```json
[
  [
    [
      0,
      0,
      390,
      490
    ],
    246,
    395
  ],
  [
    [
      390,
      0,
      765,
      490
    ],
    610,
    395
  ],
  [
    [
      765,
      0,
      1135,
      490
    ],
    975,
    395
  ],
  [
    [
      1135,
      0,
      1536,
      490
    ],
    1290,
    395
  ],
  [
    [
      0,
      490,
      390,
      1024
    ],
    235,
    414
  ],
  [
    [
      390,
      490,
      765,
      1024
    ],
    610,
    414
  ],
  [
    [
      765,
      490,
      1135,
      1024
    ],
    970,
    414
  ],
  [
    [
      1135,
      490,
      1536,
      1024
    ],
    1290,
    414
  ]
]
```

## ART47 — Raya crystal, voice glyph and their EX versions

Eight V2 drawings replace held crystal/voice poses with distinct preparation, release, withdrawal and ready. Crystal ShotA/ExB now bend through the hips and knees, release low with an empty hand, curl the forearm back and rise. Voice ShotB/ExA inhale at the collar, speak beside the cheek with an empty low palm, fold inward and recover. The actual projectile remains independent. Clean cuts remove the old body crossfade; authored phases suppress redundant procedural body displacement. The familiar idle's small held crystal returns only after the action ends.

Benchmark refraction uses original Guilty Gear XX Accent Core footage from fightclubhubbs, GVN Winner Finals part1, Jamie/Testament versus Damian/A.B.A (https://www.youtube.com/watch?v=vSESVFomvWA&t=85s), reopened at 1× in a 1280×720 viewport. Delivered observations cover 82.228–87.957s; the seek overlay partly covers the first second, with clear 83.496–87.957s images. A.B.A recovers her key-bearing upright silhouette while Testament falls/rises independently; a blue floor burst then red light briefly dominates the bodies. Translate recognizable withdrawal and distinct body/effect lifetimes through Raya's restrained linen/copper gestures. Aeon deliberately keeps a much smaller effect hierarchy. Prior paused A2/S2 phases supplement the comparison. No source move classification or exact frame counts are inferred; failed small-viewport observations and playback beyond the last delivered image are excluded.

V1's complete figures were usable but its crystal release remained too high. V2 lowers only the first two drawings with coherent hip/knee flexion. Focus frames 26/27/30/31 show the release meeting the visible projectile's left edge at actual spawn and the subsequent empty withdrawal. Ready roots 1330→1290 remove most of the backward/forward stance jump found in focus frames 37/44. Final candidate frames 36/38/44 show the corrected transition and return to the familiar idle. Full bodies, fingers, sandals and cloth remain intact.

Original timings remain ShotA 15/1/16, ShotB 13/1/22, ExA 11/1/20 and ExB 13/1/18. EX consumes the fixture's 50 gauge; ordinary starts at zero. Glyphs retain their fixed placement, hitboxes and lifetimes. Crystals retain flight, plant, arm, touch/blast and expiry. Existing crystal contact produces two hits (100% then 80% damage: 162 normal, 198 EX), or two chip contacts totaling 24; the fixture verifies rather than changes that behavior. No sim, geometry, damage, gauge or input-window edits.

The legal preview adds all 64 Raya cases: four moves × hit/standing guard/crouched guard/whiff × both facings at center/corners, 210 ticks each. Kogan's existing 48 ranged cases and victory/rematch callers remain intact. All 64 before and candidate1 cases / 224 seconds each played fully at 1×1280×720. Exact final diagnostics supplement brief phases: ShotA 0036/0038/0044/0070/0072; ShotB 0026/0048/0056; ExA 0024/0044/2396/2620/2640; ExB 0026/0040/2336/2350. These cover release, withdrawal, ready, hitstop blast, low corner guard and independent glyph expiry. Before ShotB0026 and ExB0040 were directly compared. Missing ExA2642 was not viewed or counted. The final ready-to-idle cloth/hand change is minor retained style polish; close idle overlap remains a separate open finding.

Citadel checks5 passes 177 tests (89 sim + 88 client), clippy with warnings denied and locked/offline release. Earlier compile-arity and one-versus-two crystal-contact fixture failures are retained in check logs and superseded. All 13,440 focused ticks and four case tables equal the before capture. New 2,100-tick integration trace, 35s video and all 71 diagnostic PNGs are byte-identical to ART46 (and the fully viewed ART45 integration); its complete motion review is explicitly reused. Eight fresh smoke PNGs: seven identical, changed versus-glyph pair directly inspected, confirming a complete body without the previous ghost or painted second projectile and intact HUD.

Evidence in `notes/media/2026-09-05-full-kit/`: `raya-ranged-before/`, `raya-ranged-focus-v1/`, `raya-ranged-focus-v2/`, `raya-ranged-candidate1/`, `raya-ranged-polish/`, `raya-ranged-smoke/`, `raya-ranged-review-progress.json`, `raya-ranged-verification.json` and check logs. Complete four-move videos, traces/cases and 3,136 candidate diagnostics are archived and checksum verified. Remote raw PNGs were thinned only after full local checksum verification. Original V1 is retained as rejected calibration evidence; runtime consumes V2, SHA256 `df61ae9cbcfecf602ff3e0d31d65a06c817dc6b6476c64ab468bcb8a336b46c0`. Exact prompts, originals and eight source regions/roots: `RAYA-RANGED-2026-09-06.md`.

Use `--kit-preview --kit-ranged --kit-raya`, optionally `--kit-move=ShotA` (or ShotB/ExA/ExB); `--kit-legacy-ranged` retains the visual baseline. Continue Raya Detonate/Charge/Super/Feint/Victory, shared polish and legitimate approved-art work for the remaining Kogan lights. The blocked generation remains un-retried. Whole goal and physical stick acceptance remain incomplete.
