# Raya signature normals — reviewed September 6

Built-in image generation, V1 original retained unchanged in `raya-signature-v1-green.png`. StS, StHS and StHSClose accepted with ten V1 drawings and two V4 contacts under ART37. Identity/style references: approved standing lights V1 and original Raya animation V1. No simulation changes.

## Baseline inspection

All three normal clips /150 seconds played fully at 1× in 1280×720. StS selected ticks13/21/33/38/42/45/46/47 and miss2421/2427/2431/2436: high palm overreaches above the body contact; eye appears after first-active hitstop; recovery leaves duplicate arm/head. StHS ticks13/24/38/45/54/62/65/66/67 and miss2424/2437/2448: high two-hand extension, delayed glyph, translucent arm/glyph over folded body; original bottom row touches image edge. StHSClose ticks13/20/34/40/48/49/50 and legal jump-miss2420/2436: same far-heavy pose at much shorter/lower contact, high hands above low guard, duplicate head during return.

Reference refraction: G2/Garou outward limb and distinct return; S2/SSV Special committed first-contact hold and withdrawal; A1/A2/original Accent Core recognizable supported rest and controlled body/effect hierarchy. Preserve Aeon timing and reach; create distinct waist-level normals rather than copy the reference moves.

## V1 prompt

```text
Use case: stylized-concept. Asset type: high-resolution 2D fighting-game animation sheet for Aeon.
Create a NEW square sprite sheet with exactly twelve full-body drawings in a clean 3-column by 4-row grid, viewed side-on facing right. Solid uniform chroma green background, no grid lines, lettering, floor shadows or scenery. Keep ample empty green gutters around every complete body, both sandals and flowing cape. Each cell must have the same anatomical scale and floor line.
Reference image 1 is approved Raya identity, anatomy, costume and finished painting style. Reference image 2 is her established composed ritual vocabulary. Preserve her face, jeweled brow, white linen, copper cloak and cyan jewelry. Do not copy the old high hand-contact placement from reference 2.
Columns are three distinct grounded normals: left = medium single palm, middle = far heavy two-handed written-light extension, right = compact close heavy two-hand press. Rows show four consecutive phases of each action: row 1 preparation; row 2 active contact; row 3 withdrawal; row 4 prayer-ready return.
All twelve poses keep a poised upright linen-column body with planted, fully visible sandals. These are grounded gestures, not jumps or running. Cloak folds carry restrained follow-through. Preserve graceful restrained torso and unmistakable limb paths.
Row 1: left gathers one hand toward lower ribs while the other remains close; middle gathers both hands near upper belly in preparation; right bends both elbows close to the ribs, hands gathered near the belt.
Row 2 is crucial: ALL contacting hands and small cyan glyphs must be at waist/upper-belly level, roughly 53 percent of body height above the soles, clearly below the chest and shoulders. Left extends one forearm and open palm diagonally downward toward the right at waist level, with a tiny thin cyan eye-like mark immediately ahead of the palm. Middle extends both arms forward and slightly downward at waist level, with a short fine horizontal written-light extension ending in a small open glyph; this contact reaches noticeably farther right than the left-column palm. Right keeps both elbows bent for a compact two-palm press just ahead of her belt, with a tiny open cyan glyph between the palms and their close contact point; this reach is much shorter than the other two. No giant disc, beam, airborne projectile, large opaque bloom or trailing duplicate arms. The contact symbol is present in this row only and does not obscure the hands.
Row 3: left palm visibly folds inward toward lower ribs; middle draws both forearms back toward her waist with falling cloak; right relaxes the compact press inward toward the body. No attack glyph in recovery.
Row 4: all three have hands nearly together near sternum in familiar prayer-ready support, stable feet and settled but naturally varied cape. This is visibly restored readiness, not another reaching contact pose.
Render clean detailed high-resolution painted sprites matching the first reference, consistent anatomy and costume, separate readable hands with plausible fingers, complete uncut hem and feet, no ghost images. The movement principles are clear preparation, committed contact and readable withdrawal; retain Raya's own futuristic mystical identity.

```

## Chant baseline retained for subsequent work

All three chant videos completed1×1280×720 playback: ChantI50s, ChantII50s, ChantIII60s (paused at30.007s between two continuous halves). Ten temporal samples each. This is baseline audit, not chant acceptance.

ChantI reproduces the high bare palm above the waist flash and only later reveals its small eye. Selected playback around13/22/35/39/47/48/49 shows gathered startup, delayed eye, prayer withdrawal and duplicate head at47/48. Fast browser seeks sometimes displayed an earlier decoded frame; claims follow the visible HUD, and exact archived PNGs provide subsequent phase evidence.

ChantII exact diagnostics0033/0041/0053/0061/0073 (HUD34/42/54/62/74) show first-chant eye, second-chant cross-body gather, upward curved glyph beside high palm, gathered return and idle. A thin cyan mark behind the body during the active/return source is imported from a neighboring cell. Its lower curved-glyph drawing improves gesture distinction but still needs measured extraction and contact-height review.

ChantIII full playback shows a high two-hand reach and delayed disc above its much lower body flash, then a waist-ready return; low guard face stays below the disc. The victim carries the knockdown to the floor. Its original bottom cells still touch the source image edge. New normal selectors do not change these chant poses.

## V1 gameplay review and focused revision

All60 V1 normal cases /150s completed1×1280×720 playback, with ten temporal samples per move spanning hit, standing/low guard, crouched hit and miss in both facings and corners. Complete traces equal the baseline for all9000ticks. The medium palm reaches the waist contact with its tiny eye present from first active. Exact diagnostics0013/0021/0033/0041/0044/0046 and2421 (HUD14/22/34/42/45/47 and miss22) show gather, low contact, withdrawal, prayer-ready, clean idle and a compact miss gesture. Foot/cloth boundaries are complete. V1 long-heavy glyph remains about18screenpx high/short; close-heavy glyph roughly35screenpx high and about18px short. Both are revised in V2, retaining the other ten V1 cells. All three last prayer poses sit about20screenpx behind idle; calibrated recovery roots will bring the supported return forward without moving simulation bodies.

Additional baseline ChantIII exact PNG0061/0073/0085/0097/0106 (HUD62/74/86/98/107) confirms chest orb, high bare hands, late disc, waist-ready and victim floor. No chant art is accepted by this normal batch.

## V2 prompt

```text
Use case: precise-object-edit. Edit target: the attached Raya signature-normal sprite sheet. Preserve its square size, 3 columns by 4 rows, all body positions, anatomical scale, faces, costume, complete feet and cloak, solid green background and empty gutters.
Change ONLY the arm/hand/contact-symbol placement in the SECOND ROW, MIDDLE and RIGHT cells. Retain the other ten cells unchanged.
SECOND ROW MIDDLE: lower both extended forearms, hands, the fine horizontal cyan writing and its small eye-like endpoint by about 28 pixels in this 1254px reference. The hands and entire thin writing line should run at lower-waist/belt level, visibly below the chest, with arms sloping gently downward from the shoulders. Keep the current horizontal reach, tiny open glyph and slim line, no bigger or brighter effect.
SECOND ROW RIGHT: lower both compact bent-elbow hands and their tiny cyan glyph by about 50 pixels. The short two-palm press should meet just in front of the upper hips, slightly below the belt, with elbows bent and forearms extending slightly downward. Preserve its much shorter reach than the middle-cell gesture. Keep the tiny open glyph directly ahead of the palms, no extra energy ring.
Do not shift, shrink, rotate or redraw the whole figures. Do not alter the medium palm in the left cell. Do not change preparation, withdrawal or prayer-ready rows. No duplicate hands, detached wrists, additional characters, labels, borders, ground shadows or missing feet. Maintain the approved high-resolution painted linen/copper identity.

```

## V2 extraction check and V3 spacing correction

V2 lowers the contacts as requested but moves their complete figures downward: the middle sandal and next-row hair leave insufficient empty rows for a safe rectangular extraction. Boundary tests failed at 594,648 and 588,645 for two candidate bounds. V2 remains unaccepted. V3 asks the built-in tool to retain only those two poses in a roomy two-column layout; no programmatic image edits.

```text
Use case: precise-object-edit. Edit target: the attached twelve-pose Raya sheet. Extract and re-layout ONLY the SECOND ROW MIDDLE and SECOND ROW RIGHT poses into a new wide 2-column, 1-row sheet. The left new cell is the far heavy two-hand downward extension with the thin cyan writing line ending in the tiny eye. The right new cell is the compact close heavy with bent elbows and the tiny cyan glyph directly before her upper hips. Keep these two poses' exact gesture, hands, contact height relative to the body, complete anatomy, face, linen, copper cloak, jewelry and painted style. Preserve their relative body scale. This is a spacing/layout correction, not a new action design.
There must be exactly TWO complete full-body figures total, with a shared floor line and generous solid chroma green blank margins on every side, especially at least 50 pixels of empty green below both sandals and all copper cloth. Each body and every cyan line must fit fully inside its own cell. No neighboring figures above or below, no labels, borders, floor shadows or grid. Keep the long thin glyph line fully visible and horizontal at the existing low-waist height, and the close glyph small and low at the existing upper-hip height. Do not raise the contacting hands, lengthen the close press, add effects, change the costume, duplicate arms or shorten the cloak. Flat solid green background. Deliver only this two-pose sheet.
```

V3 two-case calibration: StHS diagnostic0023 shows glyph29px below the flash; the requested V2 downward edit overshot after whole-body regeneration. StHSClose0020 meets the flash, and both0053/0061 and0046/0050 ready/idle pairs show corrected placement. V4 raises only the far contact group65sourcepx; the close contact stays fixed. V3 is a calibration candidate, not accepted full matrix.

## V4 prompt

```text
Use case: precise-object-edit. Edit the attached two-pose Raya sheet. Change ONLY the LEFT figure's two forearms, hands, and entire thin horizontal cyan writing line with its small eye endpoint: raise this contacting group by about 65 pixels in the 1774 by 887 reference image. Keep its horizontal reach exactly unchanged. The line should be horizontal at the upper waist, a little above her copper belt; both elbows remain natural and bent, forearms extend slightly downward toward that line. Do not move the shoulders, head, torso, feet or cloak. Preserve the complete right figure and its current low compact hand/glyph contact exactly. Preserve image dimensions, full body positions, anatomy, head heights, costume, green background, blank margins, style and all other pixels. No extra effects, ghost limbs, floating detached hands, labels or borders. This is a precise small contact-height correction to the left arms and glyph only.
```

V4 original: `raya-signature-v4-green.png`, copied unchanged from built-in output `exec-0fb3876a-3281-42dd-b071-f22e00feb282.png`. Two selected V4 contacts use reference1774×887, regions[0,0,890,887]/[890,0,1774,887], roots380/1230 and anatomical heights590/570. Ten other drawings retain V1. Six withdrawal/ready roots are205/575/970 and200/575/970; the medium contact retains225. V2/V3 remain archived calibration candidates. Full corrected final2 review accepted; see the full-kit report.

All60 final2 cases/150s and new35s integration reviewed at1×1280×720, with exact phase PNGs listed in raya-signature-verification.json. All9000/2100 ticks equal baseline; eight smoke PNGs equal inspected Flash/Style evidence.172 tests,clippy and locked/offline release pass. Built-in generation only; unchanged V1–V4 originals and all prompts retained. ART37 accepted; whole goal remains active.
