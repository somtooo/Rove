#!/usr/bin/env python3
"""Grid reference verification (docs/09-verification.md), run with `python3 -I tools/verify.py`.

For each scene it renders `--snap` (2x) and `--tree`, then checks:
  1. geometry: keyed tree rects against reference/redlines.json, within +-1 px;
  2. colour spot checks in Solid scenes, +-1 per channel;
  3. pixels: `magick compare -metric AE -fuzz 6%` against the reference PNG, with the context
     menu region (absent until the first menu-backed feature) and every text rect masked out
     (text is checked by position, size and colour, not pixels).
Writes out/grid-<scene>@2x.png, out/cmp-<scene>.png (snap over the backdrop) and
out/diff-<scene>.png.
"""
import json
import os
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HANDOFF = os.path.join(ROOT, ".plan")
REF = os.path.join(HANDOFF, "reference")
OUT = os.path.join(ROOT, "out")
BIN = os.path.join(ROOT, "target", "debug", "rove")
FRAME = (40, 40)  # the window's place in a reference frame
# The context menu and submenu, window-relative (masked until a menu-backed slice lands).
MENU = (572, 364, 1058, 782)
# Redline keys that belong to later milestones.
LATER = ("menu", "submenu")

SCENES = ["dark-glass", "dark-solid", "latte-glass", "latte-solid"]

# Spot checks at window-relative points. The selected tile is checked against its own
# computed value: the reference's #2A3147 / #E1EAFD include the context menu's shadow (the
# point is 42 px left of the menu), which the current grid doesn't draw.
SPOTS = {
    "dark-solid": [
        ("sidebar background", (100, 760), "0F1013"),
        ("panel background", (1000, 700), "17181C"),
        ("selected tile fill (accent .20 over panel, no menu shadow)", (530, 450), "2B3249"),
    ],
    "latte-solid": [
        ("sidebar background", (100, 760), "ECEDF1"),
        ("panel background", (1000, 700), "FFFFFF"),
        ("selected tile fill (accent .13 over white, no menu shadow)", (530, 450), "E2EBFE"),
    ],
}


def run(*args):
    r = subprocess.run(args, capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"{' '.join(args)} failed:\n{r.stderr}")
    return r.stdout


def render(scene):
    snap = os.path.join(OUT, f"grid-{scene}@2x.png")
    tree = os.path.join(OUT, f"tree-{scene}.json")
    run(BIN, "--snap", snap, "--scene", scene, "--size", "1440x920", "--scale", "2")
    run(BIN, "--tree", tree, "--scene", scene, "--size", "1440x920", "--scale", "2")
    return snap, json.load(open(tree))


def keyed(tree):
    """key -> window-relative rect, first element with that key."""
    out = {}
    for e in tree["elements"]:
        k = e.get("key")
        if k and k not in out:
            x, y, w, h = e["rect"]
            out[k] = (x - FRAME[0], y - FRAME[1], w, h)
    return out


def check_geometry(tree):
    red = json.load(open(os.path.join(REF, "redlines.json")))["grid-dark-glass"]
    rects = keyed(tree)
    rows, worst = [], 0.0
    for key, r in red.items():
        if key.startswith(LATER):
            continue
        want = (r["x"], r["y"], r["w"], r["h"])
        got = rects.get(key)
        if got is None:
            rows.append((key, want, None, None))
            worst = float("inf")
            continue
        err = max(abs(a - b) for a, b in zip(got, want))
        worst = max(worst, err)
        rows.append((key, want, got, err))
    return rows, worst


def sample(png, x, y):
    X, Y = x * 2 + FRAME[0] * 2, y * 2 + FRAME[1] * 2
    return run("magick", png, "-format", f"%[hex:p{{{X},{Y}}}]", "info:").strip()[:6].upper()


def close(a, b, tol=1):
    return all(abs(int(a[i:i + 2], 16) - int(b[i:i + 2], 16)) <= tol for i in (0, 2, 4))


def pixel_diff(scene, snap, tree):
    """Composite the snap over the backdrop, mask the menu and text, count differing pixels."""
    mode = "latte" if scene.startswith("latte") else "dark"
    cmp_png = os.path.join(OUT, f"cmp-{scene}.png")
    run("magick", os.path.join(REF, "backdrops", f"{mode}@2x.png"), snap, "-composite", cmp_png)
    ref = os.path.join(REF, "screens", f"grid-{scene}@2x.png")
    draws = []
    mx0, my0, mx1, my1 = MENU
    draws.append(f"rectangle {(mx0 + 40) * 2},{(my0 + 40) * 2} {(mx1 + 40) * 2},{(my1 + 40) * 2}")
    n_text = 0
    for e in tree["elements"]:
        if e["role"] == "text":
            x, y, w, h = e["rect"]
            # Pad by 2 pt: anti-aliasing and the reference's slightly different advance widths.
            draws.append(f"rectangle {int((x - 2) * 2)},{int((y - 2) * 2)} {int((x + w + 2) * 2)},{int((y + h + 2) * 2)}")
            n_text += 1
    mask_args = []
    for d in draws:
        mask_args += ["-draw", d]
    masked = {}
    for name, src in (("ref", ref), ("out", cmp_png)):
        dst = os.path.join(OUT, f"masked-{name}-{scene}.png")
        run("magick", src, "-fill", "#FF00FF", *mask_args, dst)
        masked[name] = dst
    diff_png = os.path.join(OUT, f"diff-{scene}.png")
    r = subprocess.run(["magick", "compare", "-metric", "AE", "-fuzz", "6%", masked["ref"], masked["out"], diff_png],
                       capture_output=True, text=True)
    ae = float(r.stderr.split()[0])
    return ae, 2880 * 1840


def main():
    scenes = sys.argv[1:] or SCENES
    os.makedirs(OUT, exist_ok=True)
    ok = True
    for scene in scenes:
        print(f"\n=== {scene}")
        snap, tree = render(scene)
        rows, worst = check_geometry(tree)
        bad = [r for r in rows if r[3] is None or r[3] > 1.0]
        print(f"geometry: {len(rows) - len(bad)}/{len(rows)} keys within ±1 px (worst {worst:.2f})")
        for key, want, got, err in bad:
            print(f"  ✗ {key}: want {want} got {got} (err {err})")
        ok &= not bad
        for name, pt, want in SPOTS.get(scene, []):
            got = sample(snap, *pt)
            good = close(got, want)
            ok &= good
            print(f"  {'✓' if good else '✗'} {name} {pt}: #{got} (want #{want})")
        ae, total = pixel_diff(scene, snap, tree)
        pct = 100 * ae / total
        if "solid" in scene:
            good = pct < 0.5
            ok &= good
            print(f"pixels: {int(ae)} differ outside text and menu = {pct:.3f}% {'✓' if good else '✗'} (target < 0.5%)")
        else:
            print(f"pixels: {pct:.3f}% differ (Glass: the reference blurs the backdrop; checked by eye and geometry)")
    print("\nALL CHECKS PASS" if ok else "\nSOME CHECKS FAIL")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
