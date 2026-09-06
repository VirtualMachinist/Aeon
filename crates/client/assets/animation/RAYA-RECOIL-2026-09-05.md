# Raya grounded recoil and floor recovery — September 5

Status: V1 source and corrected runtime reviewed; publication recorded in the full-kit report. Baseline45d103c. All36 before cases /90 s fully played at1×1280×720, four temporal samples per10 s clip. StS-Hit19/24/37/41/44 reveals held recoil and duplicate return. CrST60(seek displayed61)/72/78/84/90/96/102/108 shows intact floor/support/kneel/half-rise drawings with duplicate old bodies; reuse the existing source before considering new floor art. Both throw routes and launch fully played.

Built-in imagegen original exec-66b338fd-e2d4-4e4d-9ec4-af5054d164d0.png, 941×1672, unchanged raya-recoil-v1-green.png. Approved idle and ground sheet references inspected. Eight complete right-facing defensive drawings: standing hit/release, crouched hit/release, standing guard/release, crouched guard/release. Shared anatomy and empty hands; no effects/weapons. All full figures inspected.

Refraction: reuse observed G2/S2 victim consequence and recognizable return; A1 supported floor rise and S1 cloth continuity. Keep original stun and24f getup; do not display false recovered control during pending knockdown.

## Exact V1 prompt

```text
Use case: stylized-concept.
Create eight full-body animation drawings for Aeon's existing adult female fighter Raya, a fictional high-resolution 2D fighting game character. Exactly TWO columns and FOUR rows on a perfectly flat bright green chroma background, portrait sheet with generous uninterrupted green gutters.
Input1 approved idle fixes Raya's identity, proportions and painterly materials. Input2 approved ground movement fixes anatomical size through crouching and continuous linen/copper. These are reference images, not edit targets. Create new defensive body gestures with empty hands and no floating crystal.
Same Raya throughout: warm brown complexion, composed face, jeweled brow and white linen hood, long layered white linen dress over loose trousers, large copper cloak with fine cyan-script trim, copper shoulder and wrist ornaments, blue jewels, copper strapped footwear. Every body faces RIGHT in a side-on three-quarter view. Same head size and limb proportions in every cell; lower crouches occupy less height and must never be enlarged to fill the cell.
Each row is a chronological pair, impact then release:
Row1 standing hit reaction. Left: feet remain planted in familiar stance, knees soften, chest and head displaced gently LEFT away from the opponent, chin lifted slightly, shoulders reacting backward, front hand raised open near shoulder and rear hand drawn beside ribs. Read a brief loss of balance from a game hit, not a block. Right: chest and head return above pelvis while knees remain slightly bent; hands lower toward the quiet ready gesture, shoulders settle, cloak catches up behind. An unmistakable recovery toward standing, not another big recoil.
Row2 crouching hit reaction. Left: low pelvis over deeply folded legs, torso and head rocked modestly LEFT over the rear supporting knee rather than leaning into the opponent; front forearm is loose near upper chest and rear hand balances near waist. Right: remain equally low, bring torso upright above pelvis and let one hand lower near the forward knee while the other relaxes beside ribs. Both phases preserve deep crouch height and complete feet. Cloth pools behind above sole level.
Row3 standing guard. Left: both feet support slightly bent knees; upright head tucked behind a compact raised forward forearm, palm toward RIGHT near cheek height, rear forearm quietly braces across chest. Shoulders receive force through elbows and grounded legs. Face remains partly visible behind hand. Right: release elbow tension and lower forward open palm toward chest/waist, straighten torso toward ready while keeping feet placed. Copper cloak settles. Guard must read deliberate bracing, distinct from the backward hit reaction.
Row4 crouching guard. Left: deeply bent legs and low pelvis, controlled upright torso with a compact forward forearm raised across face/chest and rear hand near ribs. Head remains above rear support and does not lunge right. Right: retain the same low pelvis, relax forward forearm downward toward knee and ease the rear hand toward waist. This returns to the approved low crouch in input2.
All eight must have complete natural two arms, two legs, hands, feet, hood and cloak, full silhouettes contained inside their own cells. Shared anatomical scale, no swelling or shrinking. No weapons, shield, magic, crystals, attack effects, opponents, injuries, blood, shadows, ground, scenery, trails, blur, borders, labels or text. Render only eight clear coherent defensive drawings on flat green.
```

Read-only approximate solid bounds: [[121, 87, 421, 463], [503, 76, 781, 465], [120, 558, 418, 829], [487, 565, 780, 833], [117, 885, 392, 1270], [510, 889, 784, 1270], [111, 1346, 382, 1617], [493, 1359, 779, 1631]]. Final calibration is recorded below.

## Initial integration

Regions use columns0/470/941, rows0/510/860/1320/1672. Anatomical height390; rootX300/690/300/690/280/670/285/685, vertical roots at lowest keyed support. Shared recoil selector now serves both bodies; final four existing stun ticks release, pending knockdowns hold impact, airborne routes retain their reviewed selectors. Original Reaction4..7 floor/support/kneel/rise remains intact. Clean cuts now include all Raya reaction and new recoil cells; same-cell history preserves single-body trails.

All170 tests (89sim81client), clippy and locked/offline release pass in raya-reaction-checks3.log. Tests now exercise all72 two-body reaction cases, four-frame release/freeze/override, all old/new Raya defensive return cells and source bounds. Initial integration assertion prevented replacing six unrelated Kogan guards; no code was written before that failure. Corrected function-scoped edit integrated, tests passed, then clippy caught one duplicate Recoil pattern; removed before clean checks3/release. No failed-build capture was launched.

Fresh final review is pending. Existing floor art is retained provisionally; acceptance depends on actual clean runtime transitions.

Fresh V1 StP-Hit and StS-Hit final clips (8 cases /20 s) fully played at1×1280×720. StS19/24/37/40/41/42/44 stepped: impact carries shoulders back, final four stun ticks show upright release, idle contains no prior body. Remaining final clips and integrated review are pending.

## Corrected runtime acceptance

All36 V1 final cases /90 s were fully played at1×1280×720. Low-guard31 exposed a repeated half-crouch entry; shared history now retains low stance after Recoil2/3/6/7 for both bodies, with the ordinary brief rise available on a standing release. Kogan legacy CrK receiver ordering now preserves Raya's low forearm/body through cape overlap. Eight corrected Raya and eight Kogan regression cases /20 s each were fully played and release phases stepped. V1 source image, extraction regions and roots are unchanged.

Original floor art is accepted: CrST84/90/96/102/108 and both corner equivalents show single prone/support/kneel/rise/idle bodies. Throw, command-grab and launch arcs are fully reviewed; command-grab release and floor return additionally stepped. Exact standing/low reaction phases and archive paths are in the full-kit report. Accepted union: seven unaffected raya-reaction-final Raya clips plus both final2 Raya CrK outcomes. All5,400 Raya,1,200 Kogan and2,100 integration trace ticks equal baseline. New35 s integration played fully; fresh smoke3 has all eight PNGs identical to inspected ground evidence, after smoke2 saved six.

Final171 tests (89 sim +82 client), clippy and locked/offline release pass in raya-reaction-checks5.log. A moved-value error in the new regression test was fixed before the clean checks and final2 capture. All complete evidence checksum archived. This is victim/floor acceptance; legacy attack defects and the rest of the whole goal remain open.
