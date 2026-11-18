#![cfg(feature = "schema")]

use reiwand_core::Edit;
use reiwand_core::explicit::{Rule, all_required, block_at, omission, read, round_trip, scan};
use serde_json::{Value, json};
use std::path::Path;

const FULL: &str = include_str!("edit/full.json");

fn sample() -> Value {
    let mut doc: Value = serde_json::from_str(FULL).unwrap();
    doc["depth"] = serde_json::to_value(reiwand_core::edit::Depth::default()).unwrap();
    doc["depth"]["cuts"] = json!([0.2, 0.6]);
    doc["parallax"] = serde_json::to_value(reiwand_core::edit::Parallax::default()).unwrap();
    doc
}

fn load(v: &Value) -> Result<(), reiwand_core::explicit::ReadError> {
    read::<Edit>("edit.json", v).map(|_| ())
}

#[test]
fn the_schema_requires_every_field() {
    let schema = serde_json::to_value(schemars::schema_for!(Edit)).unwrap();
    assert!(all_required(&schema).is_empty(), "{:?}", all_required(&schema));
}

#[test]
fn every_removed_key_is_refused_with_its_path() {
    load(&sample()).unwrap();
    let schema = serde_json::to_value(schemars::schema_for!(Edit)).unwrap();
    let value = sample();
    let wrong = omission(
        &value,
        |p| {
            matches!(p.split('.').collect::<Vec<_>>()[..], ["masks" | "outputs", _])
                || p.contains(".strokes[")
                || block_at(&schema, &schema, &value, p)
        },
        load,
    );
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn the_edit_sources_have_no_hidden_defaults() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/edit");
    let paths = |files: &[&str]| files.iter().map(|f| src.join(f)).collect::<Vec<_>>();
    let (all, some) =
        (paths(&["mod.rs", "masks.rs", "history.rs"]), paths(&["depth.rs", "touch.rs", "regions.rs"]));
    fn refs(list: &[std::path::PathBuf]) -> Vec<&Path> {
        list.iter().map(|p| p.as_path()).collect()
    }
    let found = scan(&refs(&all), &[Rule::SerdeDefault, Rule::SkipSerializing, Rule::OptionWithoutNull]);
    assert!(found.is_empty(), "{found:?}");
    let found = scan(&refs(&some), &[Rule::SerdeDefault, Rule::SkipSerializing]);
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn writing_a_file_again_gives_the_same_bytes() {
    let text = Edit::from_value(sample()).unwrap().to_json();
    let again = |t: &str| Edit::parse(t).map(|e| e.to_json()).map_err(|e| e.to_string());
    round_trip(&text, again).unwrap();
}

#[test]
fn an_old_edit_is_refused_naming_the_command() {
    for old in [1, 2] {
        let mut doc = sample();
        doc["version"] = json!(old);
        let said = Edit::from_value(doc).unwrap_err().to_string();
        assert!(said.contains("reiwand-upgrade client"), "{said}");
    }
}

#[test]
fn a_missing_field_is_named() {
    let mut doc = sample();
    doc["ops"][0].as_object_mut().unwrap().remove("tint");
    let said = Edit::from_value(doc).unwrap_err().to_string();
    assert!(said.contains("ops[0]") && said.contains("tint"), "{said}");
}

#[test]
fn every_optional_field_is_classed_as_block_or_value() {
    let table = include_str!("../prefill/edit.tsv");
    let rows: Vec<Vec<&str>> = table.lines().map(|l| l.split('\t').collect()).collect();
    assert!(rows.iter().all(|r| r.len() == 3 && ["bloque", "valor"].contains(&r[1])));
    let count = |class: &str| rows.iter().filter(|r| r[1] == class).count();
    assert_eq!((count("bloque"), count("valor")), (6, 4));
    let options: usize = ["mod.rs", "depth.rs"]
        .iter()
        .map(|f| {
            std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/edit").join(f)).unwrap()
        })
        .map(|t| t.lines().filter(|l| l.trim().starts_with("pub ") && l.contains(": Option<")).count())
        .sum();
    assert_eq!(options, rows.len(), "an optional field is missing from prefill/edit.tsv");
}
