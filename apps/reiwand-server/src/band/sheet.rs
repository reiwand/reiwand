use crate::validate::picture_limits;
use image::{ImageReader, RgbImage};
use reiwand_palette::{FROST, MAX_PIXELS, Palette, Rgb, contrast, derive};
use std::path::Path;

pub const LEAST_VEIL: f32 = 0.75;
pub const NEEDED: f64 = 4.5;
const MOST: f32 = 1.34;

pub fn read(path: &Path) -> Option<RgbImage> {
    let mut reader = ImageReader::open(path).ok()?.with_guessed_format().ok()?;
    reader.limits(picture_limits());
    Some(reader.decode().ok()?.to_rgb8())
}

pub fn tones(cover: &RgbImage) -> Option<(Palette, f32)> {
    let (width, height) = (cover.width() as usize, cover.height() as usize);
    let palette = derive(cover.as_raw(), 3, width, height)?;
    let mut veil = palette.veil;
    while veil < MOST && !readable(&palette, cover, (veil * LEAST_VEIL).min(1.0)) {
        veil = ((veil + 0.01) * 100.0).round() / 100.0;
    }
    Some((palette, veil.min(MOST)))
}

pub fn over(night: Rgb, alpha: f32, pixel: [u8; 3]) -> Rgb {
    let mix = |n: u8, p: u8| (alpha * n as f32 + (1.0 - alpha) * p as f32).round() as u8;
    Rgb([mix(night.0[0], pixel[0]), mix(night.0[1], pixel[1]), mix(night.0[2], pixel[2])])
}

pub fn brightest(night: Rgb, alpha: f32, cover: &RgbImage) -> Rgb {
    let total = cover.as_raw().len() / 3;
    let step = total.div_ceil(MAX_PIXELS).max(1);
    let mut seen: Vec<(f64, Rgb)> = cover
        .as_raw()
        .as_chunks::<3>()
        .0
        .iter()
        .step_by(step)
        .map(|p| over(night, alpha, *p))
        .map(|c| (c.luminance(), c))
        .collect();
    seen.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    seen.get((seen.len().saturating_sub(1) * 95).div_ceil(100)).map_or(night, |s| s.1)
}

pub fn readable(palette: &Palette, cover: &RgbImage, alpha: f32) -> bool {
    let ground = brightest(palette.night, alpha, cover);
    [FROST, palette.mist].iter().all(|text| contrast(*text, ground) >= NEEDED)
}

pub fn css(palette: &Palette, veil: f32) -> String {
    let [r, g, b] = palette.night.0;
    format!(
        ":root{{--night:{};--night-high:{};--night-line:{};--mist:{};--mist-low:{};--accent:{};\
         --accent-deep:{};--veil:{veil:.2};--night-rgb:{r} {g} {b}}}\n",
        palette.night,
        palette.night_high,
        palette.night_line,
        palette.mist,
        palette.mist_low,
        palette.moon,
        palette.moon_deep,
    )
}

pub fn of(path: &Path) -> Option<String> {
    let cover = read(path)?;
    let (palette, veil) = tones(&cover)?;
    Some(css(&palette, veil))
}
