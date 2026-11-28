#![cfg(feature = "schema")]

#[path = "explicit/sample.rs"]
mod sample;

use reiwand_core::explicit::{
    Prefill, Rule, all_required, annotate, block_at, null_blocks, omission, prune_blocks, read, read_str,
    round_trip, scan,
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
fn error_text_names_the_path_and_asks_to_write_it() {
    let mut v = full();
    v.as_object_mut().unwrap().remove("parent");
    let text = load(&v).unwrap_err().to_string();
    assert!(text.starts_with("scene.json: \"parent\" is missing."), "{text}");
    assert!(text.ends_with("Reiwand does not fill in values: write it."), "{text}");
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

#[test]
fn bindings_inside_tuples_are_completed() {
    let schema = serde_json::json!({
        "$defs": { "Bind": { "type": "object", "required": ["keys", "prop"],
            "properties": { "keys": { "type": "array", "items": { "type": "number" } },
                            "prop": { "type": ["string", "null"], "x-null": true } } } },
        "type": "array", "minItems": 2, "maxItems": 2,
        "prefixItems": [{ "type": "number" }, { "$ref": "#/$defs/Bind" }]
    });
    let mut value = serde_json::json!([1.0, { "keys": [] }]);
    reiwand_core::explicit::walk::fill(&schema, &schema, &mut value);
    assert_eq!(value, serde_json::json!([1.0, { "keys": [], "prop": null }]));
}

fn schema() -> Value {
    serde_json::to_value(schemars::schema_for!(Sample)).unwrap()
}

fn badged() -> Value {
    let mut v = full();
    v["badge"] = serde_json::json!({ "label": "new", "size": 2.0 });
    v
}

#[test]
fn a_block_may_be_absent_or_null_and_means_the_same() {
    let absent = full();
    let mut nulled = full();
    nulled["badge"] = Value::Null;
    let a = read::<Sample>("x", &absent).unwrap();
    let b = read::<Sample>("x", &nulled).unwrap();
    assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
    assert!(a.badge.is_none() && b.badge.is_none());
    assert!(!serde_json::to_string(&a).unwrap().contains("badge"));
}

#[test]
fn removing_a_whole_block_is_allowed_but_not_a_field_inside_it() {
    let schema = schema();
    let value = badged();
    read::<Sample>("x", &value).unwrap();
    let wrong = omission(&value, |p| block_at(&schema, &schema, &value, p), load);
    assert!(wrong.is_empty(), "{wrong:#?}");
    let loose = omission(&value, |_| false, load);
    assert_eq!(loose, ["badge: accepted without it"]);
    let mut cut = value.clone();
    cut["badge"].as_object_mut().unwrap().remove("size");
    assert_eq!(load(&cut).unwrap_err().path, "badge.size");
}

#[test]
fn the_schema_lets_blocks_be_absent_and_the_writer_leaves_no_null_block() {
    let mut schema = schema();
    assert!(all_required(&schema).is_empty(), "{:?}", all_required(&schema));
    annotate(&mut schema, &Prefill(&[("Sample.badge", "null")]));
    assert!(schema["required"].as_array().unwrap().iter().all(|k| k != "badge"));
    let mut nulled = full();
    nulled["badge"] = Value::Null;
    assert_eq!(null_blocks(&schema, &schema, &nulled), ["badge"]);
    prune_blocks(&schema, &schema, &mut nulled);
    assert_eq!(nulled, full());
    let written = serde_json::to_string_pretty(&read::<Sample>("x", &badged()).unwrap()).unwrap();
    assert!(null_blocks(&schema, &schema, &serde_json::from_str(&written).unwrap()).is_empty());
}
