use super::fields::{agree, alive, both, system};
use super::*;
use reiwand_scene::{Particle, SIMPLEX_PERIOD, STEP, simplex_field};

const STRENGTH: [f32; 2] = [20.0, 120.0];
const PHASE: [f32; 2] = [0.0, 300.0];
const SCALE: f32 = 0.02;
const DRIFT: f32 = 15.0;
const MASK: [f32; 3] = [1.0, 0.7, 0.0];

fn layers() -> Value {
    json!([{ "blend_space": "canvas", "id": "p", "type": "particles", "system": "s" }])
}

fn noisy(more: Value) -> Value {
    let field = json!({ "strength": { "min": STRENGTH[0], "max": STRENGTH[1] }, "phase": { "min": PHASE[0], "max": PHASE[1] },
        "scale": SCALE, "drift": DRIFT, "mask": MASK });
    let mut s =
        system(json!({ "box": { "from": [20, 15], "to": [140, 65] } }), json!({ "life": [30, 30] }), more);
    s["emitters"] =
        json!([{ "shape": { "box": { "from": [20, 15], "to": [140, 65] } }, "bursts": [{ "count": 40 }] }]);
    s["forces"] = json!([{ "noise_field": field }]);
    s
}

/// One step of the formula: the acceleration of the noise read where the particle is, then the
/// velocity and the position, as every force of the system integrates.
fn formula(p: &mut Particle, t: f64) {
    let lerp = |[a, b]: [f32; 2], u: f32| a * (1.0 - u) + b * u;
    let (strength, phase) = (lerp(STRENGTH, p.rand), lerp(PHASE, p.rand));
    let moved = (f64::from(DRIFT) * f64::from(SCALE) * t).rem_euclid(SIMPLEX_PERIOD) as f32;
    let n = simplex_field([p.pos[0], p.pos[1], 0.0].map(|v| (v + phase) * SCALE + moved));
    let acc = [0, 1].map(|i| strength * MASK[i] * n[i]);
    p.vel = [0, 1].map(|i| p.vel[i] + acc[i] * STEP);
    p.pos = [0, 1].map(|i| p.pos[i] + p.vel[i] * STEP);
}

#[test]
fn forty_particles_move_on_the_gpu_as_the_noise_field_formula_says() {
    let Some(r) = renderer() else { return };
    let Some(mut run) = Run::new(r, json!({ "s": noisy(json!({})) }), layers(), true) else { return };
    drop(run.frame(r, 1.0 / 30.0));
    let mut want = alive(r, &run);
    assert_eq!(want.len(), 40);
    let frames = 20;
    (0..frames).for_each(|_| drop(run.frame(r, 1.0 / 30.0)));
    let found = alive(r, &run);
    for step in 0..2 * frames {
        want.iter_mut().for_each(|p| formula(p, f64::from(STEP) * (1 + step) as f64));
    }
    assert_eq!(found.iter().map(|p| p.id).collect::<Vec<_>>(), want.iter().map(|p| p.id).collect::<Vec<_>>());
    let far = found.iter().zip(&want).map(|(g, w)| (g.pos[0] - w.pos[0]).hypot(g.pos[1] - w.pos[1]));
    let worst = far.fold(0.0f32, f32::max);
    assert!(worst < 0.05, "the GPU strays {worst} px from the formula");
    let speeds: Vec<f32> = found.iter().map(|p| p.vel[0].hypot(p.vel[1])).collect();
    assert!(speeds.iter().all(|v| *v > 0.0) && found.iter().all(|p| p.depth == [0.0, 0.0]));
}

#[test]
fn the_noise_field_agrees_on_both_paths_in_the_plane_and_in_depth() {
    let Some(r) = renderer() else { return };
    let Some(flat) = both(r, &json!({ "s": noisy(json!({})) }), &layers(), 20) else { return };
    agree("noise field", &flat, 0.05);
    let mut deep = noisy(json!({ "perspective": { "focal": 600, "near": 1 } }));
    deep["init"]["depth"] = json!([0, 200]);
    deep["forces"][0]["noise_field"]["mask"] = json!([1, 1, 1]);
    let Some(found) = both(r, &json!({ "s": deep }), &layers(), 20) else { return };
    agree("noise field in depth", &found, 0.05);
    assert!(found[1].iter().any(|p| p.depth[1].abs() > 1.0), "depth moves with the third axis");
}
