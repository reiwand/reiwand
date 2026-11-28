use reiwand_palette::{Rgb, dominant};

pub const PALETTE_SIDE: u32 = 64;
pub const FALLBACK: [[f32; 4]; 5] = [
    [0.16, 0.16, 0.18, 1.0],
    [0.36, 0.36, 0.4, 1.0],
    [0.6, 0.6, 0.64, 1.0],
    [1.0, 1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0, 1.0],
];
const READABLE: f64 = 4.5;

fn unit(c: [f32; 4]) -> Rgb {
    Rgb([c[0], c[1], c[2]].map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8))
}

fn contrast(a: [f32; 4], b: [f32; 4]) -> f64 {
    reiwand_palette::contrast(unit(a), unit(b))
}

fn readable(primary: [f32; 4], others: [[f32; 4]; 2]) -> ([f32; 4], [f32; 4]) {
    let (white, black) = ([1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0]);
    let plain = if contrast(primary, white) >= contrast(primary, black) { white } else { black };
    let best = others.into_iter().max_by(|a, b| contrast(primary, *a).total_cmp(&contrast(primary, *b)));
    let text = best.filter(|c| contrast(primary, *c) >= READABLE).unwrap_or(plain);
    (text, plain)
}

pub fn palette(rgba: &[u8]) -> Option<[[f32; 4]; 5]> {
    let [primary, secondary, tertiary] = dominant(rgba, 4)?.map(|c| {
        let [r, g, b] = c.0.map(|v| f32::from(v) / 255.0);
        [r, g, b, 1.0]
    });
    let (text, plain) = readable(primary, [secondary, tertiary]);
    Some([primary, secondary, tertiary, text, plain])
}

pub fn hex(c: [f32; 4]) -> String {
    unit(c).to_string()
}
