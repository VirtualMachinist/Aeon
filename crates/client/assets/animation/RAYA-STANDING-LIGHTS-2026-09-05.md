# Raya standing palm and low kick

Status: eight selected V1/V2 drawings and corrected final2 gameplay reviewed. Built-in image generation; baseline `224256a`. ART34 passes; publication recorded in the full-kit report.

The inspected G2 Garou arm-fold return and K2 KOF XIII knee chamber/extension/withdrawal inform these original gestures. Raya intentionally uses a low shin kick because her existing StK contact box is x8/y8/w40/h18; Kim's observed torso-height extension is not copied. StP remains 5/2/8 and StK 5/3/9. No simulation edits.

Baseline: both 20-case clips / 40 seconds played at 1× in a 1280×720 browser viewport. StK ticks 13/18/28/37/40 inspected: old contact is near the face while the actual flash is at the shin, followed by a second-body return ghost. Exact evidence: `notes/media/2026-09-05-full-kit/raya-lights-before/`.

## Exact V1 prompt

```text
Use case: stylized-concept
Asset type: eight original animation drawings for Aeon, a high-resolution 2D fighting game.
Create one portrait sprite sheet, two columns by four rows, with eight separate complete full-body drawings of the same adult woman Raya. All face screen right in side/three-quarter fighting view. Use solid uniform bright chroma green background, no shadows, ground, grid, labels or effects. Leave generous green gutters around every full figure, including copper cape and all fingers and toes.
References: image 1 is her approved identity and ready silhouette; image 2 supplies matching anatomical scale, grounded support and copper/linen rendering; image 3 supplies the open-palm contact idea only. Preserve her composed brown face, white hood, copper shoulder armor, large copper cloak with narrow cyan ornament, layered white linen robe over loose trousers, cyan jewelry, copper strapped sandals. Same head, trunk and limb scale across all eight drawings. Original richly painted game art, crisp readable anatomy, no pixel art. Empty hands; omit the floating crystal and palm ring from these new body drawings.
Reading order left-to-right, top-to-bottom:
0. Palm preparation: upright quiet trunk, knees softly flexed, lead open palm gathered beside chest with bent elbow, other hand close to sternum. Both sandals supported.
1. Palm contact: lead arm extends forward at upper chest height, elbow nearly straight, palm vertical and fingers together; modest forward shoulder weight. Other hand remains near sternum. A short direct palm strike, not a lunge. Complete hand visible with green clearance.
2. Palm withdrawal: lead elbow folds visibly back toward ribs, vertical palm now near chest, shoulders return upright. Cape and linen follow the returning arm naturally.
3. Palm ready: two-foot stance restored, lead forearm relaxed forward with palm up at waist/chest level as in identity reference, rear arm lowers toward hip. Quiet upright support, familiar full cape behind her.
4. Low kick preparation: weight over rear support sandal, lead knee only slightly raised forward, lead foot gathered near the supporting shin. Trunk remains poised and nearly upright; hands quietly gathered near chest. This prepares a LOW shin-level kick.
5. Low kick contact: extend the lead leg diagonally forward and DOWN from the hip. Its complete sandal is only about one-sixth of her standing body height above the supporting floor, aiming at an opponent's lower shin. The toe/heel are far below her own knee line. Rear leg bears weight, with a small counterbalance through hip and torso. The loose white trouser leg follows the downward diagonal and remains distinct from the hanging robe. This is not a waist or chest-height kick. Keep the entire extended sandal and cape within the cell.
6. Low kick withdrawal: bend the kicking knee again and draw the foot close to the support shin, ready to set down. Clearly different from the extended leg, with linen folds following the bent knee. Composed upper body and consistent head height.
7. Low kick ready: replace the lead sandal on the floor, restore the familiar two-foot stance and relaxed palm-up ready arm from drawing 3. Cape settles behind, all limbs complete.
All drawings share the same camera and adult anatomical dimensions. Keep the gestures compact enough for fast existing light attacks, with real joint changes between phases. Do not include opponents, weapons, lettering, contact flashes, translucent ghosts or borrowed commercial-game costumes.
```

References, in order: `art/fight-ready/raya/idle.png`, `art/fight-ready/animation/raya-ground-v1-green.png`, `art/fight-ready/raya/p.png`. All inspected before generation. These are identity/support/contact references, not edit targets.

## Exact V2 targeted edit

```text
Use case: precise-object-edit
Edit the supplied original Raya animation sheet. Change only the extended striking arm in the TOP RIGHT figure (row 1 column 2). Lower the palm contact from shoulder/upper-chest height to LOWER STERNUM height, about 60 percent of her full standing height above the floor. Her upper arm should slope slightly downward, elbow softly extended, forearm reaching forward and hand vertical with fingers together. Bring the wrist inward a little for a short compact palm strike. The palm heel should be near the level of the lower end of her blue chest pendant, above her belt. Keep the other hand by her chest. Preserve her exact head, face, hood, body size, shoulder placement, feet, cape, materials, palette and all seven other drawings. Keep the two-column/four-row sheet dimensions, cell positions, solid green background and complete body margins identical. No effects, new objects, lettering or ghosts.
```

Edit target: the inspected original V1 sheet. Keep sound V1 cells in runtime; select only the corrected contact cell if the output passes review.

V1 original: `exec-bb7cc39c-2c68-4b6b-badb-8a4f97aa798c.png`, copied unchanged as `raya-standing-lights-v1-green.png` (1024×1536). All eight complete source figures inspected. Initial runtime regions use column boundary500 and rows0/385/745/1115/1536. Initial common height340 and roots280/715/280/710/280/705/280/710 remain in the superseded first capture.

Both V1 20-case final clips /40s played fully at1×1280×720 after complete archive; an earlier partially transferred video stalled and is not counted as review. Palm13/18/30/34/37 shows clean phases but high contact and shifted ready. Low kick now reads near the shin, with full foot and visible return. Correct arm height and calibrate ready placement before acceptance.

V2 original: `exec-d2b9dcb9-ea26-4d58-bafc-51cdf08a48ec.png`, copied unchanged as `raya-standing-lights-v2-green.png` (1024×1536). Only its top-right contact is selected; the other seven remain V1. Complete corrected arm, five-finger silhouette, hood/face, cloth and feet inspected. Final candidate uses common standing height330, regions unchanged and roots255/690/255/685/255/680/255/685. Checks2 passes171 tests, clippy and locked/offline release. Corrected gameplay accepted after complete final2 and integration review below.

## Accepted final2 runtime

Seven V1 cells plus the V2 top-right contact use unchanged generated PNGs. Final common anatomical height330 and roots255/690/255/685/255/680/255/685; source regions unchanged. All40 final2 cases /40 s at1×1280×720, both facings and corners, hit/guards/ducks/misses reviewed. Palm13/18/30/34/35/36/37 and kick13/18/28/30/33/36/37/40 confirm separate phases and single idle returns; additional corner/guard/miss ticks are in the full-kit report. The lowered palm meets the flash, the low sandal replaces its support, and complete linen/cape/limbs remain visible. Full35 s integration reviewed;2,400 focused and2,100 integration ticks unchanged; eight smoke hashes equal inspected ground evidence.171 tests, clippy and locked/offline release pass. Complete archives checksum verified; ART34 passes.
