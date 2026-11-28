use image::DynamicImage;
use reiwand_palette::{Rgb, dominant};

const SIDE: u32 = 32;
const BRIGHT: f64 = 0.7;
const DIM: f64 = 0.3;
const DARK_PIXEL: f64 = 0.225;
const DARK_AREA: f64 = 0.05;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palette {
    pub colors: Vec<u32>,
    pub dark_text: bool,
    pub dark_theme: bool,
}

pub fn palette(image: &DynamicImage) -> Palette {
    let small = image.thumbnail(SIDE, SIDE).to_rgb8();
    let (mut light, mut dark) = (0.0, 0usize);
    for pixel in small.pixels() {
        let l = Rgb(pixel.0).luminance();
        (light, dark) = (light + l, dark + usize::from(l <= DARK_PIXEL));
    }
    let pixels = (small.width() * small.height()).max(1) as usize;
    let (mean, dark) = (light / pixels as f64, dark as f64 / pixels as f64);
    let colors = dominant(small.as_raw(), 3).map_or_else(Vec::new, |found| {
        found.map(|Rgb([r, g, b])| u32::from(r) << 16 | u32::from(g) << 8 | u32::from(b)).to_vec()
    });
    Palette { colors, dark_text: mean > BRIGHT && dark <= DARK_AREA, dark_theme: mean < DIM }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    fn bands(rows: [(u32, [u8; 3]); 3]) -> DynamicImage {
        let mut image = RgbaImage::new(64, 64);
        let mut y = 0;
        for (height, [r, g, b]) in rows {
            (y..y + height).for_each(|y| (0..64).for_each(|x| image.put_pixel(x, y, Rgba([r, g, b, 255]))));
            y += height;
        }
        DynamicImage::ImageRgba8(image)
    }

    #[test]
    fn the_main_color_is_the_largest_cluster() {
        let mut image = RgbaImage::from_pixel(64, 64, Rgba([0x33, 0x66, 0xff, 255]));
        (0..64).for_each(|x| image.put_pixel(x, 0, Rgba([255, 0, 0, 255])));
        assert_eq!(palette(&DynamicImage::ImageRgba8(image)).colors[0], 0x3366ff);
    }

    #[test]
    fn secondary_colors_follow_the_largest_areas() {
        let image = bands([(40, [20, 40, 160]), (16, [200, 120, 30]), (8, [36, 56, 176])]);
        assert_eq!(palette(&image).colors[..2], [0x1428a0, 0xc8781e]);
    }

    #[test]
    fn hints_follow_the_brightness() {
        let light = palette(&bands([(64, [240, 240, 230]), (0, [0; 3]), (0, [0; 3])]));
        let dark = palette(&bands([(64, [10, 12, 20]), (0, [0; 3]), (0, [0; 3])]));
        let mixed = palette(&bands([(56, [250, 250, 250]), (8, [0, 0, 0]), (0, [0; 3])]));
        assert!(light.dark_text && !light.dark_theme);
        assert!(dark.dark_theme && !dark.dark_text);
        assert!(!mixed.dark_text && !mixed.dark_theme);
    }
}
