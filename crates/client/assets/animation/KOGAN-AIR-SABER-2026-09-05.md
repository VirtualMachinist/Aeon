# Kogan airborne saber — 2026-09-05

Reviewed under the ongoing full-kit animation goal. Built-in imagegen; no simulation changes.

Reference inputs visually inspected: approved `kogan-movement-v2-green.png` for identity/anatomy and older `../kogan/air_heavy.png` for the existing downward gesture. The old broad, clipped crescent is a defect to replace. Benchmark lenses: S1 weapon/cloth continuity, S2 commitment/withdrawal, A2 effect hierarchy and G1 landing connection from the inspected reference dossier.

## V1 request

```text
Use case: stylized-concept. Asset type: production drawn animation atlas for Aeon, a high-resolution 2D fighting game.
Create SIX full-body airborne SABER drawings of the existing adult male Kogan, arranged TWO columns by THREE rows on a tall canvas. Image 1 is the approved movement atlas: identity, body proportions, dark etched armor, copper nemes/cobra, cyan visor/chest eye, large flowing copper cape, cyan saber and ornate holstered revolver. Image 2 is an older airborne saber silhouette as gesture reference only; replace its oversized solid effect crescent with an intact slender saber. Do not copy either sheet layout.
All six face RIGHT in the same side-on three-quarter camera, exactly the same anatomical scale and head size. Orthographic painted 2D, crisp controlled copper highlights, black armor and fine cyan writing. The cape is large copper flowing cloth/aura, lifted behind to the LEFT, changes folds through the gesture. Right hand holds ONE complete long cyan saber with its copper hilt visible; left arm counterbalances, revolver stays holstered. Exactly two arms and two legs each. Keep the entire body, cape and blade well within its cell, generous empty gaps on all sides; no cropping or adjacent-cell overlap. Uniform flat technical chroma green #00e600 in all background and gaps, no shadows, floor, text, borders or opponents. No pixel art, blur, afterimages, ghost limbs, hit sparks, energy circles, or broad blade trails.
Read cells left-to-right then top-to-bottom:
1. Gather: knees tucked below hips, torso upright and slightly turned, bent right elbow draws saber back beside the right shoulder, blade points diagonally up/back but stays below the top margin. Left hand near ribs. Clearly preparing to cut downward, no extension yet.
2. Compact diagonal cut: torso tips forward moderately, right elbow remains slightly bent with hand just forward of low waist. Saber points DOWN and RIGHT about 55 degrees below horizontal, tip below the tucked boots and modestly in front. Left arm bends back. This is the short-reaching airborne cut.
3. Long diagonal cut: stronger forward shoulder extension, right arm stretches ahead of waist, saber points DOWN and RIGHT about 30 degrees below horizontal, full blade reaching conspicuously farther forward than cell 2. Legs trail behind, knees bent. One clear long weapon line, no additional effect.
4. Steep close cut: torso more upright, right hand just ahead of hip, saber points DOWN and RIGHT about 70 degrees below horizontal, full tip below the boots and close in front of the body. More compact forward reach than cell 3. Left forearm across ribs; knees tucked behind. Distinct steep downward line.
5. Withdrawal: blade has risen out of the low front line; right elbow folded toward ribs, wrist near hip, complete saber now diagonally down/right about 25 degrees with tip near knee height. Chest lifts, knees begin lowering, cape still lifts behind. The cut has finished.
6. Ready descent: torso upright, knees softly bent, both boots reaching downward to prepare to land; right hand holds saber in the familiar low front ready line across the body with tip safely above boot soles. Left hand settles toward the holster. Cape lifts then starts falling behind.
These are selected key drawings within existing short game timings, not six different characters. Retain the same adult proportions across compressed and extended poses. Anatomically connected shoulders, elbows, hands, grips and full weapons are essential. Weapon paths and contact should remain readable without an effect cloud.
```

Output and runtime acceptance pending. JS/JHS/JST keep their existing 5/6/6, 6/5/8 and 6/5/8 timing and different downward box reach. All are High; no new attack mechanics.

V1 original: `exec-f67e00e5-e7df-48ec-8bd8-09232802d64b.png`, preserved as `kogan-air-saber-v1-green.png`. Six distinct paths and connected anatomy are promising; the top-left gather incorrectly shortens the blade. Not accepted yet.

## V2 targeted blade correction

```text
Use case: precise-object-edit. Edit ONLY the cyan saber blade in the TOP-LEFT cell of this six-cell Kogan animation atlas. Its current blade is much shorter than the other five. Keep the existing copper hilt, closed gripping hand, arm and entire character unchanged. From that same hilt, draw the complete long slender cyan saber extending diagonally LEFT and slightly UP, nearly horizontal (about 15–20 degrees above leftward horizontal), so its visible length matches the full blades in the other cells. Its pointed tip stays inside the top-left cell with at least 25 pixels of empty green margin; do not cross the top or left edge. Match the cyan core and narrow edge glow of the other blades. Change no other pixels or pose: preserve the six drawings, all anatomy, armor, nemes/cobra, large copper cape, green gaps, exact canvas dimensions and all five other cells. No second saber, no added limb, no motion effect.
```

V2 original: `exec-50b0b861-3c9d-45a9-b65a-b83296970bec.png`, preserved as `kogan-air-saber-v2-green.png`. Its gather keeps a full-length blade; the generated edit also shifts small details in the other poses, so the complete V2 sheet was inspected again. All six V2 cells are integrated and accepted after final gameplay review. V1 remains archived and is not loaded.

## Integration measurements and phase policy

Source reference 1024×1536; shared unfolded anatomical height 430. Runtime extracts only measured regions from the intact atlas, with the usual two-pixel technical gutter.

| Cell | Region L/T/R/B | Root x/y | Purpose |
|---|---|---|---|
| 0 | 0/0/535/500 | 365/480 | Gather |
| 1 | 535/0/1024/500 | 770/480 | JS compact downward cut |
| 2 | 0/510/565/977 | 230/930 | JHS extended diagonal cut |
| 3 | 565/510/1024/977 | 735/955 | JST steep close cut |
| 4 | 0/980/520/1536 | 270/1410 | Withdrawal |
| 5 | 520/980/1024/1536 | 740/1450 | Ready descent |

Startup uses gather; the existing active interval alone selects the move-specific contact pose. Recovery uses cell 4 until the last three existing attack ticks, then cell 5. An expired air normal retains ready descent during the spent-air-action Jump state; every new legal action takes ownership immediately. The full-jump landing reuses the approved front-saber Reaction(8) drawing for its existing two ticks when recent drawn history proves an air-saber landing. This avoids a front/back/front blade jump and does not affect subsequent standalone jumps. Hops retain immediate control with zero landing ticks. No pose is held by extending simulation recovery.

`--kit-air --kit-air-early` adds legal apex-input spaced misses for the six normals. Full jumps expose all recovery phases; the low hop still lands before recovery. Existing close-contact fixture inputs remain unchanged. Focused V2 clips were viewed completely at 1× for all three blades, both jump types and ordinary/early miss (12 × 2.5s); JST full early ticks 39/45/50/55/57 were stepped. Clean complete blades, distinct reach and coherent anatomy are visible. The focused candidate predates the final descent/landing handoff refinement; final 120-case review remains pending.


## Final acceptance

All 120 final cases / 300s played completely at 1×, including hop/full, hit/standing guard/defeated low guard/miss and early miss, both facings at center/corners. Final JS early full 55/56/57/58/59, JHS 44/49/54/57 and JST 45/50/55/56/57 stepped. Complete blades, distinct reaches, coherent anatomy and front-saber landing remain visible. The final recent-history landing refinement corrects the focused candidate. 140 tests, clippy and locked/offline release pass; 18,000 final and 2,100 integration ticks equal baseline. Complete 35s integration viewed and eight smoke hashes match preceding inspected evidence. Exact before/candidate/final review limits and S1/S2/A2/G1 refraction are in the full-kit report. Remaining kit coverage stays open.
