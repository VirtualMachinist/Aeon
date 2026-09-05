# Kogan ground movement — V4 reviewed

Built-in imagegen. References: reviewed `kogan-cape-step-v3-green.png` for current anatomy/materials/full saber/cape and original `kogan/crouch.png` for the low grounded concept. Both inspected before use. No blocked lights request was retried.

Baseline full RunStop, BackDash, WalkForward, WalkBack and Crouch clips (6s each, four placements) viewed at 1×. The old run leans excessively and hides the head at close range, with source-clipped equipment. Backdash uses a forward lunge and a second body remains at idle (paused ticks 20/29). Walk imports a neighboring cape fragment. Crouch uses an abrupt held drop/rise and entry/exit crossfades. New restrained glide, crouch transition/hold and backward gather/travel/brake/settle are requested. Existing walk drawings will be remeasured. G1/K1/S1 inform gathering and equipment continuity; Aeon keeps its grounded glide and immediate exits.

Exact prompt:

```text
Use case: stylized-concept.
Asset: eight new grounded movement drawings for Aeon's original adult male armored duelist Kogan. Reference 1 supplies the approved current armor, full cyan saber and large copper cape; reference 2 supplies the low crouch concept. Create one coherent FOUR-COLUMN, TWO-ROW animation sheet on a wide canvas. Eight complete right-facing full-body side/three-quarter gameplay figures, all at identical anatomical scale. Every empty pixel flat chroma green #00e600. Generous green gutters and outside margins around every FULL cape, foot and saber tip. No labels, grid, floor, shadows, opponent, detached sparks or painted motion trails.
Identity: copper Egyptian nemes and cobra, narrow cyan visor, dark ornate etched armor, cyan chest eye, holstered revolver, LARGE copper cape/aura. One full long cyan saber in the right hand in every cell; consistent blade length about two-thirds of standing height, never clipped or shortened. Preserve current painted copper/cyan detail.

Top row:
1. Forward glide A: body leans only 12 degrees toward RIGHT, head stays close above the front hip rather than far beyond it. Both feet low at the same floor line in a compact planted stagger, front knee softly bent, rear leg extended a little. No sprint stride and no airborne flying body. Right hand holds the saber beside the hip with blade diagonally down-BACK toward LEFT, full tip ABOVE both soles. Left fist near ribs. Broad cape streams behind to LEFT, with lifted upper folds.
2. Forward glide B: exact same head, torso, hips, legs, feet and weapon placement as cell 1. Only the flowing cape folds change: the upper fold settles while lower curls lift. This is a cloth cycle for a glide, NOT a running leg cycle.
3. Half crouch / rising transition: hips lower, both knees bent, torso upright enough to keep the face toward RIGHT; left hand approaches the front knee. Right hand shifts the full saber to a shallow near-horizontal forward line toward RIGHT, tip clearly above soles. Cape gathers closer behind. Anatomy is not shrunk.
4. Low crouch hold: hips near the rear heel, front knee folded, one left hand lightly braced near the floor beside the front boot. Full right-hand saber lies horizontally forward across the knees, with complete tip above the floor. Low head, about two-thirds of standing height, same head size. Large cape pools behind without becoming detached.

Bottom row: one backward grounded retreat toward LEFT while still facing RIGHT.
5. Gather for retreat: knees soften, hips move slightly LEFT, torso leans just 5 degrees back, head above hips, left palm gathered near chest. Full saber held diagonally down-forward toward RIGHT, above soles. Cape gathers behind.
6. Backward glide: body remains facing RIGHT but center of weight moves LEFT over the rear boot; chest leans 10 degrees away from the opponent, front leg extends slightly to RIGHT, both feet stay low. Saber remains a complete low forward ready line toward RIGHT. Cape curls lag toward RIGHT around the flanks but stay below shoulder height so face/chest are readable. Not a forward lunge, jump or kick.
7. Retreat brake: rear knee bends to absorb backward travel, torso becomes upright, front foot returns toward its ready spacing. Left palm folds beside ribs, full saber keeps the low forward line. Cape folds catch up toward the rear.
8. Settle: nearly upright familiar ready stance matching reference 1's final figure, both boots planted, full saber diagonally down-forward toward RIGHT with tip above soles, large cape settling behind.

All 8 cells must show a single complete body and complete equipment. Keep the forward glide modest and feet near the ground, and distinguish the backward weight shift from it. All crouched bodies use the SAME anatomy and head size as standing. Render only the eight-cell green sheet.
```

V1 generation completed; correction history follows.

V1 original: `exec-383e9949-7dc6-4a53-8300-05e18f20fc77.png`, retained in `full-kit-sources/kogan-ground-v1-green.png` (1536×1024). The top glide pair is restrained with matching legs and changed cape folds; crouch phases and equipment are complete. The backward travel drawing is wrongly a forward lunge. Targeted correction:

```text
Use case: precise-object-edit. Correct ONLY the SECOND FIGURE IN THE BOTTOM ROW of this eight-cell Kogan ground-movement sheet. That figure is meant to RETREAT LEFT while facing RIGHT, but currently leans forward to the right. Redraw just its body/cloth into a BACKWARD weight shift: keep the feet low with rear/left boot near x=480 and front/right boot near x=665, pelvis centered near x=565. Put the head center near x=510, clearly LEFT of the pelvis and above the rear boot. The torso slants up-LEFT from pelvis to shoulders; the cyan visor still looks to RIGHT. Bend the rear/left knee to receive weight; front/right leg reaches low to the right. Do not create a forward lunge, kick, jump or flying body. Keep the right-hand full cyan saber angled down-forward toward RIGHT, complete tip above the soles. Left palm stays near chest. Copper cape remains large; its folds lag slightly toward RIGHT below shoulder level, leaving the face clear. Keep identical anatomical size, copper nemes/cobra, visor, armor/chest eye and saber length. Preserve ALL SEVEN OTHER FIGURES, all green background, the overall layout and image dimensions unchanged. Render the complete eight-cell sheet with only that second bottom-row retreat figure corrected.
```

V2 original: `exec-48d72794-32ea-419d-9a95-ae7ada8abf1e.png`, retained in `full-kit-sources/kogan-ground-v2-green.png`. The retreat now has the head left of pelvis and weight over the rear leg, facing right. Two rectangular source gutters still overlap: the third top blade extends past the fourth cape's leftmost x, and the second bottom cape crosses the preceding blade's rightmost x. Targeted layout correction:

```text
Use case: precise-object-edit. Correct only green gutters in this eight-cell movement sheet. Keep every figure's exact anatomy, pose, scale, equipment, colors and detail. Translate the ENTIRE THIRD FIGURE IN THE TOP ROW 40 pixels LEFT, including its full horizontal saber and cape, as one unchanged figure. This creates a clear vertical green gap between its saber tip and the fourth figure's cape. Also translate the ENTIRE SECOND FIGURE IN THE BOTTOM ROW 30 pixels RIGHT, including its cape, both boots and full saber, as one unchanged figure. This creates a clear vertical green gap after the first bottom figure's saber tip. Do not resize, rotate, shorten, redraw or crop any figure. Preserve the other six figures unchanged and the exact 1536×1024 canvas. Flat green fills the vacated locations. All eight complete figures must remain separated by uninterrupted green vertical gutters within each row. Render the complete corrected sheet.
```

V3 generation completed; one source gutter remains unresolved.

V3 original: `exec-d4969872-c75c-437d-a5a1-21753ea6ac55.png`, retained in `full-kit-sources/kogan-ground-v3-green.png`. The second bottom figure moved right, clearing its neighbor. The third top figure did not move sufficiently; its tip still extends to about x=1155 while the next cape begins around x=1145. No integration acceptance. One-figure layout retry:

```text
Use case: precise-object-edit. Move ONLY the THIRD FIGURE IN THE TOP ROW 45 pixels LEFT within this sheet, including its complete cape, body, boots and long horizontal cyan saber. Its head is currently centered around x=990: place that head around x=945. Its saber hilt is currently x=910: place it around x=865. Its full saber tip is currently around x=1155: place it around x=1110. Its leftmost cape is currently around x=780: place it around x=735. Keep the blade length, body proportions and anatomical scale identical. The complete figure must fit between x=730 and x=1115. An uninterrupted vertical green gutter at least 25 pixels wide must separate this third figure from the fourth top figure, whose leftmost cape is around x=1145. Preserve the other seven figures and the exact 1536×1024 canvas unchanged. Do not crop the blade or change any pose, angle, size, material or detail. Render the complete sheet with only that third top-row figure relocated left.
```

V4 original: `exec-afa25244-79c4-491c-944b-149562260eb1.png`, retained as `kogan-ground-v4-green.png` (1536×1024). The third top figure moves left enough for a clear extraction gutter. Eight complete figures retain full weapons and ground support; source inspection accepted V4 for the subsequent runtime review.

Measured bounds (inclusive), extraction regions and roots:

| Cell | Solid bounds | Region | Root x |
|---|---|---|---|
| Glide A | (25,116)..(336,448) | [0,0,365,500] | 250 |
| Glide B | (392,119)..(704,448) | [365,0,723,500] | 615 |
| Half crouch | (741,187)..(1123,447) | [723,0,1135,500] | 915 |
| Low crouch | (1144,240)..(1512,452) | [1135,0,1536,500] | 1345 |
| Retreat gather | (44,558)..(361,893) | [0,500,374,1024] | 232 |
| Retreat travel | (385,562)..(757,891) | [374,500,780,1024] | 570 |
| Retreat brake | (810,563)..(1135,895) | [780,500,1150,1024] | 953 |
| Settle | (1162,561)..(1481,905) | [1150,500,1536,1024] | 1350 |

Shared anatomical height is 340 source pixels; runtime anchors each lowest keyed support pixel. Existing utility gather/brake/settle drawings supply the brief run entry/exit. Run cycles the two changed cape drawings with fixed legs every eight simulation frames after the initial two-frame gather. Crouch enters through the half drawing for two presentation frames and holds low; release can rise through the same half drawing while already actionable. Backdash samples gather/travel/brake/settle inside its unchanged 14f. New inputs always override ground settling. No sim or input timing change.



Runtime acceptance: all 36 final cases / 54 seconds were played at 1×, both facings at center/corners. Selected run gather/cloth/brake/settle, crouch entry/hold/rise and backward retreat phases were frame-stepped. Measured existing walk regions remove a neighboring cape fragment. A focused candidate exposed close-crouch face occlusion; the final drawing order preserves the low body and brief rise while retaining attacker priority. Full weapons, floor support and large cape remain visible. Same-cell travel trails are deliberate. Complete before/final evidence and G1/K1/S1/S2/A2 refraction are in the full-kit report.

129 workspace tests (89 sim + 40 client), clippy and locked/offline release pass. All 3,240 focused ground ticks and 2,100 retained integration ticks match baseline. The complete new 35-second integration video was played at 1×; eight fresh smoke images are byte-identical to inspected preceding images. This accepts Kogan ground movement, not remaining attacks/reactions or Raya's kit.
