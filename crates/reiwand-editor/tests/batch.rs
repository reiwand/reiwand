use reiwand_editor::Project;
use reiwand_editor::actions::Action;
use reiwand_editor::starters::starter;
use serde_json::json;

fn actions(list: serde_json::Value) -> Vec<Action> {
    serde_json::from_value(list).unwrap()
}

fn made() -> (tempfile::TempDir, Project) {
    let root = tempfile::tempdir().unwrap();
    let doc = starter("empty", "org.example.batch", "Batch", "en").unwrap();
    let project = Project::create(&root.path().join("batch"), doc).unwrap();
    (root, project)
}

#[test]
fn several_actions_are_one_undo_step() {
    let (_root, mut p) = made();
    let before = p.doc.clone();
    let steps = p.history.done.len();
    p.act_all(&actions(json!([
        {"add_layer": {"kind": "solid", "id": "a"}},
        {"add_layer": {"kind": "solid", "id": "b"}},
        {"hide": {"id": "a", "hidden": true}}
    ])))
    .unwrap();
    assert_eq!(p.history.done.len(), steps + 1);
    assert_eq!(p.doc["scene"]["layers"].as_array().unwrap().len(), 2);
    p.act(&Action::Undo {}).unwrap();
    assert_eq!(p.doc, before);
    p.act(&Action::Redo {}).unwrap();
    assert_eq!(p.doc["scene"]["layers"].as_array().unwrap().len(), 2);
}

#[test]
fn a_failing_action_leaves_the_project_untouched() {
    let (_root, mut p) = made();
    let before = (p.doc.clone(), p.history.clone());
    let refused = p.act_all(&actions(json!([
        {"add_layer": {"kind": "solid", "id": "a"}},
        {"remove_layer": {"id": "missing"}}
    ])));
    assert!(refused.is_err());
    assert_eq!((p.doc.clone(), p.history.clone()), before);
    assert!(p.act_all(&actions(json!([{"undo": {}}]))).is_err());
}

#[test]
fn the_undo_step_of_a_batch_names_its_actions() {
    let (_root, mut p) = made();
    p.act_all(&actions(json!([
        {"add_layer": {"kind": "solid", "id": "a"}},
        {"hide": {"id": "a", "hidden": true}},
        {"add_layer": {"kind": "solid", "id": "b"}}
    ])))
    .unwrap();
    let ask = p.ask("", &json!({ "history": {} }));
    let said = |name| reiwand_editor::history::step(name).local();
    assert_eq!(ask["undo"]["text"], format!("{}, {}", said("add_layer"), said("hide")));
    assert_eq!(ask["undo"]["label"], "batch:add_layer,hide");
}

#[test]
fn removing_several_layers_is_one_undo_step() {
    let (_root, mut p) = made();
    p.act_all(&actions(json!([
        {"add_layer": {"kind": "solid", "id": "a"}},
        {"add_layer": {"kind": "solid", "id": "b"}},
        {"add_layer": {"kind": "solid", "id": "c"}}
    ])))
    .unwrap();
    let full = p.doc.clone();
    p.act_all(&actions(json!([
        {"remove_layer": {"id": "a"}},
        {"remove_layer": {"id": "b"}},
        {"remove_layer": {"id": "c"}}
    ])))
    .unwrap();
    assert_eq!(p.doc["scene"]["layers"].as_array().unwrap().len(), 0);
    p.act(&Action::Undo {}).unwrap();
    assert_eq!(p.doc, full);
}

#[test]
fn the_undo_step_of_several_same_actions_says_how_many() {
    let (_root, mut p) = made();
    let add = |id| json!({"add_layer": {"kind": "solid", "id": id}});
    p.act_all(&actions(json!([add("a"), add("b"), add("c")]))).unwrap();
    let ask = p.ask("", &json!({ "history": {} }));
    assert_eq!(
        ask["undo"]["text"],
        reiwand_editor::history::step("add_layer*3").local()
    );
    let remove = |id| json!({"remove_layer": {"id": id}});
    p.act_all(&actions(json!([remove("a"), remove("b")]))).unwrap();
    assert_eq!(p.ask("", &json!({ "history": {} }))["undo"]["text"], reiwand_editor::history::step("remove_layer*2").local());
}
