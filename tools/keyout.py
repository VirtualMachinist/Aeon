#!/usr/bin/env python3
"""Chroma-key generated poses and compose them onto the fixed sprite canvas.

Usage: keyout.py <body> <pose> <input.png> [--scale F] [--fit-width]

Every pose lands on the same 800x800 canvas with the feet row at y=752, so the
client can draw all poses with one size anchored at the feet. Standing poses
are scaled to 620px tall; crouches to ~420; a lying pose is fitted by width.

Writes to crates/client/assets/<body>/<pose>.png (game) and
../../art/fight-ready/<body>/<pose>.png (vault provenance copy).
"""

import sys
from pathlib import Path

from PIL import Image, ImageFilter

CANVAS = 800
FEET_ROW = 752
STAND_H = 620

ROOT = Path(__file__).resolve().parents[1]
VAULT_ART = ROOT.parents[1] / "art" / "fight-ready"


def key_green(img: Image.Image) -> Image.Image:
    img = img.convert("RGBA")
    px = img.load()
    w, h = img.size
    for y in range(h):
        for x in range(w):
            r, g, b, a = px[x, y]
            # Pure-ish green: strongly green-dominant.
            dom = g - max(r, b)
            if dom > 90 and g > 140:
                px[x, y] = (r, g, b, 0)
            elif dom > 40 and g > 110:
                # Edge: partial alpha and despill the green fringe.
                t = (dom - 40) / 50.0
                alpha = int(255 * (1.0 - t))
                g2 = max(r, b)
                px[x, y] = (r, g2, b, alpha)
    # Despill any remaining green cast on semi-transparent edges.
    return img


def bbox_alpha(img: Image.Image):
    alpha = img.split()[3]
    # Ignore faint fringe when finding the bbox.
    solid = alpha.point(lambda a: 255 if a > 40 else 0)
    return solid.getbbox()


def compose(img: Image.Image, scale: float, fit_width: bool) -> Image.Image:
    box = bbox_alpha(img)
    if not box:
        raise SystemExit("nothing left after keying")
    fig = img.crop(box)
    fw, fh = fig.size
    if fit_width:
        target_w = int(STAND_H * scale)
        ratio = target_w / fw
    else:
        target_h = int(STAND_H * scale)
        ratio = target_h / fh
    new = (max(1, int(fw * ratio)), max(1, int(fh * ratio)))
    fig = fig.resize(new, Image.LANCZOS)
    canvas = Image.new("RGBA", (CANVAS, CANVAS), (0, 0, 0, 0))
    x = (CANVAS - new[0]) // 2
    y = FEET_ROW - new[1]
    canvas.alpha_composite(fig, (x, y))
    return canvas


def main():
    args = sys.argv[1:]
    if len(args) < 3:
        print(__doc__)
        sys.exit(1)
    body, pose, src = args[0], args[1], Path(args[2])
    scale = 1.0
    fit_width = False
    if "--scale" in args:
        scale = float(args[args.index("--scale") + 1])
    if "--fit-width" in args:
        fit_width = True
    img = key_green(Image.open(src))
    out = compose(img, scale, fit_width)
    game = ROOT / "crates" / "client" / "assets" / body / f"{pose}.png"
    vault = VAULT_ART / body / f"{pose}.png"
    for p in (game, vault):
        p.parent.mkdir(parents=True, exist_ok=True)
        out.save(p, optimize=True)
    print(f"{body}/{pose}: {src.name} -> {game.relative_to(ROOT)} (+vault)")


if __name__ == "__main__":
    main()
