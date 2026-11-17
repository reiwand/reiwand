use serde_json::Value as Json;
use std::sync::LazyLock;

const TABLE: &str = include_str!("prefill.json");

static ROWS: LazyLock<Vec<(String, String)>> = LazyLock::new(|| {
    let table: serde_json::Map<String, Json> = serde_json::from_str(TABLE).unwrap_or_default();
    table.into_iter().map(|(row, value)| (row, value.to_string())).collect()
});

pub fn prefill(raw: &mut Json) -> reiwand_core::explicit::Annotated {
    let rows: Vec<(&str, &str)> = ROWS.iter().map(|(row, value)| (row.as_str(), value.as_str())).collect();
    reiwand_core::explicit::annotate_rows(raw, &rows)
}
