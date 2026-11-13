//! Images for the static milestones: icon-theme icons and thumbnails.
//!
//! Icon-theme icons are the WhiteSur reference PNGs in `.plan/reference/html/assets`,
//! read from disk at startup. They are GPL-3.0 and never bundled into the binary. The
//! freedesktop icon-theme lookup replaces this in Milestone 5.
//!
//! Rune has no alpha-shaped drop shadow, so each icon's CSS `filter: drop-shadow(0 y blur c)` is
//! made once here: the icon's alpha, Gaussian-blurred (σ = blur / 2, in device pixels), tinted
//! with the shadow colour. It draws as an image under the icon.

use std::collections::HashMap;
use std::path::PathBuf;

use rune::{Image, ImageCache, Waker};

use crate::mock::{self, Thumb};

/// Device pixels per point the images are prepared at. The 128 px reference PNGs draw at
/// 64 pt, so 2× is their native density; other scales sample their mips.
pub const PREP_SCALE: f32 = 2.0;

pub struct DropShadow {
    pub image: Image,
    /// Where the shadow image's top-left sits relative to the icon's, and its size, in points.
    pub offset: [f32; 2],
    pub size: f32,
}

pub struct ThemeIcon {
    pub image: Image,
    pub dark_shadow: DropShadow,
    pub light_shadow: DropShadow,
}

pub struct Assets {
    _cache: ImageCache,
    pub icons: HashMap<&'static str, ThemeIcon>,
    pub thumbs: HashMap<Thumb, Image>,
}

fn reference_assets_dir() -> PathBuf {
    std::env::var_os("ROVE_REFERENCE_ASSETS").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../.plan/reference/html/assets"))
    })
}

impl Assets {
    /// Loads every image the mock shows and waits for them, so the first frame (and every
    /// headless snap) draws them. Runs once from `AppSpec::setup`, before the first frame.
    pub fn load(waker: Waker) -> Assets {
        let cache = ImageCache::new(waker);
        let dir = reference_assets_dir();
        let mut icons = HashMap::new();
        for name in mock::theme_icon_names() {
            let path = dir.join(format!("whitesur-{name}.png"));
            let bytes = match std::fs::read(&path) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("rove: missing reference icon {}: {e}", path.display());
                    continue;
                }
            };
            let Some((w, h, rgba)) = decode_png(&bytes) else {
                eprintln!("rove: can't decode {}", path.display());
                continue;
            };
            let image = cache.load(bytes);
            let dark_shadow = drop_shadow(&cache, w, h, &rgba, 64.0, 3.0, 6.0, [0, 0, 0], 0.28);
            let light_shadow = drop_shadow(&cache, w, h, &rgba, 64.0, 2.0, 4.0, [30, 40, 80], 0.14);
            icons.insert(name, ThemeIcon { image, dark_shadow, light_shadow });
        }
        let thumbs: HashMap<Thumb, Image> =
            Thumb::ALL.iter().map(|&t| (t, cache.svg(t.svg().as_bytes().to_vec(), 92.0, PREP_SCALE))).collect();

        for icon in icons.values() {
            for img in [&icon.image, &icon.dark_shadow.image, &icon.light_shadow.image] {
                if let Err(e) = img.wait() {
                    eprintln!("rove: image failed: {e}");
                }
            }
        }
        for img in thumbs.values() {
            if let Err(e) = img.wait() {
                eprintln!("rove: thumbnail failed: {e}");
            }
        }
        Assets { _cache: cache, icons, thumbs }
    }
}

fn decode_png(bytes: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::ALPHA | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().ok()?;
    let mut buf = vec![0u8; reader.output_buffer_size()?];
    let info = reader.next_frame(&mut buf).ok()?;
    let (w, h) = (info.width, info.height);
    buf.truncate(info.buffer_size());
    let rgba = match info.color_type {
        png::ColorType::Rgba => buf,
        png::ColorType::GrayscaleAlpha => buf.as_chunks::<2>().0.iter().flat_map(|&[g, a]| [g, g, g, a]).collect(),
        _ => return None,
    };
    Some((w, h, rgba))
}

/// The CSS `drop-shadow(0 dy blur colour)` of an image drawn `size_pt` points square.
#[allow(clippy::too_many_arguments)]
fn drop_shadow(cache: &ImageCache, w: u32, h: u32, rgba: &[u8], size_pt: f32, dy: f32, blur: f32, rgb: [u8; 3], alpha: f32) -> DropShadow {
    // Device pixels per image pixel is 1 at PREP_SCALE for the 2× reference PNGs.
    let device_per_pt = w as f32 / size_pt;
    let sigma = blur * 0.5 * device_per_pt;
    let pad = (3.0 * sigma).ceil() as usize;
    let (w, h) = (w as usize, h as usize);
    let (ow, oh) = (w + 2 * pad, h + 2 * pad);
    let mut a = vec![0f32; ow * oh];
    for y in 0..h {
        for x in 0..w {
            a[(y + pad) * ow + x + pad] = rgba[(y * w + x) * 4 + 3] as f32 / 255.0;
        }
    }
    let kernel = gaussian(sigma, pad);
    let a = blur_pass(&blur_pass(&a, ow, oh, &kernel, true), ow, oh, &kernel, false);
    let out: Vec<u8> = a.iter().flat_map(|&v| [rgb[0], rgb[1], rgb[2], (v * alpha * 255.0).round().clamp(0.0, 255.0) as u8]).collect();
    let image = cache.rgba(ow as u32, oh as u32, out);
    let pad_pt = pad as f32 / device_per_pt;
    DropShadow { image, offset: [-pad_pt, dy - pad_pt], size: ow as f32 / device_per_pt }
}

fn gaussian(sigma: f32, radius: usize) -> Vec<f32> {
    let k: Vec<f32> = (0..=2 * radius).map(|i| {
        let x = i as f32 - radius as f32;
        (-x * x / (2.0 * sigma * sigma)).exp()
    }).collect();
    let sum: f32 = k.iter().sum();
    k.into_iter().map(|v| v / sum).collect()
}

fn blur_pass(src: &[f32], w: usize, h: usize, k: &[f32], horizontal: bool) -> Vec<f32> {
    let r = (k.len() / 2) as isize;
    let mut out = vec![0f32; src.len()];
    for y in 0..h as isize {
        for x in 0..w as isize {
            let mut acc = 0.0;
            for (i, kv) in k.iter().enumerate() {
                let d = i as isize - r;
                let (sx, sy) = if horizontal { (x + d, y) } else { (x, y + d) };
                if sx >= 0 && sy >= 0 && (sx as usize) < w && (sy as usize) < h {
                    acc += src[sy as usize * w + sx as usize] * kv;
                }
            }
            out[y as usize * w + x as usize] = acc;
        }
    }
    out
}
