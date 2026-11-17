use super::*;

fn scaled(aspect: f32) -> f32 {
    let big = with(
        sparks(),
        json!({ "capacity": 8000, "init": { "size": [40, 60] }, "render": { "aspect": aspect } }),
    );
    let s = Scene::from_json(&scene(big, true)).unwrap();
    Bake::new(&s, 0, Tier::T0, true, |_| None).unwrap().sizes[0]
}

#[test]
fn the_budget_counts_the_rectangle_and_not_the_square() {
    let square = scaled(1.0);
    assert!(square < 0.5, "{square}");
    for aspect in [0.25, 4.0] {
        let slim = scaled(aspect);
        assert!((slim / square - 2.0).abs() < 1e-3, "{aspect}: {slim} {square}");
    }
    let sys = |aspect| {
        let s = Scene::from_json(&scene(with(sparks(), json!({ "render": { "aspect": aspect } })), true))
            .unwrap();
        s.particles.values().next().unwrap().overdraw([200, 100])
    };
    assert!((sys(0.25) * 4.0 - sys(1.0)).abs() < 1e-4 && (sys(5.0) * 5.0 - sys(1.0)).abs() < 1e-4);
}

#[test]
fn the_aspect_stays_between_one_twentieth_and_twenty() {
    let p = "/particles/dots/render/aspect";
    for bad in [0.0, 0.04, 21.0] {
        let found = issues(with(sparks(), json!({ "render": { "aspect": bad } })));
        assert!(
            found.iter().any(|i| matches!(i, Issue::Range { path, .. } if path.ends_with("render/aspect"))),
            "{p} {bad} {found:?}"
        );
    }
    assert!(issues(with(sparks(), json!({ "render": { "aspect": 0.05 } }))).is_empty());
    assert!(issues(with(sparks(), json!({ "render": { "aspect": 20 } }))).is_empty());
}

#[test]
fn the_sides_keep_the_longer_one_at_the_size() {
    let base = super::prefill::system(super::sparks()).render;
    let render = |a: f32| Render { aspect: a, ..base.clone() };
    assert_eq!(render(0.25).sides(), [0.25, 1.0]);
    assert_eq!(render(4.0).sides(), [1.0, 0.25]);
    assert_eq!(render(1.0).sides(), [1.0, 1.0]);
}
