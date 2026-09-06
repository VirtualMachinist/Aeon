# Raya Convergence — 2026-09-06

ART49 reviewed September 6. Built-in imagegen V1 supplies four complete gather, expanded crystal orbit, dismissal and familiar ready drawings. Full source inspected: coherent anatomy/materials, clear face and hands, complete feet and cloth, orbit below shoulders. Candidate2 accepted after full before/after motion and exact-frame review.

Original: `/Users/evanpincham/.codex/generated_images/01a07081-3028-76e3-8486-af4a0cdf6e0f/exec-6dcf2eb6-b43b-4058-8fcf-54a34fdd8944.png`.
Asset: `raya-convergence-v1-green.png`, 1536×1024. References: inspected approved ritual V1 identity/rendering and older `raya/super.png` motif.

Exact prompt:

Use case: stylized-concept. Asset type: high-resolution painted 2D animation sheet for Aeon. Reference image1 is Raya's approved face, anatomy, linen/copper materials and current rendering; reference image2 supplies the familiar expanded-arms crystal-orbit motif of Convergence, not its old perpetual held pose. Create FOUR complete RIGHT-FACING poses, two rows of two with broad flat green gutters. Keep identical anatomical scale and aligned floor lines within each row. Preserve her calm female face, white linen hood and layered robes, copper cloak and shoulder armor, copper sandals, cyan jeweled brow and bracers. This is a grounded composed glide, not running or jumping. Pose1: compact preparation, soft knees, torso upright, elbows drawn in with two palms near the sternum, three small cyan crystal shards gathering close below shoulder height, cloak slightly gathered behind. Pose2: decisive expanded Convergence silhouette, both arms opening diagonally outward just below shoulder height, face calmly focused right, front foot supporting and rear leg long behind in a poised glide; a sparse cyan crystal orbit around the waist and upper legs with a few separated shards, leaving face, hands and torso plainly visible. No crystal above the head. Copper cloak and linen stream backward to the left without hiding either foot. Pose3: dismissal and withdrawal, elbows folding down inward, palms turning down beside waist, same grounded support, three small separated cyan shards fading away behind the waist, cloak settling nearer the body. Pose4: upright familiar ready, one palm open at waist with one small upright cyan crystal above it, other hand relaxed near hip, cloak resting naturally behind, both feet supported. All four complete bodies, consistent head and limb sizes, complete separate hands/fingers, sandals and uncut cloak hems. High-resolution painted shading and material detail, not pixel art. Uniform flat chroma green RGB(0,210,0) background. No stage, floor, shadow, text, labels, borders, extra people or duplicate bodies. Give generous empty green margins around all arms, crystals and cloth.

Accepted source SHA256:`c43d0eba620476d3598e731567d53a459030c46f873090b5c7db390fcf398d4b`. Regions, horizontal roots and anatomical heights:

```json
[
  [
    [
      0,
      0,
      720,
      510
    ],
    460,
    420
  ],
  [
    [
      720,
      0,
      1536,
      510
    ],
    1140,
    420
  ],
  [
    [
      0,
      510,
      720,
      1024
    ],
    460,
    420
  ],
  [
    [
      720,
      510,
      1536,
      1024
    ],
    1060,
    420
  ]
]
```

## ART49 — Raya Convergence

Four V1 drawings replace the single held Super pose: close gathering, expanded arms with a low crystal orbit, dismissal and familiar ready. Existing 6f startup / 6f active / 30f recovery remains exact. Drawings select attack frames 0–5, 6–11, 12–31 and 32–41. Full anatomical scale, floor support, linen hood, copper cloak, hands and feet survive both facings. The orbit stays below the shoulders and leaves the face and HUD clear.

Convergence retains legal 236236+S, 1,000 meter cost, 340 damage / 28 chip and its original 14px/tick grounded travel. It continues moving during recovery; a spaced active miss may approach the opponent later without hitting. No pass-through mechanic is inferred from the move's descriptive comment. Sim, geometry, timing and combat rules are unchanged.

The fixture now supports Raya with twenty legal cases: hit, standing guard, crouching guard, crouched hit and whiff, each at center/corner in both facings, 150 ticks each. Kogan's sixteen existing Super cases remain intact. A regression checks legal activation, meter, damage/chip/miss, knockdown, travel, all four selected cells, frozen presentation and complete return. Source-boundary and clean-return checks include the new asset.

### Reference refraction and review

Original Guilty Gear XX Accent Core, A.B.A/Testament, [83–94s excerpt](https://www.youtube.com/watch?v=vSESVFomvWA&t=83s), reopened and visually observed at 1× in a 1280×720 viewport. Samples 84.229, 85.530, 86.861, 88.175, 89.493, 90.809, 92.136 and 93.469 seconds show broad orange/red and purple weapon silhouettes dominating a contact, then a recognizable green scythe diagonal and supported body returning as the opponent falls. The old tab's failed seek was not counted; a fresh visible tab supplied these observations. Reuse the dossier's paused A1/A2/S2 phases. No source Super name, precise frame count or edition-equivalent mechanics inferred.

Aeon translates recognizable expansion and restoration through Raya's restrained crystal orbit and composed dismissal. Her sparse effects deliberately preserve more of the body and opponent than the source's brief dominant effect. Before, the expanded orbit persisted through recovery with four old-body shadows. Candidate1 introduced proper drawings but a paused dismissal at frame 42 still carried the prior expanded silhouette. It was rejected before full motion acceptance. Candidate2 limits Super to two matching-pose trails and discards the old drawing at the cut.

All twenty before and accepted candidate2 cases, 50 seconds each at 1×, reviewed at 1280×720. Candidate2 playback was repeated in four uninterrupted-speed segments after a tool output truncation; the truncated output alone was not credited as visual review. Exact accepted PNGs 0014, 0018, 0042, 0052, 0062, 0064, 0618, 0642, 1230, 2430, 2442 and 2452 were inspected. Before/candidate1 comparison includes 0014, 0018, 0042, 0052, 0062 and 1230. Gather, active orbit, supported dismissal and restored ready are distinct; the expanded pose no longer ghosts the dismissal. Both standing and low guard reactions remain readable during contact. Close neutral body crowding remains a shared presentation finding for the final pass.

### Verification and evidence

179 tests pass (89 sim + 90 client), clippy with warnings denied and locked/offline release pass on Citadel. All 3,000 focused trace ticks and case metadata equal the before capture. The retained integration's 2,100 ticks also equal ART48. Its new 35-second video was fully reviewed at 1×; 69 of 71 diagnostic PNGs are byte-identical to ART48, while changed 1020/1035 before/after pairs were inspected and show the new Super contact and return. All eight fresh smoke PNGs are byte-identical to inspected ART48/ART47 evidence, including training and versus.

Vault evidence under `notes/media/2026-09-05-full-kit/`: `raya-super-before/`, rejected `raya-super-candidate1/`, accepted `raya-super-candidate2/` (50s video, complete trace/cases, 960 diagnostics), `raya-super-polish/`, `raya-super-smoke/`, checks2 log, `raya-super-review-progress.json` and `raya-super-verification.json`. Captures were checksum archived before thinning owned remote diagnostics. Prompt, original location, asset hash and calibration are retained in `RAYA-CONVERGENCE-2026-09-06.md`.

Run `--kit-preview --kit-super --kit-raya` for the twenty-case fixture. Raya Feint/Victory, Kogan remaining lights and shared presentation polish remain open. The full milestone and physical stick acceptance remain incomplete.
