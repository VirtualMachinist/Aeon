# Kogan overheads — reviewed

Standing overhead baseline: all20 cases/30s reviewed at1× in1280×720; miss24/35/43/53/54 stepped. The same raised rear blade persists through commitment and recovery; body scale shrinks under raised arms, and the source hides the receiver at close range. S2 committed blade/withdrawal and A1 recognizable equipment rest motivate a real forward-downward cut, supported return and coherent scale. Built-in imagegen V1 generated, from the approved standing poke sheet. Falling-saber reuse of existing air-saber and landing drawings is reviewed below. No simulation changes.

## Exact standing V1 prompt

```text
Use case: stylized-concept.
Asset type: production animation atlas for Aeon's existing adult armored duelist Kogan.
Input image: the approved standing saber sheet is an IDENTITY, ANATOMY, EQUIPMENT and PAINTING-STYLE REFERENCE. Create a new action, retaining its copper nemes and cobra, cyan horizontal visor, dark engraved armor, chest eye, ornate holstered revolver, long narrow cyan plasma saber and very large flowing copper cape. High-resolution painted 2D game art, side-on three-quarter game camera, all bodies facing RIGHT.

Draw FOUR complete full-body drawings in chronological order, exactly TWO COLUMNS by TWO ROWS on a square 1024x1024 canvas. Same adult head size and body proportions in every cell; head-to-boot standing anatomy approximately 350 pixels high. Raised hands and weapon may occupy extra space above the head. Do not shrink the body to fit a raised weapon. Both boots stay planted at the same baseline within each row. Leave wide uniform green gutters around every complete silhouette, including all saber tips and cape tails.

Action: a deliberate STANDING OVERHEAD SABER CUT, then visible withdrawal and return to the familiar front ready. The body stays grounded. The right hand grips the one intact copper saber hilt; during preparation/contact the left hand can support the right wrist just below the grip, with two distinct hands and natural elbows. Never grip the blade.

1 TOP LEFT — PREPARATION. Upright grounded duelist, knees softly bent, weight gathered slightly over rear boot. Raise the right-hand hilt just above and slightly in front of the forehead, elbows bent; left hand supports the wrist. The complete long blade points back and slightly UP toward the LEFT, clear above the cape. The visor still looks right and remains visible. This is a held, loaded position before a downward cut.
2 TOP RIGHT — CONTACT. Shift the torso and weight toward the front planted boot, with a shallow forward bend and bent front knee. The hands have now driven DOWN AND FORWARD to LOWER-CHEST height, right elbow opening naturally. From that forward hilt the complete straight blade points DOWN-FORWARD RIGHT at approximately 30 degrees below horizontal, extending well beyond the front knee. Blade starts around lower-chest height and its tip ends at approximately shin height, clearly ABOVE the boot baseline. This must be visibly different from the raised preparation: the blade and arms now occupy the space IN FRONT of the body. Large cape trails left after the shoulders.
3 BOTTOM LEFT — WITHDRAWAL. Bring the right hilt back beside the waist by bending the elbow, left hand releases toward the ribs. Body straightens and weight returns between both boots. Keep the complete blade pointing shallowly down-forward-right, fully above the boot line. Hands are visibly farther back than contact, shoulders recovering. The large cape tips still travel left as the arm returns.
4 BOTTOM RIGHT — READY. Familiar relaxed upright side-on ready stance matching the reference's bottom-right drawing: right hilt beside front hip, full saber down-forward-right with tip above the boot line, left hand relaxed near the body. Both knees soft and boots firmly planted. Cape settles broadly behind. Full adult anatomy and equipment remain unchanged.

Flat technical chroma green #00e600 background. No floor, shadow, opponent, lettering, grid, boxes, borders or extra objects. No crescents, slash effects, sparks, blur, ghosts or duplicate limbs. One complete narrow straight cyan blade in each cell; generous empty green beyond its tip. Preserve the copper cape as a large flowing silhouette separate from face, hands and saber.

```

## Source and first integration

Built-in output `exec-c345c5fe-8f52-4514-8d7a-146efe53a444.png` retained at its original generated_images location and copied unchanged to `kogan-overhead-v1-green.png` here. Actual canvas1254×1254. Fullsheet inspected: four complete silhouettes/blades, distinct high gather/front cut/two returns, coherent equipment. Measured runtime regions `[0,0,520,625]`, `[520,0,1254,625]`, `[0,625,615,1254]`, `[615,625,1254,1254]`; roots330/900/315/905, common unfolded anatomy450. Bottom gutter is x610–620; x625 would cut the right cape. Existing alpha-boundary regression passes the chosen615 cut.

Falling baseline: all20 cases/50s reviewed at1×. Miss18/24/31/33/34/36/38/40/41/42 and standing-guard/crouched-hit31/38/44/52 stepped. Rear blade never reaches the front at active contact; old pose/trails overlap the receiver. The authored trajectory lands on miss tick34, during the nominal active span, then shows eight landing ticks and returns on42. Contact freeze delays those transitions; preserve all timings. Candidate reuses reviewed air-saber V2 gather0/contact3 and existing reaction8–11 for four two-tick landing phases. Existing forced front-blade hold is restricted to2f full-jump landings; the8f special uses its full landing sequence. Stale last_move does not select an air attack in a later jump.

First integration passes149 tests(89sim60client), clippy with warnings denied and locked/offline release. Focused playback acceptance is recorded below. No simulation or move-data edits.

## Focused candidate review

All4 focused clips/480ticks/8s viewed completely at1×,1280×720. Standing crouched-hit24/35/43/46/54/64 and miss35/38/45/53/54 stepped. The high gather now cuts in front at35, contact holds through authored hitstop, and withdrawal visibly straightens the body before the familiar stance. Complete blade above floor; planted boots/common body scale survive all four drawings. Falling crouched-hit31/38/43/44/46/48/50/51/52 and miss24/31/33/34/36/38/40/42 stepped. The steep blade stays just above floor on final air tick33; landing immediately replaces it on34. Four supported landing phases fit the existing8ticks. In contact, Kogan's compact torso stays above Raya's crouched head and the front weapon clearly traverses the contact region; no rear-blade ghost persists. Existing Raya floor/getup blend is a separate open family.

These focused candidates are accepted for full40-case validation. All480 trace ticks equal the baseline excerpts. Generated source is unchanged; no further image edits were needed.

## Final validation

All40 final cases/80s at1× reviewed, plus standing guard35 in all four positions and left-corner50/61; falling guard31 in all positions and left-corner39/44; defeated low guard left-corner31/43/44/50/52/125. All4800 final and480 focused ticks equal baseline; all35s retained integration viewed and2100ticks equal.149 tests, clean clippy and release pass. Accepted repeat smoke has all8 PNGs byte-identical to previously inspected crouching-saber smoke. Incomplete first6-PNG smoke retained separately. Exact report and evidence paths: full-kit implementation note. This accepts these two actions; the whole goal remains active.
