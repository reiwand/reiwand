pub use reiwand_core::explicit::walk::{branches, child, choose, deref, fill, fits, properties, sample};
use serde_json::Value;
use std::sync::LazyLock;

pub static SCENE: LazyLock<Value> = LazyLock::new(reiwand_scene::schema);
pub static PROPERTY: LazyLock<Value> = LazyLock::new(reiwand_scene::property_schema);
pub static MANIFEST: LazyLock<Value> = LazyLock::new(reiwand_format::manifest_schema);
static NULL: Value = Value::Null;

pub fn root_of(pointer: &str) -> Option<(&'static Value, Vec<String>)> {
    let tokens: Vec<String> =
        pointer.split('/').skip(1).map(|t| t.replace("~1", "/").replace("~0", "~")).collect();
    match tokens.first().map(String::as_str) {
        Some("manifest") if tokens.get(1).is_some_and(|t| t == "properties") && tokens.len() > 2 => {
            Some((&PROPERTY, tokens[3..].to_vec()))
        }
        Some("manifest") => Some((&MANIFEST, tokens.into_iter().skip(1).collect())),
        Some("scene") => Some((&SCENE, tokens.into_iter().skip(1).collect())),
        _ => None,
    }
}

pub fn at(doc: &Value, pointer: &str) -> Option<Value> {
    let (root, tokens) = root_of(pointer)?;
    let base = pointer.len() - tokens.iter().map(|t| t.len() + 1).sum::<usize>();
    let mut value = doc.pointer(&pointer[..base]).unwrap_or(&NULL);
    let mut schema = root.clone();
    for token in &tokens {
        schema = child(root, &schema, token, value)?;
        value = value.get(token).or_else(|| value.get(token.parse::<usize>().ok()?)).unwrap_or(&NULL);
    }
    Some(schema)
}
