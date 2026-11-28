use crate::Color;
use reiwand_palette::{from_oklch, oklab};
use std::f32::consts::{PI, TAU};

fn lch(Color([r, g, b, _]): Color) -> [f32; 3] {
    let [l, a, b] = oklab([r, g, b].map(f64::from));
    [l as f32, a.hypot(b) as f32, b.atan2(a) as f32]
}

fn rgb([l, c, h]: [f32; 3], alpha: f32) -> Color {
    let [r, g, b] = from_oklch(f64::from(l), f64::from(c), f64::from(h));
    Color([r as f32, g as f32, b as f32, alpha])
}

pub(crate) fn mix(from: Color, to: Color, f: f32) -> Color {
    if f <= 0.0 || from == to {
        return from;
    }
    if f >= 1.0 {
        return to;
    }
    let ([l0, c0, h0], [l1, c1, h1]) = (lch(from), lch(to));
    let grey = 1e-4;
    let (h0, h1) = match (c0 < grey, c1 < grey) {
        (true, false) => (h1, h1),
        (false, true) => (h0, h0),
        _ => (h0, h1),
    };
    let turn = (h1 - h0 + PI).rem_euclid(TAU) - PI;
    let at = |a: f32, b: f32| a + (b - a) * f;
    rgb([at(l0, l1), at(c0, c1), h0 + turn * f], at(from.0[3], to.0[3]))
}
