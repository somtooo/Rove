//! Deterministic themes for reference renders and native material review.
//!
//! `--scene` works in a native window and with `--snap` / `--tree`. Reference runs draw the
//! window at (40, 40) on a transparent 1440×920 desktop for backdrop compositing.

use crate::app::State;
use crate::theme::{self, Material, OmarchyTheme};

fn scene(s: &mut State, theme: OmarchyTheme, material: Material) {
    s.tokens = theme::resolve(&theme, material);
}

/// Review variants only: keep the designer's original Glass scenes unchanged.
fn glass_90(s: &mut State, theme: OmarchyTheme) {
    scene(s, theme, Material::Glass);
    s.tokens.panel = s.tokens.panel.alpha(0.90);
}

type Scene = (&'static str, fn(&mut State));

pub const SCENES: &[Scene] = &[
    ("dark-glass", |s| scene(s, OmarchyTheme::reference_dark(), Material::Glass)),
    ("dark-solid", |s| scene(s, OmarchyTheme::reference_dark(), Material::Solid)),
    ("latte-glass", |s| scene(s, OmarchyTheme::reference_latte(), Material::Glass)),
    ("latte-solid", |s| scene(s, OmarchyTheme::reference_latte(), Material::Solid)),
    ("dark-glass-90", |s| glass_90(s, OmarchyTheme::reference_dark())),
    ("latte-glass-90", |s| glass_90(s, OmarchyTheme::reference_latte())),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_variants_change_only_panel_opacity() {
        for theme in [OmarchyTheme::reference_dark(), OmarchyTheme::reference_latte()] {
            let original = theme::resolve(&theme, Material::Glass);
            let mut state = crate::app::init();
            glass_90(&mut state, theme);
            assert_eq!(state.tokens.panel.hex(), original.panel.alpha(0.90).hex());
            assert_eq!(state.tokens.base.hex(), original.base.hex());
            assert_eq!(state.tokens.popover.hex(), original.popover.hex());
        }
    }
}
