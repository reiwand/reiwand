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
    assert_eq!(ask["undo"]["text"], reiwand_editor::history::step("add_layer*3").local());
    let remove = |id| json!({"remove_layer": {"id": id}});
    p.act_all(&actions(json!([remove("a"), remove("b")]))).unwrap();
    assert_eq!(
        p.ask("", &json!({ "history": {} }))["undo"]["text"],
        reiwand_editor::history::step("remove_layer*2").local()
    );
}

fn scripts(root: &tempfile::TempDir) -> Vec<String> {
    let dir = root.path().join("batch/scripts");
    let names = std::fs::read_dir(dir).into_iter().flatten().flatten();
    names.map(|e| e.file_name().to_string_lossy().into_owned()).collect()
}

#[test]
fn a_failing_batch_takes_away_the_files_its_actions_made() {
    let (root, mut p) = made();
    let before = p.doc.clone();
    let refused = p.act_all(&actions(json!([
        {"import_script": {"file": "", "template": "empty"}},
        {"remove_layer": {"id": "missing"}}
    ])));
    assert!(refused.is_err());
    assert_eq!(scripts(&root), Vec::<String>::new());
    assert_eq!(p.doc, before);
}

#[test]
fn a_failing_batch_gives_a_script_back_its_code() {
    let (root, mut p) = made();
    p.act_all(&actions(json!([{"import_script": {"file": "", "template": "empty"}}]))).unwrap();
    let path = p.doc["scene"]["scripts"][0]["path"].as_str().unwrap().to_owned();
    let file = root.path().join("batch").join(&path);
    let written = |text: &str| actions(json!([{"script": {"write": {"path": path, "text": text}}}]));
    p.act_all(&written("export function update() { return 1; }")).unwrap();
    let kept = std::fs::read_to_string(&file).unwrap();
    let mut failing = written("export function update() { return 2; }");
    failing.extend(actions(json!([{"remove_layer": {"id": "missing"}}])));
    assert!(p.act_all(&failing).is_err());
    assert_eq!(std::fs::read_to_string(&file).unwrap(), kept);
    assert_eq!(scripts(&root), vec![file.file_name().unwrap().to_string_lossy().into_owned()]);
}

#[test]
fn a_batch_ends_like_the_same_actions_one_by_one_and_its_time_grows_with_its_size() {
    let adds = |n: usize, from: usize| {
        let list: Vec<_> = (from..from + n)
            .map(|k| json!({"add_layer": {"kind": "solid", "id": format!("l{k}")}}))
            .collect();
        actions(json!(list))
    };
    let (_one, mut alone) = made();
    adds(40, 0).iter().for_each(|action| drop(alone.act(action).unwrap()));
    let (_all, mut batched) = made();
    batched.act_all(&adds(40, 0)).unwrap();
    assert_eq!(batched.doc, alone.doc, "the batch did not end like its actions one by one");
    let timed = |n: usize| {
        let (_root, mut p) = made();
        let start = std::time::Instant::now();
        p.act_all(&adds(n, 0)).unwrap();
        start.elapsed().as_secs_f64()
    };
    let (small, large) = (timed(100), timed(400));
    // Four times the actions: about 5 times the time now, 7 when each action synced the manifest.
    assert!(large < small * 6.0, "400 adds took {large:.2} s, 100 took {small:.2} s: far more than linear");
}
