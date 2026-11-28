use serde_json::{Value, json};

pub fn scene(text: &str) -> String {
    reiwand_fill::whole(text)
}

pub fn descriptor(mut descriptor: Value) -> String {
    reiwand_fill::descriptors::complete(&mut descriptor);
    descriptor.to_string()
}

pub fn properties(properties: Value) -> Value {
    let mut doc = json!({ "schema": "reiwand.wpak/3", "id": "org.example.props", "version": "1.0.0",
        "title": "Props", "license": "MIT", "kind": "scene", "entry": "scene.json", "properties": properties });
    reiwand_fill::manifests::complete(&mut doc);
    doc["properties"].take()
}
