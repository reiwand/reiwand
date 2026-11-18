#[path = "support/whole.rs"]
mod whole;
use reiwand_editor::Project;
use reiwand_editor::actions::{Action, plan};
use reiwand_editor::ops::apply;
use reiwand_editor::starters::starter;
use serde_json::{Value, json};

fn act(doc: &mut Value, action: Value) -> anyhow::Result<()> {
    let action: Action = serde_json::from_value(whole::action_doc(doc, action))?;
    apply(doc, &reiwand_editor::completing::complete_all(doc, plan(doc, &action)?)).map(drop)
}

fn run(p: &mut Project, action: Value) -> anyhow::Result<()> {
    p.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

fn spaces(doc: &Value) -> Vec<Value> {
    let layers = doc["scene"]["layers"].as_array().unwrap();
    layers.iter().map(|l| l["blend_space"].clone()).collect()
}

#[test]
fn every_new_layer_starts_in_the_space_of_the_canvas() {
    let mut doc = starter("parallax", "org.example.test", "Test", "en").unwrap();
    assert!(spaces(&doc).iter().all(|s| s == "canvas"));
    for kind in ["solid", "group", "particles", "adjustment"] {
        act(&mut doc, json!({"add_layer": {"kind": kind, "id": format!("new_{kind}")}})).unwrap();
    }
    assert!(spaces(&doc).iter().all(|s| s == "canvas"), "{:?}", spaces(&doc));
}

#[test]
fn the_space_of_one_layer_is_chosen_by_an_action() {
    let state = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", state.path()) };
    unsafe { std::env::set_var("XDG_CACHE_HOME", state.path().join("cache")) };
    let root = tempfile::tempdir().unwrap();
    let doc = starter("empty", "org.example.spaces", "Spaces", "en").unwrap();
    let mut p = Project::create(&root.path().join("spaces"), doc).unwrap();
    run(&mut p, json!({"add_layer": {"kind": "solid", "id": "smoke"}})).unwrap();
    for space in ["srgb", "linear", "canvas"] {
        run(&mut p, json!({"layer": {"blend_space": {"layer": "smoke", "space": space}}})).unwrap();
        assert_eq!(p.doc["scene"]["layers"][0]["blend_space"], json!(space));
    }
    assert!(run(&mut p, json!({"layer": {"blend_space": {"layer": "nope", "space": "srgb"}}})).is_err());
}
