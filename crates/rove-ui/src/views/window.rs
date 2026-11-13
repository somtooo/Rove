//! The window base: sidebar | panel. In a real window it fills the surface (Hyprland rounds,
//! borders and shadows it). In reference frames it sits at (40, 40) of a 1440×920 desktop and
//! draws its own radius, border and shadow, as the reference HTML does.

use rune::{Cx, El, col, el, row};

use crate::State;
use crate::theme::Tokens;
use crate::views::{footer, grid, sidebar, tabs, toolbar};

pub const WINDOW_SIZE: (f32, f32) = (1360.0, 840.0);

pub fn view(s: &State, cx: &mut Cx<State>) -> El {
    crate::views::text::set_scale(cx.env().scale);
    let t = &s.tokens;
    let body = row().w_full().h_full().child(sidebar::view(t)).child(panel(s));
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
        .mt(8.0).mr(8.0).mb(8.0)
        .radius(12.0)
        .fill(t.panel)
        .border(1.0, t.line)
        .border_layout()
        .clip_rounded(12.0)
        .child(toolbar::view(t))
        .child(tabs::view(t))
        .child(grid::view(s))
        .child(footer::view(t))
}
