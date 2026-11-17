use super::fields::{agree, both, system};
use super::*;

fn layers() -> Value {
    json!([{ "blend_space": "canvas", "id": "p", "type": "particles", "system": "s" }])
}

#[test]
fn a_ball_with_a_hole_and_one_side_agrees_on_both_paths() {
    let Some(r) = renderer() else { return };
    let shape = json!({ "sphere": { "center": [80, 40], "depth": 20, "radius": [30, 20, 60], "inner": 0.3, "sign": [0, 1, -1] } });
    let systems =
        json!({ "s": system(shape, json!({ "depth": [5, 5] }), json!({ "perspective": { "focal": 200 } })) });
    let Some(found) = both(r, &systems, &layers(), 6) else { return };
    agree("sphere", &found, 0.05);
    assert!(found[0].iter().all(|p| p.pos[1] >= 40.0 - 1e-3));
    let z: Vec<f32> = found[1].iter().map(|p| p.depth[0]).collect();
    assert!(z.iter().all(|z| *z <= 25.5 + 1e-2) && z.iter().any(|z| *z < -20.0), "{z:?}");
}

#[test]
fn a_flat_ball_agrees_and_keeps_depth_zero() {
    let Some(r) = renderer() else { return };
    let shape = json!({ "sphere": { "center": [80, 40], "depth": 0, "radius": [20, 20, 50], "inner": 0, "sign": [0, 0, 0] } });
    let systems = json!({ "s": system(shape, json!({}), json!({})) });
    let Some(found) = both(r, &systems, &layers(), 6) else { return };
    agree("flat ball", &found, 0.05);
    assert!(found[1].iter().all(|p| p.depth[0] == 0.0));
}

#[test]
fn a_ball_around_the_eye_draws_big_near_and_small_far_ones() {
    let Some(r) = renderer() else { return };
    let shape = json!({ "sphere": { "center": [80, 40], "depth": 0, "radius": [60, 30, 120], "inner": 0, "sign": [0, 0, 0] } });
    let init = json!({ "size": [6, 6], "life": [60, 60] });
    let systems =
        json!({ "s": system(shape, init, json!({ "perspective": { "focal": 120 }, "capacity": 128 })) });
    let layer = json!([{ "blend_space": "canvas", "id": "p", "type": "particles", "system": "s", "blend": "add" }]);
    for gpu in [false, true] {
        let Some(mut run) = Run::new(r, systems.clone(), layer.clone(), gpu) else { return };
        let pixels = (0..12).map(|_| run.frame(r, 1.0 / 30.0)).last().unwrap();
        assert!(pixels.chunks(4).any(|p| p[0] > 0), "something is drawn, gpu {gpu}");
        looks_like("volume", &pixels);
    }
}
