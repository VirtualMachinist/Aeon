# Kogan crouching kick — September 7, 2026

Status: V5 accepted for ART66 publication after complete family review and regression checks.

Selected source: `kogan-crouching-kick-v5-green.png`, 1254×1254 PNG, SHA-256 `31d30d51d300cf1f6eeaaedc796ced9b85168f89f2e6d078118826b91917e46c`. Built-in image generation/editing. Column gutter x600, row gutter y627; source roots300/890/330/900, common anatomical height510. Supporting-hand anchors preserve body scale and prevent V2's large return shift.

## Source history

- V1 `exec-1780b758-5409-4a06-abcb-0f05f72889ba.png`: approved `kogan-crouch-punch-v1-green.png` edit target; `raya-crouch-lights-v2-green.png` for limb geometry only. Extra raised third arm in contact; rejected before integration.
- V2 `exec-17c193b9-0b43-4d6e-a4ed-b87c03892fac.png`: edit V1 to remove the extra arm. Preserved as `kogan-crouching-kick-v2-green.png`, SHA-256 `c2ea473239214d39708c7c582387b65dd7b0bb04a52d6e25c03ba1466292496d`. Focused case13 reviewed at1× and phases13/19/26/30/34. Low contact works, but long extension required roots that caused a large return shift. Rejected. Initial region test detected cape at625,414 crossing x627; moving the cut to the actual x600 gutter resolved the test. Redundant runtime V2 copy removed only after preserved source hash confirmation.
- V3 `exec-4b6078ba-54f6-49f0-a0d2-e20366b61790.png`: edit V2; requested shortening collapsed the active kick into gathered crouch. Rejected before integration.
- V4 `exec-8f516543-4cbe-4806-90e6-bd3d61920811.png`: edit V2; modest knee bend scarcely shortened extension. Not integrated.
- V5 `exec-a828c21b-ba2a-4f40-a582-c12d5dda0260.png`: edit V3; rotating the shin forward retains the raised knee and gives deliberate compact extension. Source inspected, then focused and full runtime review passed.

## ART66 — Kogan crouching kick phases

Kogan's crouching kick now draws a compact low boot extension from a planted supporting hand, then folds the leg and returns to the existing crouch. Four V5 poses replace the legacy punch and clipped rear blade. The full passive saber stays above the boot line. The original 4/2/8 Low move, x8/y0/42×16 local contact box, input rules and simulation remain unchanged. Standing guard is correctly hit; crouching guard still blocks. Existing Kogan crouching punch and Raya light selection remain intact, with legacy fallback if the new atlas is unavailable.

The comparison reuses inspected KOF XIII K2 (KalabelaiGaming, https://www.youtube.com/watch?v=siJoDHxttqQ&t=82s): supported low leg extension, folded withdrawal and grounded return, including Kyo's hand-supported low sweep in the longer context. Garou G2 supplies connected recovery. Aeon deliberately uses a short bent-knee poke, quieter armored torso and its own short clock. No exact source move values or copied mechanics are inferred.

The complete 20-case baseline and final matrices were played at 1× in 1280×800 footage with temporal samples, plus focused case13 playback. Paused contacts: frame19 + case×60 for cases0–15, frame16 + case×60 for cases16–19. These cover hit, standing guard, crouching guard, crouched hit and whiff in both facings at center/corner. Additional final phases13/26/30/34 and1153/1158/1162/1166, plus focused13/19/26/30/34, verify support, withdrawal and direct low return. This is selected phase inspection, not continuous frame-by-frame review. Boot, full saber, floor/cell/corner margins, opponent face and HUD readability pass. Fine curl coloration and brighter older crouch-cape highlights remain minor polish.

185 tests (96 client +89 sim), clippy with warnings denied and locked/offline release pass. Existing legal-input tests now exercise Kogan CrK outcomes, four phases and hitstop; actual-PNG region and single-body/low-return tests include the new atlas. All1,200 family and60 focused trace rows match the baseline, and all2,100 integration ticks remain unchanged. The 35-second integration video,71 diagnostic PNGs and eight smoke PNGs are byte-identical to inspected ART65/ART64, so that visual review is reused.

Retained evidence: kogan-ground-kicks-2026-09-07/before/CrK, crk-focused-v2/CrK (rejected alignment), crk-focused-v5/CrK, crk-final-v5/CrK, crk-checks-v5, crk-v5-polish, crk-v5-smoke and crk-v5-verification.json. Captures stream raw PNGs into the encoder and remove the spool. The final matrix retains39 diagnostics; focused clips retain10 each; all completed captures and current source originals are preserved. Relevant remote/vault archives pass checksum comparison.

All69 implemented move slots now have reviewed phases. Legacy jab contact finish and the final milestone audit remain open. Physical stick play and competitive balance remain subsequent acceptance.

## Exact V1 prompt

```text
Use case: precise-object-edit.
Edit target: image 1, the approved four-cell Kogan crouching-punch atlas. Convert its action into a short, supported CROUCHING LOW KICK with four coherent full-body phases. Image 2 is supporting limb-geometry reference ONLY: its THIRD ROW contains Raya's low kick preparation and compact extension. Keep Kogan as the same fully armored adult male; never transfer Raya's identity, costume, colors or equipment.

Keep image 1's square 2-by-2 layout, copper nemes helmet with thin cyan visor, etched black armor, cyan chest eye, large copper cape/aura, ornate holstered revolver and single full-length cyan saber. Match its dark painted material finish and head/limb scale. Every drawing faces right with a complete silhouette inside its own cell.

Read left to right, top row then bottom:
1. PREPARATION: a deep compact crouch, torso upright enough to read the visor. The rear leg folds beneath the hips. The free hand plants beside and slightly behind the hip for stable support; the front knee gathers with the boot just above the floor.
2. EXTENSION: remain equally low over the folded supporting leg and grounded free hand. Extend the front boot a SHORT distance forward near ankle height, toes up, into empty space. The thigh slopes gently downward; knee remains partly bent. The boot is only slightly above the supporting floor line. This is a compact low push, not a broad split, high kick, punch or airborne gesture. Keep the head/torso close to the preparation position instead of lunging the shoulders forward.
3. WITHDRAWAL: fold that kicking knee clearly back toward the hip, boot drawing inward while the same grounded hand and rear leg support the body. Show a visibly different folded leg from the extended contact.
4. RETURN: settle the front boot and return to the familiar deep crouched ready shape, releasing the supporting free hand toward a low guard. Do not stand up.

The saber is passive in the other hand through every phase, held across the lap pointing FORWARD to the right on a nearly horizontal shallow-down diagonal. Its single long straight blade remains about the length shown in image 1's TOP LEFT pose, consistent across all four cells. Keep the entire tip distinctly ABOVE the boot soles and above the extended kicking foot; never plant it in the ground. Leave generous green margins around every blade, cape curl, supporting hand and boot. Keep the saber hand/hilt stable as the leg moves. The cape stays large behind Kogan, with restrained changing folds, and must not hide the kicking leg or supporting hand.

Flat uniform chroma green #00e600 background; equal cells and generous gutters, consistent floor line within each row, no drawn floor or shadows. Keep crisp high-resolution painting, no pixel art. No second person, target, impact, injury, effects, text, labels, ghost limbs or motion trails. Preserve one head, two arms, two legs and one saber per figure.
```

## Exact V2 prompt

```text
Edit only the TOP-RIGHT cell of this four-pose Kogan sheet. Correct an anatomical error: that figure currently has an extra raised fist and forearm to the right of the chest, in addition to the grounded supporting hand and saber hand. REMOVE the extra raised fist and its entire forearm. Reconstruct that small region as the appropriate green background and unchanged torso armor. The finished figure must have exactly TWO arms: one supporting arm from shoulder to the planted open hand at lower left; one other arm bent naturally across the waist, with its hand gripping the existing cyan saber hilt. No additional fist or arm beside the chest.

Preserve the grounded support hand, saber hand and exact full-length blade, complete low extended boot, folded rear leg, head, torso, copper cape, cell margins, background and body scale. Do not move or shorten the saber. Leave the other THREE cells entirely unchanged. Keep the 2-by-2 atlas, flat green field, complete silhouettes, original painted materials and equipment. No added limbs, detached hands, labels, effects or shadows.
```

## Exact V3 prompt

```text
Edit only the TOP-RIGHT kicking leg in this four-cell Kogan crouching-kick sheet. The current extension is too long and straight for a compact low poke. Bend its knee more deeply and bring the entire extended boot about 100 pixels LEFT, closer to the torso, while preserving the boot's size, ankle-height sole, toes-up angle and plausible full limb lengths. On this 1254-square canvas the boot currently reaches x≈1190; its new outermost toe should reach only x≈1090. Keep the support floor at y≈480 and the extended boot sole near y≈465. Fold the thigh/shin at a visibly bent knee so the leg occupies less forward space; do not shrink the boot, shorten a bone unnaturally or turn it into a standing kick.

Preserve the TOP-RIGHT head, torso, grounded supporting hand, folded support leg, cape, full saber and hilt exactly. The straight cyan saber remains above the boot and may project slightly farther forward than the newly compact toe. Keep exactly two arms, with one grounded open hand and one hand gripping the saber; no extra fist. Leave all other THREE cells unchanged. Preserve all materials, identity, scale, full margins and flat chroma-green background. No new effects, shadows, labels, detached limbs or ghost drawings.
```

## Exact V4 prompt

```text
Edit only the TOP-RIGHT cell of this Kogan crouching-kick atlas. Keep it unmistakably an EXTENDED LOW KICK, visibly different from the folded preparation and recovery in the other cells. Make a small adjustment: partially bend the currently straight kicking knee, moving its entire toes-up boot about 70 pixels LEFT while keeping its size and height. The toe currently reaches x≈1190 on this 1254-wide image; the new toe should reach x≈1120, well to the RIGHT of the torso and supporting knee. The heel must remain OFF the floor with the boot sole near y≈465, close above the supporting floor y≈485. Preserve the long diagonal thigh and forward shin; just add a modest knee bend. Do NOT replace it with the planted boot or gathered crouch shown in the other cells.

Keep the top-right torso, head, grounded supporting hand, folded rear leg, cape, saber and hilt in their exact existing positions. The saber remains full-length and almost horizontal above the leg. Exactly two arms: grounded hand and saber hand. Leave ALL THREE other cells unchanged. Preserve all green gutters, silhouette edges, armor details and character scale. No new effects, text, extra limbs or shadows.
```

## Exact V5 prompt

```text
Edit only the TOP-RIGHT front lower leg and boot in this four-cell Kogan crouching pose atlas. Turn that cell's planted front boot into a compact LOW KICK.

Keep the existing raised front knee in place, near (995,365) on this 1254-square image. Swing its SHIN forward from that knee, lifting the whole boot off the floor and moving the boot about 65 pixels RIGHT. The shin must visibly slope DOWN AND FORWARD from knee to ankle. The ankle should be near (1060,435), the heel near (1060,470), and the raised toe near (1110,445). Thus the boot sole hovers just above the support floor around y=485; its toes tilt upward. The front boot must be visibly farther forward than the three folded/ready poses and must NOT be planted beneath the knee. Retain plausible leg lengths by rotating the shin around the existing knee, with only a small necessary adjustment to that knee.

Keep all other top-right pixels and anatomy: the folded rear leg, planted supporting hand, visor/head, armor/torso, copper cape, saber hand and full long saber are unchanged. Exactly two arms and two legs. The saber remains a quiet complete line above the kicked boot. Leave the other THREE cells unchanged. Keep flat green gutters and complete margins. No effects, extra limbs, text, shadows or additional figures.
```
