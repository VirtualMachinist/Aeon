# Kogan crouching punch — 2026-09-05

Independent CrP batch, after full 20-case baseline playback. The blocked original StP/StK/CrK request is unchanged and un-retried; CrP was absent from it. G2 outbound/return and S2 deliberate contact/withdrawal inform this short grounded action. Existing 4/2/6 and all sim geometry stay fixed.

## V1 exact prompt

References (viewed): `kogan-crouch-saber-v1-green.png` (identity/material) and `kogan-ground-v4-green.png` (low crouch). Built-in imagegen. V1 accepted after complete runtime review.

```text
Use case: stylized-concept. Asset type: production sprite atlas for Aeon's existing Kogan.
Create FOUR chronological full-body drawings of Kogan's CROUCHING PUNCH, arranged exactly TWO columns by TWO rows on a square canvas. The supplied images are identity and rendering references, not edit targets: image 1 fixes the armored anatomy, copper nemes hood, horizontal cyan visor, chest eye, enormous copper cape and right-hand saber; image 2 fixes the low crouch and its return height. Make a new atlas for this one crouching fist action.
Every drawing faces RIGHT in the same orthographic three-quarter side view. Keep the same head size, armor, limb lengths and camera. Kogan remains on a deeply bent rear leg and planted front boot throughout, torso low, never standing in recovery. His LEFT hand makes a compact straight punch at his own crouched upper-chest height. His RIGHT hand continuously retains the same complete cyan saber below the fist line, near his hip with blade pointing forward and slightly downward, its complete tip safely above the boot soles. The saber is quiet, not the attacking limb; retain a single connected hand, hilt and blade. Revolver stays holstered.
Reading order: top left = anticipation, closed left fist gathered beside ribs, elbow folded, low stable support; top right = contact, left fist extended horizontally toward the right in a short straight line, shoulder slightly engaged, elbow almost straight, no overlong reach or upward punch; bottom left = withdrawal, left elbow visibly bent halfway home while shoulders unwind and the cape lags; bottom right = low ready, left hand open and lowered toward the planted front foot, torso and legs returning to image 2's low crouch, saber held near hip along the same forward low line.
Draw changing shoulder, elbow and cape folds across the four keys, preserving the crouching body's anatomical scale and support. Keep the cape LARGE and flowing behind to the LEFT, complete in every cell. High-resolution painted game illustration with controlled copper highlights, dark etched armor and thin cyan accents, matching the supplied drawings. No pixel art or photographic texture.
Uniform flat technical chroma green #00e600 background in every gap. Each complete body, blade, hood and cape must have generous empty green margins on all sides of its own cell; no figure may cross a cell boundary. No opponents, stage, ground line, shadows, text, grid, labels, combat sparks, trails, ghost limbs or motion blur. This is four discrete original character animation drawings, with a clear outbound fist and return path.
```

## Baseline audit

`crp-before/Kogan-CrP/`: all 20 cases/20 seconds viewed at 1× in a 1280×720 browser viewport, with four temporal samples. Paused steps at video ticks 12,13,16,17,25,26,30,32,33,499 and979 show low entry, premature held contact, hit freeze, unchanged extended fist during recovery, upward return bob, low guard overlap and spaced miss. Browser HUD can land one tick adjacent; trace provides exact boundaries.

All non-whiff contacts occur at HUD tick17: eight hits for35 damage, eight zero-damage guards; four whiffs have no events. Hit contact holds frame4 through tick24; frame5 at25, withdrawal time26–28, final ready time29–31, control returns at32. Whiff finishes at25 without seven hitstop ticks. The legacy fist and saber are hard clipped in the source, and its large forward torso obscures crouched Raya. No sim issue was found.

V1 returned by `exec-d80b18c4-94d3-473e-b0a6-7d4df75065ca`, original retained in Codex generated images and copied unchanged to `kogan-crouch-punch-v1-green.png` (1254×1254). Four complete drawings inspected: gathered fist, horizontal extension, bent withdrawal and open low ready. Regions split at x625/y620. Keyed opaque bounds are [36,127,609,487], [634,127,1200,485], [35,712,609,1070], [630,750,1206,1089]. All whole blades and cape tips are inside the extraction gutters. Candidate roots355/955, shared anatomical height510, measured solid bottoms. This calibration was retained after gameplay-size contact/guard/return review.

Selector uses startup0–3, active4–5, withdrawal6–8, low ready9–11; it holds the current key during hitstop and yields immediately to other actions. Low return history suppresses the previous upward entry bob. Authored drawings receive no extra rotation/squash/translation or old-cell ghost.

## Accepted runtime review

All20 V1 cases/20s viewed at1× in1280×720 with four temporal samples. Targeted video ticks13/17/26/29/32 cover gathered startup, active contact, bent withdrawal, low ready and immediately actionable crouch. Additional steps137/197 inspect both corner hits;259 standing guard;499/559/619/679 all four crouching guards;799/919 mirrored crouched hits;977/979/982/985 spaced contact/withdraw/ready/return and1105/1165 both corner misses. Heads, fingers, blade tips, support and cape are complete; low opponent torso remains visible. The new attack's slight preparation rise and low release are deliberate joint changes; the former artificial entry bob is removed. No recapture or art revision was needed. Candidate was byte-copied to final. Raya's existing reaction ghosts remain a separate open item.
