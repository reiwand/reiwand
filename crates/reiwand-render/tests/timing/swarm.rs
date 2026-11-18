use super::hostile::{renderer, show};
use super::{book, draw, one};
use reiwand_render::{drawn_effects, retry_effects, set_build_timeout, set_gpu_budget, stop_effects};
use reiwand_scene::{SCHEMA, Scene};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::Duration;

const PLAIN: &str =
    "fn particle(p: Sprite) -> Sprite {\n    var o = p;\n    o.size = p.size * 2.0;\n    return o;\n}\n";

fn heavy() -> String {
    let step = "v = fract(v * 1.37 + vec4<f32>(0.13));".repeat(4);
    format!(
        "fn particle(p: Sprite) -> Sprite {{\n    var o = p;\n    var v = p.color + vec4<f32>(frame.time * 1e-3);\n    for (var i = 0u; i < 1024u; i++) {{ {step} }}\n    o.color = v;\n    return o;\n}}\n"
    )
}

fn scene(wgsl: &str) -> Scene {
    let system = json!({ "capacity": 400, "emitters": [{ "shape": { "box": { "from": [0, 0], "to": [64, 32] } }, "rate": 600 }],
        "init": { "life": [5, 5], "speed": [0, 0], "size": [12, 12] }, "render": { "shader": "own" } });
    let effects = json!({ "own": { "descriptor": "fx/own.json", "wgsl": "fx/own.wgsl" } });
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [64, 32] }, "particles": { "s": system }, "effects": effects,
        "layers": [{ "blend_space": "canvas", "id": "p", "type": "particles", "system": "s" }] });
    let mut scene = Scene::from_json(&crate::gpu::whole(&text.to_string())).unwrap();
    let read = |path: &str, _: usize| {
        let json = r#"{ "id": "own", "stage": "particle", "cost": {} }"#;
        Ok::<_, &str>(if path.ends_with(".json") { json.to_owned() } else { wgsl.to_owned() }.into_bytes())
    };
    scene.adopt(&BTreeMap::new(), read).unwrap();
    scene
}

#[test]
fn slow_particle_shaders_are_left_out_for_the_run_and_the_particles_still_draw() {
    let _one = one();
    let Some(r) = renderer() else { return };
    book("slow-particles");
    let s = scene(&heavy());
    let key = s.loaded["own"].key;
    set_gpu_budget(Some(Duration::from_micros(1)));
    let mut shown = show(r, s).unwrap();
    for _ in 0..60 {
        draw(r, &mut shown, (64, 32));
        if !shown.view.left_out().is_empty() {
            break;
        }
    }
    set_gpu_budget(None);
    let left = shown.view.left_out();
    assert!(left.len() == 1 && left[0].contains("own") && left[0].contains("too slow"), "{left:?}");
    draw(r, &mut shown, (64, 32));
    r.finish().unwrap();
    assert!(!drawn_effects().contains(&key), "the slow shader is drawn no more");
    let mut again = show(r, scene(&heavy())).unwrap();
    draw(r, &mut again, (64, 32));
    assert!(again.view.left_out().iter().any(|l| l.contains("too slow")), "a new view leaves it out too");
    assert!(retry_effects() >= 1);
    let mut fresh = show(r, scene(&heavy())).unwrap();
    draw(r, &mut fresh, (64, 32));
    assert!(fresh.view.left_out().is_empty(), "{:?}", fresh.view.left_out());
    r.finish().unwrap();
}

#[test]
fn particle_shaders_that_build_too_long_or_stopped_the_gpu_are_left_out() {
    let _one = one();
    let Some(r) = renderer() else { return };
    book("slow-particle-builds");
    set_build_timeout(Duration::ZERO);
    let mut shown = show(r, scene(PLAIN)).unwrap();
    draw(r, &mut shown, (64, 32));
    set_build_timeout(Duration::from_secs(10));
    let left = shown.view.left_out();
    assert!(left.len() == 1 && left[0].contains("own") && left[0].contains("to build"), "{left:?}");
    reiwand_scene::forgive_strikes().unwrap();
    let s = scene(PLAIN);
    stop_effects([s.loaded["own"].key]);
    let mut stopped = show(r, s).unwrap();
    draw(r, &mut stopped, (64, 32));
    assert!(stopped.view.left_out().iter().any(|l| l.contains("stopped")), "{:?}", stopped.view.left_out());
    retry_effects();
    let mut fine = show(r, scene(PLAIN)).unwrap();
    draw(r, &mut fine, (64, 32));
    assert!(fine.view.left_out().is_empty(), "{:?}", fine.view.left_out());
    r.finish().unwrap();
}
