use super::*;

fn light(r: &Renderer, size: f32, spot: [f32; 2], gpu: bool) -> Option<f32> {
    let dot = json!({ "capacity": 4, "seed": 5,
        "emitters": [{ "shape": { "point": spot }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "speed": [0, 0], "size": [size, size], "color": ["#ffffff", "#ffffff"] } });
    let layers =
        json!([{ "blend_space": "canvas", "id": "s", "type": "particles", "system": "s", "blend": "add" }]);
    let mut run = Run::new(r, json!({ "s": dot }), layers, gpu)?;
    run.frame(r, 0.0);
    let pixels = (0..3).map(|_| run.frame(r, 1.0 / 30.0)).last().unwrap();
    let linear = |c: u8| ((f32::from(c) / 255.0 + 0.055) / 1.055).powf(2.4);
    Some(
        (37..44).flat_map(|y| (77..84).map(move |x| (x, y))).map(|(x, y)| linear(at(&pixels, x, y)[0])).sum(),
    )
}

#[test]
fn a_particle_under_a_pixel_leaves_light_by_its_area() {
    let Some(r) = renderer() else { return };
    for gpu in [false, true] {
        let Some(one) = light(r, 1.0, [80.5, 40.5], gpu) else { return };
        let half = light(r, 0.5, [80.5, 40.5], gpu).unwrap();
        let edge = light(r, 0.5, [80.0, 40.0], gpu).unwrap();
        assert!(one > 0.9, "gpu {gpu}: a one pixel particle lights its pixel: {one}");
        assert!(
            (half / one - 0.25).abs() < 0.05,
            "gpu {gpu}: half a pixel leaves a quarter: {half} of {one}"
        );
        assert!(
            (edge / one - 0.25).abs() < 0.05,
            "gpu {gpu}: off the centre it shows as much: {edge} of {one}"
        );
    }
}
