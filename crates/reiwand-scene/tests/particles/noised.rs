use super::*;

const REFERENCE: [([f32; 3], f32); 8] = [
    ([0.1, 0.2, 0.3], 0.399469),
    ([1.5, -2.25, 0.75], 0.719762),
    ([-3.7, 4.1, -0.6], -0.17144),
    ([10.3, 20.7, -5.9], -0.14074),
    ([-0.31, 0.0, 2.62], -0.071093),
    ([123.4, -56.7, 8.9], 0.171428),
    ([7.77, 7.77, -7.77], 0.366634),
    ([0.5, 0.5, 0.5], 0.0),
];

#[test]
fn simplex_noise_gives_the_numbers_of_the_reference_and_stays_within_one() {
    for (p, want) in REFERENCE {
        assert!((simplex(p) - want).abs() < 2e-5, "{p:?}: {} against {want}", simplex(p));
    }
    let n = simplex_field([1.5, -2.25, 0.75]);
    assert_eq!(n, [simplex([1.5, -2.25, 0.75]), simplex([0.75, 1.5, -2.25]), simplex([-2.25, 0.75, 1.5])]);
    let mut most = 0.0f32;
    for i in 0..20_000 {
        let v = |k: u32| ((i * 7919 + k * 104_729) % 10_007) as f32 / 97.3 - 50.0;
        most = most.max(simplex([v(1), v(2), v(3)]).abs());
    }
    assert!(most > 0.8 && most <= 1.0, "{most}");
    let p = [1.3, -2.1, 0.4];
    let far = p.map(|v| v + SIMPLEX_PERIOD as f32);
    assert!((simplex(p) - simplex(far)).abs() < 1e-4, "the noise repeats along the diagonal");
}

fn noisy(field: Json) -> Json {
    with(
        sparks(),
        json!({ "emitters": [{ "shape": { "box": { "from": [60, 30], "to": [140, 70] } }, "bursts": [{ "count": 40 }] }],
            "init": { "life": [30, 30], "speed": [0, 0], "angle": [0, 0], "size": [2, 2] },
            "forces": [{ "noise_field": field }], "collide": [] }),
    )
}

fn field(strength: [f32; 2], drift: f32) -> Json {
    json!({ "strength": { "min": strength[0], "max": strength[1] }, "phase": { "min": 0, "max": 50 },
        "scale": 0.02, "drift": drift, "mask": [1, 1, 1] })
}

#[test]
fn the_noise_field_moves_each_particle_at_a_strength_of_its_own_and_gathers_them() {
    let all = |f: Json| run(&scene(noisy(f), true), 90, Quality::Q3).particles;
    let moved = all(field([40.0, 120.0], 0.0));
    assert_eq!(moved.iter().filter(|p| p.age < p.life).count(), 40);
    let speeds: Vec<f32> = moved.iter().map(|p| p.vel[0].hypot(p.vel[1])).collect();
    let (lo, hi) = speeds.iter().fold((f32::MAX, 0.0f32), |(a, b), v| (a.min(*v), b.max(*v)));
    assert!(lo < hi * 0.5 && hi > 20.0, "speeds differ per particle: {lo} to {hi}");
    let still = all(field([0.0, 0.0], 30.0));
    assert!(still.iter().all(|p| p.vel == [0.0, 0.0]), "no strength, no motion");
    let drifted = all(field([40.0, 120.0], 30.0));
    assert_ne!(
        drifted.iter().map(|p| p.pos).collect::<Vec<_>>(),
        moved.iter().map(|p| p.pos).collect::<Vec<_>>()
    );
    let mut flat = field([40.0, 120.0], 0.0);
    flat["mask"] = json!([0, 1, 1]);
    assert!(all(flat).iter().all(|p| p.vel[0] == 0.0), "the mask keeps x still");
}

#[test]
fn the_noise_field_moves_depth_only_with_perspective() {
    let mut deep = noisy(field([60.0, 60.0], 0.0));
    deep["perspective"] = json!({ "focal": 800, "near": 1 });
    deep["init"]["depth"] = json!([0, 0]);
    let lifted = run(&scene(deep.clone(), true), 30, Quality::Q3).particles;
    assert!(lifted.iter().any(|p| p.depth[1].abs() > 1.0), "{lifted:?}");
    deep["forces"][0]["noise_field"]["mask"] = json!([1, 1, 0]);
    let kept = run(&scene(deep, true), 30, Quality::Q3).particles;
    assert!(kept.iter().all(|p| p.depth[1] == 0.0));
    let flat = run(&scene(noisy(field([60.0, 60.0], 0.0)), true), 30, Quality::Q3).particles;
    assert!(flat.iter().all(|p| p.depth == [0.0, 0.0]));
}

#[test]
fn the_noise_field_needs_its_feature_and_checks_its_ranges() {
    let p = "/particles/dots/forces/0/noise_field";
    let made = Scene::from_json(&scene(noisy(field([10.0, 20.0], 5.0)), true)).unwrap();
    assert!(made.validate(&props()).is_empty() && made.features().contains("particle_noise"));
    assert!(!Scene::from_json(&scene(sparks(), true)).unwrap().features().contains("particle_noise"));
    let mut bad = field([10.0, 20.0], 5.0);
    (bad["scale"], bad["mask"]) = (json!(0), json!([1, 2, 1]));
    let range =
        |path: &str, value: f32, min: f32, max: f32| Issue::Range { path: path.into(), value, min, max };
    assert_eq!(
        issues(noisy(bad)),
        [range(&format!("{p}/scale"), 0.0, 1e-6, 10.0), range(&format!("{p}/mask/1"), 2.0, 0.0, 1.0)]
    );
}

#[test]
fn the_noise_field_phase_of_time_wraps_without_a_jump() {
    let mut s = Sim::default();
    s.counts[2] = 1;
    (s.forces[0], s.forces[1]) = ([17.0, 1.0, 1.0, 0.01], [0.0, 0.0, 30.0, 0.0]);
    for t in [3.0, 5.2e5 + 0.37, 1.0e7] {
        let at = |t: f64| {
            let mut x = s;
            x.phases(t);
            x.forces[1][3]
        };
        let moved = (at(t + 1.0 / 60.0) - at(t)).rem_euclid(SIMPLEX_PERIOD as f32);
        assert!((moved - 0.3 / 60.0).abs() < 1e-4, "at {t} s the noise moved {moved}");
    }
}
