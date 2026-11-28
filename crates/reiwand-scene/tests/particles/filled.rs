use super::*;

fn loaded(patch: Json) -> Scene {
    Scene::from_json(&reiwand_fill::whole(&scene(with(sparks(), patch), true))).unwrap()
}

fn fog(capacity: u32) -> Scene {
    loaded(json!({ "capacity": capacity, "init": { "life": [3, 3], "size": [60, 60] },
        "emitters": [{ "shape": { "box": { "from": [0, 0], "to": [200, 100] } }, "rate": 2 }] }))
}

fn live(s: &Scene) -> f32 {
    alive(s, Some(&s.layers[0]), s.particles.values().next().unwrap())
}

#[test]
fn a_fog_of_few_live_particles_keeps_its_size() {
    let s = fog(10_000);
    assert!((live(&s) - 6.0).abs() < 1e-4, "{}", live(&s));
    assert!((lives(&s, &flocks(&s)[0])[0] - 6.0).abs() < 1e-4);
    for tier in [Tier::T1, Tier::T2, Tier::T3] {
        let bake = Bake::new(&s, 0, tier, false, |_| None).unwrap();
        assert_eq!(bake.sizes[0], 1.0, "{tier:?}");
    }
    let t0 = Bake::new(&s, 0, Tier::T0, false, |_| None).unwrap().sizes[0];
    assert!((t0 - (1.0f32 / (6.0 * 0.18)).sqrt()).abs() < 1e-4, "{t0}");
}

#[test]
fn a_system_past_the_cap_shrinks_as_before() {
    let s = loaded(json!({ "capacity": 8000, "init": { "size": [40, 60] },
        "emitters": [{ "shape": { "box": { "from": [0, 0], "to": [200, 100] } }, "rate": 100_000 }] }));
    assert_eq!(live(&s), 8000.0);
    let bake = Bake::new(&s, 0, Tier::T3, false, |_| None).unwrap();
    let cover = 8000.0 * 50.0 * 50.0 / (200.0 * 100.0);
    assert!((bake.sizes[0] - (6.0f32 / cover).sqrt()).abs() < 1e-5, "{}", bake.sizes[0]);
}

#[test]
fn bursts_steps_and_cues_bound_what_lives() {
    let burst = |b: Json, most: Option<u32>| {
        let mut e =
            json!({ "shape": { "box": { "from": [0, 0], "to": [200, 100] } }, "rate": 0, "bursts": [b] });
        most.iter().for_each(|n| e["max_per_step"] = json!(n));
        live(&loaded(json!({ "capacity": 10_000, "init": { "life": [2.5, 2.5] }, "emitters": [e] })))
    };
    let click = json!({ "count": 5, "every": null, "on": "click", "at": 0 });
    assert_eq!(burst(json!({ "count": 5, "every": 1, "at": 0 }), None), 15.0);
    assert_eq!(burst(json!({ "count": 40, "every": null, "at": 0 }), None), 40.0);
    assert_eq!(burst(click.clone(), None), 10_000.0);
    assert_eq!(burst(click, Some(2)), 2.0 * ((2.5 / STEP).floor() + 1.0));
}

#[test]
fn the_cost_counts_what_can_live() {
    let (many, few) = (fog(10_000), fog(6));
    let unit = Quality::Q3.particles() * Quality::Q3.render_scale_for(1.0).powi(2);
    let slots = (10_000.0 - 6.0) / 100_000.0 * unit;
    let gap = many.cost(Quality::Q3) - few.cost(Quality::Q3);
    assert!((gap - slots).abs() < 1e-3, "{gap} {slots}");
    let empty = loaded(json!({ "capacity": 6, "init": { "size": [0, 0] } }));
    let drawn = few.cost(Quality::Q3) - empty.cost(Quality::Q3);
    assert!((drawn - 6.0 * 0.18 * unit).abs() < 1e-3, "{drawn}");
}
