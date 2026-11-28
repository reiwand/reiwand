use super::*;

fn ring(shape: Json, count: u32) -> Vec<f32> {
    let s = with(
        sparks(),
        json!({ "capacity": 1000, "emitters": [{ "shape": { "sphere": shape }, "rate": 0, "bursts": [{ "count": count }] }],
            "init": { "life": [30, 30], "speed": [0, 0], "size": [4, 4] }, "forces": [], "collide": [] }),
    );
    let all = run(&scene(s, true), 2, Quality::Q3).particles;
    let mut d: Vec<f32> =
        all.iter().filter(|p| p.age < p.life).map(|p| (p.pos[0] - 100.0).hypot(p.pos[1] - 50.0)).collect();
    d.sort_by(f32::total_cmp);
    d
}

fn flat(push: Json) -> Json {
    json!({ "center": [100, 50], "depth": 0, "radius": [200, 200, 0], "inner": 0, "sign": [0, 0, 0], "push": push })
}

fn at(d: &[f32], q: f32) -> f32 {
    d[((d.len() - 1) as f32 * q).round() as usize]
}

#[test]
fn a_flat_ball_pushed_out_is_a_ring_with_the_spread_of_a_ball_seen_from_the_front() {
    let d = ring(flat(json!(200)), 200);
    assert_eq!(d.len(), 200);
    assert!(d[0] >= 200.0 - 1e-3 && d[199] <= 400.0 + 1e-3, "{} {}", d[0], d[199]);
    for (q, want) in [(0.25, 284.0), (0.5, 322.0), (0.75, 356.0), (0.9, 376.0)] {
        let got = at(&d, q);
        assert!((got - want).abs() <= 10.0, "p{}: {got} against {want}", q * 100.0);
    }
}

#[test]
fn many_births_follow_the_density_of_the_projected_ball() {
    let d = ring(flat(json!(200)), 1000);
    for (q, rho) in [(0.25, 0.4178_f32), (0.5, 0.6083), (0.75, 0.7766), (0.9, 0.8858)] {
        let got = (at(&d, q) - 200.0) / 200.0;
        assert!((got - rho).abs() < 0.04, "p{}: {got} against {rho}", q * 100.0);
    }
}

#[test]
fn no_push_and_a_push_of_zero_are_the_same_ball() {
    let mut bare = flat(json!(0));
    bare.as_object_mut().unwrap().remove("push");
    assert_eq!(ring(bare.clone(), 200), ring(flat(json!(0)), 200));
    assert_eq!(ring(bare, 200), ring(flat(Json::Null), 200));
}

#[test]
fn push_is_checked_and_named_as_a_feature() {
    let s = |push: Json| {
        let mut s = sparks();
        s["emitters"][0]["shape"] = json!({ "sphere": flat(push) });
        s
    };
    assert_eq!(issues(s(json!(-1))).len(), 1);
    let found =
        |push: Json| Scene::from_json(&reiwand_fill::whole(&scene(s(push), true))).unwrap().features();
    assert!(found(json!(20)).contains("particle_push") && found(json!(20)).contains("particle_depth"));
    assert!(!found(Json::Null).contains("particle_push"));
}
