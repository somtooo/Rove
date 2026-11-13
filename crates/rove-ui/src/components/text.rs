//! Shared text component: labels placed on Chromium's baselines.
//!
//! The reference PNGs were rendered by Chromium on Linux, which places a line's baseline from
//! font metrics taken at CSS px: ascent and descent rounded to whole pixels, one pixel moved from
//! ascent to descent when the descent rounded down (subpixel positioning), the half-leading
//! floored, and the glyph baseline snapped to a whole CSS px. Rune places it the CSS way from
//! metrics rounded at device pixels. The two differ by up to 1.5 pt for the styles Rove uses
//! (14/20 text sits 1 pt lower in Rune, for example).
//!
//! `label` moves the glyphs onto Chromium's baseline without changing the label's box: it
//! trims the line height by twice the shift and pads the box back to its full height.

use std::cell::Cell;

use rune::tok::TextStyle;
use rune::{Color, El, text};

/// Inter's vertical metrics (hhea and OS/2 typo agree): 1984 and 494 units of 2048.
const ASCENT: f32 = 1984.0 / 2048.0;
const DESCENT: f32 = 494.0 / 2048.0;

thread_local! {
    /// Device pixels per point for the view being built (set once per view by `window::view`).
    static SCALE: Cell<f32> = const { Cell::new(2.0) };
}

pub fn set_scale(scale: f32) {
    SCALE.with(|s| s.set(if scale > 0.0 { scale } else { 1.0 }));
}

fn scale() -> f32 {
    SCALE.with(Cell::get)
}

/// Chromium's baseline, in CSS px from the line box's top.
pub fn chromium_baseline(style: &TextStyle) -> f32 {
    let (a, d) = (ASCENT * style.size, DESCENT * style.size);
    let (mut ra, mut rd) = (a.round(), d.round());
    if rd < d && ra >= 1.0 {
        rd += 1.0;
        ra -= 1.0;
    }
    ra + ((style.line_px() - ra - rd) / 2.0).floor()
}

/// Rune's baseline, in points from the line box's top, at `k` device pixels per point
/// (Parley's quantized box metrics; Rune then rounds the baseline to a device pixel).
pub fn rune_baseline(style: &TextStyle, k: f32) -> f32 {
    let (a, d) = ((ASCENT * style.size * k).round(), (DESCENT * style.size * k).round());
    (a + ((style.line_px() * k - a - d) / 2.0).floor()) / k
}

/// A label on Chromium's baseline. `top_frac` is the fractional CSS px of the box's top in the
/// window (0 for most; 0.5 for a 20 pt line centred in an odd-height bar), since Chromium snaps
/// the absolute baseline to whole CSS px.
pub fn label_at(s: &str, style: TextStyle, color: Color, top_frac: f32) -> El {
    let k = scale();
    let target = (top_frac + chromium_baseline(&style)).round() - top_frac;
    // Shift in whole device pixels.
    let shift = ((target - rune_baseline(&style, k)) * k).round() / k;
    if shift == 0.0 {
        return text(s).font(style).color(color);
    }
    let trim = shift.abs();
    let line = style.line_px() - 2.0 * trim;
    text(s).font(style.lh(line)).color(color).pt(trim + shift).pb(trim - shift)
}

/// A label on Chromium's baseline, its box on whole CSS px.
pub fn label(s: &str, style: TextStyle, color: Color) -> El {
    label_at(s, style, color, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::type_;

    /// Baselines measured from the reference PNGs (device px at 2× from the line box top).
    #[test]
    fn chromium_model_matches_the_reference() {
        let cases = [
            (type_::BODY, 28.0),      // sidebar rows
            (type_::BRAND, 30.0),     // "Rove"
            (type_::SECTION, 24.0),   // "Favourites"
            (type_::TILE_NAME, 24.0), // tile names
            (type_::GROUP, 28.0),     // "Folders"
            (type_::LABEL, 28.0),     // tabs
            (type_::TITLE, 40.0),     // "Documents"
            (type_::TILE_META, 22.0), // "48 items"
        ];
        for (style, dev) in cases {
            assert_eq!(chromium_baseline(&style) * 2.0, dev, "{style:?}");
        }
    }

    #[test]
    fn the_shift_lands_rune_on_chromium() {
        for style in [type_::BODY, type_::TILE_NAME, type_::GROUP, type_::FOOT] {
            let k = 2.0;
            let shift = chromium_baseline(&style) - rune_baseline(&style, k);
            let trimmed = style.lh(style.line_px() - 2.0 * shift.abs());
            let moved = rune_baseline(&trimmed, k) + shift + shift.abs();
            assert_eq!(moved, chromium_baseline(&style), "{style:?}");
        }
    }
}
