use rusqlite::{Row, types::Type};
use serde_json::Value;

pub const OUTPUT: [&str; 7] = ["item", "path", "playlist", "fit", "background", "preset", "filter"];
pub const AUTO: [&str; 2] = ["outputs", "on_battery"];
pub const PLACING: [&str; 2] = ["fit", "background"];
pub const SETTINGS: [&str; 2] = ["package", "values"];

fn refuse(index: usize, why: String) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(index, Type::Text, why.into())
}

fn needs(value: &Value, keys: &[&str], what: &str, index: usize) -> rusqlite::Result<()> {
    let object = value.as_object().ok_or_else(|| refuse(index, format!("{what} is not an object")))?;
    match keys.iter().find(|key| !object.contains_key(**key)) {
        Some(key) => Err(refuse(
            index,
            format!(
                "{what}: \"{key}\" is missing; run `reiwand-upgrade client` once to convert the database"
            ),
        )),
        None => Ok(()),
    }
}

fn parsed(row: &Row<'_>, index: usize) -> rusqlite::Result<Value> {
    let text: String = row.get(index)?;
    serde_json::from_str(&text).map_err(|e| refuse(index, e.to_string()))
}

pub(crate) fn outputs(row: &Row<'_>, index: usize) -> rusqlite::Result<Value> {
    let value = parsed(row, index)?;
    let map = value.as_object().ok_or_else(|| refuse(index, "profile outputs is not an object".into()))?;
    for (name, one) in map {
        needs(one, &OUTPUT, &format!("profile outputs.{name}"), index)?;
    }
    Ok(value)
}

pub(crate) fn auto(row: &Row<'_>, index: usize) -> rusqlite::Result<Value> {
    let value = parsed(row, index)?;
    needs(&value, &AUTO, "profile auto", index)?;
    Ok(value)
}

pub(crate) fn options(row: &Row<'_>, index: usize) -> rusqlite::Result<Value> {
    let value = parsed(row, index)?;
    needs(&value, &[], "profile options", index)?;
    Ok(value)
}

pub(crate) fn preset(row: &Row<'_>, index: usize) -> rusqlite::Result<Value> {
    let value = parsed(row, index)?;
    let keys = if value.get("package").is_some() { &SETTINGS } else { &PLACING };
    needs(&value, keys, "preset", index)?;
    Ok(value)
}
