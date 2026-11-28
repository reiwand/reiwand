use reiwand_core::explicit::{Annotated, Prefill, annotate};
use serde_json::Value;

pub const TABLE: Prefill = Prefill(&[
    ("Manifest.schema", "\"reiwand.provider/1\""),
    ("Manifest.id", "\"\""),
    ("Manifest.version", "\"1.0.0\""),
    ("Manifest.title", "\"\""),
    ("Manifest.license", "\"\""),
    ("Manifest.hosts", "[]"),
    ("Manifest.secret", "null"),
    ("Manifest.settings", "[]"),
    ("Manifest.secret.label", "\"\""),
    ("Manifest.secret.header", "\"Authorization\""),
    ("Manifest.secret.prefix", "\"Bearer \""),
    ("Manifest.secret.hosts", "[]"),
    ("Manifest.secret.required", "true"),
    ("Setting.key", "\"\""),
    ("Setting.label", "\"\""),
    ("Setting.kind", "\"text\""),
    ("Setting.prefill", "\"\""),
    ("Setting.min", "null"),
    ("Setting.max", "null"),
    ("Setting.values", "[]"),
]);

pub const OPTIONAL: &[(&str, &str)] =
    &[("Manifest.secret", "value"), ("Setting.min", "value"), ("Setting.max", "value")];

pub fn schema() -> (Value, Annotated) {
    let mut schema = serde_json::json!(schemars::schema_for!(crate::Manifest));
    let report = annotate(&mut schema, &TABLE);
    (schema, report)
}
