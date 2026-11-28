use crate::builtin::{SHIPPED, list, package};

#[test]
fn every_example_scene_of_the_repository_is_built_in_with_a_title_and_a_picture() {
    let folders = std::fs::read_dir("../../scenes").unwrap().flatten();
    let scenes = folders.filter(|e| e.path().join("manifest.json").is_file()).count();
    let shown = list();
    assert_eq!(shown.len(), scenes);
    assert_eq!(SHIPPED.len(), scenes);
    for entry in &shown {
        assert!(!entry["title"].as_str().unwrap().is_empty(), "{entry}");
        assert!(entry["picture"].as_str().unwrap().starts_with("data:image/"), "{}", entry["name"]);
    }
}

#[test]
fn a_built_in_example_becomes_a_package_the_copy_step_reads() {
    let room = tempfile::tempdir().unwrap();
    let name = SHIPPED[0].0;
    let made = package(name, room.path()).unwrap();
    let facts = reiwand_editor::copying::facts(&made).unwrap();
    assert!(facts["new_id"].as_str().unwrap().ends_with(".copy"), "{facts}");
    assert!(!room.path().join(name).exists());
    assert!(package("no-such-example", room.path()).is_err());
}
