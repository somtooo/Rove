//! Tabs bar: always visible, 40 high, rules top and bottom (04-layout.md › Tabs bar).

use rune::{El, el, icon, row};

use crate::icons;
use crate::fixtures;
use crate::theme::{Mode, Tokens, type_};
use crate::components::{icon_button, label, nth};

pub fn view(t: &Tokens) -> El {
    let mut bar = row().key("tabs.bar").h(40.0).min_h(40.0).px(12.0).gap(4.0).items_center()
        .border_sides([1.0, 0.0, 1.0, 0.0], t.line)
        .border_layout();
    let mut plain = 0;
    for (i, tab) in fixtures::TABS.iter().enumerate() {
        let active = i == fixtures::ACTIVE_TAB;
        let key = if active { "tab.active".to_string() } else { plain += 1; nth("tab", plain - 1) };
        bar = bar.child(tab_chip(key, tab, active, fixtures::TABS.len() > 1, t));
    }
    bar.child(icon_button("tabs.new", &icons::PLUS, 26.0, 14.0, t.t2))
}

/// A tab: 30 high, padding 0 8 0 10, gap 8, radius 8; icon 15 t3, 13 t2. Active: fill2 (Light:
/// a white chip with the control shadow), t1 500, and a close button when there are 2+ tabs.
fn tab_chip(key: String, tab: &fixtures::Tab, active: bool, closable: bool, t: &Tokens) -> El {
    let (style, color) = if active { (type_::LABEL_500, t.t1) } else { (type_::LABEL, t.t2) };
    let mut chip = row().key(key.clone()).h(30.0).pl(10.0).pr(8.0).gap(8.0).radius(8.0).items_center()
        .child(icon(tab.icon, 15.0).color(t.t3))
        .child(label(tab.name, style, color));
    if active {
        chip = match t.mode {
            Mode::Dark => chip.fill(t.fill2),
            Mode::Light => {
                let mut c = chip.fill(t.fill3);
                for s in &t.shadows.tab_on { c = c.box_shadow(*s); }
                c
            }
        };
        if closable {
            chip = chip.child(el().key(format!("{key}.close")).size(18.0, 18.0).radius(5.0).center()
                .child(icon(&icons::X, 12.0).color(t.t3)));
        }
    }
    chip
}
