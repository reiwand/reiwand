use super::{Format, Report, complete};
use serde_json::Value;
use toml_edit::{Array, DocumentMut, InlineTable, Item, Table, value};

pub fn complete_toml(format: Format, text: &str) -> anyhow::Result<(String, Report)> {
    let mut doc: DocumentMut = text.parse()?;
    let before: Value = serde_json::to_value(toml::from_str::<toml::Value>(text)?)?;
    let mut after = before.clone();
    let report = complete(format, &mut after);
    apply(doc.as_table_mut(), &before, &after);
    Ok((doc.to_string(), report))
}

fn apply(table: &mut Table, before: &Value, after: &Value) {
    let (Some(old), Some(new)) = (before.as_object(), after.as_object()) else { return };
    for (key, now) in new {
        match old.get(key) {
            _ if now.is_null() => {}
            None => {
                if let Some(item) = item(now) {
                    table.insert(key, item);
                }
            }
            Some(was) => {
                if let Some(Item::Table(inner)) = table.get_mut(key) {
                    apply(inner, was, now);
                }
            }
        }
    }
}

fn item(v: &Value) -> Option<Item> {
    match v {
        Value::Object(map) => {
            let mut table = Table::new();
            for (k, inner) in map {
                if let Some(i) = item(inner) {
                    table.insert(k, i);
                }
            }
            Some(Item::Table(table))
        }
        other => scalar(other).map(Item::Value),
    }
}

fn scalar(v: &Value) -> Option<toml_edit::Value> {
    Some(match v {
        Value::Null => return None,
        Value::Bool(b) => value(*b).into_value().ok()?,
        Value::String(s) => value(s.as_str()).into_value().ok()?,
        Value::Number(n) => match n.as_i64() {
            Some(i) => value(i).into_value().ok()?,
            None => value(n.as_f64()?).into_value().ok()?,
        },
        Value::Array(list) => toml_edit::Value::Array(list.iter().filter_map(scalar).collect::<Array>()),
        Value::Object(map) => {
            let mut inline = InlineTable::new();
            for (k, inner) in map {
                if let Some(i) = scalar(inner) {
                    inline.insert(k, i);
                }
            }
            toml_edit::Value::InlineTable(inline)
        }
    })
}
