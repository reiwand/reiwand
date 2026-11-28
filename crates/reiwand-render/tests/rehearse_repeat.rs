use reiwand_render::{Rehearsal, Renderer, SceneAssets};
use serde_json::json;

fn fog() -> reiwand_scene::Scene {
    let mut doc = json!({ "schema": reiwand_scene::SCHEMA, "canvas": { "size": [160, 80], "background": "#000000" },
        "particles": { "fog": { "capacity": 2048, "seed": 3,
            "emitters": [{ "shape": { "box": { "from": [0, 0], "to": [160, 80] } }, "rate": 900 }],
            "init": { "life": [0.2, 0.9], "speed": [5, 40], "angle": [0, 360], "size": [6, 14],
                "color": ["#ff303080", "#3060ff60"] } } },
        "layers": [{ "id": "f", "type": "particles", "system": "fog", "blend": "normal" }] });
    reiwand_fill::complete(reiwand_fill::Format::Scene, &mut doc);
    reiwand_scene::Scene::load(&doc.to_string(), &Default::default()).unwrap()
}

#[test]
fn a_render_of_translucent_particles_gives_the_same_bytes_every_time() {
    let r = match Renderer::shared() {
        Ok(r) => r,
        Err(e) if std::env::var_os("REIWAND_REQUIRE_GPU").is_none() => {
            return eprintln!("skipping GPU test: {e}");
        }
        Err(e) => panic!("GPU required but unavailable: {e}"),
    };
    let scene = fog();
    let wanted = Rehearsal {
        edge: 160,
        wall: 0.0,
        time: 2.0,
        frames: 30,
        fps: 30,
        simulate: true,
        ..Rehearsal::default()
    };
    let reel = || r.rehearse(&scene, &SceneAssets::default(), &Default::default(), &wanted).unwrap().frames;
    let (a, b) = (reel(), reel());
    assert!(a.iter().any(|f| f.pixels().any(|p| p.0[0] > 40)), "the fog is drawn");
    let differ: Vec<usize> = (0..a.len()).filter(|&i| a[i].as_raw() != b[i].as_raw()).collect();
    assert!(differ.is_empty(), "frames {differ:?} differ between two renders");
}
