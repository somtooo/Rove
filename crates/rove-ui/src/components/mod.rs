//! Design components. Each major component owns its parts and tests in its own directory.
//! Every colour comes from `theme::Tokens`; no literal colours here.
//!
//! Keys follow `reference/redlines.json` so `--tree` output can be checked against it. Where the
//! reference has several elements of one kind, the first takes the bare key (`sidebar.row`) and
//! the others take `#n` suffixes (`sidebar.row#2`).

pub mod footer;
pub mod file_grid;
pub mod sidebar;
pub mod tabs;
pub mod text;
pub mod toolbar;
pub mod window;

mod element_key;
mod icon_button;
mod keycap;

pub use element_key::nth;
pub use icon_button::icon_button;
pub use keycap::keycap;
pub use text::{label, label_at};
