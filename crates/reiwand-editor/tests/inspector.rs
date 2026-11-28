use reiwand_editor::ACTIONS;
use reiwand_editor::actions::{Action, plan};
use reiwand_editor::inspect::fields;
use reiwand_editor::ops::apply;
use reiwand_editor::starters::starter;
use serde_json::{Value, json};

fn act(doc: &mut Value, action: Value) {
    let action: Action = serde_json::from_value(action).unwrap();
    assert!(ACTIONS.contains(&action.name().as_str()));
    apply(doc, &reiwand_editor::completing::complete_all(doc, plan(doc, &action).unwrap())).unwrap();
}

#[test]
fn the_inspector_follows_the_schema() {
    let mut doc = starter("parallax", "org.example.test", "Test", "en").unwrap();
    act(&mut doc, json!({"set": {"pointer": "/scene/layers/1/effects", "value": [{"effect": "vignette"}]}}));
    let shown = fields(&doc, "/scene/layers/1");
    let field = |key: &str| shown.iter().find(|f| f.key == key).unwrap_or_else(|| panic!("{key}"));
    assert_eq!((field("opacity").widget.as_str(), field("opacity").bindable), ("number", true));
    assert_eq!(field("opacity").range, json!([0, 1]));
    assert_eq!((field("blend").widget.as_str(), field("blend").options.len()), ("enum", 34));
    assert_eq!((field("position").widget.as_str(), field("position").unit.as_str()), ("vec2", "px"));
    // A row of numbers not set yet starts from as many zeros as it holds.
    assert_eq!((field("margin").widget.as_str(), &field("margin").sample), ("numbers", &json!([0, 0, 0, 0])));
    assert_eq!(field("fill").widget, "color");
    assert_eq!(field("shape").widget, "variant");
    assert!(
        field("shape").options.contains(&json!("star"))
            && field("size").pointer.ends_with("/shape/ellipse/size")
    );
    let effect = fields(&doc, "/scene/layers/1/effects/0");
    let id = effect.iter().find(|f| f.key == "effect").unwrap();
    assert!(id.widget == "ref:effect" && id.options.contains(&json!("vignette")) && id.options.len() > 30);
    let clock = fields(&doc, "/scene/clock");
    let fps = clock.iter().find(|f| f.key == "max_fps").unwrap();
    assert_eq!(
        (fps.widget.as_str(), &fps.options, &fps.value),
        ("integer", &vec![json!("display")], &json!("display"))
    );
    let intensity = effect.iter().find(|f| f.key == "intensity").unwrap();
    assert_eq!(intensity.range, json!([0.0, 1.0]));
    assert!((intensity.default.as_f64().unwrap() - 0.35).abs() < 1e-6);
    act(&mut doc, json!({"add_layer": {"kind": "particles"}}));
    let system = fields(&doc, "/scene/particles/particles");
    assert!(system.iter().any(|f| f.key == "emitters" && f.widget == "list" && f.sample.is_object()));
    assert!(system.iter().any(|f| f.key == "size_over_life" && f.widget == "curve"));
    let prop = fields(
        &starter("particles", "org.example.test", "Test", "en").unwrap(),
        "/manifest/properties/density",
    );
    assert!(prop.iter().any(|f| f.key == "max" && f.widget == "number"));
    let layer = fields(&doc, "/scene/layers/5");
    assert!(
        layer
            .iter()
            .any(|f| f.key == "system" && f.widget == "ref:system" && f.options == [json!("particles")])
    );
    assert!(layer.iter().any(|f| f.key == "system" && f.required));
    assert!(layer.iter().any(|f| f.key == "opacity" && f.required));
    act(
        &mut doc,
        json!({"set": {"pointer": "/scene/layers/1/opacity", "value": {"audio": "bass", "kind": "relative", "unit": "db", "smooth": 0.2}}}),
    );
    let binding = fields(&doc, "/scene/layers/1/opacity");
    let field = |key: &str| binding.iter().find(|f| f.key == key).unwrap_or_else(|| panic!("{key}"));
    assert_eq!((field("audio").widget.as_str(), &field("audio").value), ("enum", &json!("bass")));
    assert_eq!((field("timeline").widget.as_str(), field("smooth").set), ("ref:timeline", true));
    let label = prop.iter().find(|f| f.key == "label").unwrap();
    assert_eq!(label.value, json!({"en": "Density", "es": "Densidad"}));
}
