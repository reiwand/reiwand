use super::seen_strokes::{Stroke, measure};
use super::*;

fn star(at: [f32; 2], z: f32, depth: &str) -> Value {
    json!({ "capacity": 8, "seed": 5, "perspective": { "focal": 100, "near": 1 },
        "emitters": [{ "shape": { "point": at }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "speed": [0, 0], "size": [6, 6], "depth": [z, z], "depth_speed": [-20, -20],
            "color": ["#ffffff", "#ffffff"] },
        "render": { "stretch": { "mode": "add", "depth": depth, "time": 3, "min": 0, "max": 1000 } } })
}

fn shot(r: &Renderer, at: [f32; 2], z: f32, depth: &str, gpu: bool) -> Option<(Stroke, Vec<u8>)> {
    let systems = json!({ "s": star(at, z, depth) });
    let layers =
        json!([{ "blend_space": "canvas", "id": "s", "type": "particles", "system": "s", "blend": "add" }]);
    let mut run = Run::new(r, systems, layers, gpu)?;
    run.frame(r, 0.0);
    let pixels = (0..6).map(|_| run.frame(r, 1.0 / 30.0)).last().unwrap();
    Some((measure(&pixels), pixels))
}

fn lit_x(pixels: &[u8]) -> (u32, u32) {
    let xs: Vec<u32> = (0..SIZE.0).filter(|x| (0..SIZE.1).any(|y| at(pixels, *x, y)[0] > 100)).collect();
    (xs.first().copied().unwrap_or(0), xs.last().copied().unwrap_or(0))
}

#[test]
fn segment_strokes_run_towards_the_viewer_centred_and_shorten_with_distance() {
    let Some(r) = renderer() else { return };
    for gpu in [false, true] {
        let Some((near, pixels)) = shot(r, [110.0, 40.0], 0.0, "segment", gpu) else { return };
        let (far, _) = shot(r, [110.0, 40.0], 200.0, "segment", gpu).unwrap();
        let (below, _) = shot(r, [80.0, 70.0], 0.0, "segment", gpu).unwrap();
        let (slant, _) = shot(r, [100.0, 60.0], 0.0, "segment", gpu).unwrap();
        assert!(near.wide > 16 && near.high < 8, "gpu {gpu}: {} x {}", near.wide, near.high);
        assert!(far.wide + 10 < near.wide, "gpu {gpu}: far {} against near {}", far.wide, near.wide);
        assert!(below.high > 12 && below.wide < 8, "gpu {gpu}: {} x {}", below.wide, below.high);
        assert!((slant.angle - 45.0).abs() < 5.0, "gpu {gpu}: {}", slant.angle);
        let (lo, hi) = lit_x(&pixels);
        let (z, k) = (-4.0f32, |z: f32| 100.0 / (100.0 + z));
        let (back, front) = (80.0 + 30.0 * k(z + 30.0), 80.0 + 30.0 * k(z - 30.0));
        let off = ((lo as f32) - back).abs().max(((hi as f32) - front).abs());
        assert!(off < 4.0, "gpu {gpu}: {lo}..{hi} against {back}..{front}");
    }
}
