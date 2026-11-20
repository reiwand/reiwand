use super::*;

const W: u32 = 800;
const H: u32 = 100;

fn shot(r: &Renderer, levels: u32, scatter: f32, brightness: f32) -> Vec<u8> {
    let layer = json!([{ "blend_space": "canvas", "id": "d", "type": "solid", "color": "#ffffff",
        "size": [10, 10], "brightness": brightness, "transform": { "position": [W / 2, H / 2] } }]);
    let hdr = json!({ "threshold": 1, "strength": 1, "knee": 0, "levels": levels, "scatter": scatter });
    let bloom = json!({ "threshold": 1, "strength": 0, "radius": 8, "hdr": hdr });
    let canvas =
        json!({ "size": [W, H], "background": "#000000", "hdr": true, "tonemap": "none", "bloom": bloom });
    let mut shot = Shot::new(r, canvas, layer, &[]);
    let frame = shot.runtime.frame(shot.view.view((W, H)), Quality::Q3);
    let image = r.render_scene(&mut shot.view, &shot.runtime, &frame, (W, H), Quality::Q3, &[]).unwrap();
    let mut target = r.create_target(W, H).unwrap();
    let mut pixels = Vec::new();
    let (fit, flip) = (Fit::Stretch, [false; 2]);
    let layer = Layer {
        image: Some(image),
        fit,
        background: [0; 3],
        opacity: 1.0,
        uv: None,
        relief: None,
        flip,
        grade: None,
    };
    r.render_layers(&mut target, &[layer], &mut pixels).unwrap();
    pixels
}

fn light(p: &[u8], d: u32) -> f32 {
    let c = f32::from(p[((H / 2 * W + W / 2 + d) * 4 + 2) as usize]) / 255.0;
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

fn spilled(p: &[u8]) -> f32 {
    (10..W / 2).map(|d| light(p, d)).sum()
}

#[test]
fn eight_levels_with_a_scatter_of_one_and_a_half_fall_as_one_over_the_distance_and_grow_with_the_excess() {
    let Some(r) = renderer() else { return };
    let (bright, faint) = (shot(r, 8, 1.5, 2.0), shot(r, 8, 1.5, 1.1));
    for d in [20, 40, 80, 160] {
        let fall = light(&bright, d) / light(&bright, 2 * d);
        assert!((1.5..2.6).contains(&fall), "{d}: {fall}");
    }
    assert!(light(&bright, 320) > 0.005, "{}", light(&bright, 320));
    assert!(spilled(&bright) > 6.0 * spilled(&faint), "{} {}", spilled(&bright), spilled(&faint));
    let near = shot(r, 1, 0.0, 2.0);
    assert!(spilled(&bright) > 20.0 * spilled(&near), "{} {}", spilled(&bright), spilled(&near));
    assert_eq!(light(&near, 80), 0.0);
}
