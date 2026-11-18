use reiwand_core::explicit::{Annotated, Prefill, annotate};
use serde_json::Value;

const ITEM: &str = r#"{"repo_id":null,"id":""}"#;

pub const TABLE: Prefill = Prefill(&[
    ("Manifest.format", "2"),
    ("Manifest.program", "\"\""),
    ("Manifest.version", "\"\""),
    ("Manifest.created", "\"\""),
    ("Manifest.sections", "[]"),
    ("Manifest.encrypted", "false"),
    ("Manifest.sums", "{}"),
    ("Item.repo_id", "null"),
    ("Item.id", "\"\""),
    ("Repository.address", "\"\""),
    ("Repository.repo_id", "\"\""),
    ("Repository.origin", "\"\""),
    ("Repository.name", "\"\""),
    ("Repository.pin", "null"),
    ("Provider.kind", "\"\""),
    ("Provider.name", "\"\""),
    ("Provider.config", "{}"),
    ("List.name", "\"\""),
    ("List.items", "[]"),
    ("List.query", "null"),
    ("List.rules", "null"),
    ("Rating.item", ITEM),
    ("Rating.rating", "0"),
    ("Tagging.item", ITEM),
    ("Tagging.tags", "[]"),
    ("Installed.repo_id", "\"\""),
    ("Installed.id", "\"\""),
    ("Installed.version", "\"\""),
    ("OwnFile.id", "\"\""),
    ("OwnFile.name", "\"\""),
    ("Session.repo_id", "\"\""),
    ("Session.origin", "\"\""),
    ("Session.account", "\"\""),
    ("Session.token", "\"\""),
    ("Content.secrets.sessions", "[]"),
    ("Content.secrets.provider_keys", "{}"),
    ("Content.repositories", "[]"),
    ("Content.providers", "[]"),
    ("Content.lists", "[]"),
    ("Content.favourites", "[]"),
    ("Content.ratings", "[]"),
    ("Content.tags", "[]"),
    ("Content.installed", "[]"),
    ("Content.settings", "{}"),
    ("Content.own", "[]"),
    ("Content.secrets", "null"),
]);

pub const OPTIONAL: &[(&str, &str)] = &[
    ("Item.repo_id", "valor"),
    ("Repository.pin", "valor"),
    ("List.query", "valor"),
    ("List.rules", "bloque"),
    ("Content.secrets", "bloque"),
];

pub fn schema() -> (Value, Annotated) {
    let mut schema = serde_json::json!(schemars::schema_for!(crate::Content));
    let manifest = serde_json::json!(schemars::schema_for!(crate::Manifest));
    if let Some(defs) = schema["$defs"].as_object_mut() {
        defs.insert("Manifest".to_owned(), manifest);
    }
    let report = annotate(&mut schema, &TABLE);
    (schema, report)
}
