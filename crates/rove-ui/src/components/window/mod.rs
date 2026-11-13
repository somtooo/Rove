//! The window base: sidebar | panel. In a real window it fills the surface (Hyprland rounds,
//! borders and shadows it). In reference frames it sits at (40, 40) of a 1440×920 desktop and
//! draws its own radius, border and shadow, as the reference HTML does.

use rune::{Cx, El, col, el, row};

use crate::app::State;
use crate::theme::Tokens;
use crate::components::{file_grid, footer, sidebar, tabs, text, toolbar};

#[cfg(test)]
mod tests;

pub const WINDOW_SIZE: (f32, f32) = (1360.0, 840.0);

pub fn view(s: &State, cx: &mut Cx<State>) -> El {
    text::set_scale(cx.env().scale);
    let t = &s.tokens;
    let mut body = row().w_full().h_full();
    if s.sidebar_visible {
        body = body.child(sidebar::view(t));
    }
    let body = body.child(panel(s));
    match s.frame {
        Some([x, y]) => {
            let [a, b] = &t.shadows.window;
            let win = row().key("window").absolute(x, y).size(WINDOW_SIZE.0, WINDOW_SIZE.1)
                .radius(16.0)
                .fill(t.base)
                .border(1.0, t.window_border)
                .border_layout()
                .box_shadow(*a)
                .box_shadow(*b)
                .child(body);
            el().w_full().h_full().child(win)
        }
        None => row().key("window").w_full().h_full().fill(t.base).child(body),
    }
}

fn panel(s: &State) -> El {
    let t: &Tokens = &s.tokens;
    col().key("panel").grow(1.0).min_w(0.0).min_h(0.0)
        .mt(8.0).mr(8.0).mb(8.0).ml(if s.sidebar_visible { 0.0 } else { 8.0 })
        .radius(12.0)
        .fill(t.panel)
        .border(1.0, t.line)
        .border_layout()
        .clip_rounded(12.0)
        .child(toolbar::view(s))
        .child(tabs::view(t))
        .child(file_grid::view(s))
        .child(footer::view(s))
}
