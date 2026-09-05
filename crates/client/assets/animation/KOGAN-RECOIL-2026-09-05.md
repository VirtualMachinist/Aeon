# Kogan grounded hit and guard — reviewed

V2 correction requested after focused runtime stepping: V1 top-right release folds the torso far forward rather than returning halfway from the backward recoil. At StS ticks 40→43 the head travels excessively toward the opponent. Keep the planted feet and correct the upper-body drawing.

Exact V2 edit prompt:

```text
Use case: stylized-concept.
Edit this exact 1024×1536 Kogan recoil sheet. Change ONLY the TOP-RIGHT standing-hit RECOVERY figure's torso, head and off arm. Keep all seven other figures identical, and preserve this figure's two boot positions, knees, root, costume, full saber, cape identity, source-cell boundaries and flat green background.
The current top-right recovery hunches far forward over the front knee. It must instead be the controlled halfway return from the TOP-LEFT chest-back impact: shoulders and head return toward vertical but remain slightly BACK toward LEFT of the hip center. Put the head centered roughly x675–690 (instead of x750), at about y75; neck and shoulders naturally connect above a softly reclined torso centered near x680. Keep the visor looking RIGHT. The chest should be more upright than the top-left impact, not folded forward. Off hand settles near ribs, elbow compact. Knees stay softly bent and both existing boots stay fixed. Preserve anatomy, Egyptian copper nemes/cobra, cyan visor/chest eye, etched dark armor and large copper cape; adjust nearby cape folds naturally only where needed behind the changed upper body. Preserve the right-hand saber's complete length and low forward-right line with tip above soles.
This is a small correction to one recovery pose, not a redesign or a new layout. Preserve all eight full drawings and clean green gutters. No extra limbs, weapons, effects, floor, labels or other changes.
```

Built-in imagegen. References (both inspected): `kogan-reactions-v1-green.png` for established reaction body language and identity, `kogan-ground-v4-green.png` for reviewed armor/cape/saber and crouch anatomy. These trace back to the approved original plate; no plate is sliced. G2/S2 consequence beyond the spark and recognizable recovery inform this batch. Floor/getup drawings are retained for focused review.

Exact prompt:

```text
Use case: stylized-concept.
Asset: eight coherent new defensive reaction drawings for Aeon's original adult male armored duelist Kogan. Reference 1 supplies the established reaction body language and identity; reference 2 supplies the currently reviewed detailed armor, cape, full saber and crouch anatomy. Use them as references only. Create a NEW portrait 1024×1536 sheet, EXACTLY TWO COLUMNS and FOUR ROWS, eight complete full-body figures. Read each row left to right as contact response then recovering control. Flat chroma green #00e600 fills all empty space. No labels, borders, grid, shadows, floor, opponent, impact sparks, detached trails or scenery.
Identity throughout: copper Egyptian nemes/cobra, cyan visor, dark intricately etched armor, cyan chest eye, holstered revolver, enormous copper aura-cape, one long cyan saber in his right hand. Painted high-resolution 2D game art, right-facing side/three-quarter camera. Keep the same head size, shoulder width, costume, lighting and full saber length across all eight. Each standing anatomical body would be 275 source pixels high; crouching figures bend joints without shrinking anatomy. Leave wide uninterrupted green gutters and at least 18 pixels around EVERY full cape, boot, hand and saber tip. Grounded feet and cloth stay on the same invisible floor near the bottom of each cell; every complete blade stays ABOVE that floor.

ROW 1 — standing hit recoil and recovery:
LEFT: the chest opens and shoulders/head recoil back toward LEFT, both knees soften, rear heel takes weight. Face still points RIGHT. Off hand opens near chest, saber held low forward toward RIGHT, full blade angled shallowly down with tip well above boot soles. Cape fans behind and lifts at the upper folds. A physical recoil without wounds or damage to clothing.
RIGHT: regain control, torso returns halfway upright, knees remain flexed, off hand folds toward ribs; full saber draws nearer the familiar low forward ready line. Cape folds begin settling. This is a distinct later recovery pose, not an identical recoil and not yet the complete idle stance.

ROW 2 — crouched hit recoil and recovery:
LEFT: remain deeply crouched at original anatomical scale, head and shoulders recoil slightly backward LEFT, chest twists open, one knee stays close to ground and forward foot remains planted. Off palm opens near chest. Full saber remains shallow/horizontal forward RIGHT across knees, tip above floor. Cape gathers behind without hiding the face or hand.
RIGHT: shoulders return toward RIGHT over the low support, off hand approaches the front knee, head remains at crouching height, complete saber restores a low horizontal ready line. Cape settles. Both drawings stay low; do not turn this into standing recoil.

ROW 3 — standing block brace and release:
LEFT: feet planted in a compact stagger, knees compress slightly, chin tucked. Right saber hand is held in front of chest; full blade rises diagonally forward-right at about 60 degrees, complete tip below the cell top. Left forearm braces close beside the hilt without an extra hand. Torso absorbs pressure backward a little, cape upper folds lift. Defensive compact silhouette, no attacking arm extension.
RIGHT: pressure eases, knees and shoulders rise a little, elbows relax closer to the ribs, full blade lowers to a 35-degree forward-up angle. Face remains clear and focused toward RIGHT; cape catches up. Distinct controlled release from the brace, still a guarded posture.

ROW 4 — crouched block brace and release:
LEFT: low hips, one knee deeply bent, forward foot planted, chin tucked behind compact forearms at chest. Full saber aims diagonally forward-up RIGHT at about 35 degrees, above the low body, complete tip well inside the cell. Off forearm stays close beside the hilt. Large cape pools behind and keeps the face readable.
RIGHT: stay crouched while shoulders untwist slightly and elbows relax; saber lowers toward a shallow forward-right line above the knees. Off hand settles toward the front knee, cape folds settle. Same low anatomy and full weapon.
Avoid enlarged crouching heads, shrinking bodies, shortened sabers, blade tips below soles, cropped equipment, neighbor overlap, duplicate hands/weapons, pixel art and big effects. Eight distinct complete drawings only.
```



V1 generated original: `exec-8e4254f3-ada1-4cc3-a8f2-2ce985522e41.png`, retained unchanged as `kogan-recoil-v1-green.png` (1024×1536). Eight complete drawings provide standing/crouched hit response and regained control, plus standing/crouched guard brace and release. Source inspection finds full weapons above support, coherent armor/head identity and large capes. Crouched recoil raises the chest while knees remain low; runtime scale/height and narrow row-four gutter still require measurement and review. No acceptance from the sheet alone.

Runtime-key alpha≥24 bounds: row one x51–457/y52–378 and x522–919/y79–378; row two x44–517/y451–706 and x524–929/y472–707; row three x53–435/y743–1101 and x494–872/y784–1101; row four x26–462/y1155–1432 and x471–981/y1198–1432. Extraction splits 490/521/465/467 and row edges 0/410/730/1130/1536 preserve the narrow second-row gap. Shared anatomy 330 px, including crouches. Initial client tests validate complete boundaries. Runtime selection uses remaining stun: final four frames regain control, except knockdown recoil stays committed until the fall. No stun or recovery is added.

V2 original: `exec-de6dbb64-25c3-4dc8-b2b5-99948c6543ac.png`, unchanged archive `kogan-recoil-v2-green.png` (1024×1536). The corrected standing release brings the head over the rear/central support and returns the chest toward vertical. Re-measured top-right root x715 compensates the slight boot-placement change. Other source regions retain their measured boundaries; the Citadel complete-silhouette test passes. Final selection uses stun 3/2/1/0 (four actual visible ticks); the initial focused candidate used 4 through 0, and was tightened before final capture.


Final review: all 36 cases / 90 seconds at 1×, both facings and center/corners, plus selected impact/release/floor/getup/idle frame steps. Complete evidence and benchmark refraction are in the full-kit report. 131 tests (89 sim + 42 client), clean clippy and locked/offline release; 5,400 focused-family and 2,100 integration ticks remain byte-identical to baseline. The complete new 35-second integration preview was viewed. Original files and exact prompts remain unchanged.
