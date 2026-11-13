//! Views. Every colour comes from `theme::Tokens`; no literal colours here.
//!
//! Keys follow `reference/redlines.json` so `--tree` output can be checked against it. Where the
//! reference has several elements of one kind, the first takes the bare key (`sidebar.row`) and
//! the others take `#n` suffixes (`sidebar.row#2`).

pub mod footer;
pub mod grid;
pub mod sidebar;
pub mod tabs;
pub mod text;
pub mod toolbar;
pub mod window;

use rune::text::Icon;
use rune::{Color, El, el, icon, row};

use crate::theme::{Tokens, type_};
pub use text::{label, label_at};

/// A key for the `n`th element of a kind: the bare key for the first, `key#n` after.
pub fn nth(key: &str, n: usize) -> String {
    if n == 0 { key.to_string() } else { format!("{key}#{n}") }
}

/// A square icon button (`.ib`): `size` box, radius 8, icon centred.
pub fn icon_button(key: impl Into<String>, i: &'static Icon, box_size: f32, icon_size: f32, color: Color) -> El {
    el().key(key).size(box_size, box_size).radius(8.0).center().child(icon(i, icon_size).color(color))
}

/// A keycap (`.kbd`): min 20×20, padding 0 5, radius 5, 1 px border with a 2 px bottom.
pub fn keycap(key: impl Into<String>, s: &str, t: &Tokens) -> El {
    row().key(key).h(20.0).min_w(20.0).px(5.0).radius(5.0).center()
        .fill(t.keycap_bg)
        .border_sides([1.0, 1.0, 2.0, 1.0], t.keycap_border)
        .border_layout()
        // The 10.5 pt line sits 3.25 pt into the 17 pt content box (20 less the borders).
        .child(label_at(s, type_::KEYCAP, t.t3, 0.25))
}
