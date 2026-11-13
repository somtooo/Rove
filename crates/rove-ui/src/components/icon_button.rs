//! Shared square icon-button component (`.ib`).

use rune::text::Icon;
use rune::{Color, El, el, icon};

pub fn icon_button(key: impl Into<String>, i: &'static Icon, box_size: f32, icon_size: f32, color: Color) -> El {
    el().key(key).size(box_size, box_size).radius(8.0).center().child(icon(i, icon_size).color(color))
}
