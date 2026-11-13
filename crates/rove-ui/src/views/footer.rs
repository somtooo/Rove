//! Footer: breadcrumb, selection summary, free space, icon-size slider (04-layout.md › Footer).

use rune::{El, el, icon, row, spacer};

use crate::icons;
use crate::mock;
use crate::theme::{Tokens, type_};
use crate::views::{label_at, nth};

/// The footer's 37 pt content box centres 20 pt lines half a point off the pixel grid.
const HALF: f32 = 0.5;

pub fn view(t: &Tokens) -> El {
    row().key("footer").h(38.0).min_h(38.0).pl(8.0).pr(10.0).gap(10.0).items_center()
        .border_sides([1.0, 0.0, 0.0, 0.0], t.line)
        .border_layout()
        .child(crumbs(t))
        .child(spacer())
        .child(label_at(mock::SELECTION_SUMMARY, type_::FOOT_NUM, t.t2, HALF).key("footer.summary"))
        .child(el().size(1.0, 14.0).fill(t.line2))
        .child(label_at(mock::FREE_SPACE, type_::FOOT_NUM.lh(16.0), t.t3, HALF).key("footer.free"))
        .child(slider(t))
}

/// Crumbs: 26 high, padding 0 7, gap 6, radius 7; icon 15 t3, label t2; chevron separators 14 t4.
/// The last is current: fill, t1 500, icon accent.
fn crumbs(t: &Tokens) -> El {
    let mut r = row().key("footer.crumbs").gap(2.0).items_center().min_w(0.0);
    let last = mock::CRUMBS.len() - 1;
    for (i, c) in mock::CRUMBS.iter().enumerate() {
        if i > 0 {
            r = r.child(icon(&icons::CHEV_R, 14.0).color(t.t4));
        }
        let current = i == last;
        let key = if current { "footer.crumb.current".to_string() } else { nth("footer.crumb", i) };
        let (style, color, icon_color) = if current { (type_::FOOT_500, t.t1, t.accent) } else { (type_::FOOT, t.t2, t.t3) };
        let mut crumb = row().key(key).h(26.0).px(7.0).gap(6.0).radius(7.0).items_center()
            .child(icon(c.icon, 15.0).color(icon_color))
            .child(label_at(c.name, style, color, HALF));
        if current {
            crumb = crumb.fill(t.fill);
        }
        r = r.child(crumb);
    }
    r
}

/// The icon-size slider: grid 14, a 72×4 track with the filled part and a 14 px knob, grid 20.
fn slider(t: &Tokens) -> El {
    let k = mock::ICON_SIZE;
    let track = el().key("footer.slider").size(72.0, 4.0).radius(2.0).fill(t.fill2)
        .child(el().absolute(0.0, 0.0).size(72.0 * k, 4.0).radius(2.0).fill(t.t3))
        .child(el().absolute(72.0 * k - 7.0, -5.0).size(14.0, 14.0).radius(7.0).fill(t.knob).box_shadow(t.shadows.knob));
    row().ml(6.0).gap(8.0).items_center()
        .child(icon(&icons::GRID, 14.0).color(t.t3))
        .child(track)
        .child(icon(&icons::GRID, 20.0).color(t.t3))
}
