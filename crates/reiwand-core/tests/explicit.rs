#![cfg(feature = "schema")]

#[path = "explicit/sample.rs"]
mod sample;

use reiwand_core::explicit::{
    Prefill, Rule, all_required, annotate, omission, read, read_str, round_trip, scan,
};
use sample::{FULL, Sample};
use serde_json::Value;
use std::path::Path;

fn full() -> Value {
    serde_json::from_str(FULL).unwrap()
}

fn load(v: &Value) -> Result<(), reiwand_core::explicit::ReadError> {
    read::<Sample>("scene.json", v).map(|_| ())
}

#[test]
fn full_sample_reads() {
    load(&full()).unwrap();
}

#[test]
fn every_removed_key_is_refused_with_its_path() {
    let wrong = omission(&full(), |_| false, load);
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn paths_name_flatten_array_and_untagged_fields() {
    let mut v = full();
    v["layers"][1]["effects"][0]["params"].as_object_mut().unwrap().remove("reach");
    assert_eq!(load(&v).unwrap_err().path, "layers[1].effects[0].params.reach");
    let mut v = full();
    v.as_object_mut().unwrap().remove("tone");
    assert_eq!(load(&v).unwrap_err().path, "tone");
    let mut v = full();
    v["shape"]["round"].as_object_mut().unwrap().remove("corner");
    assert_eq!(load(&v).unwrap_err().path, "shape.round.corner");
}

#[test]
fn error_text_names_the_path_and_the_way_out() {
    let mut v = full();
    v.as_object_mut().unwrap().remove("parent");
    let text = load(&v).unwrap_err().to_string();
    assert!(text.starts_with("scene.json: \"parent\" is missing."), "{text}");
    assert!(text.contains("reiwand-upgrade"), "{text}");
}

#[test]
fn option_must_be_written_as_null() {
    let mut v = full();
    v["parent"] = Value::String("x".into());
    load(&v).unwrap();
    v.as_object_mut().unwrap().remove("parent");
    assert!(load(&v).is_err());
}

#[test]
fn a_wrong_type_is_named_too() {
    let mut v = full();
    v["name"] = Value::from(3);
    let e = load(&v).unwrap_err();
    assert_eq!(
        (e.path.as_str(), matches!(e.fault, reiwand_core::explicit::Fault::Invalid(_))),
        ("name", true)
    );
}

#[test]
fn schema_requires_everything_and_carries_prefill_as_annotation() {
    let mut schema = serde_json::to_value(schemars::schema_for!(Sample)).unwrap();
    assert!(all_required(&schema).is_empty(), "{:?}", all_required(&schema));
    let table = Prefill(&[
        ("Layer.id", "\"layer\""),
        ("Layer.opacity", "1.0"),
        ("Layer.effects", "[]"),
        ("Params.reach", "512.0"),
        ("Params.mask", "null"),
        ("Nowhere.field", "1"),
    ]);
    let found = annotate(&mut schema, &table);
    assert_eq!(found.unknown, ["Nowhere.field"]);
    assert!(found.uncovered.contains(&"Sample.name".to_owned()));
    assert!(all_required(&schema).is_empty(), "{:?}", all_required(&schema));
    assert_eq!(schema["$defs"]["Params"]["properties"]["reach"]["default"], 512.0);
    assert!(read::<Sample>("x", &full()).is_ok(), "the loader never reads the annotation");
}

#[test]
fn round_trip_gives_the_same_bytes() {
    let again = |t: &str| {
        let s: Sample = read_str("x", t).map_err(|e| e.to_string())?;
        serde_json::to_string_pretty(&s).map_err(|e| e.to_string())
    };
    round_trip(FULL.trim_end(), again).unwrap();
}

#[test]
fn source_scan_flags_what_the_rules_forbid() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/explicit/bad");
    let rules = [
        Rule::SerdeDefault,
        Rule::SkipSerializing,
        Rule::UnwrapOr,
        Rule::DeriveDefault,
        Rule::OptionWithoutNull,
    ];
    let found: Vec<Rule> = scan(&[&dir], &rules).into_iter().map(|v| v.rule).collect();
    for rule in rules {
        assert!(found.contains(&rule), "{rule:?}");
    }
    let sample = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/explicit");
    let own = scan(&[&sample.join("sample.rs")], &rules);
    assert!(own.is_empty(), "{own:?}");
}
