# Animation generation — first polish pass

Built-in imagegen, reference-image generation. Source plates are unsliced. The generated green atlases are keyed once by the client at load; cells are selected by source rectangles. Existing single-pose assets remain fallbacks.

## Kogan atlas

Create a production 2D fighting-game ANIMATION SPRITE ATLAS, a perfectly regular 4-column by 4-row grid of exactly 16 full-body poses. Square 2048x2048 canvas; sixteen equal 512x512 cells. TRANSPARENT background with real alpha, absolutely no background, ground shadow, checkerboard, borders, grid lines, labels, text or numbers. Each cell is independently usable as an animation frame. Every figure faces RIGHT in the SAME fixed side/three-quarter fighting-game camera. Consistent body proportions, face, costume, scale, lighting, weapons, palette across all sixteen poses. Each cell: feet baseline at 94% height, body's grounded hip center at 50% width, standing crown at 16% height. Keep complete cape and weapon within each cell with clear margins. High-resolution painted 2D matching the references, sharp clean contours, copper matter and cyan light. NOT pixel art. No redesign, no other people. Rows read left to right in temporal sequence. Limb and cloth positions must actually change to show continuous movement, not sixteen copies. No enormous opaque effects hiding the body. Keep adjacent frames coherent and in-place; gameplay supplies world translation. KOGAN: identity reference image 1 is the hero plate; reference image 2 is his established game idle pose; image 3 shows the existing saber attack. Preserve the masculine armored copper nemes hood/cobra, cyan visor, eye chest, ornate dark armor, enormous flowing copper cape-aura, single cyan saber. Keep the ornate revolver holstered when swordwork uses his hands. Elegant precise swordsmanship that can suddenly strike with force. ROW 1: four-phase subtle forward WALK loop: lead heel extends; weight settles; trailing foot passes; opposite stride, body stays measured, saber held low in the same hand, cape follows. ROW 2: four phases of a forward CUT: compact ready/windup, striking arm extends with blade across front, follow-through lowered blade, arm returns toward guard. Specify saber tip arc centered at sword shoulder: 12 o'clock → 3 o'clock → 6 o'clock (right-facing front semicircle ')'); moving blade and hand, not only glowing effect. ROW 3: four phases of BACKCUT: low loaded guard, sweeping upward cut through the front, high follow-through, composed return. Shoulder-centered tip arc 6 → 3 → 12 o'clock, right semicircle ')', never behind body. ROW 4: four phases of THRUST finisher: saber retracted beside torso, torso and sword arm drive straight toward 3 o'clock, long extended thrust held, withdraw to guard. All feet anchored consistently; purposeful range and readable anatomy. Deliver only the transparent sprite atlas.

## Raya atlas

Create a production 2D fighting-game ANIMATION SPRITE ATLAS, a perfectly regular 4-column by 4-row grid of exactly 16 full-body poses. Square 2048x2048 canvas; sixteen equal 512x512 cells. TRANSPARENT background with real alpha, absolutely no background, ground shadow, checkerboard, borders, grid lines, labels, text or numbers. Each cell is independently usable as an animation frame. Every figure faces RIGHT in the SAME fixed side/three-quarter fighting-game camera. Consistent body proportions, face, costume, scale, lighting, weapons, palette across all sixteen poses. Each cell: feet baseline at 94% height, body's grounded hip center at 50% width, standing crown at 16% height. Keep complete cape and weapon within each cell with clear margins. High-resolution painted 2D matching the references, sharp clean contours, copper matter and cyan light. NOT pixel art. No redesign, no other people. Rows read left to right in temporal sequence. Limb and cloth positions must actually change to show continuous movement, not sixteen copies. No enormous opaque effects hiding the body. Keep adjacent frames coherent and in-place; gameplay supplies world translation. RAYA: identity reference image 1 is her congregation plate; reference image 2 is her established game idle; reference image 3 shows her existing chant attack. Preserve this exact woman, jeweled brow, white linen dress, copper cloak with cyan trim, jeweled wrists, sandals, calm beautiful face. She remains composed, graceful, fluid, beautiful, deadly, and terrifying through every action. Her weapons are voice and crystals. Never give her a sword, gun, fist-fighting stance, angry grimace, or different costume. ROW 1: four-phase subtle forward WALK loop, feet make a small measured step then pass and return, linen/cape flow fluidly with her body; right palm holds a small cyan crystal; serene carriage. ROW 2: four phases of CHANT I: composed palm by chest, precise forward open-palm extension, fully extended palm with a tiny cyan glyph at fingertips, relaxed withdrawing wrist. ROW 3: four phases of CHANT II: graceful forearm drawn across chest, outward wrist sweep, composed outward palm with small cyan written-light arc, hand floats back to guard. ROW 4: four phases of CHANT III finisher: both hands gather near sternum, extend in a decisive forward ritual gesture, a compact concentrated cyan glyph held ahead, hands separate and return to rest. Her force comes from controlled gestures, not aggression. All sixteen poses maintain same grounded hip anchor, same face and body scale, no jumps or big scene effects. Deliver only the transparent sprite atlas.

## Background extraction edit (both)

Edit target: the supplied animation atlas. Preserve EXACTLY its full canvas, all 16 character poses, grid positions, anatomy, color, scale, facial identity, weapons, and effects. Change only the empty checkerboard background: replace it everywhere between and around the sprites with a completely FLAT PURE GREEN #00FF00 chroma-key background, with no checkerboard, variation, shadows, lines, text, or transparency preview. The green is a technical key color for a game asset pipeline. Do not paint green inside the character. Keep copper, cyan, white linen and all original details intact. Repair any checkerboard slivers along the silhouette to pure green. Same square 4x4 atlas, no crop, no new art, no repositioning. Output the atlas only.

## Kogan wide thrust v2 — final selected thrust asset

Generate a game-ready 2D sprite ANIMATION atlas for KOGAN's saber THRUST, using the two provided images as identity/style references only. First reference is his established idle with exact costume, metal detail and big flowing copper aura cape. Second reference is the current game animation sheet; keep that same right-facing view and proportions. This new sheet corrects the very short saber in the old sheet: the saber must remain LONG, approximately 70% of his standing body height from guard to tip, straight cyan energized cutting edge. Keep complete body, both feet, full cape and entire long blade visible. Do not crop anything. NEW LAYOUT: wide 3:2 canvas, TWO columns by TWO rows, four equal WIDE cells. Each cell is 1.5 times wider than tall to leave room for the extended saber. No lines or labels. Every empty pixel flat PURE GREEN #00FF00 for technical keying, no shadows or gradients on background. Read left-to-right then next row. Four poses: (1) anticipation: grounded duelist coil, elbow draws the saber back next to ribs, long blade horizontal and pointing right, cape gathers; (2) active thrust: deliberate rightward full-arm extension, small forward lean, grounded wide stance, the LONG blade extends a full arm's length beyond the hand and points horizontally at opponent chest; (3) follow-through: same long blade extended, front knee absorbs the lunge and cape settles, both feet grounded; (4) recovery: draws elbow and saber back into composed guard, sword angled down toward right. Fixed hips at 45% cell width, feet baseline 92% cell height, crown 15% cell height in each of the four WIDE cells. Saber tip never beyond 94% cell width and cape never before 5%. Full blade should fit because each cell is wide. Intricate sharp metallic copper ornament, dark armor, cyan visor and chest eye, masculine athletic duelist, painterly high resolution matching original idle. No cartoon outline, no pixel art, no simplified anatomy, no gun in hands. Keep identical scale/identity/light/view across all four temporal frames. Only the four-frame green-background sprite atlas.

Output: `kogan-thrust-v2-green.png`, built-in image generation, 1536×1024. The lower-left saber extends beyond the nominal half-width; the client uses explicit source regions at x=845 to retain it intact. The original narrow-sheet thrust cells are superseded.

## Kogan thrust-margin correction

Edit this exact Kogan 4x4 green-background sprite atlas. Keep the same art, identity, all four rows and 16 equal cells, and pure flat #00FF00 background. Fix ONLY the bottom-row thrust poses in columns TWO and THREE: their saber tips currently cross the right cell boundary into the neighboring frame. Redraw those two thrust poses so each FULL saber blade ends well inside its own cell, with at least 8% of cell width of clear green margin to the right. Keep the feet, hips, and head where they are; slightly shorten the perspective foreshortened blade and bend the elbow as needed while preserving a convincing forward thrust. Absolutely no sprite or effect may cross a grid boundary. The bottom row second saber must stop before 48% of total canvas width; third saber must stop before 73% of total canvas width. Preserve upper three rows unchanged. No grid lines or text. All backgrounds pure green.

## Motion pass (2026-09-05) — cells the next generation pass should produce

The client now sequences every state through anticipation, contact and recovery (`crates/client/src/anim.rs`), crossfades between pictures, and draws afterimages, hit sparks, dust and flashes. It selects pictures per simulation frame through `SpriteSet::cell_for`. New cells drop into that selector without touching the sim. Generate in this order; each is a 4×4 green atlas in the format of the first two atlases above, same camera, same identity references, feet baseline at 94% of the cell, no effects hiding the body.

### Kogan (saber: Ukyo's stillness, Baiken's sudden weight)

| Priority | Row family | Four cells, left to right |
|---|---|---|
| 1 | Reactions | stand hit (head snaps back, saber low) · crouch hit · launched (back arched, cape flares upward) · falling to floor (nearly horizontal, feet toward camera-right) |
| 1 | Sunward cut 623 | coil with saber at 6 o'clock · rising cut 6→3 with the body leaving the ground · apex, saber at 12, cape a comet tail · descent, saber lowered |
| 2 | Jump family | prejump crouch · rise (knees tucked, cape trailing down) · apex (long line of the saber) · fall (feet reaching) |
| 2 | Lights and kicks | 5P jab (off hand) · 5K shin kick · 2P · 2K low kick |
| 3 | Cape-snare 63214 | arms open · arms close on the body · hold, opponent inside the cape · release/step back |
| 3 | Revolver and wave | draw (revolver rises, saber lowers) · fire (muzzle at chest height) · recoil · reholster; wave: saber raised overhead · overhead cut releasing the wave · follow-through · return |
| 4 | Guard, feint, win | disc bloom kneel · disc held · feint (half-cut, weight settling back) · win (saber planted, hood raised) |

### Raya (composed; the force is in the gesture, never the grimace)

| Priority | Row family | Four cells, left to right |
|---|---|---|
| 1 | Reactions | stand hit (turns from the blow, hand to brow) · crouch hit · launched (linen and cloak rising above her) · falling (horizontal, arms open) |
| 1 | Ascension 623 | gather at the sternum · rise with one palm lifting a crystal column · apex, column fully written · descent, hands folding |
| 2 | Footsies | 5P palm · 2K low sweep of the sandal · 5S written arc · 2HS crystal rising from the floor |
| 2 | Jump family | prejump · rise (cloak trailing) · apex · fall |
| 3 | The rite 63214 | chains written from the wrist · chains bind the body · the rite spoken (head bowed) · release |
| 3 | Glyph and crystal | glyph: palm opening · glyph placed · hold · withdraw; crystal: underhand toss · release · hands settle · rest |
| 4 | Consecrate, feint, win | kneel with crystal at the brow · fill (light grows) · feint (gesture stopped mid-air) · win (crystal held out, the offering) |

Row layout convention for the selector: row 0 = the four phases used for a single move, in temporal order; a family of four different moves uses one row per move. Foot anchors are measured per cell as with the first atlases; record them beside the asset.

## Reaction iteration (2026-09-05)

The priority-one reaction/uppercut families above now have an initial authored implementation, with floor/getup and landing cells. See [full prompts, corrections and selection](REACTIONS-2026-09-05.md). Five selected PNG assets provide 32 consumed drawings. Most of the remaining family list is still open; one-cell grounded recoil and a few uppercut keys do not complete full-kit animation.


## Saber refinement (2026-09-05)

Four short high-poke drawings and two compact uppercut phases are integrated and reviewed. Exact prompts, original IDs and acceptance: [standing poke](KOGAN-STANDING-POKE-2026-09-05.md) and [compact reversal](KOGAN-UPPERCUT-COMPACT-2026-09-05.md). The existing cut/backcut regions were measured again to preserve complete capes. Full 128-case motion review and remaining coverage are recorded in the full-kit report.

## Disc-shield (2026-09-05)

Four reviewed Kogan phases use open copper linework and shared standing anatomy. Exact creation/edit prompts, originals, blade correction and measurements: [disc provenance](KOGAN-DISC-2026-09-05.md). The full 20-case review and remaining coverage are in the full-kit report.


## Ground movement (2026-09-05)

Eight reviewed Kogan V4 drawings cover cloth-changing glide, half/low crouch and four retreat phases. Existing utility drawings supply brief run transitions; existing walk drawings have measured green-gap regions. Exact creation/edit prompts, originals, rejected variants and roots: [ground provenance](KOGAN-GROUND-2026-09-05.md). Full 36-case review, immediate-exit validation and reference comparisons are in the full-kit report.


## Kogan recoil and floor recovery — reviewed

Selected `kogan-recoil-v2-green.png` (8 drawings) and `kogan-floor-v1-green.png` (4 drawings). Exact built-in imagegen prompts, original IDs, corrections and measured roots/scales: [recoil](KOGAN-RECOIL-2026-09-05.md), [floor](KOGAN-FLOOR-2026-09-05.md). V1 recoil remains in the vault as rejected source; only V2 is loaded. Full 36-case/90s final and 35s integration playback reviewed at 1×, with impact/release/corner-getup frame steps. 131 tests pass and full traces are unchanged.

## Kogan Judgment — reviewed

Four V3 gather/dual-rush/withdrawal/reholster drawings. [Exact prompts and corrections](KOGAN-JUDGMENT-2026-09-05.md) retain the V1 dangling gun, rejected V2 grip error, V3 source and final root measurements. Complete corrected 16-case/40s and 35s integration playback reviewed at 1×; 133 tests pass and traces remain unchanged.

Airborne saber: six selected V2 drawings and the rejected V1 gather blade are documented in [KOGAN-AIR-SABER-2026-09-05.md](KOGAN-AIR-SABER-2026-09-05.md). Complete prompts, source IDs, measured roots and final gameplay acceptance are retained.

Crouching saber: sixteen selected V1/V3 drawings cover CrS/CrHS/CrFL/CrST. [Exact prompts, source corrections and roots](KOGAN-CROUCH-SABER-2026-09-05.md) retain both rejected low candidates. All 80 cases / 140s and full integration reviewed; original combat timing retained.

Overheads: four standing drawings plus reviewed falling saber/landing reuse. [Exact prompt, source and measured regions](KOGAN-OVERHEAD-2026-09-05.md). All 40 final cases / 80s and complete integration reviewed; original combat timings retained.

Normal throw and tech: reviewed cape-step reuse plus two new supported disengagement drawings. [Exact tech prompt, original and regions](KOGAN-THROW-2026-09-05.md). All 32 final cases / 80s and full integration reviewed; original timing retained.

Feints reuse reviewed equipment withdrawal and ready drawings. [Exact reused cells and acceptance](KOGAN-FEINT-2026-09-05.md). No new bitmap generated; all 88 final cases and integration reviewed at original timing.

Kogan victory V1: four complete draw/raise/settle/hold drawings. [Exact prompt, original, regions and acceptance](KOGAN-VICTORY-2026-09-05.md). Real KO, supported entry and reset reviewed at original timing.

Both-body KO reuses existing recoil, airborne and floor/support drawings with no new generation. [Exact reuse and review](KO-CONTINUITY-2026-09-05.md). Grounded collapse, actual landing, sustained floor and reset reviewed without simulation changes.

Kogan crouching punch uses four V1 full-body drawings: [exact prompt and review](KOGAN-CROUCH-PUNCH-2026-09-05.md). Independent CrP request; original blocked standing-light/kick batch remains un-retried.

Kogan non-knockdown airborne recovery uses four V1 defensive drawings: [exact prompt and review](KOGAN-AIR-RECOVERY-2026-09-05.md).

Raya standalone movement uses eight V1 preparation/hop/full-jump/landing drawings: [exact prompt, source, calibration and review](RAYA-MOVEMENT-2026-09-05.md).

Raya airborne non-knockdown recovery uses four V1 defensive drawings: [exact prompt, source, calibration and review](RAYA-AIR-RECOVERY-2026-09-05.md).

Raya ground movement: eight V1 glide/crouch/retreat drawings, measured original walk cells and full36-case review. Exact prompt/source/calibration: [RAYA-GROUND-2026-09-05.md](RAYA-GROUND-2026-09-05.md).

Raya grounded recoil V1: eight defensive impact/release drawings, clean retained floor recovery and corrected low return. [Exact prompt, source, calibration and review](RAYA-RECOIL-2026-09-05.md).

Raya standing palm/low kick: seven V1 cells plus corrected V2 palm contact, complete40-case final2 and integration review. [Exact prompts, source and calibration](RAYA-STANDING-LIGHTS-2026-09-05.md).

Raya crouching palm and ankle kick: seven V1 cells plus corrected V2 kick contact; complete40-case accepted union and integration review. [Exact prompts, source and calibration](RAYA-CROUCH-LIGHTS-2026-09-05.md).

Raya standing Flash/Style: seven V1 cells plus V3 curled cape contact, complete 40-case final2 and integration verification. [Exact prompts, source and calibration](RAYA-FLASH-STYLE-2026-09-05.md).

Raya signature normals: ten V1 drawings and two V4 contacts; complete60-case final2 and integration verification. [Exact prompts, source and calibration](RAYA-SIGNATURE-2026-09-06.md).

Raya chants: four medium-palm phases reused plus eight new V2/V1 drawings;60-case final3 and integration verified. [Exact prompts, source and calibration](RAYA-CHANTS-2026-09-06.md).

Raya Ascension: retained gather/descent/landing with two compact V2 phases, low active crystal and folded hands below timer. [Prompts and calibration](RAYA-ASCENSION-2026-09-06.md).

Raya crouching normals: sixteen V1 phases for horizontal/vertical crystals, low Flash and supported sweep. [Prompts and calibration](RAYA-CROUCH-NORMALS-2026-09-06.md).

Raya airborne JP/JK/JFL: six V1 gathered/low-contact/fold/ready drawings. [Exact prompt and review](RAYA-AIR-LIGHTS-2026-09-06.md).

Raya airborne JS/JHS/JST: three V1 low crystal contacts plus approved air-light gather/fold/ready. [Exact prompt and review](RAYA-AIR-CRYSTALS-2026-09-06.md).

Raya CrHS: one longer vertical-crystal contact with normalized body scale and visor-clearing root. [Exact prompt and review](RAYA-AIR-EXCHANGES-2026-09-06.md).

Raya overhead: six drawn phases with downward contact and composed return at unchanged timing. [Exact prompt and review](RAYA-OVERHEAD-2026-09-06.md).

Raya Rite and Processional: eight deliberate grounded phases with contained contact and supported glide/return. [Exact prompt and review](RAYA-UTILITY-2026-09-06.md).

Raya normal throw/escape: new empty-hand reach and approved utility/recoil phases. [Exact prompt and review](RAYA-THROW-2026-09-06.md).

Raya ranged and EX use eight V2 low-crystal/spoken-glyph phases. V1 retained as rejected calibration. [Exact prompts and review](RAYA-RANGED-2026-09-06.md).

Raya consecrate and manual shatter use eight V1 supported knee/rise and empty-command phases. [Exact prompt and review](RAYA-RITUAL-2026-09-06.md).

Raya Convergence uses four V1 gather,expanded orbit,dismissal and ready phases. [Exact prompt and review](RAYA-CONVERGENCE-2026-09-06.md).

Raya feints reuse approved withdrawal/ready drawings,with supported Charge rise and airborne descent. [Exact reuse and review](RAYA-FEINT-2026-09-06.md). No new generation.

Raya victory:four V1 gather/cup/lift/settle drawings. [Exact prompt,source and review](RAYA-VICTORY-2026-09-06.md).

Kogan jab:approved Flash gathered fist and ready bracket the original complete contact. [Reuse and review](KOGAN-JAB-2026-09-06.md). No new generation.


# Kogan walking finish — September 6

Status: V1 accepted after runtime review (ART59). Built-in Image Generation skill/tool. Both reference sheets inspected directly before generation. Existing atlas top-row pose sequence is the movement reference; Flash v2 second-row-right ready is the identity/material reference. New quiet stance is published in ART58. Full forward/backward baseline captures are being retained. No blocked lights request is retried.

Runtime acceptance must retain current walk timing and sim, inspect all four placements/facings, foot support, complete saber/cape and entry/return at normal speed and selected phase steps. Broader legacy attack finish and StK/CrK stay open.

## Exact prompt

Create a production animation sheet for Aeon, the existing high-resolution 2D fighting game. This is Kogan WALKING only. Reference image 1 is the existing atlas: use ONLY its TOP ROW's four ordinary grounded walking poses as the pose and foot-placement sequence. Ignore all lower rows. Reference image 2 is the approved current material/identity style: especially the SECOND ROW RIGHT ready stance. Produce four full-body walking drawings in a spacious 2x2 grid, reading left to right then top to bottom. Preserve the exact supported leg arrangements and order from reference 1 top row: broad planted start, front foot reaching forward with toe up and rear foot planted, rear foot passing/lifting as front foot supports, then planted return. These are restrained walking steps, no running, no lunging and no attack gesture. Match reference 2's dark near-black engraved armor, broad clean copper lines, copper Nemes hood, narrow cyan visor, blue/cyan chest eye, large copper ribbon-curl aura cape, and complete cyan plasma saber held down and forward in the anatomical right hand. Keep the left hand quietly near the body. The cape is large flowing visual aura with open gaps, never a literal solid coat or hair. Match its darker copper and segmented broad highlights from reference 2 rather than bright yellow polish. Same anatomy, head size, costume construction and anatomical height in all four cells, same side-facing right orientation and stable support-foot ground level. Draw the changing knee/ankle support and subtle cape follow-through, with consistent blade length and full tip. Give each cell ample clear margins around cape, saber and boots; no clipping or crossing into adjacent cells. Clean high-resolution drawn game-sprite finish, no pixel art, no photoreal texturing, no blur, no labels, no grid lines, no shadows or scenery. Flat saturated chroma green background, including open gaps, for the established sprite-key pipeline. Four drawings only. Preserve Kogan's established identity; no new character or equipment.


## V1 source review

Generated original: `exec-c99d4120-becc-4d3e-a740-4fdcf4c8ae42.png`; archived as `kogan-walk-style-v1-green.png` (1254×1254). Directly inspected: four right-facing supported walk poses, complete sabers/boots and separate aura curls. Broader dark armor and cleaner lines reduce fine texture, but copper is still bright and cape strands differ from Flash ready. Runtime style/scale acceptance remains pending. This is not an accepted sheet yet.


## Runtime acceptance

V1 accepted after complete before/final forward/backward 12s each at 1×, all four footfall drawings, direct ready return and mirrored corner inspection. Full 35s integration and its four changed diagnostics inspected; eight smoke PNGs equal ART58. All 185 tests, clippy/release pass; 720/2,100 simulation ticks unchanged. Anatomical height 505; source regions [0,0,627,600], [627,0,1254,600], [0,600,627,1254], [627,600,1254,1254], projected x roots 318/931/330/930. Read-only source bounds are 48,39–515,542; 659,39–1133,543; 64,633–534,1143; 653,634–1135,1144. Larger source removes old soft detail; brighter moving copper strands remain intentional. This does not close legacy contact finish or the full goal.

## ART61 — first-cut finish V1

Built-in imagegen; one four-cell1254×1254 source. Original ID `exec-c712ef18-fe92-417e-930a-c307b6da10db.png`. Output `kogan-first-cut-style-v1-green.png`, SHA256 `1cf298cbfae043a143e4713809e56b63d09b63b8cd6036bc045f47a98c16851d`. Input roles: original Kogan atlas second row for choreography; Flash V2 quiet ready for finish. Accepted after complete sword/feint and integration review; all simulation timing retained. Remaining cape/backcut/thrust/jab style polish is explicit.

Exact prompt:

```text
Edit the approved Kogan animation drawings for our original high-resolution 2D fighter Aeon. Output ONE square sprite sheet, precisely FOUR complete full-body drawings in a roomy 2 by 2 grid, read left to right then top to bottom, each cell on flat vivid green (#00ff00), with generous empty green gutters and no labels or scene. This is a production redraw of an existing sword-cut sequence, preserving its choreography.

INPUT ROLES: Image 1, kogan-v1-green.png, is the movement/pose edit target: use ONLY its SECOND ROW of four drawings as the four successive poses, retaining their weight support, arm direction and saber orientations. Do not include its walking, backcut or thrust rows. Image 2, kogan-flash-v2-green.png, is the finish, body-proportion, equipment and ready-pose reference; use the second-row-right quiet front-saber ready pose as the style anchor, not the pommel or thrust moves.

Kogan is the same male adult under a copper nemes hood with cobra brow, narrow cyan visor, near-black armor with broad legible copper circuit edges, cyan chest eye, boots, holstered revolver and ONE long cyan plasma saber held in his RIGHT hand. His large copper hood-cape is visual aura with curling segmented metal ribbons behind his body. Preserve the character exactly and maintain the same helmet/torso/limb proportions and scale across all four cells. Match the current reference's dark armor, firm edges and broad highlights; avoid tiny noisy gold engraving, blurry gradients, yellow-green rim coloring or glow on the copper. Cyan stays confined to the visor, chest writing and saber.

The four cells retain the original second-row sequence: (1) Supported preparation with bent knees, right sword hand drawn back beside the right hip and the complete saber pointing diagonally down behind toward screen left; left empty hand gathered. (2) Committed forward/downward sword cut: trunk turns toward screen right and the right arm extends at lower chest/waist height; the saber sweeps in the FRONT of the body toward low screen right. Keep the original compact cyan curved blade-path accent rather than a giant effect. (3) Low forward follow-through: right hand travels down across the front, full saber angled toward the ground ahead, bent legs support the body; show a thinner finishing arc near the blade, with no cape or weapon below the soles. (4) Withdrawal finished in the familiar complete front-diagonal saber ready position, right hand low by the right hip and the relaxed left hand gathered, feet still supported, copper aura settling behind.

The character always faces screen right in three-quarter side view, side-on game camera, with full helmet, both boots, both hands, saber hilt and saber tip, holstered revolver and all cape curls visible inside every cell. No cropping, no duplicate arms or equipment, no overlapping cells, no ground shadow, no floor, no target, no opponent, no decorative frame, no redesign, no text. Keep the same physical body scale and supported baselines in all four drawings; let the arm, shoulder, knee and cape folds carry the action. Aim for individual anatomical standing height around 500 pixels on a 1254-pixel-square sheet.
```
