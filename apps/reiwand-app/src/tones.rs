use std::path::Path;

use image::imageops::FilterType;
use reiwand_palette::{INK, Palette, derive, fixed, paper};

const WIDTH: u32 = 96;
const HEIGHT: u32 = 54;

pub fn of_picture(path: &Path) -> Option<Palette> {
    let picture = image::open(path).ok()?;
    let small = picture.resize_exact(WIDTH, HEIGHT, FilterType::Triangle).to_rgb8();
    derive(small.as_raw(), 3, WIDTH as usize, HEIGHT as usize)
}

pub fn json(palette: &Palette) -> String {
    serde_json::json!({
        "night": palette.night.to_string(),
        "nightHigh": palette.night_high.to_string(),
        "nightLine": palette.night_line.to_string(),
        "mist": palette.mist.to_string(),
        "mistLow": palette.mist_low.to_string(),
        "moon": palette.moon.to_string(),
        "moonDeep": palette.moon_deep.to_string(),
        "veil": palette.veil,
    })
    .to_string()
}

pub fn for_path(path: &str) -> String {
    let found = if path.is_empty() { None } else { of_picture(Path::new(path)) };
    json(&found.unwrap_or(fixed()))
}

/// The light set of the same picture (the editor's light theme): paper surfaces, ink text.
pub fn light_for_path(path: &str) -> String {
    let found = if path.is_empty() { None } else { of_picture(Path::new(path)) };
    let mut set: serde_json::Value =
        serde_json::from_str(&json(&paper(&found.unwrap_or(fixed())))).unwrap_or_default();
    set["frost"] = serde_json::Value::from(INK.to_string());
    set.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_picture_gives_the_fixed_set() {
        let set: serde_json::Value = serde_json::from_str(&for_path("")).unwrap();
        assert_eq!(set["night"], "#0a1322");
        assert_eq!(set["moon"], "#9cc3f5");
        assert_eq!(for_path("/nonexistent/picture.png"), for_path(""));
    }

    #[test]
    fn the_light_set_is_paper_with_ink() {
        let set: serde_json::Value = serde_json::from_str(&light_for_path("")).unwrap();
        assert_eq!(set["frost"], "#1c1a17");
        let night = set["night"].as_str().unwrap();
        assert!(night > "#e", "{night}");
    }

    #[test]
    fn a_picture_gives_its_own_tones() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("warm.png");
        image::RgbImage::from_fn(192, 108, |x, y| image::Rgb([200 - (y as u8), 90 + (x as u8 / 4), 40]))
            .save(&path)
            .unwrap();
        let set: serde_json::Value = serde_json::from_str(&for_path(path.to_str().unwrap())).unwrap();
        assert_ne!(set["night"], "#0a1322");
        assert!(set["veil"].as_f64().is_some_and(|veil| (0.6..=0.97).contains(&veil)));
    }
}
