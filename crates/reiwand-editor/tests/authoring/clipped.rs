use super::{act, at, project};
use reiwand_editor::Project;
use serde_json::{Value, json};

fn keyed(p: &mut Project, pointer: &str, keys: &[(f64, Value)]) {
    for (t, v) in keys {
        act(p, json!({"keyframe": {"pointer": pointer, "time": t, "value": v}})).unwrap();
    }
}

fn keys(p: &Project, pointer: &str) -> Vec<Value> {
    at(p, &format!("{pointer}/keys")).as_array().cloned().unwrap_or_default()
}

#[test]
fn copied_keys_go_to_a_clipboard_text_that_only_they_are_read_from() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let opacity = "/scene/layers/0/opacity";
    keyed(&mut p, opacity, &[(0.0, json!(0)), (1.0, json!(1)), (2.0, json!(0.5))]);
    let ask = |p: &Project, q: Value| {
        reiwand_editor::rigs::query(root.path(), &p.doc, "", &json!({ "key_clip": q }))
    };
    let copied = ask(&p, json!({"copy": [{"pointer": opacity, "times": [1, 2]}]}));
    let text = copied["text"].as_str().unwrap().to_owned();
    assert!(text.starts_with("reiwand-keys:"), "{text}");
    let read = ask(&p, json!({ "read": text }));
    assert_eq!(read["picks"], json!([{"pointer": opacity, "keys": [[1.0, 1], [2.0, 0.5]]}]));
    for other in ["some words", "reiwand-keys:{\"picks\": []}", "reiwand-keys:[1]", "reiwand-layers:{}"] {
        assert_eq!(ask(&p, json!({ "read": other }))["picks"], json!(null), "{other}");
    }
    assert!(ask(&p, json!({"copy": [{"pointer": opacity, "times": [7]}]}))["error"].is_string());
    let pick = json!([{"pointer": opacity, "keys": read["picks"][0]["keys"]}]);
    act(&mut p, json!({"edit_keys": {"picks": pick, "at": 4}})).unwrap();
    assert_eq!(keys(&p, opacity).len(), 5);
}
