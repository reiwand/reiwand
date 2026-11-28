use super::*;

// One white square of side 6 moving right at 30 px/s; a stretch of 1 s adds 30 px of trail.
fn dash(anchor: Option<&str>) -> Value {
    let mut stretch = json!({ "mode": "add", "depth": "flat", "time": 1, "min": 0, "max": 1000 });
    if let Some(a) = anchor {
        stretch["anchor"] = json!(a);
    }
    json!({ "capacity": 4, "seed": 5,
        "emitters": [{ "shape": { "point": [60, 40] }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "speed": [30, 30], "angle": [0, 0], "size": [6, 6], "color": ["#ffffff", "#ffffff"] },
        "render": { "stretch": stretch } })
}

fn lit_x(pixels: &[u8]) -> (f32, f32) {
    let xs: Vec<u32> = (0..SIZE.0).filter(|x| (0..SIZE.1).any(|y| at(pixels, *x, y)[0] > 100)).collect();
    (xs.first().copied().unwrap_or(0) as f32, xs.last().copied().unwrap_or(0) as f32 + 1.0)
}

fn drawn(r: &Renderer, anchor: Option<&str>, gpu: bool) -> Option<((f32, f32), f32)> {
    let systems = json!({ "s": dash(anchor) });
    let layers =
        json!([{ "blend_space": "canvas", "id": "s", "type": "particles", "system": "s", "blend": "add" }]);
    let mut run = Run::new(r, systems, layers, gpu)?;
    run.frame(r, 0.0);
    let pixels = (0..6).map(|_| run.frame(r, 1.0 / 30.0)).last().unwrap();
    let x = run.census(r)[0].mean[0];
    Some((lit_x(&pixels), x))
}

#[test]
fn a_centred_stretch_sits_on_the_particle_and_a_tip_one_leads_from_it() {
    let Some(r) = renderer() else { return };
    for gpu in [false, true] {
        let Some(((lo, hi), x)) = drawn(r, Some("centre"), gpu) else { return };
        assert!(((lo + hi) / 2.0 - x).abs() < 1.5, "gpu {gpu}: centred on {x}: {lo}..{hi}");
        for tip in [Some("tip"), None] {
            let ((a, b), y) = drawn(r, tip, gpu).unwrap();
            assert_eq!(y, x, "the anchor does not move the particle");
            assert!(
                (b - a - (hi - lo)).abs() < 1.5,
                "gpu {gpu} {tip:?}: same length {a}..{b} and {lo}..{hi}"
            );
            assert!(
                b >= x && b <= x + 4.0,
                "gpu {gpu} {tip:?}: the front sits on the particle {x}: {a}..{b}"
            );
            // 30 px/s for 1 s: the centred stroke is moved forward by half of the 30 px trail.
            assert!(
                ((lo + hi) / 2.0 - (a + b) / 2.0 - 15.0).abs() < 1.5,
                "gpu {gpu} {tip:?}: {a}..{b} against {lo}..{hi}"
            );
        }
    }
}
