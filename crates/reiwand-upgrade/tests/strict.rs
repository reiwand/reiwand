use serde_json::json;

#[test]
fn context_fields_become_words_and_masks_and_shakes_gain_their_values() {
    let scene = json!({
        "schema": "reiwand.scene/2", "canvas": { "size": [100, 50], "camera": { "shake": {} } },
        "layers": [
            { "id": "v", "type": "video", "video": "a.webm", "transform": {},
              "mask": { "texture": "m.png" } },
            { "id": "s", "type": "solid", "color": "#112233", "mask": { "shape": { "ellipse": { "size": [10, 10] } } } },
            { "id": "g", "type": "group", "layers": [] }
        ],
        "assets": { "a.webm": { "path": "a.webm" }, "m.png": { "path": "m.png" } }
    });
    let text = reiwand_upgrade::fill::whole(&scene.to_string());
    let doc: serde_json::Value = serde_json::from_str(&text).unwrap();
    let layers = &doc["layers"];
    assert_eq!(layers[0]["mask"]["channel"], "luminance");
    assert_eq!(layers[1]["mask"]["channel"], "a");
    assert_eq!(layers[1]["mask"]["strength"], 1);
    assert_eq!(layers[0]["size"], "source");
    assert_eq!(layers[1]["size"], "canvas");
    assert_eq!(layers[0]["transform"]["origin"], "anchor");
    assert_eq!(layers[0]["playback"]["end"], "end");
    assert_eq!(doc["canvas"]["camera"]["shake"]["enabled"], true);
    assert_eq!(doc["canvas"]["camera"]["shake"]["roughness"], 0.5);
    assert_eq!(doc["canvas"]["camera"]["center"], "canvas");
    reiwand_scene::Scene::from_json(&text).unwrap();
}
