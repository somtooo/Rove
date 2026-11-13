//! App lifecycle and window-wide state. UI composition lives in `components::window`.

use rune::{AppSpec, Key, WindowOptions};

use crate::assets::Assets;
use crate::components::window;
use crate::fixtures::scenes::SCENES;
use crate::theme::{self, Material, OmarchyTheme, Tokens};

#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;

pub struct State {
    pub tokens: Tokens,
    /// Some((x, y)) draws the window at that offset with its own radius, border and shadow, as
    /// the 1440×920 reference frames show it. None fills the surface (a real window).
    pub frame: Option<[f32; 2]>,
    pub assets: Option<Assets>,
    pub selected: Option<&'static str>,
    pub sidebar_visible: bool,
}

/// The window's place in a reference frame.
const REFERENCE_FRAME: [f32; 2] = [40.0, 40.0];

pub(crate) fn init() -> State {
    let headless = rune::tooling::headless_run().is_some();
    let theme = if headless {
        OmarchyTheme::reference_dark()
    } else {
        // The live-theme slice adds watching; for now it is read once at launch.
        std::fs::read_to_string(theme::current_theme_path()).map(|s| OmarchyTheme::parse(&s)).unwrap_or_default()
    };
    State {
        tokens: theme::resolve(&theme, Material::Glass),
        frame: headless.then_some(REFERENCE_FRAME),
        assets: None,
        selected: Some("shore-at-dusk.jpg"),
        sidebar_visible: true,
    }
}

const FONTS: &[&[u8]] = &[
    include_bytes!("../../assets/fonts/Inter-Regular.ttf"),
    include_bytes!("../../assets/fonts/Inter-Medium.ttf"),
    include_bytes!("../../assets/fonts/Inter-SemiBold.ttf"),
    include_bytes!("../../assets/fonts/JetBrainsMono-Regular.ttf"),
];

pub fn run() {
    // Headless runs take their size from `--size` (the references use 1440×920).
    let spec = AppSpec {
        min_size: Some((720.0, 480.0)),
        scenes: SCENES,
        fonts: FONTS,
        families: ("Inter", "JetBrains Mono"),
        setup: Some(|s: &mut State, waker| s.assets = Some(Assets::load(waker))),
        on_key: Some(|s, ev, _tx| match &ev.key {
            Key::Escape => {
                s.selected = None;
                true
            }
            Key::Command('b') => {
                s.sidebar_visible = !s.sidebar_visible;
                true
            }
            _ => false,
        }),
        ..AppSpec::new("Rove", window::WINDOW_SIZE, init, window::view)
    };
    // Transparent, so Glass shows the desktop through `base` (Hyprland blurs it) and headless
    // snaps keep the area around the reference window clear for the backdrop.
    rune::App::new(spec)
        .window_options(WindowOptions { transparent: true, ..Default::default() })
        .float_on_open()
        .run();
}
