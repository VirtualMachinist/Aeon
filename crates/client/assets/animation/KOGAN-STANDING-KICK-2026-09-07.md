# Kogan standing kick — September 7, 2026

Selected source: `kogan-standing-kick-v5-green.png`, 1254×1254, SHA-256 `0f061640a4006b905292ff23da62a126687ff4d279eae8865be33da4d6c4c5fb`. Four 627-square cells, anatomical height 480, roots 300/885/300/895. The whole original source is keyed at runtime.

Built-in image generation supplied the sheet. Identity references were the approved Kogan hero plate, idle pose and reaction sheet. Initial V1/V2 solo studies established generation availability but had excessive kick height. V3 edited V2 with the approved Raya standing-lights third-row-right pose only as supporting low-leg geometry. V3's short saber was corrected in V4, whose below-floor tips required the final V5 angle correction. All generated originals and exact prompts are retained in the project art archive. Rejected variants are not runtime dependencies.

## V3 leg geometry

```text
Edit target: reference image 1, the square 2-by-2 Kogan solo-kick sheet. Reference image 2 is supporting leg-geometry guidance only: look at its THIRD ROW RIGHT low forward extension, not the other poses. Keep Kogan male, fully armored, with his own copper nemes/cape, cyan visor, dark armor, saber and holstered revolver. Do not transfer Raya's face, clothes, colors or equipment.

Make one essential correction to the TOP RIGHT Kogan drawing: lower the extended leg and boot into a very low, short forward push. The boot should hover just above the supporting ankle, with the sole about 55 pixels above the planted sole on the 1254-square edit target. The planted sole is near y=578, so the extended sole should be near y=523. Its toe should be near x=1060. Bend the supporting knee slightly. The kicking thigh must slope steeply downward from hip toward a low knee, and its shin continues down-forward to the boot. The raised knee stays clearly BELOW the hip. This is a relaxed ankle-height extension; there is no high kick or horizontal thigh. Use the low foot placement and downward leg line of the third-row-right pose in reference 2, adjusted even lower.

In TOP LEFT and BOTTOM LEFT Kogan drawings, lower the chambered knee below the hip, with the gathered boot hovering beside the supporting ankle. Keep distinct preparation and folded withdrawal. Preserve all torso/head/cape details, the inactive full saber clear of the boots, and the bottom-right ready pose. Keep identical body scale and flat green background, full silhouettes, two equal rows and two equal columns. No text, floor, shadows, additional figures, effects or ghost limbs.
```

## V4 blade length

```text
Edit the supplied 2-by-2 Kogan standing-kick atlas. Preserve all four adult armored bodies, head sizes, poses, low kicking boot positions, cape shapes, costume, green background and cell boundaries. Correct ONLY the four cyan saber blades: they are too short for his established equipment. Keep each right hand and hilt where they already are, but extend each single straight blade along a shallower DOWN-FORWARD diagonal so its entire tip stays ABOVE the boot soles.

Use this 1254-square canvas as an approximate guide:
TOP LEFT: from existing hilt near (211,345) to a full tip near (495,530).
TOP RIGHT: from existing hilt near (783,346) to a full tip near (1065,530).
BOTTOM LEFT: from existing hilt near (211,939) to a full tip near (495,1125).
BOTTOM RIGHT: from existing hilt near (796,929) to a full tip near (1080,1115).
The four blades should each be roughly 335 pixels long, consistent from hilt to tip, around seventy percent of the character's standing height. Their full pale cyan core and narrow blue-cyan edge remain one continuous rigid straight line. Preserve plausible hand grips by rotating the wrists minimally. No extra handle, second blade, afterimage, blur, slash effect or glow cloud. Do not move the boots or change the kick; do not shrink or resize any body to fit. Keep every blade in its own cell and clearly off the floor.
```

## V5 blade angle

```text
Edit ONLY the orientations of the four cyan saber blades in this Kogan kick sheet. They now have the right length, but point too steeply downward and their tips sit below the planted boots. This must be corrected before gameplay use.

In every cell, keep the existing hand/hilt position and the current full blade length. Rotate the one rigid straight blade forward to point almost horizontally right, with a shallow downward angle of about 20 degrees from horizontal. The blade tip should be near the character's KNEE HEIGHT, visibly far ABOVE BOTH ANKLES. It must never be the lowest part of the silhouette. It should travel much farther RIGHT than DOWN from the hand.

Approximate new tips on this 1254-square canvas: top-left (550,440), top-right (1125,450), bottom-left (550,1040), bottom-right (1140,1025). These are about 90 to 110 pixels ABOVE each planted boot sole. Green should remain clearly visible below every saber tip. Preserve full bright narrow cyan blade length, one hand gripping one saber, and no residual old blade.

Keep all four bodies, heads, armor, cape, boots, low kicking leg placement, green background, 2-by-2 layout and full margins unchanged. Only rotate the blade and minimally adjust its wrist. No new effects, secondary blade, motion trail, labels or other people.
```

## ART65 — Kogan standing kick phases

Replace Kogan's torso-height standing-kick drawing with four deliberate low-boot phases: chamber, extension, folded withdrawal and foot replacement. The complete V5 saber remains consistent in length and clear of the floor. Selection follows the existing 5/3/8 move clock, holds during hitstop and returns without extra recovery. The move remains Mid with its original geometry; this is presentation only. Missing kick art retains the legacy fallback without disabling the existing jab.

KOF XIII reference K2 (Kim, KalabelaiGaming: https://www.youtube.com/watch?v=siJoDHxttqQ&t=82s, inspected around 82.16–85.10 seconds with phase samples at 82.43–83.13) supplies chamber, extension, support balance, withdrawal and foot replacement. Aeon intentionally lowers the contact from Kim's torso-height example and keeps Kogan's armored upper body quieter. No reference timings or mechanics are imported.

The complete 20-case baseline and final matrices were played at 1× in 1280×800 footage, plus focused final playback. Paused final contact inspection covers hit, standing guard, crouching guard, crouched hit and whiff in both facings at center and corner: video frame 19 + case×60 for cases 0–15, frame 18 + case×60 for cases 16–19. Focused case 0 preparation/contact/withdrawal/replacement/idle frames: 13/19/30/34/40. Final left-corner whiff: 1153/1158/1162/1166/1170. Contact, support, full equipment, opponent/HUD visibility and return pass. Earlier V3 had a short saber; V4 tips crossed the floor and was rejected before integration.

185 tests (96 client + 89 sim), clippy with warnings denied and locked/offline release pass. Tests cover legal light inputs/outcomes and four phases, hitstop holds, no extra art recovery, single-body return and actual PNG region boundaries. All 1,200 family ticks and 2,100 integration ticks match ART64; focused 60 ticks also match the prior focused fixture. The new 35-second integration video, all 71 diagnostics and eight smoke images equal inspected ART64 byte for byte, so its completed visual review applies. Source SHA-256: `0f061640a4006b905292ff23da62a126687ff4d279eae8865be33da4d6c4c5fb`.

Evidence archive: `kogan-ground-kicks-2026-09-07` (before/StK, final-v5/StK, focused-v5/StK, checks-v5, stk-v5-polish, stk-v5-smoke and verification/retention manifests). The complete remote archive checksum matched the vault. Raw frames stream into the encoder and are pruned; final matrix retains 39 selected diagnostics. Baseline and rejected V3 retain seven and eleven diagnostics after verified pruning, saving 465,580,309 bytes per host. Source originals and prompts remain preserved.

68 of 69 implemented move slots now have reviewed phases. Kogan crouching kick and legacy jab contact finish remain open; brighter flowing cape highlights and fine curl coloration remain minor polish. Physical stick play and competitive balance remain subsequent acceptance.
