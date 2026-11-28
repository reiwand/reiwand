use super::seen_strokes::measure;
use super::*;

fn lit(r: &Renderer, z: f32, near: f32, gpu: bool) -> Option<usize> {
    let star = json!({ "capacity": 4, "seed": 5, "perspective": { "focal": 100, "near": near },
        "emitters": [{ "shape": { "point": [90, 40] }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "speed": [0, 0], "size": [6, 6], "depth": [z, z], "color": ["#ffffff", "#ffffff"] } });
    let layers =
        json!([{ "blend_space": "canvas", "id": "s", "type": "particles", "system": "s", "blend": "add" }]);
    let mut run = Run::new(r, json!({ "s": star }), layers, gpu)?;
    run.frame(r, 0.0);
    let pixels = (0..3).map(|_| run.frame(r, 1.0 / 30.0)).last().unwrap();
    Some(measure(&pixels).count)
}

#[test]
fn nothing_nearer_than_the_near_plane_is_drawn() {
    let Some(r) = renderer() else { return };
    for gpu in [false, true] {
        let Some(close) = lit(r, -50.0, 1.0, gpu) else { return };
        let cut = lit(r, -50.0, 60.0, gpu).unwrap();
        let beyond = lit(r, 0.0, 60.0, gpu).unwrap();
        assert!(close > 20, "gpu {gpu}: a particle 50 px from the eye shows with the plane at 1: {close}");
        assert_eq!(cut, 0, "gpu {gpu}: the plane at 60 hides it");
        assert!(beyond > 10, "gpu {gpu}: a particle 100 px away stays: {beyond}");
    }
}
