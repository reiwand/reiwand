use reiwand_editor::Project;
use reiwand_editor::actions::Action;
use reiwand_editor::starters::starter;
use serde_json::{Value, json};
use std::sync::OnceLock;
use tempfile::TempDir;

mod canvas;
mod clip;
mod names;

fn state() {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
}

pub fn act(project: &mut Project, action: Value) -> anyhow::Result<()> {
    let action = serde_json::from_value::<Action>(action).unwrap();
    assert!(reiwand_editor::ACTIONS.contains(&action.name().as_str()), "{}", action.name());
    project.act(&action).map(drop)
}

pub fn at(p: &Project, pointer: &str) -> Value {
    p.doc.pointer(pointer).cloned().unwrap_or(Value::Null)
}

pub fn picture(dir: &std::path::Path, name: &str, shade: u8) -> std::path::PathBuf {
    let file = dir.join(name);
    image::RgbaImage::from_pixel(8, 8, image::Rgba([shade, 100, 50, 255])).save(&file).unwrap();
    file
}

pub fn scene(root: &TempDir, name: &str) -> Project {
    state();
    let dir = root.path().join(name);
    let mut p = Project::create(&dir, starter("empty", "org.example.ids", "Ids", "en").unwrap()).unwrap();
    act(&mut p, json!({"set": {"pointer": "/scene/canvas/size", "value": [1000, 1000]}})).unwrap();
    p
}

pub fn family(p: &mut Project) {
    let layers = json!([
        { "blend_space": "canvas", "id": "base", "type": "solid", "color": "#ffffff", "size": [100, 100], "solid": true,
            "transform": {"position": [300, 300], "rotation": 30, "scale": [2, 2]}},
        { "blend_space": "canvas", "id": "kid", "type": "solid", "color": "#ff0000", "size": [40, 40], "parent": "base",
            "transform": {"position": [50, 0]}, "mask": {"layer": "base", "channel": "a"}},
        { "blend_space": "canvas", "id": "box", "type": "group", "layers": [
            { "blend_space": "canvas", "id": "lead", "type": "solid", "color": "#00ff00", "size": [30, 30]},
            { "blend_space": "canvas", "id": "tail", "type": "solid", "color": "#0000ff", "size": [30, 30], "parent": "lead",
                "mask": {"layer": "lead"}, "transform": {"position": [10, 10]}}
        ]}
    ]);
    act(p, json!({"set": {"pointer": "/scene/layers", "value": layers}})).unwrap();
    let timeline = json!([{"id": "pop", "duration": 1, "trigger": "click:base"}]);
    act(p, json!({"set": {"pointer": "/scene/timelines", "value": timeline}})).unwrap();
    act(p, json!({"lock": {"id": "base", "locked": true}})).unwrap();
}
