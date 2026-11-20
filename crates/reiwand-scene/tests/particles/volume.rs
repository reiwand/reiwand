use super::*;

fn ball(shape: Json, perspective: bool, init: Json) -> Vec<Particle> {
    let mut s = with(
        sparks(),
        json!({ "emitters": [{ "shape": { "sphere": shape }, "rate": 0, "bursts": [{ "count": 400 }] }],
            "init": { "life": [30, 30], "speed": [0, 0], "size": [4, 4] }, "forces": [], "collide": [] }),
    );
    init.as_object().into_iter().flatten().for_each(|(k, v)| s["init"][k] = v.clone());
    if perspective {
        s["perspective"] = json!({ "focal": 800, "near": 1 });
    }
    run(&scene(s, true), 2, Quality::Q3).particles.into_iter().filter(|p| p.age < p.life).collect()
}

fn full() -> Json {
    json!({ "center": [100, 50], "depth": 0, "radius": [40, 40, 40], "inner": 0, "sign": [0, 0, 0] })
}

fn offsets(all: &[Particle]) -> Vec<[f32; 3]> {
    all.iter().map(|p| [p.pos[0] - 100.0, p.pos[1] - 50.0, p.depth[0]]).collect()
}

fn share(v: &[f32], below: f32) -> f32 {
    v.iter().filter(|d| **d < below).count() as f32 / v.len() as f32
}

#[test]
fn births_fill_the_ball_with_a_distance_that_grows_with_the_square() {
    let all = ball(full(), true, json!({}));
    assert!(all.len() >= 380, "{}", all.len());
    let d: Vec<f32> =
        offsets(&all).iter().map(|o| (o[0] * o[0] + o[1] * o[1] + o[2] * o[2]).sqrt() / 40.0).collect();
    assert!(d.iter().all(|d| *d <= 1.0001));
    for (edge, expected) in [(0.5, 0.125), (0.75, 0.4219), (0.9, 0.729)] {
        let got = share(&d, edge);
        assert!((got - expected).abs() < 0.07, "{edge}: {got} against {expected}");
    }
}

#[test]
fn without_perspective_depth_is_dropped_and_the_flat_share_of_a_projected_ball_remains() {
    let all = ball(full(), false, json!({}));
    assert!(all.iter().all(|p| p.depth[0] == 0.0));
    let rho: Vec<f32> = offsets(&all).iter().map(|o| o[0].hypot(o[1]) / 40.0).collect();
    let got = share(&rho, 0.6);
    assert!((got - 0.488).abs() < 0.07, "{got}");
    assert!(got < share(&rho, 1.0) * 0.6 / 1.0, "the centre is not uniform: {got}");
}

#[test]
fn sign_keeps_births_on_one_side_of_each_direction() {
    let mut shape = full();
    shape["sign"] = json!([-1, 0, 1]);
    let all = offsets(&ball(shape, true, json!({})));
    assert!(all.iter().all(|o| o[0] <= 0.0 && o[2] >= 0.0));
    assert!(all.iter().any(|o| o[1] > 10.0) && all.iter().any(|o| o[1] < -10.0));
}

#[test]
fn inner_leaves_the_core_empty() {
    let mut shape = full();
    shape["inner"] = json!(0.5);
    let d: Vec<f32> = offsets(&ball(shape, true, json!({})))
        .iter()
        .map(|o| (o[0] * o[0] + o[1] * o[1] + o[2] * o[2]).sqrt() / 40.0)
        .collect();
    assert!(d.iter().all(|d| *d >= 0.4999), "{:?}", d.iter().fold(9.0, |a: f32, b| a.min(*b)));
}

#[test]
fn the_depth_of_the_shape_adds_to_the_depth_of_the_starts() {
    let mut shape = full();
    shape["depth"] = json!(50);
    let all = ball(shape, true, json!({ "depth": [100, 100] }));
    let z: Vec<f32> = all.iter().map(|p| p.depth[0]).collect();
    assert!(z.iter().all(|z| (109.0..=191.0).contains(z)));
    let mean = z.iter().sum::<f32>() / z.len() as f32;
    assert!((mean - 150.0).abs() < 5.0, "{mean}");
}

#[test]
fn radii_and_the_inner_fraction_are_checked() {
    let at = |shape: Json| {
        let mut s = helix_free();
        s["emitters"][0]["shape"] = json!({ "sphere": shape });
        issues(s)
    };
    assert_eq!(at(full()), []);
    let mut bad = full();
    bad["radius"] = json!([40, -1, 40]);
    bad["inner"] = json!(2);
    assert_eq!(at(bad).len(), 2);
}

fn helix_free() -> Json {
    let mut s = sparks();
    s["perspective"] = json!({ "focal": 800, "near": 1 });
    s
}
