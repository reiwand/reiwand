use super::fields::{both, system};
use super::*;

fn lone(y: f32, stretch: Value) -> Value {
    json!({ "capacity": 8, "seed": 5, "emitters": [{ "shape": { "point": [10, y] }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [30, 30], "velocity": [[60, 0], [60, 0]], "size": [6, 6], "color": ["#ffffff", "#ffffff"] },
        "render": { "stretch": stretch } })
}

fn length(pixels: &[u8], y: u32) -> u32 {
    let lit: Vec<u32> = (0..SIZE.0).filter(|x| at(pixels, *x, y)[0] > 100).collect();
    lit.last().zip(lit.first()).map_or(0, |(b, a)| b - a + 1)
}

#[test]
fn relative_stretch_scales_the_side_and_may_be_shorter_than_it() {
    let Some(r) = renderer() else { return };
    let systems = json!({
        "none": lone(10.0, json!({ "mode": "add", "time": 0, "min": 0, "max": 0 })),
        "added": lone(30.0, json!({ "mode": "add", "time": 0.1, "min": 0, "max": 100 })),
        "long": lone(50.0, json!({ "mode": "relative", "time": 0.05, "min": 0, "max": 10 })),
        "short": lone(70.0, json!({ "mode": "relative", "time": 0.01, "min": 0, "max": 10 })) });
    let layers = json!(
        ["none", "added", "long", "short"]
            .map(|id| json!({ "id": id, "type": "particles", "system": id, "blend": "add" }))
    );
    for gpu in [false, true] {
        let Some(mut run) = Run::new(r, systems.clone(), layers.clone(), gpu) else { return };
        run.frame(r, 0.0);
        let pixels = (0..6).map(|_| run.frame(r, 1.0 / 30.0)).last().unwrap();
        let rows = [10, 30, 50, 70].map(|y| length(&pixels, y));
        let near = |a: u32, b: u32| a.abs_diff(b) <= 2;
        assert!(
            near(rows[0], 6) && near(rows[1], 12) && near(rows[2], 18) && near(rows[3], 4),
            "gpu {gpu}: {rows:?}"
        );
        looks_like("strokes", &pixels);
    }
}

#[test]
fn oscillating_from_the_floor_agrees_on_both_paths() {
    let Some(r) = renderer() else { return };
    let mut waves = Vec::new();
    for (what, from) in [("alpha", "floor"), ("size", "floor"), ("alpha", "top")] {
        let wave = json!({ "oscillate": { "what": what, "from": from, "freq": [0.5, 2], "scale": [0.2, 1], "phase": [0, 1] } });
        waves.push(json!({ "s": system(json!({ "box": { "from": [0, 0], "to": [160, 80] } }), json!({}), json!({ "forces": [wave] })) }));
    }
    let layers = json!([{ "id": "p", "type": "particles", "system": "s" }]);
    for systems in waves {
        let Some(found) = both(r, &systems, &layers, 10) else { return };
        assert_eq!(found[0].len(), found[1].len());
        assert!(found[0].len() > 50);
        for (c, g) in found[0].iter().zip(&found[1]) {
            assert!(c.look.iter().zip(g.look).all(|(a, b)| (a - b).abs() < 0.02), "{c:?} {g:?}");
        }
    }
}
