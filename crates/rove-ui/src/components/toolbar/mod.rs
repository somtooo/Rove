//! Toolbar: nav group, title and count, search, view switch, sort, settings (04-layout.md › Toolbar).

use rune::{El, icon, row, spacer};

use crate::icons;
use crate::app::State;
use crate::fixtures;
use crate::theme::{Tokens, type_};
use crate::components::{icon_button, keycap, label, nth, text};

pub fn view(s: &State) -> El {
    let t = &s.tokens;
    let mut toolbar = row().key("toolbar").h(56.0).min_h(56.0).pl(14.0).pr(12.0).gap(10.0).items_center();
    if !s.sidebar_visible {
        toolbar = toolbar.child(icon_button("sidebar.show", &icons::SIDEBAR, 30.0, 16.0, t.t2)
            .role("button").label("Show sidebar").focusable(true).hover(t.hover)
            .on_press(|s: &mut State| s.sidebar_visible = true));
    }
    toolbar
        .child(nav_group(t))
        .child(title(t))
        .child(spacer())
        .child(search(t))
        .child(view_switch(t))
        .child(sort_button(t))
        .child(icon_button("toolbar.settings", &icons::SLIDERS, 30.0, 16.0, t.t2))
}

/// A 2 px padded track (`.grp`): fill, radius 10, gap 2.
fn track(key: &str, t: &Tokens) -> El {
    row().key(key).padding(2.0).gap(2.0).radius(10.0).fill(t.fill).items_center()
}

fn nav_group(t: &Tokens) -> El {
    track("toolbar.nav-group", t)
        .child(icon_button("toolbar.icon-button", &icons::CHEV_L, 30.0, 18.0, t.t2))
        // Forward: dimmed, there is no forward history.
        .child(icon_button(nth("toolbar.icon-button", 1), &icons::CHEV_R, 30.0, 18.0, t.t4))
        .child(icon_button(nth("toolbar.icon-button", 2), &icons::UP, 30.0, 16.0, t.t2))
}

/// The folder title and its count, on one baseline (`align-items: baseline`). The title's
/// 26 pt line sets the block's height and its baseline (20 pt down); the count's 16 pt line
/// sits where its own baseline meets it, as the reference measures (box at 32, ink on 45).
fn title(t: &Tokens) -> El {
    let title_base = text::chromium_baseline(&type_::TITLE);
    let count_top = title_base - text::rune_baseline(&type_::COUNT, 2.0);
    row().key("toolbar.title-block").ml(6.0).gap(10.0).items_start().min_w(0.0)
        .child(label(fixtures::FOLDER_TITLE, type_::TITLE, t.t1).key("toolbar.title").window_drag())
        .child(rune::text(fixtures::ITEM_COUNT).font(type_::COUNT).color(t.t3).key("toolbar.count").mt(count_top))
}

fn search(t: &Tokens) -> El {
    row().key("toolbar.search").w(232.0).h(32.0).pl(10.0).pr(6.0).gap(8.0).radius(10.0).fill(t.fill).items_center()
        .child(icon(&icons::SEARCH, 16.0).color(t.t3))
        .child(label(&format!("Search {}", fixtures::FOLDER_TITLE), type_::LABEL, t.t3).grow(1.0).truncate())
        .child(row().gap(3.0).items_center()
            .child(keycap("toolbar.search.keycap", "Ctrl", t))
            .child(keycap("toolbar.search.keycap#1", "F", t)))
}

fn view_switch(t: &Tokens) -> El {
    let views = [&icons::LIST, &icons::COLUMNS, &icons::GRID, &icons::DUAL];
    let mut grp = track("toolbar.view-switch", t);
    for (i, ic) in views.into_iter().enumerate() {
        let on = i == 2;
        grp = grp.child(if on {
            let mut b = icon_button("toolbar.view-switch.on", ic, 30.0, 16.0, t.t1).fill(t.fill3);
            for s in &t.shadows.control_on { b = b.box_shadow(*s); }
            b
        } else {
            icon_button(nth("toolbar.view-switch.off", i), ic, 30.0, 16.0, t.t2)
        });
    }
    grp
}

/// Ghost button (`.btn.ghost`): 32 high, padding 0 12, gap 7, radius 9, 13/500 t2.
fn sort_button(t: &Tokens) -> El {
    row().key("toolbar.sort-button").h(32.0).px(12.0).gap(7.0).radius(9.0).items_center().justify_center()
        .child(icon(&icons::SORT, 16.0).color(t.t2))
        .child(label(fixtures::SORT_KEY, type_::LABEL_500, t.t2))
        .child(icon(&icons::CHEV_D, 14.0).color(t.t2))
}
