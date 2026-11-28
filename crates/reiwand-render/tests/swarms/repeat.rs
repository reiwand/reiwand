use super::*;

pub fn fog() -> (Value, Value) {
    let systems = json!({ "fog": { "capacity": 2048, "seed": 3,
        "emitters": [{ "shape": { "box": { "from": [0, 0], "to": [160, 80] } }, "rate": 900 }],
        "init": { "life": [0.2, 0.9], "speed": [5, 40], "angle": [0, 360], "size": [6, 14],
            "color": ["#ff303080", "#3060ff60"] } } });
    let layers = json!([{ "blend_space": "canvas", "id": "f", "type": "particles", "system": "fog", "blend": "normal" }]);
    (systems, layers)
}

fn played(r: &Renderer) -> Option<Vec<Vec<u8>>> {
    let (systems, layers) = fog();
    let mut run = Run::new(r, systems, layers, false)?;
    Some((0..60).map(|_| run.frame(r, 1.0 / 30.0)).collect())
}

#[test]
fn translucent_particles_stepped_on_the_processor_draw_the_same_pixels_on_every_run() {
    let Some(r) = renderer() else { return };
    let (Some(a), Some(b)) = (played(r), played(r)) else { return };
    let differ: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
    assert!(differ.is_empty(), "frames {differ:?} differ between two runs");
}
