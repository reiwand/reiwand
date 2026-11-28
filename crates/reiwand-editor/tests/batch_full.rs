use reiwand_editor::Project;
use reiwand_editor::actions::Action;
use reiwand_editor::history::MAX_BYTES;
use reiwand_editor::starters::starter;
use serde_json::json;

fn actions(list: serde_json::Value) -> Vec<Action> {
    serde_json::from_value(list).unwrap()
}

fn full() -> (tempfile::TempDir, Project) {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path().join("state")) };
    let doc = starter("empty", "org.example.full", "Full", "en").unwrap();
    let mut project = Project::create(&root.path().join("full"), doc).unwrap();
    project.history.limits = Some((3, MAX_BYTES));
    for id in ["a", "b", "c"] {
        project.act_all(&actions(json!([{"add_layer": {"kind": "solid", "id": id}}]))).unwrap();
    }
    assert_eq!(project.history.done.len(), 3);
    (root, project)
}

#[test]
fn a_failing_batch_on_a_full_history_leaves_the_project_untouched() {
    let (_root, mut p) = full();
    p.act(&Action::Undo {}).unwrap();
    let before = (p.doc.clone(), p.history.clone());
    let refused = p.act_all(&actions(json!([
        {"add_layer": {"kind": "solid", "id": "d"}},
        {"add_layer": {"kind": "solid", "id": "e"}},
        {"remove_layer": {"id": "missing"}}
    ])));
    assert!(refused.is_err());
    assert_eq!((p.doc.clone(), p.history.clone()), before);
    p.act(&Action::Redo {}).unwrap();
    assert_eq!(p.doc["scene"]["layers"].as_array().unwrap().len(), 3);
}

#[test]
fn a_batch_on_a_full_history_is_one_undo_step() {
    let (_root, mut p) = full();
    let before = p.doc.clone();
    p.act_all(&actions(json!([
        {"add_layer": {"kind": "solid", "id": "d"}},
        {"add_layer": {"kind": "solid", "id": "e"}}
    ])))
    .unwrap();
    assert_eq!(p.history.done.len(), 3);
    assert_eq!(p.history.done.last().unwrap().label, "add_layer*2");
    p.act(&Action::Undo {}).unwrap();
    assert_eq!(p.doc, before);
    assert!(p.history.undone.len() == 1 && p.history.saved.is_none());
}
