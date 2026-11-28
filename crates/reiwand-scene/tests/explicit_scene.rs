use reiwand_core::explicit::{all_required, block_at, null_blocks, omission, prune_blocks, read, round_trip};
use reiwand_scene::{Scene, schema};
use serde_json::Value;

fn sample() -> Value {
    let text = reiwand_fill::whole(include_str!("newer.json"));
    serde_json::from_str(&text).unwrap()
}

#[test]
fn every_scene_property_is_required() {
    let loose = all_required(&schema());
    assert!(loose.is_empty(), "{loose:#?}");
}

#[test]
fn a_full_scene_is_refused_key_by_key_when_it_loses_one() {
    let load = |v: &Value| read::<Scene>("scene.json", v).map(|_| ());
    load(&sample()).unwrap();
    let (root, target) = (schema(), schema());
    let value = sample();
    let wrong = omission(&value, |path| block_at(&root, &target, &value, path), load);
    let untagged = |line: &&String| {
        let (path, rest) = line.split_once(": ").unwrap();
        let reported = rest.strip_prefix("reported \"").and_then(|r| r.split('"').next());
        let entry = ["assets.", "effects.", "templates.", "particles.", "pools.", "ext."]
            .iter()
            .any(|m| path.strip_prefix(m).is_some_and(|r| !r.contains(['.', '['])));
        let flat = rest.ends_with("(Missing)")
            && reported.is_some_and(|r| r.rsplit('.').next() == path.rsplit('.').next());
        let held = path.rsplit('.').nth(1).is_some_and(|m| ["text", "params", "strings", "map"].contains(&m));
        flat || held && rest == "accepted without it"
            || entry && rest == "accepted without it"
            || reported.is_some_and(|r| path.starts_with(r) && path.len() > r.len())
    };
    let wrong: Vec<&String> = wrong.iter().filter(|l| !untagged(l)).collect();
    assert!(wrong.is_empty(), "{:#?}", &wrong[..wrong.len().min(40)]);
}

#[test]
fn a_full_scene_written_again_gives_the_same_bytes() {
    let text = serde_json::to_string_pretty(&sample()).unwrap();
    let again = |t: &str| {
        let s: Scene = serde_json::from_str(t).map_err(|e| e.to_string())?;
        serde_json::to_string_pretty(&s).map_err(|e| e.to_string())
    };
    let canonical = again(&text).unwrap();
    round_trip(&canonical, again).unwrap();
    let written: Value = serde_json::from_str(&canonical).unwrap();
    assert!(null_blocks(&schema(), &schema(), &written).is_empty());
}

#[test]
fn a_scene_with_its_unused_blocks_written_as_null_reads_like_one_without_them() {
    let mut nulled = sample();
    for layer in nulled["layers"].as_array_mut().unwrap().iter_mut().filter_map(Value::as_object_mut) {
        for block in ["anchor", "attach", "brightness", "inherit", "lighting", "mask", "parent"] {
            layer.entry(block).or_insert(Value::Null);
        }
    }
    let root = schema();
    assert!(!null_blocks(&root, &root, &nulled).is_empty());
    let mut bare = nulled.clone();
    prune_blocks(&root, &root, &mut bare);
    assert!(null_blocks(&root, &root, &bare).is_empty());
    assert!(bare.to_string().len() < nulled.to_string().len());
    let (a, b) = (read::<Scene>("a", &nulled).unwrap(), read::<Scene>("b", &bare).unwrap());
    assert_eq!(a, b);
    assert_eq!(a.to_json().unwrap(), b.to_json().unwrap());
}

#[test]
fn the_frozen_schema_takes_the_words_of_the_fields_that_follow_their_context_and_no_null() {
    let frozen: Value = serde_json::from_str(include_str!("../../reiwand-fill/frozen/scene.json")).unwrap();
    let live = schema();
    let word = |node: &Value| {
        let name = node["$ref"].as_str()?.rsplit('/').next()?;
        live["$defs"][name]["x-follows"].as_str().map(str::to_owned)
    };
    let mut fields = Vec::new();
    for (name, def) in live["$defs"].as_object().unwrap() {
        let variants = def["oneOf"].as_array().into_iter().flatten().chain([def]);
        for (i, variant) in variants.enumerate() {
            for (field, node) in variant["properties"].as_object().into_iter().flatten() {
                let Some(word) = word(node) else { continue };
                let tag = &variant["properties"]["type"]["const"];
                let found = match tag.as_str() {
                    Some(kind) => frozen["$defs"][name]["oneOf"]
                        .as_array()
                        .and_then(|v| v.iter().find(|v| v["properties"]["type"]["const"] == kind))
                        .map(|v| &v["properties"][field]),
                    None if i == 0 && def["oneOf"].is_array() => continue,
                    None => Some(&frozen["$defs"][name]["properties"][field]),
                };
                let alts = found.and_then(|f| f["anyOf"].as_array()).cloned().unwrap_or_default();
                let takes = alts.iter().any(|a| a["enum"] == serde_json::json!([word]));
                let null = alts.iter().any(|a| a["type"] == "null");
                assert!(takes && !null, "{name}.{field} ({tag}) in the frozen schema: {found:?}");
                fields.push(format!("{name}.{field}"));
            }
        }
    }
    assert!(fields.contains(&"Camera.center".to_owned()), "{fields:?}");
}
