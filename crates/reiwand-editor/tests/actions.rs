use reiwand_editor::actions::{Action, plan, visible};
use reiwand_editor::check::check;
use reiwand_editor::keys::value_at;
use reiwand_editor::ops::apply;
use reiwand_editor::starters::starter;
use reiwand_editor::{ACTIONS, STARTERS};
use serde_json::{Value, json};

fn act(doc: &mut Value, action: Value) {
    let action: Action = serde_json::from_value(action).unwrap();
    assert!(ACTIONS.contains(&action.name().as_str()));
    apply(doc, &reiwand_editor::completing::complete_all(doc, plan(doc, &action).unwrap())).unwrap();
}

fn ids(doc: &Value, list: &str) -> Vec<String> {
    doc.pointer(list)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l["id"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn every_starting_point_is_a_valid_scene() {
    for name in STARTERS {
        let doc = starter(name, "org.example.test", "Test", "en").unwrap();
        assert_eq!(check(&doc), [], "{name}");
        let manifest: reiwand_format::Manifest = serde_json::from_value(doc["manifest"].clone()).unwrap();
        assert!(manifest.content.is_some() && manifest.photosensitive == Some(false));
        let pointer = name == "parallax";
        assert_eq!(manifest.inputs.len(), usize::from(pointer), "{name}");
        assert_eq!(manifest.features() == ["inputs.pointer"], pointer, "{name}");
    }
    assert!(starter("nope", "org.example.test", "Test", "en").is_err());
}

#[test]
fn layer_actions_keep_the_scene_valid() {
    let mut doc = starter("parallax", "org.example.test", "Test", "en").unwrap();
    act(&mut doc, json!({"add_layer": {"kind": "text"}}));
    act(&mut doc, json!({"add_layer": {"kind": "particles", "at": 0}}));
    assert_eq!(doc["scene"]["particles"]["particles"]["capacity"], 3000);
    act(&mut doc, json!({"duplicate_layer": {"id": "sun"}}));
    act(&mut doc, json!({"group": {"ids": ["near", "far"], "id": "hills"}}));
    assert_eq!(
        ids(&doc, "/scene/layers"),
        ["particles", "sky", "sun", "sun_copy", "hills", "middle", "text"]
    );
    assert_eq!(ids(&doc, "/scene/layers/4/layers"), ["far", "near"]);
    act(&mut doc, json!({"move_layer": {"id": "middle", "to": 0, "into": "hills"}}));
    act(&mut doc, json!({"move_layer": {"id": "text", "to": 0}}));
    assert_eq!(ids(&doc, "/scene/layers/5/layers"), ["middle", "far", "near"]);
    act(&mut doc, json!({"ungroup": {"id": "hills"}}));
    act(&mut doc, json!({"remove_layer": {"id": "sun_copy"}}));
    assert_eq!(ids(&doc, "/scene/layers"), ["text", "particles", "sky", "sun", "middle", "far", "near"]);
    act(&mut doc, json!({"hide": {"id": "sky", "hidden": true}}));
    assert_eq!(doc["scene"]["layers"][2]["visible"], false);
    act(&mut doc, json!({"hide": {"id": "sky", "hidden": false}}));
    assert_eq!(doc["scene"]["layers"][2]["visible"], true);
    act(&mut doc, json!({"lock": {"id": "sky", "locked": true}}));
    act(&mut doc, json!({"solo": {"id": "sun"}}));
    assert_eq!(doc["scene"]["ext"]["editor"], json!({"locked": ["sky"], "solo": "sun"}));
    let shown = visible(&doc);
    assert_eq!(
        (&shown["layers"][3].get("visible"), &shown["layers"][2]["visible"]),
        (&Some(&json!(true)), &json!(false))
    );
    act(&mut doc, json!({"set": {"pointer": "/scene/layers/3/opacity", "value": 0.5}}));
    let found = check(&doc);
    let covered: Vec<&str> =
        found.iter().filter(|p| p.message.contains("never shows")).map(|p| p.pointer.as_str()).collect();
    assert_eq!(covered, ["/scene/layers/0", "/scene/layers/1"], "{found:?}");
    let others = found.iter().filter(|p| !p.message.contains("never shows"));
    assert!(others.clone().all(|p| p.warning && p.pointer == "/manifest/requires/features"), "{found:?}");
    let bad: Action = serde_json::from_value(json!({"group": {"ids": ["sky", "nope"]}})).unwrap();
    assert!(plan(&doc, &bad).is_err());
    act(&mut doc, json!({"lock": {"id": "sun", "locked": true}}));
    act(&mut doc, json!({"remove_layer": {"id": "sun"}}));
    assert_eq!(doc["scene"]["ext"]["editor"], json!({"locked": ["sky"]}));
    act(&mut doc, json!({"set": {"pointer": "/scene/layers/0/visible", "value": {"prop": "shown"}}}));
    let unhide: Action = serde_json::from_value(json!({"hide": {"id": "text", "hidden": false}})).unwrap();
    assert_eq!(plan(&doc, &unhide).unwrap(), []);
    act(&mut doc, json!({"add_layer": {"kind": "text", "name": "Résumé"}}));
    act(&mut doc, json!({"add_layer": {"kind": "solid", "name": "Crème brûlée"}}));
    act(&mut doc, json!({"add_layer": {"kind": "shape", "name": "形状"}}));
    act(&mut doc, json!({"add_layer": {"kind": "text", "name": "Résumé"}}));
    assert_eq!(ids(&doc, "/scene/layers")[6..], ["resume", "creme_brulee", "shape", "resume_2"]);
    assert_eq!(doc["scene"]["layers"][6]["text"], "Résumé");
    assert!(
        check(&doc).iter().filter(|p| !p.message.contains("never shows")).all(|p| {
            p.pointer == "/scene/layers/0/visible" || p.pointer == "/manifest/requires/features"
        })
    );
}

#[test]
fn keyframes_animate_values() {
    let mut doc = starter("clock", "org.example.test", "Test", "en").unwrap();
    act(&mut doc, json!({"add": {"pointer": "/scene/timelines", "value": [{"id": "intro", "duration": 2}]}}));
    let opacity = "/scene/layers/1/opacity";
    act(&mut doc, json!({"keyframe": {"pointer": opacity, "time": 2, "value": 1}}));
    act(
        &mut doc,
        json!({"keyframe": {"pointer": opacity, "time": 0, "value": 0, "ease": "in_out_sine", "timeline": "intro"}}),
    );
    let value = doc.pointer(opacity).unwrap().clone();
    assert_eq!(
        (&value["keys"], &value["timeline"]),
        (&json!([[0.0, 0, "in_out_sine"], [2.0, 1]]), &json!("intro"))
    );
    assert!((value_at(&value, 1.0).as_f64().unwrap() - 0.5).abs() < 1e-6);
    act(&mut doc, json!({"keyframe": {"pointer": opacity, "time": 1}}));
    let middle = &doc.pointer(opacity).unwrap()["keys"][1];
    assert!(middle[0] == 1.0 && (middle[1].as_f64().unwrap() - 0.5).abs() < 1e-6);
    act(&mut doc, json!({"keyframe": {"pointer": opacity, "time": 1.5, "from": 1}}));
    let moved = &doc.pointer(opacity).unwrap()["keys"][1];
    assert!(moved[0] == 1.5 && (moved[1].as_f64().unwrap() - 0.5).abs() < 1e-6, "{moved}");
    act(&mut doc, json!({"keyframe": {"pointer": opacity, "time": 1, "from": 1.5}}));
    let missing: Action =
        serde_json::from_value(json!({"keyframe": {"pointer": opacity, "time": 3, "from": 7}})).unwrap();
    assert!(plan(&doc, &missing).is_err(), "there is no keyframe at 7 s");
    let onto: Action =
        serde_json::from_value(json!({"keyframe": {"pointer": opacity, "time": 2, "from": 1}})).unwrap();
    let refused = plan(&doc, &onto).unwrap_err();
    assert!(refused.to_string().contains("already has a keyframe at 2 s"), "{refused}");
    assert_eq!(doc.pointer(opacity).unwrap()["keys"][2], json!([2.0, 1]));
    for bad in [json!("half"), json!(2_000_000)] {
        let bad: Action =
            serde_json::from_value(json!({"keyframe": {"pointer": opacity, "time": 1, "value": bad}}))
                .unwrap();
        assert!(
            plan(&doc, &bad).is_err(),
            "a keyframe the scene refuses was planned without a package folder"
        );
    }
    act(&mut doc, json!({"keyframe": {"pointer": opacity, "time": 1, "remove": true}}));
    act(&mut doc, json!({"keyframe": {"pointer": opacity, "time": 0, "remove": true}}));
    act(&mut doc, json!({"keyframe": {"pointer": opacity, "time": 2, "remove": true}}));
    assert_eq!(doc.pointer(opacity), Some(&json!(1)));
    assert_eq!(check(&doc), []);
    let bound: Action =
        serde_json::from_value(json!({"keyframe": {"pointer": "/scene/layers/1/text", "time": 0}})).unwrap();
    assert!(plan(&doc, &bound).is_err(), "a clock text follows another source");
}
