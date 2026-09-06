# Raya crouching palm and kick

Status: seven V1 drawings and corrected V2 kick contact accepted in gameplay. Baseline153d5f8; ART35 passes. Built-in image generation.

References: inspected raya-ground-v1-green.png (identity/anatomy/low support) and raya/cr_light.png (old contact-height reference). G2 Garou supplies the arm withdrawal principle; K2 KOF XIII supplies distinct hand/leg support and low recovery. Preserve original CrP5/2/8 Mid and CrK5/2/9 Low; no sim changes.

All40 baseline cases /40 s played at1×1280×720. CrP13/18/28/31/33/36/38 stepped: held extended crystal hand through recovery and repeated half-crouch entry. CrK full playback shows that same hand gesture while contact is at the ankle. Exact archives: notes/media/2026-09-05-full-kit/raya-crouch-lights-before/.

## Exact V1 prompt

```text
Use case: stylized-concept
Asset type: eight original animation drawings for Aeon, a high-resolution 2D fighting game.
Create one portrait sprite sheet, two columns by four rows, containing eight separate complete full-body drawings of the same adult woman Raya. All face screen right in side/three-quarter fighting view. Solid uniform bright chroma green background, no floor, shadows, grid, text, labels or effects. Generous green clearance around each complete cape, hand and sandal.
Input image 1 is the approved body, costume, anatomical scale and low support reference; particularly its fourth figure, the low crouch with one hand near the floor. Input image 2 is the old crouched hand extension as an identity and contact-height reference only. Make original new phases; omit the old floating cyan crystal. Preserve Raya's brown face, white hood, jeweled brow, copper shoulder armor and large copper cloak with narrow cyan ornament, layered white linen robe over loose trousers, cyan jewelry and copper strapped sandals. Rich painted game rendering with crisp anatomy; no pixel art. Same head, torso and limb dimensions in every cell.
Read left to right, top to bottom:
0. Crouching palm preparation: low supported kneeling stance, rear leg folded under hips, forward foot planted and forward knee raised; head low, torso poised with a slight forward inclination. Lead open palm gathered beside the chest with bent elbow, other hand near sternum.
1. Crouching palm contact: keep the same low body and support. Extend the lead forearm forward almost horizontally from the lower chest, with a vertical open palm and fingers together. Palm heel is about two-thirds of the crouched figure's total height above the floor, aiming at a standing opponent's waist or crouched opponent's chest. Short compact reach with a nearly straight elbow; other hand remains by sternum.
2. Crouching palm withdrawal: keep the body low, fold the lead elbow back toward ribs, palm visibly near chest again. Shoulder weight settles rearward; linen follows the arm without changing anatomy.
3. Crouching palm ready: regain the low crouch from the fourth figure of input image 1, with one hand resting lightly near the floor just ahead of the body, other hand near chest, rear leg folded and forward foot supported. Full cape trails behind, head remains low.
4. Crouching kick preparation: stay low and gather the forward foot near the hips, knee deeply bent. Shift support to the folded rear leg and one open hand on the floor close to the hip. Keep the head composed and the other hand near chest.
5. Crouching kick contact: extend the forward leg nearly horizontally along the floor, complete copper sandal aiming at an opponent's ankle, heel only slightly above the support plane. Same planted supporting hand and folded rear leg carry the low hips. Show the straight kicking shin distinctly under the loose trousers, separate from the rear robe. Complete toes, heel, supporting fingers and cape must stay within the cell.
6. Crouching kick withdrawal: bend that knee and draw the foot back close to the body, continuing the same low hand/rear-foot support. The folded trouser leg must differ clearly from the extended contact pose.
7. Crouching kick ready: forward sandal replaced on the floor and the same low ready posture as figure 3 restored, one hand near floor and the other by chest.
All eight figures are LOW crouches. Keep their quiet face and consistent body scale; do not stand them up during recovery. Use real joint changes, connected support and linen folds. Empty hands, no crystals, weapons, circles, energy trails, hit flashes, opponents, ghosts or borrowed costumes.
```

V1 original: exec-677e564c-fc63-4605-9ef8-683a14473d21.png, copied unchanged as raya-crouch-lights-v1-green.png (1024×1536). All eight source figures inspected: complete hands/sandals/cape, same hood/face/materials, distinct low palm and supported leg extension. Final kick withdrawal lifts the supporting hand as the foot regathers; confirm its quick support transfer in gameplay.

Initial extraction regions: [0,0,500,385], [500,0,1024,385], [0,385,500,750], [500,385,1024,750], [0,750,475,1105], [475,750,1024,1105], [0,1105,480,1536], [480,1105,1024,1536]. Common anatomical standing height450; roots285/705/285/725/190/600/280/695. All non-green source bounds have clearance. Runtime review pending.

## V1 gameplay review and targeted correction

All40 V1 cases /40 s played completely at1×1280×720 after checksum archival. Palm12/13/18/28/30/33/35/36/38, mirrored corner198, low guard498 and miss978/982/986/990 show distinct phases and direct low return, with consistent scale and complete cloth. Kick13/18/28/31/33/36/38 and miss978 expose an overlong extension beyond the ankle flash, despite sound chamber/withdrawal/ready and support. The two extra baseline CrK phase seeks occurred in a319px hidden viewport and are excluded from gameplay-size phase review; the full40 s baseline had already played at1280×720.

## Exact V2 targeted edit

```text
Use case: precise-object-edit
Edit the supplied original Raya crouching animation sheet. Change ONLY the kicking leg in the THIRD ROW, RIGHT COLUMN figure (the sixth figure). Its current fully straight leg reaches too far. Make this a compact bent-knee ankle kick: keep the upper thigh close to the low hips, bend the knee downward, and extend just the lower shin and complete sandal forward near the floor. The sandal should finish only about one head-length ahead of the front edge of the folded supporting leg, with heel near the floor and toes raised. On this 1024-pixel-wide sheet, bring the kicking sandal from the far-right edge near x940 inward to approximately x780–810. Preserve natural adult leg lengths through the knee bend and side-view foreshortening; do not shrink the anatomy.
Keep the exact torso, head, face, hood, shoulder armor, planted supporting hand, folded supporting rear leg, cape, jewelry, palette and body scale of that figure. Change the white trouser folds to follow the bent thigh/knee and short forward shin. Preserve all seven other drawings, every cell position, sheet dimensions and solid green background. Complete fingers, toes, heel and cape remain visible. No effects, added objects, text or ghosts.
```

Target: inspected unchanged V1 sheet. Retain seven sound V1 cells; only use V2 contact after inspection.

V2 original: exec-92e4cfe9-ec96-403c-bea1-0f519bcd3a15.png, copied unchanged as raya-crouch-lights-v2-green.png (1024×1536). Only third-row right contact selected; all other cells retained from V1. Source review: natural bent knee, short forward sandal, complete support fingers/robe/cape and unchanged identity. Contact root620 follows the small source body shift; other roots and height450 unchanged. Corrected gameplay accepted in the complete final2 CrK and integration review below.

## Accepted runtime

Seven V1 cells and V2 cell5 are used from unchanged generated PNGs. Final anatomical height450, roots285/705/285/725/190/620/280/695; extraction regions unchanged. Complete final CrP and final2 CrK played at1×1280×720 (40 cases /40 s); selected phases, both corners, hit/guards/misses stepped as listed in the full-kit report. The compact kick heel meets its flash with complete fingers/sandal and a coherent support transfer; the palm folds before the direct low return. Full35 s integration reviewed;2,400 focused and2,100 integration ticks equal baseline. Seven smoke hashes match standing-light evidence; changed winner pair has only a one-pixel-wide text edge difference.171 tests, clippy and locked/offline release pass. Archives checksum verified; ART35 passes.
