# Raya consecrate and manual shatter — 2026-09-06

ART48 reviewed September 6. Built-in imagegen produced six supported charge/descent/hold/rise poses and two empty-hand command poses. Full source inspected: complete body/cloth, supported knee and palms, restrained breath variation, distinct empty-hand pinch/open. Runtime calibration accepted after complete before/candidate1 motion review.

Original: `/Users/evanpincham/.codex/generated_images/01a07081-3028-76e3-8486-af4a0cdf6e0f/exec-ee09b660-f63b-4638-a9d2-b9dc5e11794b.png`.
Asset: `raya-ritual-v1-green.png`,1536×1024. References: inspected approved utility V1 and older `raya/charge.png`.

Exact prompt:

Use case: identity-preserve. Asset type: high-resolution painted 2D animation sheet for Aeon. Use image1 for Raya's approved face, body proportions, copper and white linen materials and rendering; image2 is the older kneeling consecrate gesture only. Create eight complete RIGHT-FACING poses, two rows of four with broad flat green gutters and consistent anatomical scale. Preserve her calm face, white linen hood/robes, copper cloak and shoulder armor, copper sandals, cyan jeweled brow and bracers. Her ritual is composed and grounded. The first six poses form a kneeling consecration and return: 1 knees soften, hips lower halfway toward a kneel, hands reaching down; 2 one supported knee on the floor, both palms resting gently on the ground ahead, head bowed slightly, cloak pooled naturally behind; 3 the SAME supported kneel and hand positions, a subtle raised breath through chest and slightly changed cloth folds, no spatial drift; 4 one foot planted under her hips, one knee still supported, hands lifting from the floor as her torso rises; 5 half-standing on both feet with soft knees and hands drawn toward chest; 6 upright familiar ready, one empty palm open at waist and the other hand near chest. Poses7 and8 are a brief command to detonate a remote crystal: 7 upright, one hand at chest and forward hand gently pinching thumb/index near sternum, focused gaze right; 8 forward EMPTY hand extends at waist with palm facing down, fingers decisively opening, shoulder relaxed and full face visible. No painted crystals, glyphs, chains, rings, beams or effects; the game supplies effects separately. In every pose preserve full body, separate fingers, complete feet/sandals and cloak hems, generous green margins, no clipped cloth. Kneeling bodies must keep the same limb/head scale as standing bodies, not enlarge to fill the cell. All cell floor lines align within each row. Painted shading and fine materials, not pixel art. Uniform flat chroma green RGB(0,210,0) background, no text, labels, floor, cast shadow or extra characters.

Accepted source SHA256: `e77629a0b0b44a8e1c192ccff647182d5731a2143fd1c7375b4b17b62b7411ec`. Regions, horizontal roots and anatomical heights:

```json
[
  [
    [
      0,
      0,
      375,
      470
    ],
    230,
    415
  ],
  [
    [
      375,
      0,
      738,
      470
    ],
    565,
    415
  ],
  [
    [
      738,
      0,
      1090,
      470
    ],
    910,
    415
  ],
  [
    [
      1090,
      0,
      1536,
      470
    ],
    1310,
    415
  ],
  [
    [
      0,
      470,
      395,
      1024
    ],
    255,
    418
  ],
  [
    [
      395,
      470,
      770,
      1024
    ],
    585,
    418
  ],
  [
    [
      770,
      470,
      1120,
      1024
    ],
    945,
    418
  ],
  [
    [
      1120,
      470,
      1536,
      1024
    ],
    1295,
    418
  ]
]
```

## September 6 — Raya consecrate and manual shatter (ART48)

Eight V1 drawings replace Charge's held kneel and Detonate's held crystal point. Charge now descends through a supported half kneel, holds two restrained knee/palm drawings, then rises through foot-under support, prayer and empty ready. Detonate pinches, opens the empty hand, reuses the accepted Rite withdrawal, and restores ready. All cuts use matching drawing history, removing the previous body at entry and return. The palms, feet and cloth stay complete at a consistent anatomical scale. The prayer drawing is more upright than requested; its short place in the existing rise reads coherently in motion.

The comparison reuses this session's original Guilty Gear XX Accent Core observation at [82.228–87.957s](https://www.youtube.com/watch?v=vSESVFomvWA&t=85s), 1×, clearly visible from 83.496s after the seek overlay, plus previously paused A1/A2/S2 phases. The useful principle is a recognizable supported silhouette and a clear restoration, while the effect and opponent carry their own consequence. Aeon translates that into quiet knee/palm support and a brief remote command, preserving linen/copper identity. This is an animation comparison; no matching charge/detonation mechanics or source frame counts are inferred.

The new `--kit-preview --kit-ritual --kit-raya` fixture has 32 cases, each 120 ticks: tap, early release, maximum channel and legal Kogan StS interruption; then manual planted-crystal hit, standing guard, crouching guard and miss. Each outcome runs in both facings at center/corners. `--kit-move=Charge` or `Detonate` filters the 16-case clips. Detonate prepares the trap through 80 real input ticks before the viewed 120 ticks, asserting that it is armed and has not contacted the defender. It does not inject a projectile. `--kit-legacy-ritual` supports the recorded before comparison.

Original mechanics remain unchanged: Charge 10/1/8, at most 60 held channel frames, two gauge per tick capped at 100; the first held gain occurs after reaching attack frame 10. The release fixture gains 38 across 19 channel frames; tap gains zero, max reaches 100, and interruption stops at 30. Detonate remains 6/1/12, commands its one manual blast on attack frame 6, and yields 90 damage or 12 chip in these cases. The earlier touch-triggered two-contact behavior is distinct. No timing, hitboxes, gauge rules or simulation code changed.

All 32 before and 32 candidate1 cases were played completely at 1× in a 1280×720 viewport, 64 seconds per batch. Selected full-resolution PNGs were inspected: Charge 14/18/22/24/26/28/30, held/max 982/996/1042, rise 1044/1048, and interruption 1476/1478; Detonate 14/18/22/28/34/36/40/46, left-corner standing guard 858, and miss return 1470/1472. Before Charge 24 and Detonate 40 were compared directly. The new withdrawal and ready clear the old duplicate body; independent blast, guard, knockdown and recovery remain legible. The close idle overlap and brief pre-contact attack occlusion remain shared presentation findings for the final pass. Charge feint exits will be covered with Raya's remaining feint family.

Citadel passes **178 tests (89 sim + 89 client)**, clippy with warnings denied, and the locked/offline release build. The new fixture regression verifies legal activation, held gain, interruption, armed trap, manual damage/chip/miss and complete return; drawn-phase and freeze checks include the ritual cells. All 3,840 focused trace ticks and case records equal the before batch. The retained eight-scene preview's 2,100 ticks are unchanged; its new 35s video and all 71 diagnostics are byte-identical to ART47/fully reviewed ART45, so that full playback is explicitly reused. All eight fresh smoke PNGs are byte-identical to the inspected ART47 set.

Complete videos, traces, cases and 1,440 candidate diagnostics are checksum archived under `notes/media/2026-09-05-full-kit/raya-ritual-candidate1/`; the corresponding before, polish and smoke directories, check logs and `raya-ritual-review-progress.json` retain precise observations. `raya-ritual-verification.json` records acceptance. The exact prompt, source original, asset hash and runtime regions are retained in `RAYA-RITUAL-2026-09-06.md`.

This accepts Charge and Detonate's ordinary paths. Continue Raya Super/Feint/Victory, remaining Kogan lights and shared presentation cleanup. The full animation milestone and physical stick acceptance remain incomplete.
