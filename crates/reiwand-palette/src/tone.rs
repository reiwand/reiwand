use crate::oklab::linear;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb(pub [u8; 3]);

impl Rgb {
    pub const fn hex(code: u32) -> Self {
        Rgb([(code >> 16) as u8, (code >> 8) as u8, code as u8])
    }

    pub fn luminance(self) -> f64 {
        let [r, g, b] = self.0.map(linear);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0[0], self.0[1], self.0[2])
    }
}

pub const FROST: Rgb = Rgb::hex(0xf4efe6);
pub const AMBER: Rgb = Rgb::hex(0xf0c26b);
pub const CORAL: Rgb = Rgb::hex(0xff9d8a);
pub const SPROUT: Rgb = Rgb::hex(0x8fd6a8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    pub night: Rgb,
    pub night_high: Rgb,
    pub night_line: Rgb,
    pub mist: Rgb,
    pub mist_low: Rgb,
    pub moon: Rgb,
    pub moon_deep: Rgb,
    pub veil: f32,
}

pub fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (a.luminance(), b.luminance());
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

pub const fn fixed() -> Palette {
    Palette {
        night: Rgb::hex(0x0a1322),
        night_high: Rgb::hex(0x121e33),
        night_line: Rgb::hex(0x2a3a57),
        mist: Rgb::hex(0xb9c4d6),
        mist_low: Rgb::hex(0x8f9db4),
        moon: Rgb::hex(0x9cc3f5),
        moon_deep: Rgb::hex(0x6f9fdc),
        veil: 0.8,
    }
}
