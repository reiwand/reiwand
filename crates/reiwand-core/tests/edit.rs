use reiwand_core::edit::{self, Blend, Edit, Kind};
use serde_json::json;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const FULL: &str = include_str!("edit/full.json");

#[test]
fn the_documented_example_round_trips() {
    let edit = Edit::parse(FULL).unwrap();
    assert_eq!(edit.ops.len(), 8);
    assert!(!edit.ops[7].enabled && edit.ops[0].enabled);
    assert_eq!(edit.ops[6].kind, Kind::Overlay { color: "#1a2b6d".into(), blend: Blend::SoftLight });
    assert_eq!(edit.crop_for("HDMI-A-1", Some("Maker Model 1234")), Some([0.0, 0.1, 0.5, 0.5625]));
    assert_eq!(edit.crop_for("DP-1", None), Some([0.0, 0.1, 0.5, 0.5625]));
    assert_eq!(edit.crop_for("DP-2", Some("Other")), None);
    assert_eq!(Edit::parse(&edit.to_json()).unwrap(), edit);
    assert!(edit.develops() && !Edit::default().develops());
}

#[test]
fn invalid_files_name_the_field() {
    let broken = |path: &str, value: serde_json::Value| {
        let mut doc: serde_json::Value = serde_json::from_str(FULL).unwrap();
        let mut at = &mut doc;
        let parts: Vec<&str> = path.split('/').collect();
        for part in &parts[..parts.len() - 1] {
            at = match part.parse::<usize>() {
                Ok(i) => &mut at[i],
                Err(_) => &mut at[*part],
            };
        }
        at[parts[parts.len() - 1]] = value;
        Edit::from_value(doc).unwrap_err().to_string()
    };
    assert_eq!(broken("ops/1/ev", json!(7)), "ops[1].ev: 7 is out of range -5 to 5");
    assert_eq!(
        broken("ops/0/temperature", json!(900)),
        "ops[0].temperature: 900 is out of range 2000 to 12000"
    );
    assert_eq!(broken("ops/2/sharpness", json!(1)), "ops[2].sharpness: unknown field");
    assert_eq!(broken("geometry/rotate", json!(45)), "geometry.rotate: use 0, 90, 180 or 270");
    assert_eq!(broken("ops/5/mask", json!("nowhere")), "ops[5].mask: no mask is named nowhere");
    assert_eq!(broken("ops/1/id", json!("wb")), "ops[1].id: wb is used twice");
    assert_eq!(
        broken("ops/4/rgb", json!([[0.5, 0], [0.2, 1]])),
        "ops[4].rgb: x must grow from point to point"
    );
    assert_eq!(broken("ops/6/color", json!("blue")), "ops[6].color: use #rrggbb");
    assert_eq!(broken("span/zoom", json!(0.5)), "span.zoom: 0.5 is out of range 1 to 8");
    assert!(broken("version", json!(2)).contains("made by an older version"));
    assert!(broken("version", json!(9)).contains("update Reiwand"));
    assert!(broken("masks/sky/type", json!("sketch")).contains("unknown variant"));
    assert!(broken("ops/1/ev", json!("bright")).starts_with("ops[1].ev: invalid type: string \"bright\""));
    assert!(broken("masks/sky/invert", json!(3)).starts_with("masks.sky.invert: invalid type: integer `3`"));
    assert!(
        broken("geometry/rotate", json!(-90)).starts_with("geometry.rotate: invalid value: integer `-90`")
    );
    assert!(
        broken("outputs/Maker Model 1234/crop", json!("all")).starts_with("outputs.Maker Model 1234.crop:")
    );
    let crowded = json!({"version": 1, "ops": (0..25).map(|i| json!({"id": format!("e{i}"), "type": "exposure", "ev": 0})).collect::<Vec<_>>()});
    assert!(Edit::from_whole(crowded).unwrap_err().to_string().contains("at most 24 ops"));
}

#[test]
fn set_takes_shorthands_and_dotted_paths() {
    let mut edit = Edit::default();
    edit.set("ev", "0.3").unwrap();
    edit.set("temperature", "5200").unwrap();
    edit.set("tint", "4").unwrap();
    edit.set("contrast", "-10").unwrap();
    let ids: Vec<&str> = edit.ops.iter().map(|op| op.id.as_str()).collect();
    assert_eq!(ids, ["ev", "wb", "ct"]);
    assert_eq!(edit.ops[1].kind, Kind::WhiteBalance { temperature: 5200.0, tint: 4.0 });
    edit.set("ops.ev.opacity", "0.25").unwrap();
    edit.set("ops.wb.enabled", "false").unwrap();
    edit.set("geometry.rotate", "180").unwrap();
    edit.set("geometry.crop", "0.1,0.1,0.5,0.5").unwrap();
    edit.set("span.zoom", "1.5").unwrap();
    assert_eq!((edit.ops[0].opacity, edit.ops[1].enabled), (Some(0.25), false));
    assert_eq!((edit.geometry.rotate, edit.geometry.crop), (180, Some([0.1, 0.1, 0.5, 0.5])));
    assert_eq!(edit.span.as_ref().map(|s| (s.zoom, s.center)), Some((1.5, [0.5, 0.5])));
    edit.set("geometry.crop", "null").unwrap();
    edit.set("span", "null").unwrap();
    assert_eq!((edit.geometry.crop, &edit.span), (None, &None));
    let before = edit.clone();
    assert_eq!(edit.set("ev", "9").unwrap_err().to_string(), "ops[0].ev: 9 is out of range -5 to 5");
    assert_eq!(edit.set("ops.ev.ev", "bright").unwrap_err().to_string(), "ops.ev.ev: bright is not a number");
    assert_eq!(edit.set("ops.nope.ev", "1").unwrap_err().to_string(), "ops.nope.ev: unknown setting");
    assert_eq!(edit.set("ops.ct.foo", "1").unwrap_err().to_string(), "ops[2].foo: unknown field");
    assert_eq!(edit, before);
}

#[test]
fn edits_live_beside_the_image_or_in_the_data_folder() {
    let dir = tempfile::tempdir().unwrap();
    let (image, edits) = (dir.path().join("hills.jpg"), dir.path().join("data/edits"));
    std::fs::write(&image, b"not really a picture").unwrap();
    assert_eq!(edit::load(&image, &edits).unwrap(), None);
    let mut edit = Edit::default();
    edit.set("ev", "1").unwrap();
    let saved = edit::save(&edit, &image, &edits).unwrap();
    assert_eq!(saved, dir.path().join("hills.jpg.reiwand.json"));
    assert_eq!(std::fs::metadata(&saved).unwrap().permissions().mode() & 0o777, 0o600);
    assert_eq!(edit::load(&image, &edits).unwrap(), Some((saved.clone(), edit.clone())));
    let hidden = Path::new("/proc/self/status");
    let fallback = edit::save(&edit, hidden, &edits).unwrap();
    assert!(fallback.starts_with(&edits) && fallback.extension().is_some_and(|e| e == "json"));
    assert_eq!(std::fs::metadata(&edits).unwrap().permissions().mode() & 0o777, 0o700);
    assert_eq!(edit::load(hidden, &edits).unwrap().map(|(path, _)| path), Some(fallback.clone()));
    assert!(edit::remove(&image, &edits).unwrap() && !saved.exists());
    assert!(!edit::remove(&image, &edits).unwrap());
    std::fs::write(&saved, "{\"version\": 1, \"ops\": 3}").unwrap();
    assert!(edit::load(&image, &edits).unwrap_err().to_string().starts_with(&saved.display().to_string()));
    std::fs::write(&saved, " ".repeat(edit::MAX_BYTES + 1)).unwrap();
    assert!(edit::load(&image, &edits).unwrap_err().to_string().ends_with("is too large"));
    std::fs::remove_file(&saved).unwrap();
    let elsewhere = dir.path().join("elsewhere.json");
    std::os::unix::fs::symlink(&elsewhere, &saved).unwrap();
    let fallback = edit::save(&edit, &image, &edits).unwrap();
    assert!(fallback.starts_with(&edits) && !elsewhere.exists());
    std::fs::write(&elsewhere, Edit::default().to_json()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(20));
    edit::save(&edit, &image, &edits).unwrap();
    assert_eq!(edit::load(&image, &edits).unwrap(), Some((fallback, edit)));
}

#[test]
fn synced_edits_keep_the_local_source_and_removals_wait_for_known_items() {
    let dir = tempfile::tempdir().unwrap();
    let (image, edits) = (dir.path().join("hills.jpg"), dir.path().join("edits"));
    std::fs::write(&image, b"not really a picture").unwrap();
    let synced =
        Edit::whole(json!({"version": 1, "geometry": {"crop": [0.25, 0.0, 0.5, 1.0]}, "fit": "fit"}));
    assert!(edit::adopt(Some(&synced), &image, &edits, false).unwrap());
    let (_, adopted) = edit::load(&image, &edits).unwrap().unwrap();
    assert_eq!((adopted.geometry.crop, adopted.source.bytes), (Some([0.25, 0.0, 0.5, 1.0]), 20));
    assert_eq!(adopted.source.sha256.len(), 64);
    assert!(!edit::adopt(Some(&synced), &image, &edits, false).unwrap());
    assert!(edit::adopt(Some(&json!({"version": 9})), &image, &edits, false).is_err());
    assert!(
        !edit::adopt(None, &image, &edits, true).unwrap() && edit::load(&image, &edits).unwrap().is_some()
    );
    assert!(
        edit::adopt(None, &image, &edits, false).unwrap() && edit::load(&image, &edits).unwrap().is_none()
    );
    assert!(!edit::adopt(None, &image, &edits, false).unwrap());
}
