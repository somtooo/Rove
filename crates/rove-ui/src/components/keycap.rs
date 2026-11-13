//! Shared keyboard-shortcut keycap component (`.kbd`).

use rune::{El, row};

use crate::theme::{Tokens, type_};
use super::text::label_at;

/// Min 20×20, padding 0 5, radius 5, 1 px border with a 2 px bottom.
pub fn keycap(key: impl Into<String>, s: &str, t: &Tokens) -> El {
    row().key(key).h(20.0).min_w(20.0).px(5.0).radius(5.0).center()
        .fill(t.keycap_bg)
        .border_sides([1.0, 1.0, 2.0, 1.0], t.keycap_border)
        .border_layout()
        // The 10.5 pt line sits 3.25 pt into the 17 pt content box (20 less the borders).
        .child(label_at(s, type_::KEYCAP, t.t3, 0.25))
}
