# Animation coverage — first review milestone

Initial inventory: Citadel `af92c10`,September5. Final source audit: ART67,September7. All69 implemented moves and16 Action variants have recorded reviewed coverage; canonical source mirror matches. See the completion audit and final review guide. Historical checkpoint entries below retain the status at the time they were written.

Current rows identify accepted drawing/reuse,visual review and benchmark evidence. Unsupported enum combinations are N/A. This matrix records the first animation review milestone; physical stick play and competitive balance follow.

## Move definitions

The shared enum has 37 slots. Kogan defines 35; Raya defines 34. Unsupported slots are recorded explicitly below. Table rows follow the enum and character definitions, with baseline selection checked in `sprites.rs` and `sequences.rs`.

### Kogan

| Move slot | Baseline drawing coverage | Status / work | Reference / final evidence |
|---|---|---|---|
| `StP` | Approved gathered fist, V2 dark contact, withdrawal and ready | Reviewed — supported fist/full saber and material continuity | G2/K2/S2; ART67,20 final cases/20s at1× with temporal samples, every selected contact and return phases inspected;1,200 equal ticks; jab-finish provenance |
| `StK` | Four V5 low-boot phases with complete saber | Reviewed — chamber, extension, withdrawal, replacement and clean return | K2; ART65, all20 final cases/20s at1×, every contact and selected return phases stepped;1,200 unchanged ticks; standing-kick provenance |
| `StS` | Four new high short-poke phases | Reviewed — existing timing and geometry preserved | G2/S2; final 16 cases / 40s at 1×, contact and withdrawal frame steps; duck stays clear |
| `StHS` | Four ART62 V1 backcut phases | Reviewed — existing timing and geometry preserved | S2/A1; final 16 cases / 40s at 1×; complete cape/blade, clean return |
| `StHSClose` | Four ART62 V1 backcut phases | Reviewed — existing timing and geometry preserved | S2/A1; final 16 cases / 40s at 1×, including near jump escape |
| `StFL` | Four V2 low pommel phases, shared withdrawal | Reviewed — complete equipment, active contact and original return | S2/G2; 20 final cases / 20s per move at 1×, crouched contact and full recovery steps; 2,400 equal ticks across both moves |
| `StST` | Four V2 waist-level saber phases, shared withdrawal | Reviewed — complete equipment, active contact and original return | S2/G2; 20 final cases / 20s per move at 1×, crouched contact and full recovery steps; 2,400 equal ticks across both moves |
| `CrP` | Four authored crouching fist phases | Reviewed — full saber/cape, compact contact, bent withdrawal and clean low return | G2/S2; all 20 cases/20 s at 1×, focused phases/both guards/corners/misses;1,200 equal ticks |
| `CrK` | Four V5 supported low-kick phases with complete saber | Reviewed — compact extension, folded withdrawal and direct crouch return | K2/G2; ART66, all20 final cases/20s at1×, every contact and selected phases inspected;1,200 equal ticks; crouching-kick provenance |
| `CrS` | Four authored crouching saber phases | Reviewed — forward saber extension, full blade and clean low return | S2/A1/G2; all80 cases/140s at1×, focused phase and mirrored/corner contact steps;8,400 equal ticks |
| `CrHS` | Four authored crouching saber phases | Reviewed — front rising saber, full blade and clean low return | S2/A1/G2; all80 cases/140s at1×, focused phase and mirrored/corner contact steps;8,400 equal ticks  Additional airborne exchange review: four juggle cases /10 s per normal, sixteen CrHS anti-airs /40 s; G2/S2. |
| `CrFL` | Four authored crouching saber phases | Reviewed — short low saber extension, full blade and clean low return | S2/A1/G2; all80 cases/140s at1×, focused phase and mirrored/corner contact steps;8,400 equal ticks |
| `CrST` | Four authored crouching saber phases | Reviewed — supported near-floor sweep, full blade and clean low return | S2/A1/G2; all80 cases/140s at1×, focused phase and mirrored/corner contact steps;8,400 equal ticks |
| `JP` | Shared V1 gather/withdrawal/descent and distinct V4 contact | Reviewed — corrected limb height, full equipment, clean legal landings | K2/G2/G1; 40 final cases / 100s per move at 1×, early recovery and focused contact/landing frame steps; 18,000 equal ticks across three moves  Additional airborne exchange review: four juggle cases /10 s per normal, sixteen CrHS anti-airs /40 s; G2/S2. |
| `JK` | Shared V1 gather/withdrawal/descent and distinct V4 contact | Reviewed — corrected limb height, full equipment, clean legal landings | K2/G2/G1; 40 final cases / 100s per move at 1×, early recovery and focused contact/landing frame steps; 18,000 equal ticks across three moves  Additional airborne exchange review: four juggle cases /10 s per normal, sixteen CrHS anti-airs /40 s; G2/S2. |
| `JS` | Shared gather/two withdrawal drawings and distinct V2 active pose | Reviewed — full blades, separate contact and recovery, clean legal landings | S1/S2/A2/G1; 40 final cases / 100s per move at 1×, early recovery/landing frame steps; 18,000 equal ticks across three moves  Additional airborne exchange review: four juggle cases /10 s per normal, sixteen CrHS anti-airs /40 s; G2/S2. |
| `JHS` | Shared gather/two withdrawal drawings and distinct V2 active pose | Reviewed — full blades, separate contact and recovery, clean legal landings | S1/S2/A2/G1; 40 final cases / 100s per move at 1×, early recovery/landing frame steps; 18,000 equal ticks across three moves  Additional airborne exchange review: four juggle cases /10 s per normal, sixteen CrHS anti-airs /40 s; G2/S2. |
| `JFL` | Shared V1 gather/withdrawal/descent and distinct V4 contact | Reviewed — corrected limb height, full equipment, clean legal landings; empty-cylinder FL selects the knee | K2/G2/G1; 40 final cases / 100s per move at 1×, early recovery and focused contact/landing frame steps; 18,000 equal ticks across three moves  Additional airborne exchange review: four juggle cases /10 s per normal, sixteen CrHS anti-airs /40 s; G2/S2. |
| `JST` | Shared gather/two withdrawal drawings and distinct V2 active pose | Reviewed — full blades, separate contact and recovery, clean legal landings | S1/S2/A2/G1; 40 final cases / 100s per move at 1×, early recovery/landing frame steps; 18,000 equal ticks across three moves  Additional airborne exchange review: four juggle cases /10 s per normal, sixteen CrHS anti-airs /40 s; G2/S2. |
| `Overhead` | Four standing gather/cut/withdrawal/ready drawings | Reviewed — complete front blade and supported recovery | S2/A1; 20 final cases / 30s at 1×, focused phases and mirrored/corner guard steps; unchanged trace |
| `Throw` | Four reviewed utility phases at normal-throw timing | Reviewed — distinct hit hold and miss withdrawal, clean ready return | G2/A1/S2; all 32 final cases / 80s at 1×, contact/separation/return steps; 4,800 equal ticks |
| `Rekka1` | Four ART61 V1 first-cut phases | Reviewed — existing timing and geometry preserved | S2/A1; final 16 cases / 40s at 1×; contact/hold/return |
| `Rekka2` | Four ART62 V1 backcut phases | Reviewed — existing timing and geometry preserved | S2/A1; final 16 legal follow-up cases / 40s at 1× |
| `Rekka3` | Four ART63 V2 thrust phases, clean transitions | Reviewed — existing timing and geometry preserved | S2/A1; final 16 legal chains / 48s at 1×; corner ticks 64/74/85 stepped, complete tip |
| `Uppercut` | Original coil/rise/descent + two new compact phases + landing | Reviewed — existing timing and geometry preserved | S1/A2/K1; final 16 cases / 40s at 1×; ticks 30/34/47/61/71/72/83 stepped, HUD clear |
| `CommandGrab` | Four new V3 utility phases | Reviewed — low reach, release and ready | A1/G2/S2; full 28-case final at 1×, grab release/return frame steps; victim reaction coverage remains separate |
| `CommandDash` | Four new V3 utility phases | Reviewed — gather, glide, brake and settle | G1/S2; full 28-case final at 1×, launch/brake/settle/idle frame steps |
| `ShotA` | Four new ranged phases selected from V5 | Reviewed — preparation, release, compact follow-through and withdrawal | S2/A2; full 48-case final at 1×, selected release/guard frame steps in full-kit pass |
| `ShotB` | Four new ranged phases selected from V5 | Reviewed — preparation, release, compact follow-through and withdrawal | S2/A2; full 48-case final at 1×, selected release/guard frame steps in full-kit pass |
| `Guard` | Four V2 gather/brace/dismissal/settle drawings | Reviewed — open shield, consistent anatomy, clean return | S2/A2; complete 20-case 30s at 1×, projectile and crouch-block frame steps; 1,800 equal ticks |
| `Detonate` | Not defined for this body | N/A — retain unsupported | Character definition |
| `SpecialOverhead` | Reviewed air-saber gather/contact and four landing drawings | Reviewed — full blade, immediate landing and original tax | S1/A2/G1; 20 final cases / 50s at 1×, contact and full landing steps; unchanged trace |
| `AirShot` | Four selected gather/aim/recoil/holstered descent drawings | Reviewed — full weapons, downward aim/trail and clean legal landings | S1/S2/A2/G1; all 32 final cases / 80s at 1×, mirrored corner release/contact and exact landing frame steps |
| `Charge` | Not defined for this body | N/A — retain unsupported | Character definition |
| `ExA` | Four ART61 V1 first-cut phases at EX timing | Reviewed — existing timing and geometry preserved | S2/A1; final 16 cases / 40s at 1×; existing flash/gauge and clean return |
| `ExB` | Four new ranged phases selected from V5 | Reviewed — preparation, release, compact follow-through and withdrawal | S2/A2; full 48-case final at 1×, selected release/guard frame steps in full-kit pass |
| `Super` | Four V3 gather/dual-rush/withdraw/reholster drawings | Reviewed — original commitment and travel retained | S2/A2; full corrected 16 cases / 40s at 1×, standing/corner guard and holster frame steps; 2,400 equal ticks |





### Raya

| Move slot | Baseline drawing coverage | Status / work | Reference / final evidence |
|---|---|---|---|
| `StP` | Compact palm, lowered contact, folded return and ready | Reviewed — four phases, complete body and clean legal return | G2/K2;20 final2 cases /20 s at1×, phase/guard/duck/miss/corner steps;1,200 equal ticks |
| `StK` | Low knee chamber, shin extension, bent withdrawal and replaced foot | Reviewed — four phases, complete body and clean legal return | G2/K2;20 final2 cases /20 s at1×, phase/guard/duck/miss/corner steps;1,200 equal ticks |
| `StS` | Low single palm and tiny eye; four V1/V4 phases | Reviewed — active contact, withdrawal, supported ready and clean return | G2/S2/A1/A2;20 final2 cases/50s at1×, phase PNGs and guards/corners/misses;3000 equal ticks |
| `StHS` | Long two-hand written-light contact; four V1/V4 phases | Reviewed — active contact, withdrawal, supported ready and clean return | G2/S2/A1/A2;20 final2 cases/50s at1×, phase PNGs and guards/corners/misses;3000 equal ticks |
| `StHSClose` | Compact bent-elbow hip press; four V1/V4 phases | Reviewed — active contact, withdrawal, supported ready and clean return | G2/S2/A1/A2;20 final2 cases/50s at1×, phase PNGs and guards/corners/misses;3000 equal ticks |
| `StFL` | Low palm gather/press/upturned fold/ready | Reviewed — four phases, coherent support and clean legal return | G2/S2/A1/A2; 20 final2 cases / 20 s at 1×, phase/guard/hit/miss/corner steps; 1,200 equal ticks |
| `StST` | Supported cape gather/pivot/falling cloth/ready | Reviewed — four phases, coherent support and clean legal return | G2/S2/A1/A2; 20 final2 cases / 20 s at 1×, phase/guard/hit/miss/corner steps; 1,200 equal ticks |
| `CrP` | Low palm gather/contact/fold/ready | Reviewed — four phases, full body and direct legal low return | G2/K2;20 final cases /20 s at1×, phase/guard/hit/miss/corner steps;1,200 equal ticks |
| `CrK` | Supported chamber/compact ankle contact/withdrawal/low ready | Reviewed — four phases, full body and direct legal low return | G2/K2;20 final2 cases /20 s at1×, phase/guard/hit/miss/corner steps;1,200 equal ticks |
| `CrS` | Four horizontal-crystal V1 phases | Reviewed grounded exchanges — active contact, full support and clean low return | K2/S2/A1/A2;20 final1 cases/30s at1×; phase/corner/guard/miss PNGs;1800 equal ticks |
| `CrHS` | Four vertical-crystal phases with longer V1 contact | Reviewed ground and airborne exchanges — complete support, visible crouch visor and clean low return | G1/G2/S2 plus prior K2/A1/A2;16 corrected anti-airs/40s and20 ground regressions/30s at1×; exact contact/guard/whiff phases;4200 equal ticks |
| `CrFL` | Four shin-palm V1 phases | Reviewed grounded exchanges — active contact, full support and clean low return | K2/S2/A1/A2;20 final1 cases/30s at1×; phase/corner/guard/miss PNGs;1800 equal ticks |
| `CrST` | Four supported sweep V1 phases with bent withdrawal | Reviewed grounded exchanges — active contact, full support and clean low return | K2/S2/A1/A2;20 final1 cases/50s at1×; phase/corner/guard/miss PNGs;3000 equal ticks |
| `JP` | V1 downward open palm; shared gather/fold/ready | Reviewed grounded/airborne targets and misses — complete contacts, clean return and legal landing; airborne juggles reviewed | G1/G2/K2/S2/A2;48 final cases/120s at1×; phase/corner/guard/miss PNGs;7,200 equal ticks; G1/G2/S2 adds 4 juggles / 10s at 1×, first/held contacts and corner landings, 600 equal ticks |
| `JK` | V1 extended low sandal; shared gather/fold/ready | Reviewed grounded/airborne targets and misses — complete contacts, clean return and legal landing; airborne juggles reviewed | G1/G2/K2/S2/A2;48 final cases/120s at1×; phase/corner/guard/miss PNGs;7,200 equal ticks; G1/G2/S2 adds 4 juggles / 10s at 1×, first/held contacts and corner landings, 600 equal ticks |
| `JS` | V1 short diagonal crystal; approved air-light gather/fold/ready | Reviewed grounded/airborne targets and misses — calibrated hit/guard contact, clean return and legal landing; airborne juggles reviewed | S1/S2/A2;48 accepted cases/120s at1×; phase/corner/guard/miss PNGs;7,200 equal ticks; G1/G2/S2 adds 4 juggles / 10s at 1×, first/held contacts and corner landings, 600 equal ticks |
| `JHS` | V1 long diagonal crystal with rear-leg extension; approved air-light gather/fold/ready | Reviewed grounded/airborne targets and misses — calibrated hit/guard contact, clean return and legal landing; airborne juggles reviewed | S1/S2/A2;48 accepted cases/120s at1×; phase/corner/guard/miss PNGs;7,200 equal ticks; G1/G2/S2 adds 4 juggles / 10s at 1×, first/held contacts and corner landings, 600 equal ticks |
| `JFL` | V1 small open downward glyph; shared gather/fold/ready | Reviewed grounded/airborne targets and misses — complete contacts, clean return and legal landing; airborne juggles reviewed | G1/G2/K2/S2/A2;48 final cases/120s at1×; phase/corner/guard/miss PNGs;7,200 equal ticks; G1/G2/S2 adds 4 juggles / 10s at 1×, first/held contacts and corner landings, 600 equal ticks |
| `JST` | V1 steep narrow crystal; approved air-light gather/fold/ready | Reviewed grounded/airborne targets and misses — calibrated hit/guard contact, clean return and legal landing; airborne juggles reviewed | S1/S2/A2;48 accepted cases/120s at1×; phase/corner/guard/miss PNGs;7,200 equal ticks; G1/G2/S2 adds 4 juggles / 10s at 1×, first/held contacts and corner landings, 600 equal ticks |
| `Overhead` | Six V1 gather/lift/downstroke/release/fold/ready drawings | Reviewed — contact at receiver, full support, coherent scale and clean return | S2/S1/A2;20 accepted candidate1 cases/30s at1×, exact phases/guards/whiff/corner PNGs;1800 equal ticks |
| `Throw` | New empty-hand V1 contact plus utility gather/withdrawal/ready | Reviewed — distinct hit hold, immediate miss withdrawal and clean return | A1/G2; all 32 candidate1 cases / 80s at 1×, exact phases and corners; 4,800 equal ticks |
| `Rekka1` | Four reviewed medium-palm phases reused at chant timing | Reviewed — active contact, clean withdrawal/return and legal follow-ups | G2/S2/A1/A2;20 final3 cases/50s at1×, phase PNGs and guards/corners/misses;3000 equal ticks |
| `Rekka2` | Four V2 cross-body wrist/crescent phases | Reviewed — active contact, clean withdrawal/return and legal follow-ups | G2/S2/A1/A2;20 final3 cases/50s at1×, phase PNGs and guards/corners/misses;3000 equal ticks |
| `Rekka3` | Four V1 two-hand gather/ring/withdrawal/ready phases | Reviewed — active contact, clean withdrawal/return and legal follow-ups | G2/S2/A1/A2;20 final3 cases/60s at1×, phase PNGs and guards/corners/misses;3600 equal ticks |
| `Uppercut` | Retained gather/descent/landing + two V2 low-release/folded-hand phases | Reviewed — active release, gathered apex, clean return and HUD clearance | A2/K1/G1/S1;20 final V2 cases/50s at1×; phase/corner/guard/miss PNGs;3000 equal ticks |
| `CommandGrab` | Four V1 gather/contained loop/withdrawal/ready drawings | Reviewed — full support, coherent anatomy, clean recovery and original mechanics | G1/G2/A1/A2/S2;20accepted candidate1cases/50s at1×, exact phases and mirrored/corner exchanges;3000equal ticks |
| `CommandDash` | Four V1 prayer gather/glide/brake/settle drawings | Reviewed — full support, coherent anatomy, clean recovery and original mechanics | G1/G2/A1/A2/S2;8accepted candidate1cases/20s at1×, exact phases and mirrored/corner exchanges;1200equal ticks |
| `ShotA` | Four V2 low crystal gather/release/empty withdrawal/ready | Reviewed — calibrated release/return, independent effect lifetime, clean cuts | A2/S2 plus reopened original AC;16 cases /56s at1×, exact phases;3,360 equal ticks |
| `ShotB` | Four V2 inhale/spoken glyph/fold/ready | Reviewed — calibrated release/return, independent effect lifetime, clean cuts | A2/S2 plus reopened original AC;16 cases /56s at1×, exact phases;3,360 equal ticks |
| `Guard` | Not defined for this body | N/A — retain unsupported | Character definition |
| `Detonate` | V1 empty-hand pinch/open, accepted Rite withdrawal and V1 ready | Reviewed — ordinary paths and ART50 startup feints, complete support and clean return | A1/A2/S2 plus reopened original AC; 16 cases /32s at 1×, exact phases; 1,920 equal ticks |
| `SpecialOverhead` | Not defined for this body | N/A — retain unsupported | Character definition |
| `AirShot` | Not defined for this body | N/A — retain unsupported | Character definition |
| `Charge` | Six V1 supported descent/held breath/rise/ready drawings | Reviewed — ordinary paths and ART50 startup feints, complete support and clean return | A1/A2/S2 plus reopened original AC; 16 cases /32s at 1×, exact phases; 1,920 equal ticks |
| `ExA` | Four V2 spoken glyph phases at EX timing | Reviewed — calibrated release/return, independent effect lifetime, clean cuts | A2/S2 plus reopened original AC;16 cases /56s at1×, exact phases;3,360 equal ticks |
| `ExB` | Four V2 low crystal phases at EX timing | Reviewed — calibrated release/return, independent effect lifetime, clean cuts | A2/S2 plus reopened original AC;16 cases /56s at1×, exact phases;3,360 equal ticks |
| `Super` | Four V1 gather/expanded orbit/dismissal/ready drawings | Reviewed — complete supported phases, two matching-pose trails | Original AC84.229–93.469s +A1/A2/S2;20 cases /50s at1×,exact phases;3,000 equal ticks |

## Non-attack states and presentation

| State / variant | Kogan baseline | Raya baseline | Milestone evidence / work |
|---|---|---|---|
| Stand / idle | Idle pose + motion; reviewed ground return | Idle pose + motion; reviewed ground return | Both clean run/crouch/retreat returns reviewed; other returns remain family-specific |
| Crouch | Half/low V4 drawings, clean rise | Half/low V1 drawings, clean rise | Both 36-case ground matrices reviewed; close and mirrored corner rise stepped |
| Walk forward / backward | 4 existing cells, measured regions, clean cuts | 4 retained cells, measured regions/roots, reversed for backwalk | Both directions fully reviewed at center/corners in both facings |
| Run | Two cloth keys plus gather/brake/settle | Two V1 cloth keys plus shallow gather/brake/ready | Both full run-stop/crouch/block/jump/attack matrices reviewed; immediate exits and phase freeze/reset tested |
| BackDash | V4 gather/travel/brake/settle | V1 backward gather/travel/brake/ready | Both backward-weight paths and clean returns reviewed inside unchanged 14f, both facings/corners |
| Prejump / prehop | New authored compression reviewed | New supported V1 compression reviewed | Both bodies retain 4f preparation; G1/K1; Raya 24 cases / 24 s and exact phases reviewed |
| Jump / hop; both directions and neutral | New rise/apex/descent drawings reviewed | New compact hop and tall full-jump phases reviewed | Both 24-case standalone matrices at 1×, mirrored/corner phases stepped; Raya movement-to-hit and shared landings rechecked across 64 airborne exchanges. Attack artwork stays family-specific. |
| Feint | Reviewed family withdrawal/ready reuse, clean airborne descent | Reviewed family withdrawal/ready, supported Charge rise and airborne descent | Kogan88 cases/176s;Raya80 cases/160s before/final at1x plus exact phases. G1/G2/S2 and fresh Garou387.747–389.956s. Original8f and legal2f landing retained. |
| Block standing / crouching | V2 brace/release pairs reviewed | Eight V1 hit/guard drawings; brace/release reviewed | Both36-case reaction matrices reviewed; G2/S2, final four existing stun ticks release. Both low returns corrected and eight Kogan regression cases rechecked. |
| Hit standing / crouching | V2 impact/release pairs reviewed | V1 impact/release pairs reviewed | Both standing P/S and dedicated low-hit cases, both facings/corners; G2/S2. Raya receiver remains visible; Kogan CrK attack phases are now accepted in ART66. |
| Air hit rise / fall | Reviewed launches and four V1 non-knockdown keys | Four V1 non-knockdown keys and retained launch/floor reviewed | Raya 40 anti-air/juggle cases / 100 s at 1×, contact and corner landing phases stepped; G2/G1/S1, original stun and legal landing. Kogan prior launch/throw and24 normal-juggle recovery review retained; ART43 adds16 Raya CrHS anti-air exchanges with Kogan and rechecks24 corrected-art normal juggles (100s total). |
| Knockdown | New full-length prone drawing reviewed | Retained full-body prone drawing reviewed | Both sweep/launch/throws and corners; A1/S1, unchanged deliberate hold and single-body cuts. |
| Getup | 4 new support phases reviewed | 4 retained support phases reviewed | Both hand support, knee, foot and clean idle inside unchanged24f; Raya sweep90/96/102/108 and both corners stepped. |
| Thrown normal / command | Deliberate held recoil then reviewed rise/fall/floor | New grounded recoil then retained rise/fall/floor reviewed | Both victim normal/command throws reviewed, both facings/corners; A1/G2. Attacker choreography remains separate. |
| ThrowTech | Two new separation/withdrawal drawings plus utility ready | Approved recoil brace/release plus utility ready | Both bodies early/late tech reviewed in both facings/corners, complete 15 post-tick frames and clean return; G2/A1/S2. Raya exact three phases inspected in ART46. |
| Landing 0f / 2f / taxed | Reviewed family-specific support in original duration | New standalone compression plus existing clean-cut rise; longer existing keys retained | Raya 0f/2f reviewed in 24 jumps, shared 2f returns in 64 exchanges, uppercut tax in retained integration. Other move taxes remain family-specific review. |
| RC / hitstop / pause / frame-step / replay | Procedural effects + history | Procedural effects + history | ART53: flash decay corrected;24 hit/block/RC cases and576 exact scripted paused redraws reviewed, original clock retained. ART54: four actual saved/loaded logs and960 world/image pairs exact;32s complete playback and reset from hitstop inspected. ART64 verifies production pause/step/reset, save and R load; F11 is retained but macOS intercepted it as Show Desktop. |
| Win / KO / round transition | Four V1 victory drawings and reused KO support/floor drawings reviewed | Four V1 offering drawings and reused KO support/floor drawings reviewed | Each victory16 cases/64s;both-body KO48 cases/192s. Both facings/corners,legal recovery,rise,landing and resets inspected. A1/A2/G2/S2 and fresh original AC104.795–110.230s. |
| Projectiles / EX / charge effects | Wave/gun/EX ranged, shield and airborne gun reviewed | Glyph/crystal/EX ranged and Charge/Detonate reviewed | Kogan ranged8e8d6e2, discbfbb344 and airgune843893; Raya ART47/48 spawn/contact/expiry, empty withdrawal and independent effect life reviewed. A2/S2; exact family captures in full-kit ledger. |

## Resolved defects and residual polish

| Defect | Affected coverage | Status |
|---|---|---|
| Mixed close neutral hides Raya in P1 | Both bodies | Resolved ART55 — stable linen foreground in quiet grounded mixed neutral; both slots/facings,center/corners,walk/stop and contact transitions reviewed |
| Old/new style and body-scale transitions | Both full kits | ART58 quiet Kogan Stand now shares approved drawn ready; ART59 walking, ART61 first-cut and ART62 backcut finish reviewed; ART63 thrust finish reviewed; jab material reviewed in ART67 |
| Residual colored fringes on older cape edges | Kogan, legacy poses | ART56/57 fix invisible RGB and dark-green spill with unchanged alpha; fine yellow-green curl coloration remains source polish |
| Tall ascension effect overlaps timer area | Raya Uppercut | Resolved ART39 — low active crystal, folded-hand apex;20 final V2 cases and exact apex/corner PNGs reviewed |
| Dedicated crouched-hit exchange | Both grounded reactions | Raya CrK → Kogan final reviewed at 1× and impact/release/crouch stepped; Kogan CrK → Raya victim now reviewed with visible hit/guard and corrected low return; ART66 replaces the wrong punch and clipped blade with reviewed low-kick phases |
| Defeated body returns to idle/getup after the ordinary reaction expires | Both KO presentation paths | Resolved — 48 legal KO cases / 192s; grounded support collapse, actual airborne landing, persistent floor and reset reviewed without sim changes. |
| Current eight-scene preview does not isolate every move/state | Full kits | Family fixtures now provide focused coverage; all69 move slots have reviewed phases; StK/CrK are published in ART65/66; jab material accepted in ART67; final audit remains |

## Current acceptance and historical checkpoints

ART67 jab material is accepted: all69 implemented moves have reviewed phases. The completion audit cross-checks the original full-goal requirements. Shared overlap/key fixes and production training controls are already reviewed. Use the current handoff and standing/crouching-kick notes for exact progress; the following ART55–64 entries are historical checkpoints.

ART55 resolves mixed close-neutral depth. Shared old/new style, key edges and Kogan StK/CrK remain open; see latest full-kit report.

ART56 fixes transparent-green RGB filtering with32s replay/35s integration and six changed smoke scenes reviewed. Alpha/visible source pixels unchanged; fine older cape-edge residue and style consistency remain open.184 tests pass.

ART57: dark spill corrected;185 tests pass. Working review package: [review guide](ANIMATION-REVIEW.md). Shared style and Kogan StK/CrK remain material art gates; do not infer full acceptance from67 reviewed move slots.

ART58: quiet Kogan Stand reuse accepted after full 32s replay, 35s integration and 33.6s mirrored/corner freeze review; 185 tests pass. Legacy walk/contact finish and Kogan StK/CrK remain open.

ART59: Kogan walking V1 reviewed through all four supported steps, both directions/facings/corners and direct ready return. 720/2,100 ticks unchanged; 185 tests pass. Legacy contact finish and StK/CrK remain open.

ART60: approved throw-tech palm reuse rejected after one-case jab comparison; smaller torso/helmet and separation gesture fail the transition check. Exact ART59 selector restored; jab finish remains open.

ART61: first-cut V1 accepted for Kogan Rekka1/ExA, their legal chain entries and their existing feint withdrawal.80 before/final cases200s each at1×; selected source/contact/chain/guard/whiff/feint phases inspected. S2/G2/A1/A2 refraction;12,000 focused and2,100 integration ticks unchanged,185 tests/clippy/release pass. Backcut/thrust/jab finish, grounded kicks and production training keys remain open. Exact evidence in the full-kit ledger.

ART62: backcut V1 accepted for StHS/StHSClose/Rekka2, chain into Rekka3 and Rekka2 feint. All 64 sword cases / 168s and 8 feints / 16s per before/final version reviewed at 1×; ART61 chain baseline review reused explicitly. Selected contact, chain, corner guards, whiff/feint returns and changed integration frame inspected. S2/G2/A1/A2 refraction; 11,040 focused and 2,100 integration ticks unchanged. 185 tests, clippy and release pass. Legacy thrust/jab finish, grounded kicks and actual training keys remain open.

ART63: V2 thrust accepted for Rekka3 and its feint withdrawal after rejecting V1 short blades. All 16 final chains / 48s, eight feints / 16s per version and new 35s integration reviewed at 1×; exact ART62 chain baseline review reused. Selected contact, corner low guards, hold/whiff/feint returns and changed integration frames inspected. S2/G2/A1/A2 refraction; 3,840 focused and 2,100 integration ticks unchanged. 185 tests, clippy and release pass. Grounded kicks, jab finish and training keys remain open.

ART64: direct training controls verified, including F9 save and R load (F11 is retained but intercepted by macOS Show Desktop here). No move coverage changes: Kogan StK/CrK and legacy jab contact finish remain open. See Aeon/notes/2026-09-06-training-keys.

ART65: four standing-kick V5 phases accepted and published as9234920. Complete20-case1×matrix and allcontact/selectedreturn inspection;185 tests/clippy/release;1,200/2,100 unchanged ticks. CrK and legacy jab finish remain open.

ART66: four supported crouching-kick V5 phases accepted and published as ad88242. Complete20-case1× matrix and everycontact/selectedreturn inspection;185 tests/clippy/release;1,200/2,100 unchanged ticks. Legacy jab finish and final audit remain open.

ART67: jab V2 material accepted after rejecting V1 pale microtexture. Exact pose/equipment and4/2/6 Mid clock preserved. All20 selected contacts and return phases reviewed;185 tests/clippy/release, unchanged1200/60/2100 ticks, identical shared regression. Final whole-goal audit remains.
