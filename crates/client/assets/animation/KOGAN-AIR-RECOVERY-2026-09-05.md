# Kogan airborne recovery — September 5

Status: V1 integrated and reviewed. Baseline legal Raya uppercut/RC/air-normal juggle shows a flat falling Kogan switching directly to rear-saber compression. New defensive drawing request is independent of the un-retried StP/StK/CrK rejection.

References: approved kogan-reactions-v1-green.png and kogan-movement-v2-green.png, visually inspected. Built-in imagegen; no programmatic bitmap edits.

## V1 exact prompt

```text
Use case: stylized-concept.
Asset type: four high-resolution full-body animation keys for Aeon's existing Kogan, arranged as a clean 2 by 2 sheet on a perfectly flat bright green chroma background, generous empty margins and gutters.
Primary request: draw the connected recovery from a non-knockdown aerial hit, ending in supported landing compression. These are defensive recoil/recovery poses, no attack. Read left to right, top to bottom.
Input image 1 is the approved Kogan reaction sheet, reference for identity, complete equipment, anatomical proportions and the low forward saber. Input image 2 is his approved movement sheet, reference for airborne anatomy, copper cape and rendering style. Create four new drawings; do not copy either sheet layout or include its other poses.
Identity: adult male armored fighter, copper nemes hood and large flowing copper aura-cape, narrow cyan visor, cyan chest-eye, dark etched armor. One complete plasma saber continuously in his right hand; ornate revolver stays holstered. Same head and body size in every cell; same side-on three-quarter camera, facing screen right throughout.
Top left: beginning to descend after an airborne impact. Chest recoils backward, shoulders left of hips, knees bend and feet hang forward/right below the hips; body diagonal, not lying flat. The free hand opens for balance. Saber held low across the front, pointing down and screen right.
Top right: torso folds forward slightly and knees gather beneath the hips while he is still airborne. Head begins to come above the pelvis; free hand lowers toward balance. The same complete saber remains on its quiet low forward line.
Bottom left: nearly upright descending body, both boots extend below the pelvis to prepare for ground contact, knees still soft. Copper folds lift behind the shoulders from the descent. Keep saber forward and entirely above the boot soles.
Bottom right: both boots now supported on an implied floor, knees absorb the landing in a compact crouch, chest inclined toward screen right, head looking toward the opponent. Free hand out for balance. Full forward saber above the soles, no planted weapon. This ends into the reaction sheet's front-saber landing and ready stance.
Style: match the approved painterly high-resolution fighting-game artwork, crisp complete silhouette, detailed copper and black armor, controlled cyan light, no pixel art. Draw joint changes and cloth continuity between keys; do not merely rotate a rigid body. Entire cape, visor, hands, boots and every weapon tip inside its own cell with at least 35 pixels of green clearance. No floor, shadows, scenery, motion blur, trails, impact sparks, text, labels, borders, other characters or extra limbs.
```

## Original, integration and acceptance

Original built-in output `exec-e34ddaa2-ddf3-4e78-8842-00ffb213ead6.png` (1254×1254), unchanged copy `kogan-air-recovery-v1-green.png`. No bitmap edits or revised request. This defensive recovery request is independent of the original blocked grounded-light request.

Runtime source regions/root X/anatomical height: `[0,0,625,600]/400/500`, `[625,0,1254,600]/972/500`, `[0,600,625,1254]/375/500`, `[625,600,1254,1254]/950/500`. Air root Y overrides540/540/1150 project below tucked boots; final grounded root uses measured support. Read-only keyed silhouette bounds at alpha≥24: `[115,67,575,515]`, `[712,88,1093,491]`, `[153,634,516,1148]`, `[699,786,1112,1143]`. Entire saber/cape and shared head/body scale inspected in source and runtime.

Kogan airborne Hit without pending knockdown holds cell0 while rising or stun≥4; descending final stun uses tuck cell1 above24 px, feet cell2 below. Immediately following2f Landing uses cell3 then existing Reaction10. This remains the original stun/landing; it adds no control or simulation change. New actions/reset interrupt history, and opaque clean cuts prevent doubled equipment.

Complete24-case receiver /60 s final and four-case /10 s candidate playback reviewed at1× in1280×720, selected contact/tuck/feet/support/half-rise/stand stepped across facings/corners. Complete64-case before/final audit /160 s each and35 s integration reviewed. V1 accepted without revision. G2/G1/S2/S1 refraction and exact phase ticks are in the full-kit report. 169 tests, clippy/release;9,600 final and2,100 integration ticks unchanged; eight repeat smoke images equal prior inspected evidence.
