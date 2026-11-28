mod common;

use common::{sources, text};
use reiwand_scene::{AudioFrame, BANDS, Scene};
use reiwand_script::{Frame, Limits, Script, Setup};
use serde_json::json;

fn listening(source: &str, listens: usize) -> Script {
    let entries: Vec<_> = (0..listens).map(|_| json!({ "kind": "relative", "unit": "db" })).collect();
    let doc = json!({ "schema": reiwand_scene::SCHEMA, "canvas": { "size": [200, 100] },
        "listens": entries,
        "scripts": [{ "path": "main.js", "uses": ["audio", "layers"] }],
        "layers": [{ "blend_space": "canvas", "id": "a", "type": "text", "text": "", "size": 10, "font": "serif" }] });
    let scene = Scene::from_json(&reiwand_fill::whole(&doc.to_string())).unwrap();
    let setup = Setup::new(&scene, &sources(&[("main.js", source)]), 1, ("en", &json!({})));
    Script::load(&setup, Limits::default()).unwrap()
}

const SOURCE: &str = r#"import { scene, audio } from "reiwand";
    export function update(f) {
        let note = "none";
        try { audio.heard(1); } catch (e) { note = e.name; }
        scene.layer("a").text = audio.heard(0)[2] + " " + audio.heard(0).length + " " + note;
    }"#;

#[test]
fn scripts_read_the_bands_of_their_listens() {
    let mut script = listening(SOURCE, 1);
    let audio = AudioFrame { silent: false, ..AudioFrame::default() };
    let mut band = [0.0; BANDS];
    band[2] = 1.5;
    let heard = [band];
    let out = script.update(&Frame { audio: Some(&audio), heard: &heard, ..Frame::at(0.0) }).unwrap();
    assert_eq!(text(&out, "a"), "1.5 64 RangeError");
}
