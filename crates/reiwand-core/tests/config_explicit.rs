#![cfg(feature = "schema")]

use reiwand_core::explicit::{Rule, all_required, omission, read, read_str, round_trip, scan};
use reiwand_core::{Config, ConfigError};
use std::path::Path;

fn template() -> serde_json::Value {
    serde_json::to_value(toml::from_str::<toml::Value>(&Config::template()).unwrap()).unwrap()
}

fn load(v: &serde_json::Value) -> Result<(), reiwand_core::explicit::ReadError> {
    read::<Config>("config.toml", v).map(|_| ())
}

#[test]
fn first_start_writes_the_whole_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    Config::first_start(&path).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert_eq!(text, Config::template());
    assert!(text.starts_with("format = 3"));
    assert_eq!(Config::load(&path).unwrap(), Config::default());
}

#[test]
fn an_old_file_is_refused_naming_the_command_and_is_not_touched() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    for old in ["[default]\nimage = \"a.png\"\n", "format = 2\n"] {
        std::fs::write(&path, old).unwrap();
        let Err(ConfigError::Format { fix, .. }) = Config::load(&path) else { panic!("{old}") };
        assert!(fix.contains("reiwand-upgrade client"), "{fix}");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), old);
    }
    std::fs::write(&path, "format = 99\n").unwrap();
    let Err(ConfigError::Format { fix, .. }) = Config::load(&path) else { panic!() };
    assert!(fix.contains("update Reiwand"));
}

#[test]
fn the_schema_requires_every_field() {
    let schema = serde_json::to_value(schemars::schema_for!(Config)).unwrap();
    assert!(all_required(&schema).is_empty(), "{:?}", all_required(&schema));
}

#[test]
fn every_removed_key_is_refused_with_its_path() {
    let mut value = template();
    value["outputs"] = serde_json::json!({ "DP-1": value["default"].clone() });
    value["providers"] = serde_json::json!({ "daily": { "kind": "x" } });
    load(&value).unwrap();
    let wrong = omission(&value, |p| p.starts_with("providers") || p == "outputs.DP-1", load);
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn writing_the_template_again_gives_the_same_bytes() {
    let again = |t: &str| {
        let c: Config = read_str(
            "config.toml",
            &serde_json::to_string(&toml::from_str::<toml::Value>(t).unwrap()).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        toml::to_string_pretty(&c).map_err(|e| e.to_string())
    };
    round_trip(&Config::template(), again).unwrap();
}

#[test]
fn the_settings_sources_have_no_hidden_defaults() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let files = [
        "outputs.rs",
        "blank.rs",
        "settings.rs",
        "services.rs",
        "net.rs",
        "rgb.rs",
        "shortcuts.rs",
        "layout.rs",
        "gpu.rs",
        "filter.rs",
    ];
    let paths: Vec<_> = files.iter().map(|f| src.join(f)).collect();
    let refs: Vec<&Path> = paths.iter().map(|p| p.as_path()).collect();
    let rules = [Rule::SerdeDefault, Rule::SkipSerializing, Rule::OptionWithoutNull];
    let found = scan(&refs, &rules);
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn every_optional_field_is_classed_as_block_or_value() {
    let table = include_str!("../prefill/config.tsv");
    let rows: Vec<Vec<&str>> = table.lines().map(|l| l.split('\t').collect()).collect();
    assert!(rows.iter().all(|r| r.len() == 3 && ["bloque", "valor"].contains(&r[1])));
    let count = |class: &str| rows.iter().filter(|r| r[1] == class).count();
    assert_eq!((count("bloque"), count("valor")), (2, 14));
    let sources = ["config.rs", "outputs.rs", "blank.rs", "layer.rs"];
    let options: usize = sources
        .iter()
        .map(|f| std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join(f)).unwrap())
        .map(|t| t.lines().filter(|l| l.trim().starts_with("pub ") && l.contains(": Option<")).count())
        .sum();
    assert_eq!(options, rows.len() + 5, "an optional field is missing from prefill/config.tsv");
}
