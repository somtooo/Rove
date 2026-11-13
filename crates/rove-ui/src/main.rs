//! Rove: a file manager on Rune, over Flea's backend.
//!
//! Milestones 0 and 1: a static, non-functional grid view from mock data.
//!
//! Headless reference frames (docs/09-verification.md):
//!   rove --snap out.png --scene dark-solid --size 1440x920 --scale 2
//!   rove --tree tree.json --scene dark-solid --size 1440x920
//! Scenes: dark-glass, dark-solid, latte-glass, latte-solid. In a headless run the window sits
//! at (40, 40) on a transparent 1440×920 desktop, as in the references; composite the snap
//! over `reference/backdrops/*.png` to compare.

mod assets;
mod icons;
mod mock;
mod theme;
mod views;

use rune::{AppSpec, WindowOptions};

use crate::assets::Assets;
use crate::theme::{Material, OmarchyTheme, Tokens};

pub struct State {
    pub tokens: Tokens,
    /// Some((x, y)) draws the window at that offset with its own radius, border and shadow, as
    /// the 1440×920 reference frames show it. None fills the surface (a real window).
    pub frame: Option<[f32; 2]>,
    pub assets: Option<Assets>,
}

/// The window's place in a reference frame.
const REFERENCE_FRAME: [f32; 2] = [40.0, 40.0];

fn init() -> State {
    let headless = rune::tooling::headless_run().is_some();
    let theme = if headless {
        OmarchyTheme::reference_dark()
    } else {
        // Milestone 5 watches this file; for now it is read once at launch.
        std::fs::read_to_string(theme::current_theme_path()).map(|s| OmarchyTheme::parse(&s)).unwrap_or_default()
    };
    State {
        tokens: theme::resolve(&theme, Material::Glass),
        frame: headless.then_some(REFERENCE_FRAME),
        assets: None,
    }
}

fn scene(s: &mut State, theme: OmarchyTheme, material: Material) {
    s.tokens = theme::resolve(&theme, material);
}

type Scene = (&'static str, fn(&mut State));

const SCENES: &[Scene] = &[
    ("dark-glass", |s| scene(s, OmarchyTheme::reference_dark(), Material::Glass)),
    ("dark-solid", |s| scene(s, OmarchyTheme::reference_dark(), Material::Solid)),
    ("latte-glass", |s| scene(s, OmarchyTheme::reference_latte(), Material::Glass)),
    ("latte-solid", |s| scene(s, OmarchyTheme::reference_latte(), Material::Solid)),
];

const FONTS: &[&[u8]] = &[
    include_bytes!("../assets/fonts/Inter-Regular.ttf"),
    include_bytes!("../assets/fonts/Inter-Medium.ttf"),
    include_bytes!("../assets/fonts/Inter-SemiBold.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf"),
];

fn main() {
    // Headless runs take their size from `--size` (the references use 1440x920).
    let size = views::window::WINDOW_SIZE;
    let spec = AppSpec {
        min_size: Some((720.0, 480.0)),
        scenes: SCENES,
        fonts: FONTS,
        families: ("Inter", "JetBrains Mono"),
        setup: Some(|s: &mut State, waker| s.assets = Some(Assets::load(waker))),
        ..AppSpec::new("Rove", size, init, views::window::view)
    };
    // Transparent, so Glass shows the desktop through `base` (Hyprland blurs it) and headless
    // snaps keep the area around the reference window clear for the backdrop.
    rune::App::new(spec)
        .window_options(WindowOptions { transparent: true, ..Default::default() })
        .run();
}

#[cfg(test)]
mod tests {
    use rune::tok::TextStyle;

    /// Milestone 0: Inter renders with tabular figures, and JetBrains Mono is loaded.
    #[test]
    fn fonts_load_with_tabular_figures_and_mono() {
        let _text = rune::text::TextSystem::new(super::FONTS, ("Inter", "JetBrains Mono"));
        let adv = |s: &str, style: TextStyle| rune::text::advance(s, style);
        let tnum = crate::theme::type_::FOOT_NUM;
        let prop = TextStyle::new(12.5, 400.0);
        assert!((adv("1111", tnum) - adv("0000", tnum)).abs() < 0.01, "tabular: every figure one width");
        assert!(adv("1111", prop) < adv("0000", prop) - 0.5, "proportional figures differ, so tnum did something");
        let mono = crate::theme::type_::MONO;
        assert!((adv("iiii", mono) - adv("MMMM", mono)).abs() < 0.01, "mono slot is monospaced");
        // JetBrains Mono's advance is 600/1000 em.
        assert!((adv("M", mono) - 12.5 * 0.6).abs() < 0.05, "mono slot is JetBrains Mono: {}", adv("M", mono));
    }
}
