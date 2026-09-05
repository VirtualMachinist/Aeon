# Kogan crouching saber — reviewed

CrS/CrHS first batch, referenced to the approved ground V4 sheet inspected before generation. All 40 CrS/CrHS baseline cases / 60s played at 1× in 1280×720. CrS miss 15/19/27/33/34 and CrHS 16/21/31/41/42 stepped. Old CrS uses a clipped rear saber and extended punch; CrHS retains an overhead blade/crescent through startup and recovery, with miniature body and no withdrawal. S2 committed front weapon and distinct withdrawal, A1 recognizable weapon rest and G2 supported return are the relevant previously inspected references. No timing or geometry changes. Original blocked StP/StK/CrK request remains untouched.

## Exact V1 prompt

```text
Use case: stylized-concept.
Asset type: production animation atlas for Aeon's existing adult armored duelist Kogan. The provided approved ground-movement sheet fixes his identity, body proportions, crouch support, material style and orthographic side-on three-quarter game camera. Preserve the Egyptian copper nemes and cobra, cyan horizontal visor, dark etched armor, chest eye, ornate holstered revolver, long narrow cyan plasma saber in his RIGHT hand, and very large flowing copper cape. High-resolution painted 2D game art, coherent joints and restrained metal highlights.

Draw EIGHT complete full-body drawings, exactly TWO COLUMNS by FOUR ROWS on a tall 1024x1536 canvas. All face RIGHT. These are two different GROUNDED CROUCHING SABER actions, four chronological phases each. Every figure has the SAME adult anatomy and head size: standing unfolded anatomy approximately 330 pixels tall; bent crouching torso/legs approximately 200–220 pixels tall. Leave the extra space above low figures empty instead of enlarging them. A raised saber may occupy that space while the body retains its size. Knees remain deeply folded throughout, front boot supporting, rear knee/boot supported as in the reference crouch. No standing, jumping, kicks or punching.

Flat uniform technical chroma green #00e600 background, no floor, shadows, labels, boxes, borders, stage, opponents or added objects. Generous green gutters separate every whole silhouette on all sides, including full saber tips and cape tails. Each blade is a complete straight narrow cyan line with a visible copper grip in one right hand; no crescents, sparks, trails, ghosts or duplicate limbs. Keep cloth behind the torso, hands and face visible.

FIRST FOUR CELLS, read left-to-right across top TWO rows: a crouching forward SABER THRUST at the crouched body's chest height, approximately an upright opponent's upper-thigh/belt height.
1 top left, PREPARE: low balanced crouch, right elbow bent back beside the ribs, hilt gathered beside rear hip, intact saber aimed horizontally forward-right just above the raised front knee. Left hand near sternum, head watching right. Trunk loads slightly over the rear support.
2 top right, CONTACT: remain low; right shoulder moves forward, elbow opens and right hand drives the hilt forward at crouched CHEST height. Complete long straight blade extends horizontally right well beyond the front knee. Natural arm length, right hand clearly gripping hilt; left hand remains compact beside ribs. Front knee stays bent and front foot planted, rear support counterbalances the reach. Cape folds lag to the left. The reaching weapon, not an empty fist, carries this gesture.
3 second row left, WITHDRAW: visibly bend the right elbow, bring hilt back beside the chest/hip, keep the full blade pointing forward-right above the knee. Torso settles rearward while remaining deeply crouched. Cape tips continue outward after the hand retracts.
4 second row right, READY: return to the approved low crouching guard, hilt near front hip and full saber horizontal forward-right just above the knee, left hand near the front knee, head still watching right. Same supported low body, with cloth settling behind. Distinct from the extended contact.

LAST FOUR CELLS, read left-to-right across bottom TWO rows: a crouching rising SABER CUT. The BODY stays low while the WEAPON rises IN FRONT of the shoulder.
5 third row left, PREPARE: compact supported crouch, right elbow folded beside ribs, right hand near belt and blade angled slightly down-forward-right, full tip safely above the boot baseline. Left hand close beside chest. Shoulders loaded gently for the front upward cut, no overhead arm yet.
6 third row right, CONTACT: keep the same low torso and folded legs. Right hand extends forward at crouched CHEST height, elbow partially open; from that hilt the COMPLETE long straight saber rises steeply UP AND FORWARD RIGHT, about 65 degrees above horizontal. The blade runs in the clear space in FRONT of the head, spanning from the low hand toward an upright opponent's upper torso. Hand stays below the shoulder; do not put the hand over the head. This is the upward front segment of a shoulder-centered arc, never a rear arc. Front boot remains planted, hips supported, left arm tucked naturally by ribs, cape flaring behind. Full tip has generous empty green above and right.
7 bottom left, WITHDRAW: right elbow folds back toward ribs; wrist lowers the complete blade from its steep contact to a shallow upward-forward diagonal. Torso and hips settle in the same low crouch, with a clear change in arm and weapon direction and continuing rear cape folds. No blade below the feet.
8 bottom right, READY: the same familiar low crouching guard as cell 4, complete saber horizontal forward-right above the knee, right hand near hip, left hand compact near knee, both supports grounded and cloth settling.

Preserve equipment, body scale, two arms and two legs in all eight drawings. Separate preparation, extension, withdrawal and ready through real joint changes. Keep the entire saber and cape inside each cell. No pixel art, photorealism, blur, scenery or extra effects.

```

## CrS / CrHS V1 original and first integration

Built-in output `exec-b976f540-f043-4a2f-83d3-b24cc48129b8.png`, retained intact as `kogan-crouch-saber-v1-green.png`, 1024×1536. Eight complete bodies and full blades inspected. Regions split at x480 and y410/750/1140; shared anatomical height370 with grounded roots [285,750,285,750,270,740,275,735]. Forward contact and front rising cut are distinct; first runtime review pending.

## CrFL / CrST exact V1 prompt

```text
Use case: stylized-concept. Production animation atlas for Aeon's existing adult armored duelist Kogan. The supplied approved ground sheet is an identity and material reference: Egyptian copper nemes with cobra brow, cyan horizontal visor, black etched armor, chest eye, ornate holstered revolver, long narrow cyan plasma saber in RIGHT hand, very large flowing copper cape. High-resolution painted 2D game art, matching coherent adult anatomy and side-on three-quarter camera.

Draw EIGHT complete full-body drawings, TWO COLUMNS by FOUR ROWS on a tall 1024x1536 canvas. Every drawing faces RIGHT and stays deeply crouched. Same head size, same adult proportions in all cells; unfolded standing anatomy about 360px tall, low crouched body about 210px tall. Leave unused height empty; do not enlarge crouching bodies. Flat uniform technical chroma green #00e600. Generous empty green gutters around every whole figure including saber tip and cape. No text, borders, shadows, floor, stage, opponents, sparks, crescents, streaks or ghosts. Complete straight cyan blade with copper grip in one right hand throughout.

First FOUR cells, left to right across top TWO rows: a compact crouching LOW SABER TRAP. This is a blade gesture with grounded folded legs.
1 PREPARE: gather right elbow beside hip, hilt near rear thigh, blade aimed forward just above knees. Low supported trunk, front boot planted, rear knee/boot supporting; left hand compact beside ribs. Large copper cape behind.
2 CONTACT: lean torso modestly forward over the bent front knee, right hand moves forward LOW beside the front shin, and intact straight saber extends horizontally RIGHT at an upright opponent's shin height. Weapon hand distinctly below both knees, about 55px above boot baseline; complete blade stays horizontal safely above the floor. The arm drives the weapon, legs stay folded and support the weight. Left arm balances beside ribs. Clear copper grip, full luminous blade, natural arm length.
3 WITHDRAW: bend right elbow and draw low hilt back toward hip, raise intact horizontal blade toward the knee line. Torso eases back over support; cape tails follow outward. Still deep crouch.
4 READY: familiar supported low guard like the reference crouch, right hand near hip and full saber pointing horizontally forward above the front knee, left hand near knee, large cape settling.

Last FOUR cells, left to right across bottom TWO rows: a broader crouching LOW SABER SWEEP. The blade travels near the ankle line, with a distinct wide supported body shape. This is NOT a kick.
5 PREPARE: drop hips into a broad low crouch, rear leg folded beneath pelvis, front knee bent and front boot planted farther right. Left fingertips brace near rear support. Right elbow folds, hilt near rear ankle, intact blade points shallowly forward-right and stays entirely above both boot soles.
6 CONTACT: torso rotates forward over the wide grounded support, front knee remains bent with its boot planted. Right hand low beside front ankle, complete long straight saber extends nearly horizontally RIGHT, only about 25px above boot baseline, never sloping below it. The saber is the farthest forward element. Full blade tip visible, rear knee grounded, left palm provides distinct support behind the weapon hand, broad cape flows to left. No airborne or kicking leg, no crescent effect.
7 WITHDRAW: bend weapon elbow, draw hilt back and lift full blade safely toward knee-height; front boot slides visually back under its bent knee while remaining on ground, pelvis recenters, left supporting hand begins to lift. Cape tips lag behind the torso.
8 READY: return to the same low crouching guard as cell4, full horizontal forward saber above knee, hips gathered over grounded folded legs, left hand near knee, recognizable calm torso and settled large cape.

Draw real changes of arm joints, torso support and cape folds through preparation/contact/withdrawal/ready. Preserve two arms, two legs, equipment identity, stable anatomical scale, full blade and full cape in every cell. No pixels, photorealism, blur, duplicate limbs or added effects. The supplied image supplies identity only; do not reproduce its layout.

```

## Low V1 source review and exact V2 correction

Original `exec-8183d204-5298-42da-b555-a8c3d6379289.png`, intact `kogan-crouch-low-v1-green.png`. Rejected sweep tips below boot/palm support; trap contact too low. No V1 low art integrated.

```text
Edit the supplied Aeon Kogan low-saber animation atlas with one targeted correction to weapon height. Preserve the canvas, all eight cells, full bodies, adult anatomy, heads, armor, large copper capes, palette, green background, and every other cell exactly. No additions, effects, labels or cropping.

TOP RIGHT (cell 2, crouching low trap contact): raise the existing right weapon hand, copper hilt and entire horizontal cyan blade together by about 30 source pixels, so the complete blade lies at shin height, about 60 pixels above the boot baseline. Naturally bend the right elbow to connect to that raised hand. Keep the blade horizontal and full length. Keep the left hand, head, torso, knees, boots and cape exactly in place.

THIRD ROW, BOTH CELLS (cells 5 and 6, sweep preparation/contact): the weapon currently dips below the lowest boot/palm support. Raise each right weapon hand, hilt and whole blade together by about 35 source pixels, naturally bending the right elbow. In each of these two drawings, the entire straight blade must be nearly horizontal and sit about 20–30 pixels ABOVE the common boot and supporting-left-palm baseline. The complete tip must be above that line as well. Keep the low supporting torso, folded legs, planted boots, left supporting palm and cape exactly where they are. Do not move the body upward or shorten the blade.

Keep cells 1,3,4,7,8 unchanged. Preserve complete saber tips with empty green margins; no floor or shadow. The output stays a TWO-column FOUR-row 1024x1536 atlas.

```

## Low V2 source review and exact V3 correction

Original `exec-f74823b0-bdb0-46da-b134-822d97749d15.png`, intact `kogan-crouch-low-v2-green.png`. Sweep preparation/contact now clear boot/palm support. Trap blade rose only slightly and remains too low for its existing shin box. V3 targets only that cell, placing the hand/blade at the first cell’s blade line.

```text
Edit ONLY the TOP-RIGHT drawing of the supplied two-column four-row Kogan low-saber atlas. All seven other drawings must stay unchanged. Keep the exact 1024x1536 canvas, green background, full bodies, equipment and copper cape.

In that TOP-RIGHT low trap CONTACT drawing, the cyan saber is still too close to the boots. Put the entire horizontal blade on the SAME horizontal line as the blade in the TOP-LEFT drawing: about y=310 on the full canvas. Currently it lies around y=345. Move the right hand and copper hilt up with the blade to y=310, and bend the right elbow naturally so the forearm is approximately horizontal forward. Right elbow is bent beside the ribs; the hand moves forward ahead of the raised knee, actively reaching. The hilt and whole intact cyan blade should extend forward-right above the front knee and well above the boots, with blade tip fully inside the existing green margin. Keep the head, torso, cape, left hand, knees, both boots and body scale in their existing positions. Do not raise the body. The result is a short low forward saber extension with a supported crouch.

Do not alter the third-row sweep weapons: those now correctly sit above their support line. No extra effects, shadows, labels, floor, blur or cropping. Preserve the complete saber in every cell.

```

## Accepted source and runtime review

CrS/CrHS accept all eight V1 cells from original `exec-b976f540-f043-4a2f-83d3-b24cc48129b8.png`, intact as `kogan-crouch-saber-v1-green.png`. Column gutter x480; row boundaries410/750/1140. Roots left-to-right/top-to-bottom: 285,750,285,750,270,740,275,735; common anatomical height370.

CrFL/CrST accept all eight V3 cells from original `exec-eeac18d0-d705-45a1-8a99-1ff35eb2490b.png`, intact as `kogan-crouch-low-v3-green.png`. Row boundaries430/770/1110; x500 divider except third row475. Roots275,730,275,740,255,685,275,735; common anatomical height350. V3 raises trap contact to the existing low box's upper edge; both sweep preparation/contact blades clear support. V1/V2 remain source history only.

All16 complete source bodies/blades inspected before integration. All80 final cases/140s reviewed at1× in1280×720, both facings and center/corners, hit/high guard/low guard/crouched hit/miss. Accepted focused clips total14s, with contact, withdrawal and return steps; final mirrored/corner contact and sweep floor/getup steps also reviewed. History correction prevents replaying half-crouch entry after these already-low attacks. All8,400 final and840 focused ticks match baseline; 147 tests, clippy and release pass. Full35s integration reviewed with2,100 equal ticks; accepted repeat smoke's eight PNGs equal preceding inspected evidence. Exact phase steps and retained incomplete captures are in the full-kit report. No attack timing, geometry or simulation changes.
