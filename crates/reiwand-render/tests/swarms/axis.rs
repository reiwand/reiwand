use super::fields::{agree, both, system};
use super::*;

fn layers() -> Value {
    json!([{ "blend_space": "canvas", "id": "p", "type": "particles", "system": "s" }])
}

#[test]
fn rings_and_swirls_around_a_tilted_axis_agree_on_both_paths() {
    let Some(r) = renderer() else { return };
    let tilt = [0.6, 0.0, 0.8];
    let shape = json!({ "around": { "axis": tilt, "at": [80, 40], "count": 12, "radius": 20, "speed": 30 } });
    let swirl =
        json!({ "vortex": { "axis": [1, 0, 0], "center": [80, 40], "strength": 60, "falloff": "constant" } });
    let systems = json!({ "s": system(shape, json!({ "depth": [0, 0] }), json!({
        "perspective": { "focal": 200, "near": 1 }, "forces": [swirl, { "drag": 0.5 }] })) });
    let Some(found) = both(r, &systems, &layers(), 12) else { return };
    agree("tilted axis", &found, 0.1);
    let spread =
        found[0].iter().map(|p| p.depth[0]).fold((f32::MAX, f32::MIN), |(l, h), d| (l.min(d), h.max(d)));
    assert!(spread.1 - spread.0 > 10.0, "particles leave the image plane: {spread:?}");
}
