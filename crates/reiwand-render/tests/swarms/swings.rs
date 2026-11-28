use super::Run;
use crate::gpu::renderer;
use serde_json::{Value, json};

fn burst(random: Option<&str>) -> Value {
    let mut wave = json!({ "what": "position", "from": "top", "freq": [0.5, 3], "scale": [0, 40], "phase": [0, 1], "mask": [1, 0] });
    if let Some(r) = random {
        wave["random"] = json!(r);
    }
    json!({ "swing": { "capacity": 300, "seed": 9,
        "emitters": [{ "shape": { "point": [80, 40] }, "bursts": [{ "count": 300 }] }],
        "init": { "life": [100, 100], "speed": [0, 0], "size": [2, 2] },
        "forces": [{ "oscillate": wave }] } })
}

#[test]
fn a_shared_oscillation_swings_the_same_on_gpu_and_cpu() {
    let Some(r) = renderer() else { return };
    let layers = json!([{ "blend_space": "canvas", "id": "s", "type": "particles", "system": "swing" }]);
    let spread = |random: Option<&str>, gpu: bool| {
        let mut run = Run::new(r, burst(random), layers.clone(), gpu)?;
        for _ in 0..20 {
            run.frame(r, 1.0 / 30.0);
        }
        Some(run.census(r)[0])
    };
    let Some(g) = spread(Some("shared"), true) else { return };
    let c = spread(Some("shared"), false).unwrap();
    let apart = spread(None, false).unwrap();
    assert!(c.alive == 300 && g.alive == 300, "{g:?} {c:?}");
    for (a, b) in [(g.min[0], c.min[0]), (g.max[0], c.max[0]), (g.mean[0], c.mean[0])] {
        assert!((a - b).abs() < 1.0, "the shader draws as the processor does: gpu {g:?} cpu {c:?}");
    }
    let moved = (c.min[0] - apart.min[0])
        .abs()
        .max((c.max[0] - apart.max[0]).abs())
        .max((c.mean[0] - apart.mean[0]).abs());
    assert!(moved > 3.0, "one draw for the three gives another spread than three draws: {c:?} {apart:?}");
}
