//! Grid view content: grouped tiles (04-layout.md › Content (Grid view)).
//!
//! Static for Milestone 1: the mock's 16 items, laid out with flex-wrap. Milestone 6 virtualizes
//! by row of tiles (`.virtualize()`), once real folders arrive from the backend.

use rune::{El, col, el, icon, row};

use crate::State;
use crate::assets::Assets;
use crate::icons;
use crate::mock::{self, Art, Item, TileState};
use crate::theme::{Mode, Tokens, type_};
use crate::views::{label, nth};

pub const TILE_W: f32 = 124.0;

/// Running counters so each kind of keyed element gets `key`, `key#1`, …
#[derive(Default)]
struct Keys {
    rest: usize,
    icon: usize,
    name: usize,
    meta: usize,
    thumb: usize,
}

pub fn view(s: &State) -> El {
    let t = &s.tokens;
    let mut keys = Keys::default();
    let groups: [(&str, &[Item]); 2] = [("Folders", &mock::FOLDERS), ("Files", &mock::FILES)];
    let mut content = col().key("content").grow(1.0).min_h(0.0).pt(6.0).px(16.0).pb(16.0).gap(6.0);
    for (g, (name, items)) in groups.into_iter().enumerate() {
        let mut header = group_header(g, name, items.len(), t);
        if g > 0 {
            header = header.mt(14.0);
        }
        let mut tiles = row().key(nth("tiles", g)).flex_wrap().gap(6.0).items_start();
        for item in items {
            tiles = tiles.child(tile(item, s, &mut keys));
        }
        content = content.child(header).child(tiles);
    }
    content
}

/// Group header: 28 high, gap 8; label 13/600 t2, count 13/500 t4, a 1 px rule filling the rest.
fn group_header(g: usize, name: &str, count: usize, t: &Tokens) -> El {
    row().key(nth("group-header", g)).h(28.0).min_h(28.0).gap(8.0).items_center()
        .child(label(name, type_::GROUP, t.t2))
        .child(label(&count.to_string(), type_::GROUP_COUNT, t.t4).key(nth("group-header.count", g)))
        .child(el().grow(1.0).h(1.0).fill(t.line))
}

/// A tile: 124 wide, padding 12 8 10, centred column, gap 8, radius 12.
fn tile(item: &Item, s: &State, keys: &mut Keys) -> El {
    let t = &s.tokens;
    let assets = s.assets.as_ref();
    let state = if Some(item.name) == s.selected {
        TileState::Selected
    } else if s.frame.is_some() && item.state == TileState::Hover {
        TileState::Hover
    } else {
        TileState::Rest
    };
    let key = match item.state {
        TileState::Selected => "tile.selected".to_string(),
        TileState::Hover => "tile.hover".to_string(),
        TileState::Rest => { keys.rest += 1; nth("tile", keys.rest - 1) }
    };
    let name_key = { keys.name += 1; nth("tile.name", keys.name - 1) };
    let meta_key = { keys.meta += 1; nth("tile.meta", keys.meta - 1) };
    let mut tile = col().key(key).w(TILE_W).min_w(TILE_W).pt(12.0).px(8.0).pb(10.0).gap(8.0).radius(12.0).items_center()
        .child(art(item.art, assets, t, keys))
        .child(label(item.name, type_::TILE_NAME, t.t1).key(name_key).max_w(108.0).truncate())
        .child(label(item.meta, type_::TILE_META, t.t3).key(meta_key).mt(-6.0));
    match state {
        TileState::Rest => {}
        TileState::Hover => tile = tile.fill(t.hover),
        TileState::Selected => {
            tile = tile.fill(t.accent_soft)
                .box_shadow(crate::theme::inset_ring(1.0, t.accent_line))
                .child(check_badge(t));
        }
    }
    let name = item.name;
    tile = tile.role("button").label(name).focusable(true)
        .on_press(move |s: &mut State| s.selected = Some(name));
    if state != TileState::Selected {
        tile = tile.hover(t.hover);
    }
    tile
}

/// The selection check: an 18 px accent circle at top 7 right 7, a tick 11 in ink at stroke 3.2.
fn check_badge(t: &Tokens) -> El {
    el().key("tile.check").pin(None, Some(7.0), Some(7.0), None).size(18.0, 18.0).radius(9.0).fill(t.accent).center()
        .child(icon(&icons::CHECK_BOLD, 11.0).color(t.ink))
}

/// The art box: 68 high, centred; a 64 px icon-theme icon with its drop shadow, or a thumbnail.
fn art(a: Art, assets: Option<&Assets>, t: &Tokens, keys: &mut Keys) -> El {
    let boxed = row().h(68.0).min_h(68.0).items_center().justify_center();
    match a {
        Art::Icon(name) => {
            let icon_key = { keys.icon += 1; nth("tile.icon", keys.icon - 1) };
            let mut slot = el().key(icon_key).size(64.0, 64.0);
            if let Some(ic) = assets.and_then(|a| a.icons.get(name)) {
                let sh = if t.mode == Mode::Dark { &ic.dark_shadow } else { &ic.light_shadow };
                slot = slot
                    .child(el().absolute(sh.offset[0], sh.offset[1]).size(sh.size, sh.size).pixels(sh.image.pixels()))
                    .child(el().absolute(0.0, 0.0).size(64.0, 64.0).pixels(ic.image.pixels()));
            }
            boxed.child(slot)
        }
        Art::Thumb(th) => {
            let thumb_key = { keys.thumb += 1; nth("tile.thumbnail", keys.thumb - 1) };
            let [ring, drop] = t.shadows.thumb;
            let mut thumb = el().key(thumb_key).size(92.0, 62.0).radius(7.0).box_shadow(drop).box_shadow(ring);
            if let Some(img) = assets.and_then(|a| a.thumbs.get(&th)) {
                thumb = thumb.pixels(img.pixels());
            }
            if let Some(d) = th.duration() {
                thumb = thumb
                    .child(row().pin(None, None, Some(4.0), Some(4.0)).h(14.0).px(4.0).radius(4.0).fill(t.media_chip).items_center()
                        .child(label(d, type_::DURATION, t.on_media)))
                    .child(el().absolute(34.0, 19.0).size(24.0, 24.0).radius(12.0).fill(t.media_play).backdrop_blur(6.0).center()
                        .child(icon(&icons::PLAY, 12.0).color(t.on_media).ml(2.0)));
            }
            boxed.child(thumb)
        }
    }
}
