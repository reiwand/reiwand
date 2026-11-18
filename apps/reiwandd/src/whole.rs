use serde_json::{Value, json};

pub fn scene(text: &str) -> String {
    reiwand_upgrade::fill::whole(&text.replace("reiwand.scene/1", reiwand_scene::SCHEMA))
}

pub fn descriptor(mut descriptor: Value) -> String {
    reiwand_upgrade::descriptors::complete(&mut descriptor);
    descriptor.to_string()
}

pub fn properties(properties: Value) -> Value {
    let mut doc = json!({ "schema": "reiwand.wpak/2", "id": "org.example.props", "version": "1.0.0",
        "title": "Props", "license": "MIT", "kind": "scene", "entry": "scene.json", "properties": properties });
    reiwand_upgrade::fill::complete(reiwand_upgrade::fill::Format::Manifest, &mut doc);
    doc["properties"].take()
}
