use super::Run;
use crate::gpu::renderer;
use serde_json::{Value, json};

fn falling(ramp: &str) -> Value {
    json!({ "fall": { "capacity": 200, "seed": 3,
        "emitters": [{ "shape": { "point": [80, 10] }, "bursts": [{ "count": 200 }] }],
        "init": { "life": [2, 2], "speed": [0, 0], "size": [2, 2] },
        "forces": [{ "gravity": [0, 1000], "window": [0, 1, 1, 1], "ramp": ramp }] } })
}

#[test]
fn a_linear_window_ramps_the_same_on_gpu_and_cpu() {
    let Some(r) = renderer() else { return };
    let layers = json!([{ "blend_space": "canvas", "id": "f", "type": "particles", "system": "fall" }]);
    let drop = |ramp: &str, gpu: bool| {
        let mut run = Run::new(r, falling(ramp), layers.clone(), gpu)?;
        for _ in 0..30 {
            run.frame(r, 1.0 / 30.0);
        }
        Some(run.census(r)[0].mean[1] - 10.0)
    };
    let Some(g) = drop("linear", true) else { return };
    let (c, smooth) = (drop("linear", false).unwrap(), drop("smooth", false).unwrap());
    assert!(
        (g - c).abs() < 0.02 * c.abs().max(1.0),
        "the shader ramps as the processor does: gpu {g} cpu {c}"
    );
    // After 1 s of a 2 s life the gravity has faded in by half: a straight ramp has pushed more.
    assert!(c > smooth * 1.2, "linear {c} against smooth {smooth}");
}
