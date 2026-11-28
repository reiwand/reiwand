use super::plan;
use ab_glyph::{Font, FontArc};
use reiwand_scene::Layout;

const FONT: &[u8] = include_bytes!("../../../apps/reiwand-app/look/fonts/RedHatDisplay.ttf");

/// Pen positions in layer pixels of the glyphs of `text` at `size`, drawn `k` times larger.
fn pens(text: &str, size: f32, k: f32, whole: bool) -> Vec<f32> {
    let fonts = [FontArc::try_from_slice(FONT).unwrap()];
    let px = (size * k * 2.0).round() / 2.0;
    let layout = Layout { whole, ..Layout::default() };
    let plan = plan(&fonts, text, px, px / size, &layout).unwrap();
    plan.lines[0].glyphs.iter().map(|g| g.x / (px / size)).collect()
}

/// The pens the font gives at `size` layer pixels (ascender to descender), each advance and kerning `cut`.
fn expected(text: &str, size: f32, cut: fn(f32) -> f32, kern_cut: fn(f32) -> f32) -> Vec<f32> {
    let font = FontArc::try_from_slice(FONT).unwrap();
    let unit = size / (font.ascent_unscaled() - font.descent_unscaled());
    let (mut pen, mut last, mut out) = (0.0, None, Vec::new());
    for c in text.chars() {
        let id = font.glyph_id(c);
        pen += last.map_or(0.0, |l| kern_cut(font.kern_unscaled(l, id) * unit));
        out.push(pen);
        pen += cut(font.h_advance_unscaled(id) * unit);
        last = Some(id);
    }
    out
}

#[test]
fn whole_advances_place_each_glyph_at_the_sum_of_the_advances_cut_to_whole_layer_pixels() {
    let text = "12:34:56 Today, AVA";
    for (size, k) in [(60.0, 2.452), (35.0, 1.0), (146.0, 0.5)] {
        let got = pens(text, size, k, true);
        let want = expected(text, size, f32::floor, f32::trunc);
        for (i, (g, w)) in got.iter().zip(&want).enumerate() {
            assert!((g - w).abs() < 2e-3, "glyph {i} at size {size}, scale {k}: {g} not {w}");
            assert!((g - g.round()).abs() < 2e-3, "glyph {i} off whole pixels: {g}");
        }
    }
}

#[test]
fn without_whole_advances_the_pen_follows_the_font_as_before() {
    let text = "12:34:56 Today, AVA";
    let (got, want) = (pens(text, 60.0, 2.452, false), expected(text, 60.0, |v| v, |v| v));
    assert!(got.iter().zip(&want).all(|(g, w)| (g - w).abs() < 1e-2), "{got:?} {want:?}");
    assert!(got.iter().any(|g| (g - g.round()).abs() > 0.01), "exact advances came out whole: {got:?}");
}
