//! Sidebar: brand, Favourites, Tags, Network, Devices (04-layout.md › Sidebar).

use rune::{El, col, el, icon, row, spacer};

use crate::icons;
use crate::mock::{self, RemoteState};
use crate::theme::{Tokens, type_};
use crate::views::{icon_button, label, nth};

pub fn view(t: &Tokens) -> El {
    let mut side = col().key("sidebar").w(244.0).min_w(244.0).h_full()
        .pt(12.0).pr(10.0).pb(12.0).pl(12.0)
        .child(brand(t))
        .child(section("sidebar.section-label", "Favourites", None, t));
    let mut plain = 0;
    for (i, place) in mock::FAVOURITES.iter().enumerate() {
        let selected = i == mock::CURRENT_PLACE;
        let key = if selected { "sidebar.row.selected".to_string() } else { plain += 1; nth("sidebar.row", plain - 1) };
        side = side.child(place_row(key, place.icon, place.name, selected, None, t));
    }
    side = side.child(section("sidebar.section-label#1", "Tags", Some((&icons::MORE, 16.0)), t)).child(tag_dots(t));
    side = side.child(section("sidebar.section-label#2", "Network", Some((&icons::PLUS, 14.0)), t));
    for (i, remote) in mock::NETWORK.iter().enumerate() {
        let end = match remote.state {
            RemoteState::Connected => el().size(8.0, 8.0).radius(4.0).fill(t.success),
            RemoteState::Offline(s) => label(s, type_::ROW_END, t.t3),
        };
        side = side.child(place_row(nth("sidebar.row.net", i), remote.icon, remote.name, false, Some(end), t));
    }
    side = side.child(section("sidebar.section-label#3", "Devices", None, t));
    for (i, dev) in mock::DEVICES.iter().enumerate() {
        side = side.child(device(i, dev, t));
    }
    side
}

fn brand(t: &Tokens) -> El {
    row().key("sidebar.brand").h(36.0).pl(10.0).pr(2.0).items_center().justify_between()
        .child(label("Rove", type_::BRAND, t.t1))
        .child(icon_button("sidebar.toggle", &icons::SIDEBAR, 30.0, 16.0, t.t2))
}

/// A section label (`.sec`): padding 14 10 6, 12/16/500 t3, optional 20×20 trailing button.
fn section(key: &str, name: &str, button: Option<(&'static rune::text::Icon, f32)>, t: &Tokens) -> El {
    let mut sec = row().key(key).pt(14.0).px(10.0).pb(6.0).items_center().justify_between()
        .child(label(name, type_::SECTION, t.t3));
    if let Some((i, size)) = button {
        sec = sec.child(icon_button(format!("{key}.button"), i, 20.0, size, t.t2));
    }
    sec
}

/// A place row (`.row`): 32 high, padding 0 10, gap 10, radius 8; icon 18 t3, label 14 t2.
fn place_row(key: String, i: &'static rune::text::Icon, name: &str, selected: bool, end: Option<El>, t: &Tokens) -> El {
    let (fill, icon_color, text_color, style) = if selected {
        (t.row_selected, t.accent, t.t1, type_::BODY_500)
    } else {
        (rune::Color::TRANSPARENT, t.t3, t.t2, type_::BODY)
    };
    let icon_key = if key == "sidebar.row" { "sidebar.row.icon".to_string() } else { format!("{key}.icon") };
    let mut r = row().key(key).w_full().h(32.0).px(10.0).gap(10.0).radius(8.0).items_center().fill(fill)
        .child(icon(i, 18.0).key(icon_key).color(icon_color))
        .child(label(name, style, text_color));
    if let Some(end) = end {
        r = r.child(spacer()).child(row().items_center().gap(6.0).child(end));
    }
    r
}

fn tag_dots(t: &Tokens) -> El {
    let mut dots = row().key("sidebar.tags").pt(4.0).px(10.0).pb(6.0).gap(10.0);
    for (i, c) in t.tags.iter().enumerate() {
        dots = dots.child(tag_dot(nth("sidebar.tag-dot", i), *c));
    }
    dots
}

/// A 14 px tag dot with an inset 1 px black @ .2 ring.
pub fn tag_dot(key: String, c: rune::Color) -> El {
    el().key(key).size(14.0, 14.0).radius(7.0).fill(c)
        .box_shadow(crate::theme::inset_ring(1.0, rune::Color::BLACK.alpha(0.2)))
}

/// A device block (`.dev`): padding 7 10 9, gap 6; name row, capacity bar, caption.
fn device(i: usize, dev: &mock::Device, t: &Tokens) -> El {
    let mut top = row().items_center().gap(10.0)
        .child(icon(dev.icon, 18.0).color(t.t3))
        .child(label(dev.name, type_::BODY, t.t2).grow(1.0));
    if dev.removable {
        top = top.child(icon_button(format!("{}.eject", nth("sidebar.device", i)), &icons::EJECT, 22.0, 14.0, t.t2));
    }
    let bar_fill = if dev.used > 0.85 { t.warning } else { t.accent };
    let bar = el().key(nth("sidebar.device.bar", i)).w_full().h(4.0).radius(2.0).fill(t.capacity_track).clip_rounded(2.0)
        .child(el().w_frac(dev.used).h_full().radius(2.0).fill(bar_fill));
    col().key(nth("sidebar.device", i)).pt(7.0).px(10.0).pb(9.0).gap(6.0).radius(8.0)
        .child(top)
        .child(bar)
        .child(label(dev.caption, type_::CAPTION, t.t3).key(nth("sidebar.device.caption", i)))
}
