use super::*;

fn flake(at: [f32; 2], tilt: Value, render: Value) -> Value {
    json!({ "capacity": 8, "seed": 5, "emitters": [{ "shape": { "point": at }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "velocity": [[0, 0], [0, 0]], "size": [12, 12], "tilt": tilt,
            "color": ["#ffffff", "#ffffff"] },
        "render": render })
}

fn extent(pixels: &[u8], around: [u32; 2]) -> [u32; 2] {
    let lit = |x: u32, y: u32| at(pixels, x, y)[0] > 100;
    let span = |c: u32, max: u32| c.saturating_sub(14)..(c + 14).min(max);
    let across = |y: u32| span(around[0], SIZE.0).filter(|x| lit(*x, y)).count() as u32;
    let down = |x: u32| span(around[1], SIZE.1).filter(|y| lit(x, *y)).count() as u32;
    [across(around[1]), down(around[0])]
}

fn tilt(angle: [f32; 2], spin: [f32; 2]) -> Value {
    json!({ "angle": [angle, angle], "spin": [spin, spin] })
}

#[test]
fn a_turned_sprite_is_narrower_by_the_cosine_of_each_angle() {
    let Some(r) = renderer() else { return };
    let systems = json!({
        "wide": flake([40.0, 40.0], tilt([0.0, 60.0], [0.0, 0.0]), json!({})),
        "tall": flake([100.0, 40.0], tilt([60.0, 0.0], [0.0, 0.0]), json!({})),
        "plain": flake([40.0, 70.0], tilt([0.0, 0.0], [0.0, 0.0]), json!({})) });
    let layers = json!(["wide", "tall", "plain"].map(
        |id| json!({ "blend_space": "canvas", "id": id, "type": "particles", "system": id, "blend": "add" })
    ));
    for gpu in [false, true] {
        let Some(mut run) = Run::new(r, systems.clone(), layers.clone(), gpu) else { return };
        run.frame(r, 0.0);
        let pixels = run.frame(r, 1.0 / 30.0);
        let found = [extent(&pixels, [40, 40]), extent(&pixels, [100, 40]), extent(&pixels, [40, 70])];
        let full = found[2][0];
        assert!(full >= 9, "gpu {gpu}: {found:?}");
        let near = |a: u32, b: u32| a.abs_diff(b) <= 1;
        assert!(near(found[0][0], full / 2) && near(found[0][1], full), "gpu {gpu}: {found:?}");
        assert!(near(found[1][0], full) && near(found[1][1], full / 2), "gpu {gpu}: {found:?}");
    }
}

#[test]
fn the_spin_turns_the_sprite_edge_on_and_back() {
    let Some(r) = renderer() else { return };
    let systems = json!({ "s": flake([40.0, 40.0], tilt([0.0, 0.0], [0.0, 180.0]), json!({})) });
    let layers =
        json!([{ "blend_space": "canvas", "id": "s", "type": "particles", "system": "s", "blend": "add" }]);
    for gpu in [false, true] {
        let Some(mut run) = Run::new(r, systems.clone(), layers.clone(), gpu) else { return };
        run.frame(r, 0.0);
        let widths: Vec<u32> =
            [0.25, 0.25, 0.25, 0.25].iter().map(|dt| extent(&run.frame(r, *dt), [40, 40])[0]).collect();
        let want = [7u32, 1, 7, 10];
        let near = |a: u32, b: u32| a.abs_diff(b) <= 2;
        assert!(widths.iter().zip(want).all(|(a, b)| near(*a, b)), "gpu {gpu}: {widths:?}");
    }
}

#[test]
fn the_turn_follows_the_direction_of_a_velocity_oriented_sprite() {
    let Some(r) = renderer() else { return };
    let mut moving = flake([40.0, 10.0], tilt([0.0, 60.0], [0.0, 0.0]), json!({ "orient": "velocity" }));
    moving["init"]["velocity"] = json!([[0, 30], [0, 30]]);
    let systems = json!({ "s": moving });
    let layers =
        json!([{ "blend_space": "canvas", "id": "s", "type": "particles", "system": "s", "blend": "add" }]);
    for gpu in [false, true] {
        let Some(mut run) = Run::new(r, systems.clone(), layers.clone(), gpu) else { return };
        run.frame(r, 0.0);
        let pixels = run.frame(r, 1.0 / 30.0);
        let found = extent(&pixels, [40, 11]);
        assert!(found[0].abs_diff(10) <= 1 && found[1].abs_diff(5) <= 1, "gpu {gpu}: {found:?}");
    }
}

#[test]
fn a_perspective_turn_makes_the_near_edge_larger() {
    let Some(r) = renderer() else { return };
    let mut deep = flake([40.0, 40.0], tilt([0.0, 60.0], [0.0, 0.0]), json!({}));
    deep["perspective"] = json!({ "focal": 30, "near": 1 });
    deep["init"]["depth"] = json!([0, 0]);
    let layers =
        json!([{ "blend_space": "canvas", "id": "s", "type": "particles", "system": "s", "blend": "add" }]);
    for gpu in [false, true] {
        let Some(mut run) = Run::new(r, json!({ "s": deep.clone() }), layers.clone(), gpu) else { return };
        run.frame(r, 0.0);
        let pixels = run.frame(r, 1.0 / 30.0);
        let heights: Vec<u32> = [38, 42].iter().map(|x| extent(&pixels, [*x, 40])[1]).collect();
        assert!(heights[0] != heights[1], "gpu {gpu}: {heights:?}");
    }
}
