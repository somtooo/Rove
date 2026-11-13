#!/usr/bin/env python3
"""Ink extents of a region in the reference vs Rove, in device pixels (2x).

usage: python3 -I tools/ink.py SCENE X Y W H   (window-relative logical px)
Prints the first/last rows and columns whose luminance differs from the region's corner
pixel by more than a threshold, for both images. Text is checked by position this way.
"""
import os
import subprocess
import sys

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
scene, x, y, w, h = sys.argv[1], *map(int, sys.argv[2:6])
thr = int(sys.argv[6]) if len(sys.argv) > 6 else 40
ref = os.path.join(root, ".plan/reference/screens", f"grid-{scene}@2x.png")
mine = os.path.join(root, "out", f"cmp-{scene}.png")


def gray(path):
    geo = f"{w*2}x{h*2}+{(x+40)*2}+{(y+40)*2}"
    raw = subprocess.run(["magick", path, "-crop", geo, "+repage", "-colorspace", "gray", "-depth", "8", "gray:-"],
                         capture_output=True, check=True).stdout
    W = w * 2
    return [raw[i * W:(i + 1) * W] for i in range(h * 2)]


def extents(img):
    bg = img[0][0]
    rows = [i for i, r in enumerate(img) if any(abs(p - bg) > thr for p in r)]
    cols = [j for j in range(len(img[0])) if any(abs(r[j] - bg) > thr for r in img)]
    if not rows:
        return None
    # ink "mass" centre, for sub-pixel comparisons
    tot = sx = sy = 0
    for i, r in enumerate(img):
        for j, p in enumerate(r):
            d = abs(p - bg)
            tot += d; sx += d * j; sy += d * i
    return rows[0], rows[-1], cols[0], cols[-1], round(sx / tot, 2), round(sy / tot, 2), tot


for name, path in (("ref ", ref), ("rove", mine)):
    e = extents(gray(path))
    print(name, "rows %d-%d cols %d-%d centre (%.2f, %.2f) ink %d" % e if e else "no ink")
