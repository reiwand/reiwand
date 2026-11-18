use reiwand_core::Edit;
use reiwand_upgrade::edits;
use reiwand_upgrade::sink::Sink;
use serde_json::Value;

const ORIGINAL: &str = include_str!("pairs/edit.original.json");

fn contains(whole: &Value, part: &Value, at: &str) {
    match (whole, part) {
        (Value::Object(w), Value::Object(p)) => {
            for (key, value) in p.iter().filter(|(k, _)| at.is_empty() && *k != "version" || !at.is_empty()) {
                let inner = w.get(key).unwrap_or_else(|| panic!("{at}.{key} is gone"));
                contains(inner, value, &format!("{at}.{key}"));
            }
        }
        (Value::Array(w), Value::Array(p)) => {
            assert_eq!(w.len(), p.len(), "{at}");
            w.iter().zip(p).enumerate().for_each(|(i, (w, p))| contains(w, p, &format!("{at}[{i}]")));
        }
        (Value::Number(w), Value::Number(p)) => {
            assert!((w.as_f64().unwrap() - p.as_f64().unwrap()).abs() < 1e-5, "{at}")
        }
        _ => assert_eq!(whole, part, "{at}"),
    }
}

#[test]
fn the_converted_edit_loads_strictly_and_says_what_it_said() {
    assert!(Edit::parse(ORIGINAL).unwrap_err().to_string().contains("reiwand-upgrade client"));
    let converted = edits::edit(ORIGINAL).unwrap();
    let loaded = Edit::parse(&converted).unwrap();
    let before: Value = serde_json::from_str(ORIGINAL).unwrap();
    contains(&serde_json::to_value(&loaded).unwrap(), &before, "");
    assert!(
        loaded.ops.iter().find(|op| op.id == "ev").is_some_and(|op| op.enabled && op.opacity == Some(0.5))
    );
    assert!(loaded.depth.is_none() && loaded.parallax.is_none() && loaded.fit.is_none());
}

#[test]
fn a_small_edit_with_depth_and_parallax_converts_too() {
    let small = r#"{"version": 1, "depth": {"near": 0.9}, "parallax": {"source": "pointer"},
      "ops": [{"id": "b", "type": "blur", "radius": 3}], "fit": "fit"}"#;
    let loaded = Edit::parse(&edits::edit(small).unwrap()).unwrap();
    assert_eq!(loaded.depth.as_ref().map(|d| (d.near, d.input)), Some((0.9, 518)));
    assert_eq!(loaded.parallax.as_ref().map(|p| p.steps), Some(16));
    assert_eq!(loaded.fit, Some(reiwand_core::Fit::Fit));
}

#[test]
fn converting_twice_changes_nothing() {
    let once = edits::edit(ORIGINAL).unwrap();
    assert_eq!(edits::edit(&once).unwrap(), once);
}

#[test]
fn a_folder_is_converted_with_a_copy_of_each_original() {
    let dir = tempfile::tempdir().unwrap();
    let (edits_dir, copies) = (dir.path().join("edits"), dir.path().join("upgrade-copias"));
    std::fs::create_dir_all(&edits_dir).unwrap();
    std::fs::write(edits_dir.join("a.json"), ORIGINAL).unwrap();
    assert_eq!(edits::folder(&edits_dir, &mut Sink::copying(&copies)).unwrap(), 1);
    assert_eq!(std::fs::read_to_string(copies.join("1-a.json")).unwrap(), ORIGINAL);
    assert!(Edit::parse(&std::fs::read_to_string(edits_dir.join("a.json")).unwrap()).is_ok());
    assert_eq!(edits::folder(&edits_dir, &mut Sink::copying(&copies)).unwrap(), 0);
}
