//! Rove design tokens and Omarchy theme resolution, for Rune.
//!
//! Reference implementation written against Rune `dev` (rune::Color is linear sRGB with
//! straight alpha; `rune::hex` decodes sRGB hex). It was NOT compiled in the design session:
//! compile-check it first and fix any API drift. Values must match tokens/tokens.json exactly.
//!
//! Rune windows blend in sRGB-encoded space by default, as CSS does, so the alpha tokens
//! below reproduce the reference PNGs. Do not turn `srgb_blend` off.

use rune::Color;

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
    pub base: Color,       // window background + sidebar (translucent in Glass)
    pub panel: Color,      // floating content panel
    pub popover: Color,    // menus, dropdowns, toasts, sheets
    pub backdrop_blur: f32, // in-app blur sigma for popovers in Glass (Rune El::backdrop_blur); 0 in Solid
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
}

fn h(s: &str) -> Color { rune::hex(s).expect("valid hex") }
fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color { h(&format!("#{r:02x}{g:02x}{b:02x}")).alpha(a) }

pub const DARK_DEFAULT_ACCENT: &str = "#7C9CFF";
pub const LIGHT_DEFAULT_ACCENT: &str = "#2F6BFF";
pub const DARK_DEFAULT_TAGS: [&str; 7] = ["#FF7A7A", "#FFA35C", "#F2C55C", "#62D6A0", "#5CCFD9", "#7C9CFF", "#B794FF"];
pub const LIGHT_DEFAULT_TAGS: [&str; 7] = ["#D63A4F", "#D9652B", "#C98A12", "#2E9A5B", "#13929A", "#2F6BFF", "#8E52E0"];

/// What Rove reads from an Omarchy theme's colors.toml. Nothing else is read.
#[derive(Clone, Debug, Default)]
pub struct OmarchyTheme {
    pub mode: Option<String>,          // "light" | "dark" (missing = dark)
    pub accent: Option<String>,
    pub tags: [Option<String>; 7],      // red, orange, yellow, green, cyan, blue, magenta
}

impl OmarchyTheme {
    /// Minimal parser for `key = "value"` lines; use the `toml` crate if you prefer.
    pub fn parse(src: &str) -> Self {
        let get = |k: &str| src.lines().find_map(|l| {
            let (key, v) = l.split_once('=')?;
            (key.trim() == k).then(|| v.trim().trim_matches(|c| c == '"' || c == '\'').to_string()).filter(|v| !v.is_empty())
        });
        let names = ["red", "orange", "yellow", "green", "cyan", "blue", "magenta"];
        OmarchyTheme { mode: get("mode"), accent: get("accent"), tags: names.map(get) }
    }
}

fn contrast(a: Color, b: Color) -> f32 {
    let (x, y) = (a.luminance(), b.luminance());
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

/// Rule: the accent must reach 3:1 against the solid panel colour. If it does not, step its
/// OKLab lightness by 0.01 (lighter on dark, darker on light), keeping hue and chroma, until it
/// does. Every Omarchy theme shipped on 2026-11-13 already passes (see omarchy-themes-resolved.json).
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
    let raw_accent = theme.accent.as_deref().and_then(rune::hex)
        .unwrap_or_else(|| h(if dark { DARK_DEFAULT_ACCENT } else { LIGHT_DEFAULT_ACCENT }));
    let accent = fix_accent(raw_accent, solid_panel, mode);
    let (ink_dark, ink_light) = (h("#0B0D14"), h("#FFFFFF"));
    let ink = if contrast(ink_dark, accent) >= contrast(ink_light, accent) { ink_dark } else { ink_light };
    let defaults = if dark { DARK_DEFAULT_TAGS } else { LIGHT_DEFAULT_TAGS };
    let tags = std::array::from_fn(|i| theme.tags[i].as_deref().and_then(rune::hex).unwrap_or_else(|| h(defaults[i])));

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
    Tokens {
        mode, material, base, panel, popover,
        backdrop_blur: if glass { 14.0 } else { 0.0 }, // CSS blur(28px) = sigma 14
        t1, t2: t1.alpha(a2), t3: t1.alpha(a3), t4: t1.alpha(a4),
        hover: o.alpha(hover), fill: o.alpha(fill), fill2: o.alpha(fill2),
        fill3: if dark { Color::WHITE.alpha(0.15) } else { Color::WHITE },
        line: o.alpha(line), line2: o.alpha(line2), window_border: o.alpha(border), row_selected: o.alpha(row_sel),
        accent, accent_soft: accent.alpha(if dark { 0.20 } else { 0.13 }), accent_line: accent.alpha(if dark { 0.60 } else { 0.55 }), ink,
        danger: h(if dark { "#FF7A7A" } else { "#D63A4F" }),
        warning: h(if dark { "#F2C55C" } else { "#C98A12" }),
        success: h(if dark { "#62D6A0" } else { "#2E9A5B" }),
        tags,
    }
}

/// Where the active theme lives on Omarchy (the same path Flea's `src/theme/watch.rs` watches;
/// reuse that watcher). `current` is a symlink that changes when the user switches theme:
/// re-resolve on change and animate the colour change with Rune's calm colour motion
/// (`Tx::color_motion(motion::CALM)`).
pub fn current_theme_path() -> std::path::PathBuf {
    let home = std::env::var_os("HOME").map(std::path::PathBuf::from).unwrap_or_default();
    home.join(".local/state/omarchy/current/theme/colors.toml")
}
