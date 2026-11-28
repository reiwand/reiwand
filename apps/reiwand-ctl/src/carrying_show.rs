use crate::show::list;
use crate::told::{fill, told, tr};
use anyhow::Result;
use serde_json::Value;

pub(crate) fn show_preview(found: &Value, json: bool) -> Result<()> {
    if json {
        println!("{found}");
        return Ok(());
    }
    let file = &found["file"];
    let sections = list(file["sections"].as_array().into_iter().flatten().filter_map(Value::as_str));
    println!(
        "{}",
        fill(
            told("File from %1 %2, holds: %3"),
            &[
                &file["program"].as_str().unwrap_or_default(),
                &file["version"].as_str().unwrap_or_default(),
                &sections
            ]
        )
    );
    for row in found["rows"].as_array().into_iter().flatten() {
        let n = |key: &str| row[key].as_u64().unwrap_or_default();
        let (name, held, new, skipped, conflicts) =
            (row["section"].as_str().unwrap_or_default(), n("held"), n("new"), n("skipped"), n("conflicts"));
        println!(
            "{}",
            fill(
                told("%1: %2 in the file, %3 new, %4 skipped, %5 with a name you already have"),
                &[&name, &held, &new, &skipped, &conflicts]
            )
        );
    }
    for note in found["notes"].as_array().into_iter().flatten().filter_map(Value::as_str) {
        println!("{}", tr(note));
    }
    Ok(())
}

pub(crate) fn show_report(output: &Value) -> Result<()> {
    for (section, count) in output["added"].as_object().into_iter().flatten() {
        println!("{}", fill(told("added %1: %2"), &[&section, &count]));
    }
    let unresolved = output["unresolved"].as_u64().unwrap_or_default();
    if unresolved > 0 {
        println!(
            "{}",
            fill(told("%1 entries refer to wallpapers that are not here and were left out"), &[&unresolved])
        );
    }
    for pending in output["pending"].as_array().into_iter().flatten() {
        let field = |key: &str| pending[key].as_str().unwrap_or_default().to_owned();
        println!(
            "{}",
            fill(told("pending, %1 %2: %3"), &[&field("section"), &field("name"), &field("reason")])
        );
    }
    for note in output["notes"].as_array().into_iter().flatten().filter_map(Value::as_str) {
        println!("{}", tr(note));
    }
    Ok(())
}
