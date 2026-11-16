#![allow(dead_code)]
use reiwand_scene::{Scene, ScriptOutputs};
use reiwand_script::{Error, Frame, Limits, Script, Setup, Source};
use serde_json::json;

pub fn sources(list: &[(&str, &str)]) -> Vec<Source> {
    list.iter().map(|(p, s)| (p.to_string(), s.to_string())).collect()
}

pub fn load(source: &str) -> Script {
    Script::load(&Setup::plain(&sources(&[("main.js", source)])), Limits::default()).unwrap()
}

pub fn setup(scene: serde_json::Value, files: &[(&str, &str)]) -> Setup {
    let mut scene = scene;
    scene["schema"] = json!(reiwand_scene::SCHEMA);
    scene["canvas"] = json!({ "size": [200, 100] });
    let scene = Scene::from_json(&scene.to_string()).unwrap();
    Setup::new(&scene, &sources(files), 1, ("en", &json!({})))
}

pub fn staged(scene: serde_json::Value, files: &[(&str, &str)]) -> Script {
    Script::load(&setup(scene, files), Limits::default()).unwrap()
}

pub fn at(script: &mut Script, time: f64) -> Result<ScriptOutputs, Error> {
    script.update(&Frame { dt: 0.016, ..Frame::at(time) })
}

pub fn text(out: &ScriptOutputs, id: &str) -> String {
    out.layers.get(id).and_then(|l| l.text.clone()).unwrap_or_default()
}
