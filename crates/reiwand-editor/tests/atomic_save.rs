use reiwand_editor::Project;
use reiwand_editor::starters::starter;
use serde_json::json;
use std::fs;

#[test]
fn a_save_that_fails_leaves_every_file_as_it_was() {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path().join("state")) };
    let doc = starter("empty", "org.example.atomic", "Atomic", "en").unwrap();
    let dir = root.path().join("atomic");
    let mut project = Project::create(&dir, doc).unwrap();
    let before = (fs::read(dir.join("manifest.json")).unwrap(), fs::read(dir.join("scene.json")).unwrap());
    project.doc["manifest"]["title"] = json!("Changed");
    project.doc["manifest"]["entry"] = json!("../outside.json");
    assert!(project.save().is_err());
    let after = (fs::read(dir.join("manifest.json")).unwrap(), fs::read(dir.join("scene.json")).unwrap());
    assert!(before == after, "a failed save changed the files on disk");
    let left = fs::read_dir(&dir).unwrap().flatten().map(|e| e.file_name().into_string().unwrap());
    assert!(left.filter(|name| name.starts_with('.')).count() == 0, "a temporary file was left behind");
}
