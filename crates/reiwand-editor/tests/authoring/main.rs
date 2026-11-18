#[path = "../support/whole.rs"]
mod whole;
use reiwand_editor::Project;
use reiwand_editor::actions::Action;
use reiwand_editor::starters::starter;
use serde_json::{Value, json};
use std::sync::OnceLock;
use tempfile::TempDir;

mod clipped;
mod effects;
mod events;
mod graph;
mod layers;
mod marks;
mod masking;
mod mended;
mod placing;
mod pools;
mod reaching;
mod split;
mod straight;
mod timing;
mod uses;
mod valued;
mod worded;

fn state() {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
}

pub fn act(project: &mut Project, action: Value) -> anyhow::Result<()> {
    let action = whole::action(project, action);
    let action = serde_json::from_value::<Action>(action).unwrap();
    assert!(reiwand_editor::ACTIONS.contains(&action.name().as_str()), "{}", action.name());
    project.act(&action).map(drop)
}

pub fn refused(project: &mut Project, action: Value) -> String {
    let before = project.doc.clone();
    let error = act(project, action).unwrap_err().to_string();
    assert_eq!(project.doc, before, "{error}");
    error
}

pub fn project(root: &TempDir) -> Project {
    state();
    let dir = root.path().join("glow");
    let mut project =
        Project::create(&dir, starter("empty", "org.example.glow", "Glow", "en").unwrap()).unwrap();
    act(&mut project, json!({"add_layer": {"kind": "solid", "id": "card"}})).unwrap();
    project
}

pub fn at(project: &Project, pointer: &str) -> Value {
    project.doc.pointer(pointer).cloned().unwrap_or(Value::Null)
}
