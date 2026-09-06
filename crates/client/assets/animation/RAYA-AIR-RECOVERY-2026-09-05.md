# Raya airborne recovery — September 5

Status: V1 integrated and reviewed; publication recorded in the handoff. Published movement baseline is `373908d`. The 24 Kogan normal-juggle routes / 60 s in `raya-movement-exchange-final/Kogan-J*` were fully viewed at 1× in 1280×720 during the preceding shared-landing review. Exact JST descent/landing phases 74–78 and mirrored corner 526–528 show the old flat airborne recoil changing directly into supported compression. Broader ground/knockdown reactions remain separate work.

G2/Garou victim consequence and recognizable return, G1 gathered legs and S1 continuous cloth inform diagonal recoil → gathered descent → feet → support. Aeon keeps original stun, trajectory and two-tick landing. This is a new defensive Raya request, independent of the un-retried Kogan StP/StK/CrK rejection.

References: approved `raya-reactions-v1-green.png` and `raya-movement-v1-green.png`, both visually inspected. Built-in imagegen; retain originals without bitmap edits.

## V1 exact prompt

```text
Use case: stylized-concept.
Asset type: four high-resolution full-body defensive animation keys for Aeon's existing Raya, arranged as a clean 2 by 2 sheet on a perfectly flat bright green chroma background, with generous empty margins and gutters.
Input image 1 is Raya's approved reaction sheet: identity, costume, anatomical proportions and supported linen recovery reference. Input image 2 is her approved movement sheet: body scale, copper cloth and airborne joint continuity reference. Create four new drawings; do not reproduce either sheet layout.
Subject: the same adult female fighter, warm brown complexion, prayer-still composed face, jeweled brow and white linen hood, layered long white linen dress with loose trousers, flowing copper cloak with fine cyan-script trim, copper shoulder and wrist ornaments, blue jewels, copper strapped footwear. Same head size and anatomical scale in all four cells, side-on three-quarter camera, facing screen right throughout. Both hands empty.
Primary request: show a connected recovery from a non-knockdown airborne impact into supported landing. Defensive balance and regained support, no attack. Read left to right, top to bottom:
Top left: airborne recoil. Chest tilted backward with shoulders left of hips, knees bent and feet hanging forward/right below the hips; a diagonal body, not lying flat. Quietly opened hands show lost balance, chin slightly lifted. Copper cloak follows the backward recoil while linen remains connected around both legs.
Top right: still airborne, shoulders fold gently forward and knees gather beneath the pelvis. Head comes above the hips; hands draw toward balance. The torso begins to recover upright, and copper folds lift behind her back.
Bottom left: almost upright descent, both feet reach down below the pelvis with soft knees, hands low and slightly apart. The copper cloak lifts behind the shoulders while the linen follows the extending legs. Both soles below all cloth tips.
Bottom right: supported compact landing compression, both feet on an implied floor, knees absorb weight, torso quietly inclined toward screen right and eyes on the opponent. Hands open low for balance; copper cloth still lifted behind. Full linen hem at or above the footwear support. This will return through her existing upright landing key.
Style: match the approved painterly high-resolution fighting-game artwork and detailed copper/linen materials. Coherent joint and cloth changes, never rigid rotations or resized duplicates. Shorter poses occupy less height rather than being enlarged to fill their cells. Exactly two arms, two legs and one complete body per cell, no duplicated limbs.
Keep every cloth tip, hood, hand, finger and sole inside its cell with at least 35 pixels of green clearance. No floor, cast shadow, scenery, grid, border, label, text, trails, motion blur, impact sparks, projectile, crystal or other character.
```

## V1 source and candidate integration

Original built-in output `exec-6584229c-04c6-4e09-b7b5-e291960d1169.png`, 1254×1254; unchanged vault/runtime copy `raya-air-recovery-v1-green.png`. All four full figures inspected: diagonal recoil, gathered knees, feet-down descent and supported compression, continuous linen/copper and empty hands. No bitmap edits or generation revision.

Candidate source regions `[0,0,625,600]`, `[625,0,1254,600]`, `[0,600,625,1254]`, `[625,600,1254,1254]`; shared anatomical height480; rootX400/923/428/945; air rootY510/510/1100, final support measured by runtime. These unchanged calibrations passed final gameplay review.

Additional baseline JST paused ticks47/67/68/74/75/76 show contact, the unchanged flat late-stun body and abrupt supported compression. All prior 24 normal-juggle routes/60s were fully viewed; no duplicate before capture is required.

The first expanded test failed to compile because a struct variant was matched as a unit variant. After that correction, two existing tests caught an overly broad replacement that removed the character guard from Kogan air-attack selectors; those guards were restored. Only the defensive recovery selector is shared. A prematurely launched10s capture used the older release binary and is explicitly labeled INVALID.md; it is not candidate evidence. Fresh capture follows passing checks.

## Candidate2 gameplay review

Fresh new-release Kogan JP juggle capture: all four cases/10 s played fully at1× in1280×720, with four temporal samples. Exact paused seeks44/46/52/59/60/64/68/70/71/72/73/74/75 and mirrored corner522/523/524/525 inspected. The60 seek screenshot displayed HUD61; the other phases were corroborated by HUD labels. Complete contact silhouette, distinct gathered knees and feet-down recovery, intact cloak/hem/soles, supported compression, clean rise/idle and stable head scale survive both facings/corners. All600 trace bytes equal the published movement baseline. V1 calibration is accepted for this first route; full affected-case regression is in progress.

## Raya — airborne recovery reviewed September 5

Four new V1 defensive drawings give Raya diagonal recoil, gathered knees, feet-down descent and supported compression after a non-knockdown airborne hit. The baseline held a horizontal fall until it abruptly became a crouch. Common anatomical height and continuous linen/copper preserve identity and scale. The shared defensive selector holds recoil while rising or stun ≥4; descending final stun selects gathered knees above 24 px and lowered feet below. Adjacent history supplies compression then the existing Reaction10 in the original two landing ticks. Knockdowns, grounded remaining stun, KO and new actions retain their own presentation. Opaque cuts remove duplicate bodies and authored joints receive no extra rotation or squash. Simulation, trajectory, stun, landing duration and every combat value remain unchanged.

G2/Garou victim consequence and recognizable return, G1 gathered legs and S1/SSV Special cloth continuity become diagonal recoil → tuck → feet → supported linen body. The references establish visible principles, not copied mechanics or exact frame counts. Aeon's short recovery and original legal control remain intentional differences.

Before evidence reuses the fully played published movement regression: 16 CrHS anti-airs and 24 Kogan-normal juggles against Raya / 100 s within its 64-case / 160 s matrix, at 1× in 1280×720. Additional baseline JST seeks 47/67/68/74/75/76 confirm held flat recoil and abrupt support. Fresh final covers all 40 affected cases / 100 s, fully played at 1× in the same viewport. Eight CrHS temporal samples begin approximately 0.5 s, step 5 s; each of six normal videos has four samples beginning 0.74 s, step 2.5 s. Capture latency places moving screenshots later; exact paused phases were inspected separately.

Final CrHS steps 1246/1247/1257/1259/1260/1261/1262 and mirrored corner 1710/1711/1712 show recoil release, gathered descent, feet and both support ticks. Final JST steps 47/67/68/73/75/76/77/78 and mirrored corner 526/527/528 show contact and the complete return. Candidate2 JP also played fully through all four cases / 10 s; contact/descent/landing seeks 44/46/52/59/60/64/68/70/71/72/73/74/75 and 522/523/524/525 were inspected. Its 60 seek screenshot displayed HUD61; remaining phase labels were corroborated. Full cloth, hands/feet, floor support, stable head scale, opponent visibility and clean idle survive both facings/corners. V1 drawing and calibration passed without an art revision.

All 170 tests (89 sim + 81 client), clippy with warnings denied and locked/offline release pass in `raya-air-recovery-checks3.log`. Existing regressions now exercise all 48 two-body normal-juggle routes, adjacent defensive landing with freeze/reset/interruption, Raya clean return and source bounds. All 6,000 final and 600 candidate ticks equal the published movement baseline. The complete retained 2,100-tick integration trace also equals baseline, and the new 35 s video is byte-identical to the movement video already fully played and phase-stepped. That exact playback evidence is reused; no new integration playback is claimed. Eight fresh smoke PNGs are retained: seven match movement smoke; the changed training-boxes pair was directly inspected as frame72 versus71, with the same bodies, boxes and layout.

Complete checksum-verified archives under `notes/media/2026-09-05-full-kit/`: `raya-air-recovery-final/` has seven videos/traces/cases and 1,640 diagnostics; `raya-air-recovery-candidate2/Kogan-JP/` has its video/trace/cases and 164 diagnostics; `raya-air-recovery-polish/` has the identical 35 s video and 71 diagnostics; `raya-air-recovery-smoke/` has eight PNGs. Owned remote final/candidate diagnostics were thinned to eight only after complete checksum archival. `raya-air-recovery-verification.json` records hashes and scope. The first candidate used the older release after failed checks and is explicitly INVALID.md; it is not V1 evidence. Earlier test failures are retained: a struct-pattern compile error, then existing tests caught accidental removal of Kogan-only attack guards. Those guards were restored before passing checks and fresh capture. Subsequent source changes only indent that existing test.

This accepts Raya's non-knockdown airborne recovery and its legal landing. Her ordinary grounded hit/guard, launch/floor/getup, air-attack art and broader kit remain open. Kogan's original blocked StP/StK/CrK request remains un-retried. The whole goal continues; physical stick and competitive acceptance remain pending.
