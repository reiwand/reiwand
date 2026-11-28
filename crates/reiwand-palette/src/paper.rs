use crate::oklab::{lab, rgb};
use crate::tone::{Palette, Rgb};

/// The ink of a light set: the text tone where a dark set has Frost.
pub const INK: Rgb = Rgb::hex(0x1c1a17);

fn hue(tone: Rgb) -> (f64, f64) {
    let [_, a, b] = lab(tone.0);
    (a.hypot(b), b.atan2(a))
}

/// The light set of a dark one: the same hues, as paper (the surfaces), soft ink (secondary text)
/// and a darker accent, every text tone at 4.5:1 or more on every surface.
pub fn paper(dark: &Palette) -> Palette {
    let (_, ground) = hue(dark.night);
    let (glow, accent) = hue(dark.moon);
    let tint = 0.012;
    let lit = glow.min(0.12);
    Palette {
        night: Rgb(rgb(0.975, tint, ground)),
        night_high: Rgb(rgb(0.945, tint, ground)),
        night_line: Rgb(rgb(0.85, tint, ground)),
        mist: Rgb(rgb(0.42, 0.02, ground)),
        mist_low: Rgb(rgb(0.47, 0.02, ground)),
        moon: Rgb(rgb(0.46, lit, accent)),
        moon_deep: Rgb(rgb(0.38, lit, accent)),
        veil: dark.veil,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tone::{contrast, fixed};

    #[test]
    fn every_text_tone_reads_on_every_surface_of_the_light_set() {
        for dark in [
            fixed(),
            Palette { night: Rgb::hex(0x2a0f12), moon: Rgb::hex(0xffb38a), ..fixed() },
            Palette { night: Rgb::hex(0x0e2a14), moon: Rgb::hex(0xa8f0b0), ..fixed() },
        ] {
            let light = paper(&dark);
            for surface in [light.night, light.night_high] {
                for text in [INK, light.mist, light.mist_low, light.moon, light.moon_deep] {
                    assert!(
                        contrast(text, surface) >= 4.5,
                        "{text} on {surface}: {}",
                        contrast(text, surface)
                    );
                }
                assert!(contrast(light.night, light.moon_deep) >= 4.5, "paper on the deep accent");
            }
        }
    }
}
