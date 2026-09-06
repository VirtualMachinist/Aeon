# Raya — Rite and Processional, 2026-09-06

ART45 reviewed September 6. Built-in image generation; reference roles and exact prompt below. Candidate1 accepted unchanged after full in-game review. Baseline: `33f9809`.

## Prompt V1

```text
Use case: stylized-concept.
Asset type: eight-pose production animation sheet for Aeon's existing adult character Raya, high-resolution 2D fighting-game art.
Input images: image 1 is the existing Rite gesture reference; image 2 is the existing Processional prayer-gesture reference; image 3 is the approved current Raya identity, anatomy, costume, and rendering reference. Make new coherent poses, not a collage.
Composition: landscape 1536x1024, exactly FOUR columns and TWO rows, eight isolated complete full-body figures, all facing RIGHT in the same side-three-quarter fighting view. Each figure stays wholly inside its own 384x512 cell with at least 24 pixels of empty green on every side. No grid lines, lettering, numbering, environment, floor shadow, opponent, or duplicate limbs. Flat saturated chroma green background. Consistent head-to-sandal anatomical size about 400px per figure. Complete fingers, toes, sandals, cloth hems, and effects; nothing touches a cell border.
Identity/materials: Raya is a composed adult officiant with warm brown skin, refined same face and jeweled brow, ivory linen hood and loose draped trousers, copper shoulder plates and flowing copper cloak, narrow cyan woven circuit edging, copper cuffs and open sandals. Match image 3. Detailed painted high-resolution game illustration; crisp readable silhouette. Vertical linen and cloak remain her primary silhouette.
TOP ROW, left to right, four phases of the Rite:
1. Supported upright gather: hands close at chest, elbows bent, a small compressed ritual gesture, gaze ahead, both sandals planted.
2. Committed compact two-hand reach: torso inclines slightly, forearms extend forward at lower chest/waist height, open palms separated, shoulders remain relaxed. A SINGLE small complete thin cyan circuit loop curls between and just beyond her hands, fully closed and fully contained. No long chains or opaque beams. Preserve the intent of reference 1 but make both hands and the complete delicate light contour readable.
3. Release and withdrawal: upper body straightening, forward hand lowers back toward waist and the other returns toward chest, fingers relaxed; no energy effect. Cloak continues a slight backward follow-through. Clearly different from extended contact.
4. Familiar settled ready: upright balanced stance, one hand near chest and the forward forearm open with empty palm up, composed face; no crystal or other effect.
BOTTOM ROW, left to right, four phases of Processional:
5. Prayer gather: palms together before chest, upright supported body, feet close but separated, a small forward intent.
6. Grounded prayer glide: palms still together, whole body leans only about TEN degrees forward from ankles, legs remain quiet and elongated, soles nearly on the same ground line. Copper cloak streams behind with restrained lift. No running leg cycle and no horizontal flying body.
7. Supported braking: torso becoming upright, front foot settled, prayer hands beginning to part, copper cloth still lags behind gently. Distinct drawing of stopping the glide, with full feet on ground.
8. Settled ready: body upright and balanced, one hand near chest, forward hand opens into familiar empty-palm ready, trailing cloth settles vertically.
Keep all eight bodies the same person and scale, continuous costume and cloth. No giant halos, extra weapons, attack impacts, baked afterimages, or motion blur. The runtime supplies translucency and small trails.
```

## Source and first integration

- Original: `/Users/evanpincham/.codex/generated_images/01a07081-3028-76e3-8486-af4a0cdf6e0f/exec-e04f5719-3a06-48fb-887a-9f70133dfa06.png`, retained.
- Unchanged versioned copy: `raya-utility-v1-green.png`, 1536×1024, SHA256 `0ccabb4e670327cd67181766617f57d3002734edcc5f3304993aff7046388ddc`.
- Source inspection: all eight complete figures and a closed contact loop; no clipped hands/feet/effect. Unequal gutters measured independently; lower glide cloth starts at x367 and stays separated from its neighbor. Top-row anatomy408px; bottom426px, including straightened equivalence for leaning glide. Runtime anchors use complete lowest visible silhouette.
- First calibration: top regions `[0,0,360,490]`, `[360,0,775,490]`, `[775,0,1130,490]`, `[1130,0,1536,490]`, roots230/560/960/1320; lower regions `[0,490,340,1024]`, `[340,490,730,1024]`, `[730,490,1125,1024]`, `[1125,490,1536,1024]`, roots220/540/955/1310.
- CommandGrab and CommandDash only. Normal Throw keeps its existing selector pending dedicated review. First integration removes extra procedural Raya utility displacement; authored cuts, matching-cell trails and existing translucency retained. Kogan behavior unchanged.
- Reference comparison: G1/G2 normal-speed Garou Rock/Hokutomaru footage, backed by existing A1/A2/S2 dossier phases. Translate gathered intent, readable committed gesture, receiver consequence and withdrawal into composed ritual hands. Processional intentionally remains a pass-through prayer glide within its original eighteen frames and112px free travel. No Garou command-grab mechanics or frame counts inferred.

In-game acceptance completed below; both reviewed families retain V1 and the first calibration.

## September 6 — Raya Rite and Processional (ART45)

Baseline `33f9809`. The Rite's old extended hands and cropped cyan chains persisted through the throw and long recovery. Processional held a nearly horizontal flying prayer pose, then blended abruptly into idle. Eight new V1 drawings supply Rite gather/reach/withdrawal/ready and Processional prayer gather/glide/brake/settle. The thin closed Rite loop is fully contained and clears with withdrawal. Measured gutters and anatomical scales preserve full fingers, sandals and cloth; extra procedural Raya utility displacement is removed. Existing cuts, matching-cell trails and Processional translucency remain. Kogan utility selection and Raya normal Throw are unchanged.

Reference refraction: G1/G2 Garou: Mark of the Wolves, PS4, Rock versus Hokutomaru (TheInnocentSinful, https://www.youtube.com/watch?v=4-6_vD9NsSk&t=375s), reopened at 1×374.027–378.646 seconds in a1280×720 viewport. Rock gathers the knee/arms, lands into a supported strike and folds the arm while the opponent recoils independently. Translate readable intent, committed gesture, receiver consequence and withdrawal through Raya's composed ritual hands. Existing paused G1/G2 and A1/A2/S2 dossier phases support continuity, body/effect visibility and commitment. No command-grab or pass-through mechanic is inferred from this Garou exchange. Aeon keeps its own short prayer glide, copper/cyan/linen identity and authored timing.

Complete before and accepted candidate1 matrices were played at1×: **20 Rite cases/50s and8 Processional cases/20s**, both facings at center/corners. Rite includes hits, standing/crouching guard attempts, spacing misses and legal jump escapes; Processional includes near-body crossings and free travel. All four phases of each move were inspected at exact captured frames. The Rite visibly withdraws as Kogan follows the original throw arc and floor/getup; complete loop and both faces remain visible at mirrored corner captures. Processional stays supported, retains quiet legs, opens the prayer hands into braking and reaches ready before control returns. Side changes and translucency follow the existing pass-through behavior. Close idle overlap remains a shared follow-up, outside this family acceptance.

Exact diagnostics inspected: Rite before0016/0032; candidate0012/0020/0032/0044/0062,1516/1520/1670(corner crouching-guard capture),1818/1820(whiff),2870(mirrored corner jump escape). Processional before0016/0020/0028/0616; candidate0012/0016/0024/0028(all four phases),0616(free glide),0474(mirrored corner pass-through). V1 and first calibration are accepted unchanged after this review; the `candidate1` names are retained as evidence history.

The existing utility fixture now covers both bodies and verifies four phase visits, hitstop holds, original legal capture/throw outcomes, full recovery, Raya180 damage and112px free travel with center body crossing. Return-to-control regression includes every Raya utility cell. **176 tests(89sim+87client)**, clippy with warnings denied and locked/offline release pass on Citadel. All **4,200 focused ticks** and manifests equal baseline. The new35s/2,100-tick integration preview was played completely at1×; its full trace is unchanged. Changed0945/0960diagnostic pairs show new Rite gather/ready clearing the old chains, with the other69PNGs byte-identical. Eight fresh smoke images are retained: seven equal ART44, changed selection-screen pair directly inspected with complete plates, names, heading and P1 lock.

Evidence root `notes/media/2026-09-05-full-kit/`: `raya-utility-before/`, `raya-utility-candidate1/`, `raya-utility-polish/`, `raya-utility-smoke/`, `raya-utility-review-progress.json`, `raya-utility-verification.json`, and checks1/2 logs. Complete videos/traces/cases and1,204 accepted diagnostic PNGs are checksum archived before retaining eight remote family PNGs each. The initial dash capture rejected the display name at frame0; that failed run remains archived separately, and manifest validation plus bounded encoder cleanup were corrected before the successful baseline capture. No failed capture is counted as evidence.

Source `raya-utility-v1-green.png`,1536×1024,SHA256`0ccabb4e670327cd67181766617f57d3002734edcc5f3304993aff7046388ddc`, retains the unchanged built-in imagegen original. Exact prompt, source bounds, anatomical scales and roots: `RAYA-UTILITY-2026-09-06.md`.

Use `--kit-preview --kit-utility --kit-raya --kit-move=CommandGrab` or `CommandDash`. Original Rite7/2/40,180damage, throw rules and Processional18frames/112px/pass-through remain unchanged. Continue Raya normal throw/tech, ranged/charge/EX/super, feint/victory and shared polish. The original blocked Kogan lights generation remains un-retried. Full-kit and physical-play acceptance remain incomplete.
