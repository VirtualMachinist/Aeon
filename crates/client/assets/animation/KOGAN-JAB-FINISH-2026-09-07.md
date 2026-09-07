# Kogan jab contact finish — September 7, 2026

Accepted V2; publication is recorded in the implementation ledger. Built-in imagegen edits only the existing contact material. No new gesture, timing, simulation change or source slicing from identity plates.

## Sources and selection

| Candidate | Generated original | Project source | SHA-256 | Result |
|---|---|---|---|---|
| V1 | `exec-5dfc37f2-6dc0-4c0a-888c-ca1bfea80efc.png` | `kogan-jab-contact-v1-green.png` | `10df5f6b9d75f1c8b620b373bce2d9740dd26054036def3a97e6b5c011c81a35` | Geometry passes; pale microtexture remains too noisy. Rejected material. |
| V2 | `exec-cecbf58b-cde2-4bd9-9b02-e92edb65b0f3.png` | `kogan-jab-contact-v2-green.png` | `cb1f7e3665dd89488ed74654a8cbf260312f20f86a1d9445dcda2d6864dd7fc4` | Accepted broad dark armor and warm edges; supported fist/full saber retained. |

Originals remain in this task's generated-image archive. Both project sources remain in the vault; only selected V2 is shipped. The redundant untracked runtime V1 copy was removed after confirming its vault hash. V1 edited the inspected approved `art/fight-ready/kogan/p.png` (800×800) with inspected `kogan-flash-v2-green.png` as material reference; V2 edited the inspected V1. Both generations completed successfully.

The selected1254×1254 source uses full bounds `[0,0,1254,1254]`, root630 and anatomical height976, preserving the old620px anatomy at1.55/800 projection. Runtime floor/corner/support review verifies the mapping. Existing Flash2 gather/withdrawal and Flash3 ready remain intact; ART52 phase acceptance remains relevant.

Retained ART60 baseline probe played at1× and selected13/16/26/32 phases were inspected. The fresh ART66 baseline played through all20 seconds at1×; a repeated1280×860 viewport review exposed all lower diagnostics after the first1280×720 viewport clipped only that text. V1 and V2 focused reviews compare identical legal input traces. Exact final evidence follows.

## ART67 — Kogan jab contact finish

Kogan's standing jab now uses a darker contact drawing with broad black armor planes and restrained warm copper trim. It preserves the original closed fist, body proportions, planted feet and complete passive saber. Existing Flash gather, withdrawal and ready drawings remain deliberate reuse; the original 4/2/6 Mid clock,40 damage, x8/y70/36×14 local box and all simulation values are unchanged. The high-positioned Mid box geometrically misses crouched bodies. The legacy contact remains a missing-asset fallback.

This refines the already reviewed G2/K2 jab phases: Garou G2's compact outbound strike and recognizable return, with S2's legible held commitment and distinct withdrawal. The older contact's dense pale texture interrupted the surrounding dark armor. V1 preserved support but still sparkled at gameplay size; V2 simplified the surface painting. Aeon retains its own short timing, copper/cyan identity and quiet armored stance rather than importing source mechanics or pose designs.

Before/final20-case matrices played at1× in1280×800 footage with sparse temporal samples. Every selected contact was inspected: frame19+case×60 for cases0–7 (hit/standing guard), frame16+case×60 for cases8–19 (crouched geometric misses and distant whiffs), across both facings and center/corners. Additional final phases13/1153/1158/1162/1166 and focused V2 phases16/26/32 verify gather, withdrawal, ready and idle. Full body, saber, support, opponent and HUD readability pass. This is selected phase inspection, not a claim of continuous frame-by-frame review. Brighter older flowing cape highlights and fine curl coloration remain minor polish.

185 tests (96 client+89 sim), clippy with warnings denied and locked/offline release pass. Actual-PNG region validation includes the new atlas; existing legal light/outcome/phase/freeze/return checks remain green. All1,200 family,60 focused and2,100 integration ticks equal baseline. The35-second integration video,71 diagnostics and8 smoke PNGs are byte-identical to inspected ART66/ART65/ART64, so that visual review applies without another capture. No sim or authored frame-data change was made.

Evidence: `kogan-jab-finish-2026-09-07/before/StP`, rejected `focused-v1/StP`, accepted `focused-v2/StP` and `final-v2/StP`, `checks-v2`, `jab-v2-polish`, `jab-v2-smoke`, `verification-v2.json`, `archive-checksums-v2.json` and `regression-comparison-v2.json`. All six relevant remote capture directories and the three check logs match their vault archives by checksum. Full matrices retain38 selected diagnostics each; the final video is5,622,210 bytes. Raw PNG spools were removed during encoding. Generated source originals, prompts, videos, traces, cases and selected diagnostics are retained.

All69 implemented move slots have reviewed phases and jab contact material is accepted. The whole-goal completion audit and final review package remain required; physical stick play and competitive balance follow this animation milestone.

## Exact V1 prompt

```text
Use case: style-transfer.
Edit target: image 1, the existing single full-body Kogan jab. Supporting STYLE reference only: image 2, the approved Kogan Flash atlas. Preserve image 1's exact pose, anatomy, silhouette proportions, facing, foot positions, extended closed fist, saber hand and complete saber geometry. Produce ONE full-body drawing, not an atlas.

Change only the material rendering and technical background so this jab belongs beside image 2's darker painted figures. Replace the bright densely glittering gold/copper microtexture on the armor with deep near-black etched armor panels, readable broader copper edge trim, restrained metal highlights and clean high-resolution painted surfaces. Match image 2's dark armor, copper nemes helmet, narrow cyan visor, chest eye, and copper cape treatment. Keep enough visible armor planes to read the arm and bent legs at game size; do not merely underexpose the entire character. The cape stays very large with the SAME contour, full curls and folds, rendered as darker copper surfaces with controlled edge highlights rather than luminous gold. Preserve the cyan chest detail and all established equipment.

Retain image 1's straight shoulder-height closed-fist jab into empty space: one extended arm with a complete gloved fist; the other arm holds the existing saber down-forward, with the same hand position, angle and full blade length. Exactly two arms, two legs, one head and one saber. No pose substitution, open palm, weapon strike, new ornament, new joint or duplicate limb. Keep the grounded support and bent knees exactly as in image 1. No cropping or below-floor blade tip.

Single centered figure on a square canvas with flat uniform technical chroma green #00e600, generous clear margins around every boot, fist, cape curl and full blade. Keep the original body aspect ratio. No second character, target, contact effect, injury, text, labels, cast shadow, floor marks, motion blur or ghost figures. Image 2 supplies material/painting treatment only; never copy its multi-cell layout or poses.
```

## Exact V2 prompt

```text
Edit this single Kogan jab drawing to improve its readability when displayed only 250 pixels tall. Change ONLY the surface painting: preserve the exact pose, silhouette, anatomy, body proportions, fist position, saber hand, full straight saber, foot support, cape shape and green background.

Simplify the high-frequency material detail substantially. Make the main armor panels broad, smooth near-black painted planes with a few clear copper structural edges. Remove tiny white/silver glints, stippling, dense miniature runes, close parallel scratch lines and bead-like highlights across helmet, arms, torso and legs. Copper rim highlights should be warm medium copper, broad and restrained, never thin pale-white pinstripes. Keep the main nemes bands, visor, chest eye, elbow/knee structure, belt and a few larger etched accents readable. This is polished high-resolution painted game art with deliberate shapes, not a blurred or low-resolution image.

Paint the large cape as smooth dark copper folds with broad warm edge light. Eliminate pale hairline outlines and glittering microtexture from each curl while retaining every complete curling contour. Preserve the existing cyan visor, chest eye and saber as the brightest landmarks. Do not darken or shorten the cyan blade. Do not move, crop, resize or redesign any body part or equipment. The goal is the same supported closed-fist jab with quiet dark materials that remain coherent beside the approved ready pose, without shimmering specks when reduced. One figure, unchanged square layout and flat uniform #00e600 background, no other objects or effects.
```
