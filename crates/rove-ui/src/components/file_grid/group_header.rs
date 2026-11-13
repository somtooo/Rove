//! A file group's title, count and rule.

use rune::{El, el, row};

use crate::components::{label, nth};
use crate::theme::{Tokens, type_};

/// Group header: 28 high, gap 8; label 13/600 t2, count 13/500 t4, a 1 px rule filling the rest.
pub(super) fn view(g: usize, name: &str, count: usize, t: &Tokens) -> El {
    row().key(nth("group-header", g)).h(28.0).min_h(28.0).gap(8.0).items_center()
        .child(label(name, type_::GROUP, t.t2))
        .child(label(&count.to_string(), type_::GROUP_COUNT, t.t4).key(nth("group-header.count", g)))
        .child(el().grow(1.0).h(1.0).fill(t.line))
}
