use super::{act, at, project, refused};
use serde_json::json;

fn key(extra: serde_json::Value) -> serde_json::Value {
    let mut k = json!({"pointer": "/scene/layers/0/opacity"});
    k.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
    json!({ "keyframe": k })
}

#[test]
fn straightening_a_curve_is_one_step_that_keeps_the_neighbouring_curves() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let bent = json!({"in": [0.3, 0.1], "out": [0.3, 0.1]});
    let keys = json!({"keys": [[0, 0, {"in": null, "out": [0.3, 0.1]}], [1, 1, bent], [2, 0, bent], [3, 1, {"in": [0.3, 0.1], "out": null}]]});
    act(&mut p, json!({"set": {"pointer": "/scene/layers/0/opacity", "value": keys}})).unwrap();
    let before = p.history.done.len();
    act(&mut p, key(json!({"time": 1, "straighten": true}))).unwrap();
    assert_eq!(p.history.done.len(), before + 1);
    let got = at(&p, "/scene/layers/0/opacity/keys");
    assert_eq!(got[1], json!([1, 1, {"in": [0.3, 0.1]}]));
    assert_eq!(got[2], json!([2, 0, {"out": [0.3, 0.1]}]));
    assert_eq!((&got[0], &got[3]), (&keys["keys"][0], &keys["keys"][3]));
    act(&mut p, key(json!({"time": 0, "ease": "in_out_sine"}))).unwrap();
    act(&mut p, key(json!({"time": 0, "straighten": true}))).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/opacity/keys/0"), json!([0.0, 0]));
    refused(&mut p, key(json!({"time": 1, "straighten": true, "ease": "linear"})));
    refused(&mut p, key(json!({"time": 5, "straighten": true})));
}
