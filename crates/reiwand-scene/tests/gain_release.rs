use reiwand_scene::*;
use serde_json::json;
use std::collections::BTreeMap;
use std::sync::Arc;

const VIEW: [f32; 4] = [0.0, 0.0, 100.0, 50.0];
const STEP: f32 = 1.0 / 100.0;

fn runtime(audio: serde_json::Value) -> Runtime {
    let layer = json!({ "blend_space": "canvas", "id": "l0", "type": "solid", "color": "#ffffff", "opacity": { "audio": 4, "kind": "relative", "unit": "db" } });
    let text =
        json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": [layer], "audio": audio });
    let scene = Scene::load(&text.to_string(), &BTreeMap::new()).unwrap_or_else(|e| panic!("{e}"));
    Runtime::new(Arc::new(scene), Values::new())
}

fn train(release: f32) -> Vec<f32> {
    let audio = json!({ "attack": 0.001, "release": 0.01, "gain_release": release });
    let mut rt = runtime(audio);
    let mut seen = Vec::new();
    for tick in 0..300 {
        let power = if tick % 50 == 0 { 1.0 } else { 0.002 };
        let frame = AudioFrame { silent: false, power: [[power; BANDS]; 2], ..AudioFrame::default() };
        rt.advance(f64::from(STEP));
        rt.set_audio(frame);
        let drawn = rt.frame(VIEW, Quality::Q3);
        seen.push(drawn.draws.first().map_or(0.0, |d| d.opacity));
    }
    seen
}

#[test]
fn a_short_gain_release_makes_the_bands_dip_after_each_hit_and_recover() {
    let seen = train(0.4);
    for hit in [100, 150, 200] {
        let (hit_level, after, before_next) = (seen[hit], seen[hit + 6], seen[hit + 48]);
        assert!(hit_level > 0.95, "{hit_level}");
        assert!(after < 0.7 * hit_level, "dip {after}");
        assert!(before_next > after + 0.05, "recovery {after} -> {before_next}");
    }
}

#[test]
fn the_default_gain_release_keeps_the_reference_where_it_was() {
    let seen = train(5.0);
    for hit in [100, 150, 200] {
        assert!((seen[hit + 6] - seen[hit + 48]).abs() < 0.03, "{} {}", seen[hit + 6], seen[hit + 48]);
    }
}

#[test]
fn the_gain_release_is_required_and_limited() {
    let layer = json!({ "blend_space": "canvas", "id": "l0", "type": "solid", "color": "#ffffff" });
    let load = |audio: serde_json::Value| {
        let text =
            json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": [layer], "audio": audio });
        Scene::load(&text.to_string(), &BTreeMap::new()).map(|s| s.validate(&BTreeMap::new()).is_empty())
    };
    assert!(load(json!({ "attack": 0.02, "release": 0.2 })).is_err());
    assert_eq!(load(json!({ "gain_release": 0.4 })).ok(), Some(true));
    assert!(load(json!({ "gain_release": 0.01 })).is_err());
    assert!(load(json!({ "gain_release": 11 })).is_err());
}
