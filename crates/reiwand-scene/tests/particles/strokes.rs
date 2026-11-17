use super::*;

fn waving(from: &str, what: &str) -> Json {
    with(
        sparks(),
        json!({ "capacity": 400, "emitters": [{ "shape": { "box": { "from": [0, 0], "to": [200, 100] } }, "rate": 400 }],
            "init": { "life": [30, 30], "speed": [0, 0], "size": [4, 4] }, "collide": [],
            "forces": [{ "oscillate": { "what": what, "from": from, "freq": [0.5, 2], "scale": [0.2, 1], "phase": [0, 1] } }] }),
    )
}

fn looks(from: &str, what: &str, slot: usize) -> Vec<f32> {
    let text = scene(waving(from, what), true);
    run(&text, 12, Quality::Q3).particles.iter().filter(|p| p.age < p.life).map(|p| p.look[slot]).collect()
}

#[test]
fn an_alpha_oscillation_from_the_floor_never_drops_below_scale_start() {
    let floor = looks("floor", "alpha", 0);
    assert!(floor.len() > 100, "{}", floor.len());
    let (low, high) = floor.iter().fold((9.0f32, 0.0f32), |(l, h), v| (l.min(*v), h.max(*v)));
    assert!(low >= 0.199 && high <= 1.001, "{low} {high}");
    let mut distinct: Vec<i32> = floor.iter().map(|v| (v * 40.0) as i32).collect();
    distinct.sort_unstable();
    distinct.dedup();
    assert!(distinct.len() > 12, "peaks differ between particles: {distinct:?}");
}

#[test]
fn an_alpha_oscillation_from_the_top_dives_towards_zero() {
    let top = looks("top", "alpha", 0);
    let (low, high) = top.iter().fold((9.0f32, 0.0f32), |(l, h), v| (l.min(*v), h.max(*v)));
    assert!(low < 0.15 && high <= 1.0, "{low} {high}");
}

#[test]
fn a_size_oscillation_from_the_floor_scales_between_the_two_ends() {
    let size = looks("floor", "size", 1);
    let (low, high) = size.iter().fold((9.0f32, 0.0f32), |(l, h), v| (l.min(*v), h.max(*v)));
    assert!(low >= 0.199 && high <= 1.001 && high - low > 0.3, "{low} {high}");
}

#[test]
fn stretch_and_oscillate_need_every_field_and_position_has_no_floor() {
    let refused = |patch: Json| {
        Scene::from_json(&reiwand_upgrade::fill::whole(&scene(with(sparks(), patch), true))).is_err()
    };
    let stretch = |s: Json| json!({ "render": { "stretch": s } });
    assert!(refused(stretch(json!({ "mode": "wide", "depth": "flat", "time": 0.1, "min": 0, "max": 9 }))));
    assert!(!refused(stretch(
        json!({ "mode": "relative", "depth": "flat", "time": 0.1, "min": 0, "max": 9 })
    )));
    let wave = |o: Json| json!({ "forces": [{ "oscillate": o }] });
    assert!(!refused(wave(json!({ "what": "alpha", "from": "floor", "freq": [1, 2], "scale": [0.2, 1] }))));
    let bad = wave(json!({ "what": "position", "from": "floor", "freq": [1, 2], "scale": [2, 4] }));
    assert!(
        Scene::load(&reiwand_upgrade::fill::whole(&scene(with(sparks(), bad), true)), &props())
            .is_err_and(|e| e.to_string().contains("from"))
    );
}
