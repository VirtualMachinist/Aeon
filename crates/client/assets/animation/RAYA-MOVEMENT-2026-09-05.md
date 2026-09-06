# Raya movement — September 5

Status: V1 movement and shared landing regression accepted; publication recorded in the handoff. Existing full-jump body shrinks and takeoff/landing crossfades duplicate the body. All 24 baseline cases /24 s reviewed at 1× in 1280×720; phases 12/13/14/17/27/35/36 and 737/754/769/770/771/772 stepped. Archive: `notes/media/2026-09-05-full-kit/raya-movement-before/Raya-movement/` (1,440 ticks, 600 diagnostics).

References: approved `raya/idle.png` and `raya-reactions-v1-green.png`, both inspected. G1/K1 inform gathered movement and body mechanics; S1 informs cloth descent. Built-in imagegen; retain original, no bitmap edits.

## V1 exact prompt

```text
Use case: stylized-concept.
Asset type: eight high-resolution full-body movement animation keys for Aeon's existing Raya, exactly four columns and two rows on a perfectly flat bright green chroma background. Wide landscape sheet; generous empty gutters and margins.
Input image 1 is the approved Raya idle pose, an identity, costume, proportions and painterly rendering reference. Input image 2 is the approved Raya reaction sheet, reference for continuous linen, cloak, body scale and supported landing. Create eight new movement drawings; do not reproduce the reference sheets.
Subject: the same adult female fighter, warm brown complexion, composed prayer-still face, jeweled brow under a white linen hood, long white linen layered dress and loose trousers, flowing copper cloak with fine cyan-script trim, copper shoulder and wrist ornaments, blue jewels, copper strapped footwear. All eight face screen right in the same side-on three-quarter camera. Same head size and anatomical scale in every cell: crouching and tucked poses occupy less height, never enlarge them to fill their cells. Preserve full cloth, hands and feet, two arms and two legs.
Read order left to right, top to bottom:
1. Grounded prejump compression: both feet supported, knees bend, pelvis lowers, torso gently inclines, hands draw quietly near waist/chest. Cloak gathers behind her, hem stays on or above the soles. This prepares either hop or full jump.
2. Hop rising: compact tuck, knees drawn toward torso and both feet gathered below them, elbows tucked, calm gaze forward. Copper cloth begins lifting behind the shoulders.
3. Hop apex: compact but distinct gathered silhouette, knees closest to torso, head steady, elbows relax slightly; cloak rises and rolls behind the back in a continuous fold.
4. Hop descent: hips begin to unfold, knees lower and both feet reach beneath pelvis before touching ground, hands open quietly for balance. Cloak still lifts behind, preparing an immediate standing return.
5. Full-jump rising: a much taller elegant body than the hop, spine long and softly extended, one knee loosely bent, other leg descending, elbows opening with palms up below shoulders. Linen and copper trail down/back from the rising body.
6. Full-jump apex: same full-size head and body, upright torso, both knees gently soften rather than fully tuck, hands open calmly, cloth floats outward behind the shoulders in a new connected fold.
7. Full-jump descent: full-size torso slightly inclines, both feet extend under hips, soft knees ready to absorb support, hands lower toward her neutral gesture; cloak lifts behind while linen follows the legs. Both footwear soles lower than every cloth tip.
8. Grounded landing compression: both feet supported on an implied floor, knees absorb weight, torso controlled and slightly forward, hands quietly balance, copper cloak still raised from descent; full linen hem remains at or above footwear support. The next existing key will rise toward idle.
Style: match the approved high-resolution painterly fighting-game art and material detail. Eight coherent joint and cloth changes, not rigid body rotations or resized duplicates. Her grace is composed and deliberate, with a fast tucked hop distinct from the long full jump. Both hands are empty during these movement gestures; no projectile, attack, or new weapon.
Every full body, cloth tip and fingertip stays inside its own cell with ample bright-green clearance. No ground, cast shadows, scenery, grids, borders, text, labels, trails, motion blur, sparks or extra characters.
```

## V1 candidate integration

Original built-in output `exec-0c38e072-8b07-46be-bc27-31084bdfc203.png`, 1672×941; unchanged vault/runtime copy `raya-movement-v1-green.png`. All eight complete figures inspected in source. Candidate uses common anatomical height400 and regions `[0,0,410,420]`, `[410,0,820,420]`, `[820,0,1210,420]`, `[1210,0,1672,420]`, then the same columns fromy420 to941. RootX287/662/1057/1435/269/646/1050/1437; rootYmeasured/425/380/430/855/850/825/measured. These unchanged V1 calibrations passed the final runtime review below. Read-only approximate green exclusion bounds are saved in `/tmp/aeon-raya-movement-bounds.json`; no bitmap edits.

Selection shares the established Kogan movement phases: prejump, velocity-driven hop/full rise/apex/descent, first2f landing compression. Raya's existing Reaction10 gives the second landing tick. Clean cuts remove previous movement and existing landing-cell overlays. Both bodies retain original jump inputs and landing duration; the movement regression now checks both sets. New Raya return-layer test checks one body, unchanged hash and immediate attack override. All170 tests, clippy and locked/offline release pass after simplifying the boolean condition (first attempt passed tests but failed clippy). Runtime review is complete in the acceptance sections below.

## Incomplete first candidate capture

The first `raya-movement-candidate` capture stopped in case 7 because Citadel exhausted disk space. The game panicked during PNG export; its remaining owned wrapper and encoder were stopped. The complete stopped directory, including the unencoded spool, was re-archived and checksum verified in the vault, then labeled `INCOMPLETE.md`. It is not valid visual acceptance evidence. Archive older own raw captures before pruning redundant remote PNGs; capture the candidate again in a fresh `candidate2` directory.

## Standalone V1 acceptance

The fresh `raya-movement-candidate2/Raya-movement/` capture completed: 1,440 ticks, 24 s at 60 fps, 24 cases and 600 diagnostics. The entire directory is checksum verified in the vault. All simulation trace bytes equal the baseline. This unchanged candidate supplies the final standalone evidence; its directory name records its origin.

All 24 cases played completely at 1× in a 1280×720 browser viewport. Temporal screenshots sampled from approximately 0.1 s at 2 s intervals; their capture latency places them later in the moving clip. Selected exact paused ticks: 12, 13, 14, 17, 27, 35, 36; 737, 754, 769, 770, 771, 772; mirrored corner 193, 215, 216, 934, 949, 950, 951, 952. HUD case/tick labels corroborate the inspected phases.

The new compression keeps both feet and the linen hem supported; rise and apex gather the legs while changing cloak folds, and descent lowers the feet. The full jump retains a tall body and stable head scale, with complete fingers, cloak and soles. Both supported full-jump ticks and idle now have one opaque body. The hop descends directly to control without an added landing pose or recovery; the full jump retains the existing two ticks. Both facings and corner silhouettes remain intact and clear of the HUD. The unchanged V1 calibration above is accepted for standalone movement.

G1/K1 joint gathering and extension become a compact, fast hop and a long upright full jump. S1 cloth continuity becomes different rising, floating and descending copper folds around Raya's composed linen body. These observed principles replace the baseline held tuck/tiny full-jump silhouette and duplicate transition bodies; Aeon's own trajectory and landing rules remain intentional differences. This accepts standalone movement only. Shared attack/air-hit landing regression and retained integration are accepted below.

### Shared landing and integration acceptance

All 64 shared airborne exchange cases / 160 s played completely at 1× in 1280×720: sixteen Kogan CrHS anti-airs and four juggles for each of six normals on both bodies. All 9,600 trace ticks match the published airborne-exchange baseline byte for byte. Exact paused steps: Kogan JST 74/75/76/77/78 and 526/527/528; Raya JS 56/57/58/59/357/508/509; Raya JST 63/64/65/66/364/515/516; Kogan CrHS 20/22/1224/1840. These cover movement-to-hit, both landing ticks, idle, both facings and corner support. The new compression and old rising key stay single and intact. Raya's legacy horizontal air recoil and old air-attack art remain separate open families; this regression does not accept them wholesale.

The complete new 35 s retained integration played at 1× in 1280×720, with eight temporal samples. Additional paused seeks at simulation-time ticks 1838/1842/1846/1850/1852 on the 30 fps video inspect the longer uppercut landing through compression, rise and idle; no duplicate body remains. All 2,100 integration trace ticks equal baseline. Eight smoke images are retained: seven match the previously inspected airborne-recovery smoke exactly. The changed training-boxes pair was directly inspected: frame 71 versus 72 changes the frame/hash text while bodies, boxes and layout agree.

All 170 tests (81 client + 89 sim), clippy with warnings denied and locked/offline release pass in `raya-movement-checks2.log`. Complete archives were checksum verified: before and accepted candidate2 retain 600 diagnostics each, `raya-movement-exchange-final/` retains thirteen videos and 2,624 diagnostics, `raya-movement-polish/` retains its video and 71 diagnostics, and `raya-movement-smoke/` retains eight PNGs. The original incomplete candidate remains labeled and is not acceptance evidence. Owned remote PNGs were thinned only after full checksum archival; retention manifests identify that work. `raya-movement-verification.json` records trace hashes and review scope.

This accepts Raya standalone prejump, hop/full-jump rise/apex/descent and their legal returns, plus the shared landing regression. All authored timing, inputs, trajectory, geometry and simulation values remain fixed. G1/K1/S1 refraction is recorded above. The full-kit goal continues into Raya's remaining reactions and kit; Kogan's original blocked StP/StK/CrK request remains un-retried. Physical stick and competitive acceptance remain pending.
