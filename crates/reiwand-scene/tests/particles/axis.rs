use super::*;

fn alive(system: Json, frames: usize) -> Vec<Particle> {
    run(&scene(system, true), frames, Quality::Q3).particles.into_iter().filter(|p| p.age < p.life).collect()
}

fn helix(axis: Json, perspective: bool) -> Json {
    let mut s = with(
        sparks(),
        json!({ "emitters": [{ "shape": { "around": { "axis": axis, "at": [100, 50], "count": 8, "radius": 30 } }, "rate": 100 }],
            "init": { "life": [30, 30], "speed": [0, 0], "size": [4, 4] },
            "forces": [], "collide": [] }),
    );
    if perspective {
        s["perspective"] = json!({ "focal": 800 });
    }
    s
}

#[test]
fn a_ring_around_the_screen_normal_stays_flat() {
    let [x, y, z] = ring_point([0.0, 0.0, 1.0], 1.0);
    assert_eq!((x, y, z), (1f32.cos(), 1f32.sin(), 0.0));
}

#[test]
fn a_ring_around_an_axis_in_the_image_plane_reaches_into_depth() {
    let [x, y, z] = ring_point([1.0, 0.0, 0.0], 0.0);
    assert_eq!((x, y, z), (0.0, 1.0, 0.0));
    let [x, y, z] = ring_point([1.0, 0.0, 0.0], std::f32::consts::FRAC_PI_2);
    assert!(x.abs() < 1e-6 && y.abs() < 1e-6 && (z - 1.0).abs() < 1e-6, "{x} {y} {z}");
}

#[test]
fn distance_to_the_axis_ignores_what_lies_along_it() {
    let (n, r) = toward_axis([0.0, 1.0, 0.0], [3.0, 50.0, 4.0]);
    assert!((r - 5.0).abs() < 1e-5 && (n[0] - 0.6).abs() < 1e-5 && n[1] == 0.0, "{n:?} {r}");
}

#[test]
fn births_on_a_tilted_ring_spread_over_depth_only_with_perspective() {
    let deep = alive(helix(json!([1, 0, 0]), true), 3);
    assert!(deep.len() >= 8, "{}", deep.len());
    assert!(deep.iter().all(|p| (p.pos[0] - 100.0).abs() < 1e-3));
    let (lo, hi) = deep.iter().fold((f32::MAX, f32::MIN), |(l, h), p| (l.min(p.depth[0]), h.max(p.depth[0])));
    assert!(lo < -20.0 && hi > 20.0, "{lo} {hi}");
    let flat = alive(helix(json!([1, 0, 0]), false), 3);
    assert!(flat.iter().all(|p| p.depth[0] == 0.0));
}

#[test]
fn a_swirl_around_a_horizontal_axis_pushes_particles_along_depth() {
    let mut s = helix(json!([0, 0, 1]), true);
    s["emitters"][0]["shape"] = json!({ "point": [100, 70] });
    s["init"]["depth"] = json!([0, 0]);
    s["forces"] = json!([{ "vortex": { "axis": [1, 0, 0], "center": [100, 50], "strength": 200, "falloff": "constant" } }]);
    let all = alive(s, 30);
    assert!(!all.is_empty());
    assert!(all.iter().all(|p| p.pos[0] == 100.0));
    assert!(all.iter().any(|p| p.depth[0].abs() > 10.0 && p.depth[1].abs() > 10.0), "{:?}", all[0]);
}

#[test]
fn an_axis_that_is_not_a_unit_vector_is_refused() {
    let mut s = helix(json!([0, 0, 2]), false);
    assert!(issues(s.clone()).iter().any(|i| i.to_string().contains("axis")));
    s["emitters"][0]["shape"]["around"]["axis"] = json!([0, 0, 1]);
    assert_eq!(issues(s), []);
}
