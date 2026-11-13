//! Grouped file grid (04-layout.md › Content (Grid view)).
//!
//! The preview wraps 16 fixed items. The real-directory slice will virtualize rows of tiles
//! and feed the component a bounded window of entries from Rove's backend fork.

mod group_header;
mod tile;
#[cfg(test)]
mod tests;

use rune::{El, ScrollSpec, col, row};

use crate::app::State;
use crate::components::nth;
use crate::fixtures::{self, Item};

use tile::Keys;

pub fn view(s: &State) -> El {
    let mut keys = Keys::default();
    let groups: [(&str, &[Item]); 2] = [("Folders", &fixtures::FOLDERS), ("Files", &fixtures::FILES)];
    let mut content = col().key("content").grow(1.0).min_h(0.0)
        .scroll(ScrollSpec { stick_bottom: false })
        .pt(6.0).px(16.0).pb(16.0).gap(6.0);
    for (g, (name, items)) in groups.into_iter().enumerate() {
        let mut header = group_header::view(g, name, items.len(), &s.tokens);
        if g > 0 {
            header = header.mt(14.0);
        }
        let mut tiles = row().key(nth("tiles", g)).flex_wrap().gap(6.0).items_start();
        for item in items {
            tiles = tiles.child(tile::view(item, s, &mut keys));
        }
        content = content.child(header).child(tiles);
    }
    content
}
