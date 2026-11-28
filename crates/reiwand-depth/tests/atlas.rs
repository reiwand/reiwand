use image::{Luma, Rgba, RgbaImage};
use reiwand_core::parallax::LAYERS;
use reiwand_depth::{FORMAT, Info, Layers, MOST_EDGE, Map, field_size, fields_of};

fn set(w: u32, h: u32, n: usize) -> Layers {
    let (fw, fh) = field_size((w as usize, h as usize));
    let image = |k: usize| RgbaImage::from_fn(w, h, |x, _| Rgba([k as u8, (x % 251) as u8, 9, 255]));
    let field = |k: usize| Map::from_pixel(fw as u32, fh as u32, Luma([1000 * k as u16]));
    let info = Info {
        format: FORMAT,
        size: [w, h],
        depths: (0..n).map(|k| k as f32 / n as f32).collect(),
        field: [fw as u32, fh as u32],
        ..Info::default()
    };
    Layers { info, images: (0..n).map(image).collect(), fields: (0..n).map(field).collect() }
}

#[test]
fn the_largest_set_the_builder_makes_comes_back() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(format!("{}-0123456789abcdef.layers.png", "cd".repeat(32)));
    let made = set(MOST_EDGE, MOST_EDGE * 9 / 16, usize::from(*LAYERS.end()));
    reiwand_depth::save(&path, &made).unwrap();
    let back = reiwand_depth::load(&path).unwrap().unwrap();
    assert_eq!(back.info, made.info);
    assert!(back.images == made.images && back.fields == made.fields);
    assert!(fields_of(&path).is_file());
}
