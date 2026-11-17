#[path = "support/whole.rs"]
mod whole;
use reiwand_editor::Project;
use reiwand_editor::actions::Action;
use reiwand_editor::inspect::fields_from;
use reiwand_editor::starters::starter;
use serde_json::json;

#[test]
fn fluid_emitters_are_a_list_placed_on_the_canvas_that_may_be_keyframed() {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path().join("state")) };
    let dir = root.path().join("fluid");
    let mut project =
        Project::create(&dir, starter("empty", "org.example.fluid", "Fluid", "en").unwrap()).unwrap();
    let layer = json!({"add_layer": {"kind": "solid", "id": "fill"}});
    project.act(&serde_json::from_value::<Action>(whole::action(&project, layer)).unwrap()).unwrap();
    let pointer = reiwand_editor::layers::find(&project.doc, "fill").unwrap();
    let effect = json!({"add": {"pointer": format!("{pointer}/effects"), "value": [{ "effect": "fluid" }]}});
    project.act(&serde_json::from_value::<Action>(whole::action(&project, effect)).unwrap()).unwrap();
    let at = format!("{pointer}/effects/0");
    let fields = fields_from(None, &project.doc, &at, 0.0, &project.effects());
    let emitters = fields.iter().find(|f| f.key == "emitters").unwrap();
    assert_eq!(
        (emitters.widget.as_str(), emitters.bindable, &emitters.default),
        ("emitters", true, &json!([]))
    );
    assert!(fields.iter().find(|f| f.key == "strength").unwrap().bindable);
    let list = json!([{ "from": [0.2, 0.9], "strength": 4 }, { "from": [0.1, 1], "to": [0.9, 1], "angle": -80, "strength": 6 }]);
    let set = json!({"set": {"pointer": format!("{at}/params/emitters"), "value": list}});
    project.act(&serde_json::from_value::<Action>(whole::action(&project, set)).unwrap()).unwrap();
    assert!(reiwand_editor::check::review(&project.doc, Some(&dir)).iter().all(|p| p.warning));
    let far = json!({"set": {"pointer": format!("{at}/params/emitters/0/strength"), "value": 99}});
    project.act(&serde_json::from_value::<Action>(whole::action(&project, far)).unwrap()).unwrap();
    let found = reiwand_editor::check::review(&project.doc, Some(&dir));
    assert!(found.iter().any(|p| !p.warning && p.message.contains("emitters")), "{found:?}");
}
