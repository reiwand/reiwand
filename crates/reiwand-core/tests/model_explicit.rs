use reiwand_core::explicit::{Rule, read, scan};
use reiwand_core::model::{HistoryEntry, MediaStatus, Work};
use serde_json::json;
use std::path::Path;

fn model_files() -> Vec<std::path::PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/model");
    let mut files: Vec<_> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    files.sort();
    files
}

#[test]
fn the_model_has_no_hidden_defaults() {
    let files = model_files();
    let refs: Vec<&Path> = files.iter().map(|p| p.as_path()).collect();
    let rules = [Rule::SerdeDefault, Rule::SkipSerializing, Rule::OptionWithoutNull];
    let found = scan(&refs, &rules);
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn a_missing_field_is_an_error_naming_it() {
    let full = json!({"author": "a", "title": null, "url": null, "license": null});
    assert!(read::<Work>("reply", &full).is_ok());
    for key in ["author", "title", "url", "license"] {
        let mut cut = full.clone();
        cut.as_object_mut().unwrap().remove(key);
        let e = read::<Work>("reply", &cut).unwrap_err();
        assert_eq!(e.path, key);
    }
    assert!(read::<HistoryEntry>("reply", &json!({"at": 1})).is_ok());
    assert!(read::<HistoryEntry>("reply", &json!({"at": 1, "item": null})).is_ok());
    let e = read::<HistoryEntry>("reply", &json!({"item": null})).unwrap_err();
    assert_eq!(e.path, "at");
    let e = read::<MediaStatus>("reply", &json!({})).unwrap_err();
    assert_eq!(e.path, "listening");
}

#[test]
fn every_optional_field_is_classed_as_block_or_value() {
    let table = include_str!("../prefill/model.tsv");
    let rows: Vec<Vec<&str>> = table.lines().map(|l| l.split('\t').collect()).collect();
    assert!(rows.iter().all(|r| r.len() == 3 && ["block", "value"].contains(&r[1])));
    let options: usize = model_files()
        .iter()
        .map(|f| std::fs::read_to_string(f).unwrap())
        .map(|t| {
            t.lines()
                .filter(|l| l.trim().starts_with("pub ") && l.contains(": Option<") && !l.contains("fn "))
                .count()
        })
        .sum();
    assert_eq!(options, rows.len(), "an optional field is missing from prefill/model.tsv");
}
