# Kogan normal throw and throw tech — reviewed, September 5

Built-in imagegen for two tech drawings; normal throw reuses the previously reviewed cape-step V3 top row. Original blocked grounded StP/StK/CrK generation is unrelated and has not been retried.

## Baseline and refraction

All 32 legal cases / 80s viewed completely at 1× in 1280×720: hit, both guards, crouched hit, miss, jump escape and early/late tech, both facings at center/corners. Hit 13/15/21/22/26/35/36 and late tech 20/21/25/30/35/36/40 stepped. Throw holds the same reaching silhouette through recovery and leaves a duplicate over idle. Tech changes to an undersized sword-block pose, then abruptly grows back at return.

G2 (Garou, 6:17.40–6:18.10) supplies visible arm withdrawal; A1 (original Accent Core, 1:23.78–1:25.05) supplies supported equipment rest and separation of attacker recovery from victim consequence. S2 (SSV Special, 3:06.25–3:07.65) informs the familiar complete weapon return. These are motion principles already inspected in the dossier, not claims that the excerpts depict throw tech or match Aeon timings.

Normal throw uses V3 utility cells 0/1/2/3 at its own 2/1/20 timing. On hit, reach holds action frames 2–8 through the seven-tick tech window; release/withdrawal owns 9–15 and ready 16–22. On miss, reach owns only active frame 2; withdrawal owns 3–12, ready 13–22. Existing command grab selection remains unchanged. No simulation, boxes or input values change.

Successful tech is created at frame 0 and advanced by the world in the same tick; post-tick captures show frames 1–15. New separation occupies 1–5, new withdrawal 6–10, and existing utility ready 11–15. Legal input and complete return regressions retain that behavior. No duration is extended to display art.

## Exact tech prompt V1

Reference image: approved `kogan-cape-step-v3-green.png`, viewed in full before generation.

```text
Use case: stylized-concept.
Create a production sprite sheet for the existing adult armored duelist Kogan in Aeon. The attached approved cape-step sheet is a REFERENCE for identity, full adult anatomy, large flowing copper cape, material rendering and equipment. Use its TOP ROW only as the character reference. Do not copy that sheet layout or its forward-reaching grab. Preserve the copper nemes hood/cobra, horizontal cyan visor, black etched armor, chest eye and ONE intact cyan saber in the RIGHT hand.

Draw exactly TWO complete full-body chronological drawings, side by side in ONE ROW on a wide 1536x1024 canvas. Both face RIGHT in the same side-on three-quarter camera. Keep identical head size and unfolded anatomical height around 700 pixels, with both boot soles on a shared baseline. The body remains a tall adult with shallowly bent knees. Give each figure generous empty green on every side and between figures, including beyond the whole blade and cape. No cut-off tip or cape.

These are two phases of a THROW-TECH DISENGAGEMENT: Kogan breaks contact and slides backward LEFT while continuing to watch the opponent on the RIGHT. There is no attack, opponent, shield or effect in this sheet. Convey the backward response through supported hips, opening palm and cloth, not a compact crouching guard.

LEFT DRAWING — BREAK AND SEPARATE. Weight over the rear LEFT boot with a shallow rear-knee bend, front RIGHT leg extending naturally toward the right with its boot still supported at the baseline. Torso leans subtly BACK LEFT from the hips, shoulders remain broad and head upright looking right. His FREE LEFT palm is open toward the right at lower-chest height, elbow softly bent; this is a brief release gesture, hand remains below the visor and clear of his face. His RIGHT hand grips the saber hilt beside the front hip. The COMPLETE narrow straight cyan blade points DOWN-FORWARD RIGHT, with its tip clearly above the boot baseline. Preserve both distinct arms and natural elbows. Large copper cape lags slightly forward from the backward shift but stays behind his shoulders; the visor, palm and blade remain clearly separated.

RIGHT DRAWING — WITHDRAW AND REGAIN STANCE. Same facing, adult proportions, head scale and supported boots. Torso straightens back over the hips, knees soften symmetrically. Free LEFT elbow folds, bringing the open left hand back beside the lower ribs, visibly closer to the body than the first drawing. RIGHT hand keeps the same hilt at the front hip, complete saber still down-forward-right, tip above both boot soles. Large copper cape begins to settle broadly behind him toward the left. This should lead naturally into the top-right ready drawing of the reference.

Crisp high-resolution painted 2D illustration, controlled copper highlights, dark armor, thin cyan script, coherent lighting matching the reference. Uniform flat technical chroma green #00e600, including all gaps. No floor, shadow, labels, text, grids, borders, effects, shields, sparks, crescents, ghost limbs or blur. Both complete silhouettes and full equipment fit well inside their own half of the canvas.

```

## Source and measurements

Original `exec-1eac9d47-99c8-41f8-bbdb-028bef9003a8.png` preserved under the task generated_images directory, copied unchanged to `kogan-throw-tech-v1-green.png` here and on Citadel. Actual 1536×1024. Full sheet viewed: complete saber/cape, distinct open palm and tucked hand, consistent adult head/anatomy, shallow supported knees. Runtime regions `[0,0,720,1024]` and `[720,0,1536,1024]`, roots 440/1150, common anatomy 670. Read-only key measurements find solid bounds (59,173)–(672,844) and (756,174)–(1382,843); gutter 720 is clear. Both saber tips sit above the lowest support. Runtime review is complete; see the accepted focused and final evidence below.

## Focused review

All three candidate clips / 450 ticks / 7.5s viewed completely at 1× in 1280×720. Whiff 13/15/16/25/26/35/36, crouched hit 13/15/21/22/28/29/35/36 and late tech 20/21/25/26/30/31/35/36 stepped. The miss now withdraws immediately, connected reach holds until the victim releases, and the saber returns in front before idle. Tech shows a full-size open-palm break, folded hand and familiar ready; complete blades stay above the floor and the previous blocking duplicate is gone. Both standing faces remain visible at capture/separation. The existing Raya guard/getup blends and brief pre-grab crouched overlap remain part of her open presentation review.

All 450 focused ticks match baseline excerpts. All 450 PNGs are archived; 151 tests (89 sim + 62 client), clippy and locked/offline release pass. Candidate accepted for the complete 32-case final validation.

## Final acceptance

All 32 final cases / 80s and the complete new 35s integration were viewed at 1× in 1280×720. Final contact and tech separation were stepped in both facings at center/corners, then release/withdrawal/ready/idle at the left corner. All 4,800 final, 450 focused and 2,100 integration ticks equal baseline. All eight smoke PNGs are present: seven match inspected overhead evidence, and changed versus-mid was directly inspected as the intended new normal-throw withdrawal. 151 tests, clippy and locked/offline release pass. No sim values changed. Full videos, traces, 1,760 final diagnostics, all 450 focused PNGs, 71 integration diagnostics and verification JSON are archived. Raya tech/guard/getup ghosts and brief pre-grab crouched overlap remain part of her open review; the full-kit goal continues.
