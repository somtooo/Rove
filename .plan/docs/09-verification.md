# Verification: proving it matches

## Render headlessly
```sh
cargo build -p rove-ui
for s in dark-glass dark-solid latte-glass latte-solid; do
  target/debug/rove --snap out/grid-$s@2x.png --scene $s --size 1440x920 --scale 2
done
target/debug/rove --tree out/tree.json --scene dark-solid --size 1440x920   # geometry check
```
- The reference frames are **1440×920 logical, rendered at 2×**. The window is at (40, 40), 1360×840.
- **Desktop backdrop:** Rune's headless snap shows only what the app paints. Rove's Glass `base` is translucent, so composite your snap over the same backdrop before comparing:
  - `reference/backdrops/dark@2x.png` (the dusk landscape)
  - `reference/backdrops/latte@2x.png` (the Latte wallpaper)
  - Example: `magick backdrop.png snap.png -composite cmp.png`

## Compare
1. **Geometry first.** Compare `--tree` rects for keyed elements with `reference/redlines.json`. Give your elements the same names as the keys there (`sidebar.row`, `toolbar.search`, `tile.selected`, …). Tolerance: **±1 px** on x, y, w and h.
2. **Pixels second.** For Solid scenes, diff against the reference PNG:
   ```sh
   magick compare -metric AE -fuzz 6% ref.png out.png diff.png   # count of differing pixels
   ```
   Target: under 0.5% of pixels differ outside text. Text anti-aliasing differs between Chromium and Rune, so check text by **position, size, weight and colour**, not pixels. Glass scenes: the CSS reference blurs the wallpaper; Rune headless doesn't. Compare those by eye and by geometry only.
3. **Colour spot checks.** Sample these in a Solid snap. Each must be exact (±1 per channel):

| Point (window-relative, 1×; ×2 and +80 for the @2x PNG) | dark-solid | latte-solid |
|---|---|---|
| sidebar background (100, 760) | #0F1013 | #ECEDF1 |
| panel background (1000, 700) | #17181C | #FFFFFF |
| menu background (700, 640) | #212328 | #FFFFFF |
| highlighted menu row "Open With" (700, 415) | #7C9CFF (accent) | #1E66F5 (accent) |
| selected tile fill (530, 450) | #2A3147 (accent @ .20 over panel) | #E1EAFD (accent @ .13 over white) |

All values above were sampled from the reference PNGs.
(Compute any other composite with sRGB-encoded source-over: `out = a·fg + (1−a)·bg` per 8-bit channel.)

## Look at it
After the numbers pass, put your snap and the reference side by side at 100% and look. Check:
- spacing rhythm
- optical alignment of icons and text
- that nothing is bolder or greyer than the reference

If something differs and the docs are ambiguous, the reference HTML is the truth: open `reference/html/<screen>.html` in Chromium and inspect the element.
