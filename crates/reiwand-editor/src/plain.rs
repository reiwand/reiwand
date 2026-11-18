use serde_json::Value;

pub fn scene(scene: &Value) -> Value {
    let mut plain = scene.clone();
    let root = &*crate::schema::SCENE;
    reiwand_core::explicit::prune_blocks(root, root, &mut plain);
    plain
}

pub fn doc(doc: &Value) -> Value {
    let mut plain = doc.clone();
    if let Some(inner) = doc.get("scene") {
        plain["scene"] = scene(inner);
    }
    plain
}
