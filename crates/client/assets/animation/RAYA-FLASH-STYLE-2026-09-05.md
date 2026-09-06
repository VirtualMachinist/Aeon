# Raya standing Flash and Style

Status: seven V1 drawings and V3 cape contact accepted in focused gameplay; integration verified. ART36 passes. Runtime baseline1378148.

Scope: StFL7/3/13 (Mid,80 damage,20 hitstun,14 blockstun; box8/34/52/24), StST10/3/16 (Mid,100 damage,22 hitstun,15 blockstun; box12/44/58/22). Simulation values remain unchanged. Existing fl.png and st.png inspected: high palm with baked cyan ring, and torso/cape turn with crystal hem. Their distinct preparation/recovery is unreviewed.

Benchmark lens: G2 Garou outbound/arm fold, S2 SSVSpecial committed contact/recognized return, A1 original AccentCore supported equipment pivot/rest and A2 body/effect visibility. These inspected excerpts may be reused; final comparison must record observed Aeon defects and deliberate differences.

## Baseline gameplay review

All40 before cases /40 s played completely at1×1280×720. Flash12/19/30/34/40/44/45/46 shows a high ring far above its waist contact flash, held extension through recovery and duplicate body at idle. Style13/22/34/38/44/49/50/52, low guard502 and miss982 shows a held cape pivot throughout recovery and ghosted idle. Contact hem is near its actual flash, but the forward cloth hides crouched Kogan's face. New drawings will lower Flash and give the cape supported gather/contact/withdrawal/settle, with forward bulk below chest level.

## Exact V1 prompt

```text
Use case: stylized-concept
Asset type: eight original full-body animation drawings for Aeon, a high-resolution 2D fighting game.
Create one portrait sprite sheet, two columns by four rows, with eight separate complete drawings of the same adult woman Raya. All face screen right in side/three-quarter view. Uniform solid bright chroma green background, generous space around every hand, sandal, cape and small effect. No ground plane, shadows, grid, writing, labels or opponents.
Input image 1 is the approved standing identity, anatomy and costume reference. Input image 2 is the old Flash palm/ring gesture reference; its palm is TOO HIGH, lower the new contact to hip height. Input image 3 is the old Style cape-pivot reference; use its copper cloth and narrow cyan ornament, but draw new supported phases and keep the forward cape below the chest so an opponent's face remains visible.
Preserve the same brown face, composed expression, white hood, jeweled brow, copper shoulders and large copper cloak with narrow cyan ornament, layered white linen robe over loose trousers, cyan jewelry and copper strapped sandals. Rich painted game rendering with crisp anatomy, no pixel art. Identical adult head, torso and limb proportions and standing height in all cells. Feet remain grounded in every drawing, at least one full sole supporting the body. This is a quiet, controlled martial ritual, no jump or full spin.
Read left to right, top to bottom:
0. Flash preparation. Upright supported stance, front knee softly bent. Lead elbow tucked low by the waist, open palm turned toward the opponent at hip level, other hand near sternum. A tiny thin cyan glyph begins at the low palm, no larger than the palm.
1. Flash contact. A short downward-forward palm press, lead upper arm angles down from shoulder and elbow stays below the waist. Complete vertical palm at hip/upper-thigh height, approximately 45 percent of total body height above the soles. A small thin cyan glyph ring centered at the palm heel, diameter about one head, stays on that LOW contact line. Keep the torso near upright and both feet planted. Other hand remains near sternum. Do not aim the palm at head or chest height.
2. Flash withdrawal. Fold the low elbow back toward the waist, open hand now close to the body and turning upward, with only a faint short cyan stroke. Shoulder weight settles over rear support; cape follows with a small late fold.
3. Flash ready. Familiar near-upright stance, lead forearm folded before lower ribs, other hand relaxed near chest, full feet planted and copper cape resting behind. No active ring or strike effect.
4. Style preparation. Feet in the same narrow staggered base. Slight torso windup toward the viewer, rear shoulder turns back while one hand gathers a fold of copper cape near the rear hip. Other elbow gathered near ribs, face still calmly looks screen right. Cape stays mostly behind, linen hangs vertically.
5. Style contact. Turn the hips and shoulders back toward the opponent with the front foot planted and rear heel pivoting. Sweep the copper cape in a curved band across the LOWER torso at waist/hip height, its forward edge extending about a forearm length ahead of the torso. Keep the cape's forward bulk BELOW the chest and well below Raya's face. One hand leads the cloth at hip level, other arm counterbalances behind. A narrow line of small cyan crystal facets accents the moving hem; no detached projectile or giant energy arc. Show the grounded pivot and complete feet beneath the cloth.
6. Style withdrawal. The torso unwinds toward the familiar right-facing stance; bring the leading hand back toward ribs. The cape falls diagonally behind the rear hip, its lower hem still curling from the completed turn. The rear heel lowers to restore full support. No active crystal sweep; a few faint cyan marks remain on the cloth ornament.
7. Style ready. Familiar upright right-facing stance restored, lead hand gathered by lower ribs, other hand near chest, both feet planted and the same full copper cape settling behind. Full white linen outline returns. No active effects.
Make real shoulder, elbow, hip and cloth changes; every contact has a visibly different folded return. Keep hands anatomically complete and all feet and flowing cloth within their separate cells. No crystal held above the idle palm, weapons, new equipment, duplicates, ghosts, ground shadows or text.
```

V1 original: exec-2abdef84-02ee-4dc3-aa58-6cc7250c1a2b.png, copied unchanged as raya-flash-style-v1-green.png (1024×1536). All eight source figures inspected: lower glyph palm, upturned release, prayer ready; wider supported cape gather, hip-level held cloth/contact, backward falling cloth and planted ready. Complete hands, sandals, hood/face and costume. Contact cape may need reach refinement in gameplay; source edge remains within its cell.

Initial regions: [0,0,500,390],[500,0,1024,390],[0,390,500,745],[500,390,1024,745],[0,745,500,1105],[500,745,1024,1105],[0,1105,500,1536],[500,1105,1024,1536]. Anatomical height342, roots270/715/270/715/280/715/270/715. Runtime review pending.

## V1 runtime review and refinement

All40 V1 cases /40 s played fully at1×1280×720. Flash12/13/19/20/30/32/36/40/43/44/45/46 shows distinct downward contact/upturned release/prayer ready and clean single idle, but its body is shifted rearward from idle. Style12/13/22/23/34/37/42/46/50/51/52/54, mirrored corner202, low guard503 and miss983/987/993/1001 shows a supported pivot, falling cloth and complete return; crouched Kogan's face is now visible. Its forward hem still overreaches the contact flash. Correct root placement for both families; slightly increase the last two anatomical heights to match their taller source drawing. Select only V2 contact5 if accepted, retaining seven unchanged V1 cells.

## Exact V2 targeted edit

```text
Use case: precise-object-edit
Edit the supplied Raya animation sheet. Change ONLY the forward overhanging copper cape hem in the THIRD ROW, RIGHT COLUMN figure, the sixth drawing. It currently extends too far beyond her leading hand to the right edge near x930. Curl and gather that same copper cloth inward so its complete forward cyan-edged tip ends near x845–850 on this 1024-pixel-wide sheet, just beyond the hand. Preserve the cape's waist/hip contact height, attachment to the shoulder, copper material, narrow cyan ornament and small crystal facets along its hem. The cloth remains a curved sweep across the lower torso with a shorter overhang, not a smaller costume.
Keep that figure's exact face, hood, torso, shoulders, arms, hands, grounded feet, linen, scale and position. Do not shorten or deform the body. Preserve all seven other drawings, every cell position, sheet dimensions and solid green background. No extra effects, objects, text, shadows or ghosts. Complete cape tip and crystal facets stay inside their cell.
```

V2 original exec-eb176c7c-c131-4411-85aa-0d51b7c543b8.png retained unchanged as raya-flash-style-v2-green.png. Source inspection: forward edge only shortened from about930 to895, still beyond requested845–850. It is not integrated or accepted. V3 requests a clear inward curl directly beside the leading hand; all body geometry and seven other drawings retained.

## Exact V3 targeted edit

```text
Use case: precise-object-edit
Target only the sixth figure, THIRD ROW RIGHT, of this Raya sprite sheet. The small previous edit still leaves a long cape extension. Fold the entire forward flap back inward toward the leading hand, making the outer right edge of her copper cape terminate immediately beside that hand, no farther right than x840–845 on this 1024-wide sheet. The hand is around x815. There should be only 25–30 pixels of cloth to its right, not the current 75–80. Turn the cyan-edged tip inward/downward into a curled fold beside the hand. Preserve the visible copper sweep from her shoulder across her waist, its hip-height cyan patterned hem and little crystal facets. Do not cut off the cloth: show its complete naturally curled tip.
Every part of her body, arms and hand positions, legs and feet, face, hood, linen, overall height and location is fixed. Do not alter the other seven drawings or their positions. Keep all source dimensions, solid green background and cell clearances unchanged. Only reshape this single forward cloth flap and its ornament; no new effects, text or objects.
```

V3 original exec-faf436ec-777b-4d4b-b06d-7a449d0df7b0.png retained unchanged as raya-flash-style-v3-green.png (1024×1536). Only contact5 selected: complete inward-curled hem beside the grasping hand, full grounded body, face/hood/linen retained. Seven cells remain V1. Corrected roots240/685/240/685/250/685/240/685; height342 for0–5,347 for6–7 to match their source anatomy. Regions unchanged. Corrected gameplay accepted below.

## Corrected focused gameplay accepted

All 40 final2 cases / 40 s played completely at 1× in a 1280×720 viewport. Flash ticks 12/19/20/31/39/43/44/45, mirrored corner201, low guard501 and miss981/990/1001 were stepped. The quiet low palm and small glyph now sit on the lower contact region; the arm turns upward, folds to prayer ready, then returns to a single idle body. The corrected root removes the pronounced backward offset and forward snap. The small authored glyph precedes the separate simulation contact flash; no exact pixel coincidence is claimed.

Style ticks13/23/34/38/46/50/51/52, mirrored corner203, low guard503 and miss983/990/1002 were stepped. V3's complete curled hem meets the contact flash without the V1 overhang. The planted pivot changes to falling backward cloth and restored linen, then prayer ready and one idle body. Crouched Kogan's face stays visible above the cape. Full sandals, hands, hood and cloth remain inside the viewport in both facings/corners. Both Mid moves hit standing/crouched bodies and block against either guard; all 2,400 ticks match the baseline byte for byte. Video seeks can display an adjacent HUD tick.

G2/S2's distinct outward action and recognized withdrawal become a low ritual palm followed by a folded return. The gesture deliberately differs from the reference high strike/slash. A1's supported equipment pivot/rest becomes grounded cloth rotation and settling; A2's body/effect hierarchy becomes the lowered, shortened cape band that preserves the receiver's face. No copied move, exact reference timing or commercial parity is claimed.

Integration: all 2,100 ticks equal crouching-light baseline, and the 35 s MP4 is byte-identical to its complete 1× review. Seven smoke hashes match; changed winner pair directly inspected with only one-pixel-wide text-edge differences. 171 tests, clippy and locked/offline release pass. Complete accepted archives checksum verified; ART36 passes.
