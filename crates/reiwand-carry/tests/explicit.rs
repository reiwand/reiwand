mod common;
use reiwand_carry::{Content, Manifest, PREFILL, schema};
use reiwand_core::explicit::{Rule, all_required, omission, read, round_trip, scan};
use serde_json::Value;
use std::path::Path;

fn content() -> Value {
    serde_json::to_value(common::content()).unwrap()
}

fn manifest() -> Value {
    serde_json::json!({
        "format": 2, "program": "reiwand", "version": "1.0.0", "created": "now",
        "sections": ["lists"], "encrypted": false, "sums": {"files/content.json": "ab"}
    })
}

#[test]
fn every_property_is_required_and_prefilled() {
    let (schema, report) = schema();
    assert!(all_required(&schema).is_empty(), "{:?}", all_required(&schema));
    assert!(report.unknown.is_empty(), "{:?}", report.unknown);
    assert!(report.uncovered.is_empty(), "{:?}", report.uncovered);
    assert!(PREFILL.0.len() > 40);
}

#[test]
fn every_optional_field_is_classed_as_block_or_value() {
    let nulls: Vec<&str> = PREFILL.0.iter().filter(|(_, v)| *v == "null").map(|(r, _)| *r).collect();
    let classed: Vec<&str> = reiwand_carry::OPTIONAL.iter().map(|(r, _)| *r).collect();
    assert_eq!(nulls, classed);
    assert!(reiwand_carry::OPTIONAL.iter().all(|(_, c)| ["block", "value"].contains(c)));
}

#[test]
fn content_is_refused_key_by_key() {
    let load = |v: &Value| read::<Content>("files/content.json", v).map(|_| ());
    load(&content()).unwrap();
    let free = |p: &str| p.starts_with("settings.") || p.contains("].config.");
    let wrong = omission(&content(), free, load);
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn manifest_is_refused_key_by_key() {
    let load = |v: &Value| read::<Manifest>("carry.json", v).map(|_| ());
    load(&manifest()).unwrap();
    let wrong = omission(&manifest(), |p| p.starts_with("sums."), load);
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn values_are_written_as_null_and_unused_blocks_are_left_out() {
    let text = serde_json::to_string_pretty(&common::content()).unwrap();
    assert!(text.contains("\"pin\": null") && text.contains("\"query\": null"));
    assert!(!text.contains("\"secrets\"") && !text.contains("\"rules\""));
}

#[test]
fn the_classes_are_the_ones_the_source_declares() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for (row, class) in reiwand_carry::OPTIONAL {
        let (ty, field) = row.split_once('.').unwrap();
        let want = if *class == "block" { "block" } else { "value" };
        assert_eq!(reiwand_core::explicit::declared(&src, Some(ty), field), Some(want), "{row}");
    }
}

#[test]
fn content_written_again_gives_the_same_bytes() {
    let again = |t: &str| {
        let c: Content = serde_json::from_str(t).map_err(|e| e.to_string())?;
        serde_json::to_string_pretty(&c).map_err(|e| e.to_string())
    };
    let first = again(&serde_json::to_string_pretty(&common::content()).unwrap()).unwrap();
    round_trip(&first, again).unwrap();
}

#[test]
fn the_format_modules_have_no_hidden_values() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let rules = [
        Rule::SerdeDefault,
        Rule::SkipSerializing,
        Rule::UnwrapOr,
        Rule::DeriveDefault,
        Rule::OptionWithoutNull,
    ];
    let runtime = ["choice.rs", "seal.rs", "write.rs"];
    let found: Vec<_> = scan(&[src.as_path()], &rules)
        .into_iter()
        .filter(|v| !runtime.iter().any(|r| v.file.ends_with(r)))
        .map(|v| format!("{}:{} {:?}", v.file.display(), v.line, v.rule))
        .collect();
    assert_eq!(found.len(), 2, "{found:#?}");
    assert!(found.iter().all(|f| f.contains("read.rs")), "{found:#?}");
}
