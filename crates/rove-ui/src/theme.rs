//! Rove design tokens and Omarchy theme resolution, for Rune.
//!
//! Ported from `.plan/starter/theme.rs` against Rune `d6a0b3cc` (`dev`). API drift fixed:
//! `rune::hex` is `rune::color::hex` in this rev. Values equal `tokens/tokens.json`.
//!
//! `rune::Color` is linear sRGB with straight alpha. Rune windows blend in sRGB-encoded space by
//! default, as CSS does, so the alpha tokens below reproduce the reference PNGs. Do not turn
//! `srgb_blend` off.

use rune::{Color, Shadow};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode { Dark, Light }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Material { Glass, Solid }

/// One resolved set of colours. Everything the UI paints comes from here.
#[derive(Clone, Debug)]
pub struct Tokens {
    pub mode: Mode,
    pub material: Material,
    // Surfaces
    pub base: Color,        // window background + sidebar (translucent in Glass)
    pub panel: Color,       // floating content panel
    pub popover: Color,     // menus, dropdowns, toasts, sheets
    pub backdrop_blur: f32, // in-app blur for popovers in Glass (El::backdrop_blur, CSS px); 0 in Solid
    // Text
    pub t1: Color, pub t2: Color, pub t3: Color, pub t4: Color,
    // Overlays
    pub hover: Color, pub fill: Color, pub fill2: Color, pub fill3: Color,
    pub line: Color, pub line2: Color, pub window_border: Color, pub row_selected: Color,
    // Accent (from the theme, contrast-fixed)
    pub accent: Color, pub accent_soft: Color, pub accent_line: Color, pub ink: Color,
    // Status (Rove's own, never from the theme)
    pub danger: Color, pub warning: Color, pub success: Color,
    // Tags (from the theme)
    pub tags: [Color; 7],
    // Mode-specific details (02-tokens.md, "Light-mode specifics")
    pub keycap_bg: Color,
    pub keycap_border: Color,
    pub capacity_track: Color,
    /// Text and marks drawn on media (thumbnails): the duration chip and play mark.
    pub on_media: Color,
    pub media_chip: Color,
    pub media_play: Color,
    /// The footer slider's knob.
    pub knob: Color,
    pub shadows: Shadows,
}

/// Shadow tokens. `blur` is the CSS box-shadow blur radius, as Rune's `Shadow` takes it.
#[derive(Clone, Debug)]
pub struct Shadows {
    pub window: [Shadow; 2],
    /// Selected segment / view button. Light adds a 0.5 px ring.
    pub control_on: Vec<Shadow>,
    /// Light mode's active tab chip (Dark uses fill2 and no shadow).
    pub tab_on: Vec<Shadow>,
    /// CSS `filter: drop-shadow` under icon-theme icons: (y, blur radius, colour).
    pub icon_drop: (f32, f32, Color),
    /// Thumbnail ring (1 px spread) plus its drop shadow.
    pub thumb: [Shadow; 2],
    /// The footer slider knob.
    pub knob: Shadow,
}

pub(crate) fn h(s: &str) -> Color { rune::color::hex(s).expect("valid hex") }
pub(crate) fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color { h(&format!("#{r:02x}{g:02x}{b:02x}")).alpha(a) }

pub fn shadow(x: f32, y: f32, blur: f32, spread: f32, color: Color) -> Shadow {
    Shadow { x, y, blur, spread, color, inset: false }
}
pub fn inset_ring(width: f32, color: Color) -> Shadow {
    Shadow { x: 0.0, y: 0.0, blur: 0.0, spread: width, color, inset: true }
}

pub const DARK_DEFAULT_ACCENT: &str = "#7C9CFF";
pub const LIGHT_DEFAULT_ACCENT: &str = "#2F6BFF";
pub const DARK_DEFAULT_TAGS: [&str; 7] = ["#FF7A7A", "#FFA35C", "#F2C55C", "#62D6A0", "#5CCFD9", "#7C9CFF", "#B794FF"];
pub const LIGHT_DEFAULT_TAGS: [&str; 7] = ["#D63A4F", "#D9652B", "#C98A12", "#2E9A5B", "#13929A", "#2F6BFF", "#8E52E0"];

/// What Rove reads from an Omarchy theme's colors.toml. Nothing else is read.
#[derive(Clone, Debug, Default)]
pub struct OmarchyTheme {
    pub mode: Option<String>, // "light" | "dark" (missing = dark)
    pub accent: Option<String>,
    pub tags: [Option<String>; 7], // red, orange, yellow, green, cyan, blue, magenta
}

impl OmarchyTheme {
    /// Minimal parser for `key = "value"` lines.
    pub fn parse(src: &str) -> Self {
        let get = |k: &str| {
            src.lines().find_map(|l| {
                let (key, v) = l.split_once('=')?;
                (key.trim() == k)
                    .then(|| v.trim().trim_matches(|c| c == '"' || c == '\'').to_string())
                    .filter(|v| !v.is_empty())
            })
        };
        let names = ["red", "orange", "yellow", "green", "cyan", "blue", "magenta"];
        OmarchyTheme { mode: get("mode"), accent: get("accent"), tags: names.map(get) }
    }

    /// Tokyo Night as the dark references use it (accent `#7C9CFF`, Rove's default tags).
    pub fn reference_dark() -> Self {
        OmarchyTheme { mode: Some("dark".into()), accent: Some("#7C9CFF".into()), tags: Default::default() }
    }

    /// Catppuccin Latte as the light references use it.
    pub fn reference_latte() -> Self {
        let tags = ["#d20f39", "#d84e2b", "#df8e1d", "#40a02b", "#179299", "#1e66f5", "#ea76cb"].map(|t| Some(t.to_string()));
        OmarchyTheme { mode: Some("light".into()), accent: Some("#1e66f5".into()), tags }
    }
}

fn contrast(a: Color, b: Color) -> f32 {
    let (x, y) = (a.luminance(), b.luminance());
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

/// Rule: the accent must reach 3:1 against the solid panel colour. If it does not, step its
/// OKLab lightness by 0.01 (lighter on dark, darker on light), keeping hue and chroma, until it
/// does. Every Omarchy theme shipped on 2026-11-13 already passes.
fn fix_accent(accent: Color, panel: Color, mode: Mode) -> Color {
    let mut lab = accent.to_oklab();
    let mut c = accent;
    for _ in 0..40 {
        if contrast(c, panel) >= 3.0 { break; }
        lab[0] += if mode == Mode::Dark { 0.01 } else { -0.01 };
        c = Color::from_oklab(lab);
    }
    c
}

pub fn resolve(theme: &OmarchyTheme, material: Material) -> Tokens {
    let mode = if theme.mode.as_deref() == Some("light") { Mode::Light } else { Mode::Dark };
    let dark = mode == Mode::Dark;
    let solid_panel = if dark { h("#17181C") } else { h("#FFFFFF") };
    let raw_accent = theme.accent.as_deref().and_then(rune::color::hex)
        .unwrap_or_else(|| h(if dark { DARK_DEFAULT_ACCENT } else { LIGHT_DEFAULT_ACCENT }));
    let accent = fix_accent(raw_accent, solid_panel, mode);
    let (ink_dark, ink_light) = (h("#0B0D14"), h("#FFFFFF"));
    let ink = if contrast(ink_dark, accent) >= contrast(ink_light, accent) { ink_dark } else { ink_light };
    let defaults = if dark { DARK_DEFAULT_TAGS } else { LIGHT_DEFAULT_TAGS };
    let tags = std::array::from_fn(|i| theme.tags[i].as_deref().and_then(rune::color::hex).unwrap_or_else(|| h(defaults[i])));

    let glass = material == Material::Glass;
    let (base, panel, popover) = match (mode, material) {
        (Mode::Dark, Material::Glass) => (rgba(18, 20, 30, 0.52), rgba(13, 15, 22, 0.80), rgba(32, 35, 48, 0.84)),
        (Mode::Dark, Material::Solid) => (h("#0F1013"), h("#17181C"), h("#212328")),
        (Mode::Light, Material::Glass) => (rgba(244, 245, 249, 0.62), rgba(255, 255, 255, 0.84), rgba(255, 255, 255, 0.86)),
        (Mode::Light, Material::Solid) => (h("#ECEDF1"), h("#FFFFFF"), h("#FFFFFF")),
    };
    let (t1, a2, a3, a4) = if dark { (h("#EDEFF5"), 0.68, 0.52, 0.34) } else { (h("#17191F"), 0.70, 0.56, 0.38) };
    let o = if dark { Color::WHITE } else { Color::BLACK };
    let (hover, fill, fill2, line, line2, border, row_sel) = if dark {
        (0.05, 0.06, 0.10, 0.07, 0.11, 0.10, 0.10)
    } else {
        (0.04, 0.05, 0.075, 0.07, 0.10, 0.08, 0.065)
    };
    let black = Color::BLACK;
    let shadows = if dark {
        Shadows {
            window: [shadow(0.0, 40.0, 120.0, 0.0, black.alpha(0.55)), shadow(0.0, 8.0, 24.0, 0.0, black.alpha(0.35))],
            control_on: vec![shadow(0.0, 1.0, 2.0, 0.0, black.alpha(0.30))],
            tab_on: vec![],
            icon_drop: (3.0, 6.0, black.alpha(0.28)),
            thumb: [shadow(0.0, 0.0, 0.0, 1.0, Color::WHITE.alpha(0.12)), shadow(0.0, 6.0, 14.0, 0.0, black.alpha(0.35))],
            knob: shadow(0.0, 1.0, 3.0, 0.0, black.alpha(0.5)),
        }
    } else {
        let ring = shadow(0.0, 0.0, 0.0, 0.5, black.alpha(0.06));
        Shadows {
            window: [shadow(0.0, 40.0, 100.0, 0.0, rgba(60, 40, 90, 0.22)), shadow(0.0, 8.0, 24.0, 0.0, rgba(60, 40, 90, 0.12))],
            control_on: vec![shadow(0.0, 1.0, 2.0, 0.0, black.alpha(0.12)), ring],
            tab_on: vec![shadow(0.0, 1.0, 2.0, 0.0, black.alpha(0.10)), ring],
            icon_drop: (2.0, 4.0, rgba(30, 40, 80, 0.14)),
            thumb: [shadow(0.0, 0.0, 0.0, 1.0, black.alpha(0.08)), shadow(0.0, 4.0, 12.0, 0.0, rgba(30, 40, 80, 0.18))],
            knob: shadow(0.0, 1.0, 3.0, 0.0, black.alpha(0.5)),
        }
    };
    Tokens {
        mode, material, base, panel, popover,
        // CSS `backdrop-filter: blur(28px)`; Rune's backdrop_blur takes the same CSS value.
        backdrop_blur: if glass { 28.0 } else { 0.0 },
        t1, t2: t1.alpha(a2), t3: t1.alpha(a3), t4: t1.alpha(a4),
        hover: o.alpha(hover), fill: o.alpha(fill), fill2: o.alpha(fill2),
        fill3: if dark { Color::WHITE.alpha(0.15) } else { Color::WHITE },
        line: o.alpha(line), line2: o.alpha(line2), window_border: o.alpha(border), row_selected: o.alpha(row_sel),
        accent,
        accent_soft: accent.alpha(if dark { 0.20 } else { 0.13 }),
        accent_line: accent.alpha(if dark { 0.60 } else { 0.55 }),
        ink,
        danger: h(if dark { "#FF7A7A" } else { "#D63A4F" }),
        warning: h(if dark { "#F2C55C" } else { "#C98A12" }),
        success: h(if dark { "#62D6A0" } else { "#2E9A5B" }),
        tags,
        keycap_bg: o.alpha(if dark { 0.07 } else { 0.04 }),
        keycap_border: o.alpha(if dark { 0.09 } else { 0.10 }),
        capacity_track: o.alpha(if dark { 0.09 } else { 0.08 }),
        on_media: Color::WHITE,
        media_chip: Color::BLACK.alpha(0.55),
        media_play: Color::BLACK.alpha(0.45),
        knob: Color::WHITE,
        shadows,
    }
}

/// Where the active theme lives on Omarchy. Watched from Milestone 5.
#[allow(dead_code)]
pub fn current_theme_path() -> std::path::PathBuf {
    let home = std::env::var_os("HOME").map(std::path::PathBuf::from).unwrap_or_default();
    home.join(".local/state/omarchy/current/theme/colors.toml")
}

/// The type scale (02-tokens.md). Tracking is in points: the em values times the size.
pub mod type_ {
    use rune::tok::TextStyle;
    /// Inter's character variants, as the reference sets them, plus tabular figures.
    pub const UI: &str = "'cv11' 1, 'ss03' 1";
    pub const NUM: &str = "'cv11' 1, 'ss03' 1, 'tnum' 1";
    const fn s(size: f32, line: f32, weight: f32) -> TextStyle { TextStyle::new(size, weight).lh(line).features(UI) }
    pub const BRAND: TextStyle = s(15.0, 20.0, 600.0).tracking(-0.15);
    pub const TITLE: TextStyle = s(20.0, 26.0, 600.0).tracking(-0.3);
    pub const BODY: TextStyle = s(14.0, 20.0, 400.0);
    pub const BODY_500: TextStyle = s(14.0, 20.0, 500.0);
    pub const SECTION: TextStyle = s(12.0, 16.0, 500.0);
    pub const CAPTION: TextStyle = s(12.0, 16.0, 400.0).features(NUM);
    pub const ROW_END: TextStyle = s(12.0, 20.0, 400.0);
    pub const COUNT: TextStyle = s(13.0, 16.0, 400.0).features(NUM);
    pub const LABEL: TextStyle = s(13.0, 20.0, 400.0);
    pub const LABEL_500: TextStyle = s(13.0, 20.0, 500.0);
    pub const GROUP: TextStyle = s(13.0, 20.0, 600.0).tracking(-0.065);
    pub const GROUP_COUNT: TextStyle = s(13.0, 20.0, 500.0).tracking(-0.065).features(NUM);
    pub const TILE_NAME: TextStyle = s(13.0, 17.0, 500.0);
    pub const TILE_META: TextStyle = s(11.5, 14.0, 400.0).features(NUM);
    pub const KEYCAP: TextStyle = s(10.5, 10.5, 600.0).tracking(0.315);
    pub const FOOT: TextStyle = s(12.5, 20.0, 400.0);
    pub const FOOT_500: TextStyle = s(12.5, 20.0, 500.0);
    pub const FOOT_NUM: TextStyle = s(12.5, 20.0, 400.0).features(NUM);
    pub const DURATION: TextStyle = s(9.5, 14.0, 600.0).features(NUM);
    #[allow(dead_code)]
    pub const MONO: TextStyle = TextStyle::new(12.5, 400.0).lh(18.0).mono();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex8(c: Color) -> String { c.hex().to_uppercase()[..7].to_string() }

    #[test]
    fn reference_themes_resolve_as_the_docs_list() {
        let d = resolve(&OmarchyTheme::reference_dark(), Material::Solid);
        assert_eq!(hex8(d.accent), "#7C9CFF");
        assert_eq!(hex8(d.ink), "#0B0D14");
        let l = resolve(&OmarchyTheme::reference_latte(), Material::Solid);
        assert_eq!(hex8(l.accent), "#1E66F5");
        assert_eq!(hex8(l.ink), "#FFFFFF");
        assert_eq!(hex8(l.tags[0]), "#D20F39");
    }

    #[test]
    fn parse_reads_only_three_things() {
        let t = OmarchyTheme::parse("mode = 'light'\naccent = \"#1e66f5\"\nbackground = \"#eff1f5\"\nred = \"#d20f39\"");
        assert_eq!(t.mode.as_deref(), Some("light"));
        assert_eq!(t.accent.as_deref(), Some("#1e66f5"));
        assert_eq!(t.tags[0].as_deref(), Some("#d20f39"));
    }
}
