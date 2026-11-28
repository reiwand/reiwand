mod declared;
mod reverts;
mod tables;
mod trees;
#[path = "../support/whole.rs"]
mod whole;

use reiwand_editor::Project;
use reiwand_editor::actions::Action;
use reiwand_editor::starters::starter;
use serde_json::Value;
use std::sync::OnceLock;
use tempfile::TempDir;

pub fn act(p: &mut Project, action: Value) -> anyhow::Result<()> {
    let action = whole::action(p, action);
    p.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

pub fn project(root: &TempDir) -> Project {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
    let made = starter("empty", "org.example.coding", "Coding", "en").unwrap();
    Project::create(&root.path().join("coding"), made).unwrap()
}

pub fn script(p: &mut Project, name: &str, text: &str) {
    std::fs::create_dir_all(p.dir.join("scripts")).unwrap();
    std::fs::write(p.dir.join("scripts").join(name), text).unwrap();
}

pub fn refused(p: &mut Project, action: Value) -> String {
    act(p, action).unwrap_err().to_string()
}
