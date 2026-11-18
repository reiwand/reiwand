mod acting;
mod docs;
mod examples;
mod language;
mod more;
mod passing;
mod presets;
mod textures;

use image::{Rgba, RgbaImage};
use language::show_in;
use serde_json::{Value, json};
use std::path::Path;
use std::process::{Command, Output, Stdio};

pub fn run(dir: &Path, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_reiwand-ctl"));
    command.args(args).current_dir(dir).stdin(Stdio::null()).env("XDG_STATE_HOME", dir.join("state"));
    command.env("LC_ALL", "C");
    command.env("DBUS_SESSION_BUS_ADDRESS", format!("unix:path={}/no-bus", dir.display()));
    command.output().unwrap()
}

pub fn ok(dir: &Path, args: &[&str]) -> String {
    let out = run(dir, args);
    assert!(out.status.success(), "{args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap()
}

pub fn fails(dir: &Path, args: &[&str]) -> String {
    let out = run(dir, args);
    assert_eq!(out.status.code(), Some(1), "{args:?}: {}", String::from_utf8_lossy(&out.stdout));
    String::from_utf8(out.stderr).unwrap()
}

const OPACITY: &str = "/scene/layers/1/opacity";

pub fn act(dir: &Path, folder: &str, action: &str, args: Value) -> String {
    ok(dir, &["scene", "act", folder, action, &args.to_string()])
}

pub fn show(dir: &Path, pointer: &str) -> Value {
    serde_json::from_str(&ok(dir, &["scene", "show", "snow", pointer])).unwrap()
}

#[test]
fn a_scene_is_made_edited_and_built_from_the_command_line() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "snow", "--from", "particles", "--title", "Snow"]);
    assert_eq!(show(dir, "/manifest/id"), "org.example.snow");
    assert!(
        ok(dir, &["scene", "open", "snow"])
            .starts_with("Snow (org.example.snow 0.1.0): 2 layers, 0 problems")
    );
    RgbaImage::from_pixel(8, 8, Rgba([200, 220, 255, 255])).save(dir.join("moon.png")).unwrap();
    assert_eq!(ok(dir, &["scene", "import", "snow", "moon.png"]), "moon\n");
    assert_eq!(act(dir, "snow", "add_layer", json!({"kind": "image", "source": "moon", "at": 1})), "image\n");
    act(dir, "snow", "set", json!({"pointer": "/scene/layers/1/transform/position", "value": [1500, 200]}));
    act(dir, "snow", "set", json!({"pointer": "/scene/canvas/background", "value": "#102030"}));
    act(dir, "snow", "add", json!({"pointer": "/scene/timelines", "value": [{"id": "rise", "duration": 4}]}));
    act(dir, "snow", "keyframe", json!({"pointer": OPACITY, "time": 0, "value": 0, "timeline": "rise"}));
    act(dir, "snow", "keyframe", json!({"pointer": OPACITY, "time": 4, "value": 1, "ease": "out_cubic"}));
    act(dir, "snow", "keyframe", json!({"pointer": OPACITY, "time": 3, "from": 4}));
    act(dir, "snow", "keyframe", json!({"pointer": OPACITY, "time": 4, "from": 3}));
    let opacity = show(dir, "/scene/layers/1/opacity");
    assert_eq!(opacity["keys"], json!([[0.0, 0], [4.0, 1, "out_cubic"]]));
    assert_eq!(opacity["timeline"], "rise");
    assert_eq!(act(dir, "snow", "duplicate_layer", json!({"id": "image"})), "image_copy\n");
    act(dir, "snow", "group", json!({"ids": ["image", "image_copy"], "id": "moons"}));
    act(dir, "snow", "move_layer", json!({"id": "moons", "to": 0}));
    act(dir, "snow", "hide", json!({"id": "sky", "hidden": true}));
    act(dir, "snow", "lock", json!({"id": "flakes", "locked": true}));
    act(dir, "snow", "solo", json!({"id": "flakes"}));
    act(dir, "snow", "ungroup", json!({"id": "moons"}));
    act(dir, "snow", "remove_layer", json!({"id": "image_copy"}));
    act(dir, "snow", "move", json!({"from": "/scene/layers/0", "to": "/scene/layers/2"}));
    act(dir, "snow", "remove", json!({"pointer": "/scene/ext/editor/solo"}));
    let ids: Vec<Value> =
        show(dir, "/scene/layers").as_array().unwrap().iter().map(|l| l["id"].clone()).collect();
    assert_eq!(ids, [json!("sky"), json!("flakes"), json!("image")]);
    let history = ok(dir, &["scene", "history", "snow"]);
    assert_eq!(history.lines().count(), 19, "{history}");
    act(dir, "snow", "undo", json!({}));
    assert_eq!(show(dir, "/scene/ext/editor/solo"), "flakes");
    act(dir, "snow", "redo", json!({}));
    assert!(fails(dir, &["scene", "act", "snow", "redo", "{}"]).contains("nothing to redo"));
    assert!(ok(dir, &["scene", "history", "snow"]).lines().all(|l| !l.ends_with("(undone)")));
    let fields: Value =
        serde_json::from_str(&ok(dir, &["scene", "fields", "snow", "/scene/layers/2"])).unwrap();
    assert!(
        fields.as_array().unwrap().iter().any(|f| f["key"] == "image" && f["options"] == json!(["moon"]))
    );
    act(dir, "snow", "set", json!({"pointer": "/scene/layers/2/opacity", "value": 3}));
    let problems = fails(dir, &["pack", "check", "snow"]);
    assert!(problems.trim_end().ends_with("1 problem"), "{problems}");
    assert!(more::ok_lines(dir).contains("/scene/layers/2/opacity"));
    assert!(fails(dir, &["pack", "build", "snow"]).contains("/scene/layers/2/opacity"));
    act(dir, "snow", "undo", json!({}));
    let built: Value =
        serde_json::from_str(&ok(dir, &["pack", "build", "snow", "-o", "snow.wpak", "--json"])).unwrap();
    assert_eq!(built["path"], "snow.wpak");
    assert!(ok(dir, &["pack", "validate", "snow.wpak"]).starts_with("ok scene org.example.snow 0.1.0"));
    ok(dir, &["scene", "open", "copy", "--from", "snow.wpak"]);
    assert!(dir.join("copy/assets/moon.png").is_file());
    act(dir, "copy", "set", json!({"pointer": "/scene/canvas/background", "value": "#111111"}));
    act(dir, "copy", "set", json!({"pointer": "/scene/canvas/background", "value": "#222222"}));
    assert_eq!(ok(dir, &["scene", "history", "copy"]), "Change a value\nChange a value\n");
    act(dir, "copy", "undo", json!({}));
    assert_eq!(ok(dir, &["scene", "show", "copy", "/scene/canvas/background"]).trim(), "\"#111111\"");
    assert_eq!(act(dir, "copy", "add_layer", json!({"kind": "text", "name": "Título"})), "titulo\n");
    assert_eq!(ok(dir, &["scene", "show", "copy", "/scene/layers/3/text"]).trim(), "\"Título\"");
    std::fs::write(dir.join("snow/assets/stale.png"), b"x").unwrap();
    assert_eq!(ok(dir, &["scene", "prune", "snow"]), "assets/stale.png\n");
    let schema: Value = serde_json::from_str(&ok(dir, &["scene", "schema"])).unwrap();
    assert!(schema["$defs"]["Layer"].is_object());
    assert!(run(dir, &["scene", "preview", "snow", "-o", "DP-1"]).status.code() == Some(1));
}

#[test]
fn an_edited_copy_of_a_signed_scene_carries_no_signature_of_the_original() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    let author = ok(dir, &["repo", "keygen", "author", "--no-password"]).trim().to_owned();
    ok(dir, &["scene", "new", "orig"]);
    act(dir, "orig", "set", json!({"pointer": "/manifest/authors", "value": ["Ana"]}));
    ok(dir, &["pack", "sign", "orig", "--key", "author.key"]);
    ok(dir, &["pack", "build", "orig", "-o", "orig.wpak"]);
    assert!(ok(dir, &["pack", "validate", "orig.wpak", "--key", "author.pub"]).contains(&author));
    assert!(ok(dir, &["pack", "check", "orig"]).contains("manifest.json.minisig"));
    ok(dir, &["scene", "open", "copy", "--from", "orig.wpak"]);
    assert!(!dir.join("copy/manifest.json.minisig").exists());
    assert_eq!(ok(dir, &["scene", "show", "copy", "/manifest/id"]).trim(), "\"org.example.orig.copy\"");
    let ancestor = json!({"author": "Ana", "title": "orig", "license": "CC0-1.0", "url": null});
    assert_eq!(
        show_in(dir, "copy", "/manifest/origin"),
        json!({"kind": "derived", "derived_from": [ancestor], "provider": null, "source_url": null})
    );
    assert_eq!(show_in(dir, "copy", "/manifest/authors"), json!([]));
    act(dir, "copy", "set", json!({"pointer": "/scene/canvas/background", "value": "#ff0000"}));
    let built: Value = serde_json::from_str(&ok(dir, &["pack", "build", "copy", "--json"])).unwrap();
    assert_eq!(built["path"], json!("org.example.orig.copy-0.1.0.wpak"));
    let checked = run(dir, &["pack", "validate", "org.example.orig.copy-0.1.0.wpak", "--key", "author.pub"]);
    assert!(!String::from_utf8_lossy(&checked.stdout).contains(&author));
    act(dir, "orig", "set", json!({"pointer": "/manifest/authors", "value": ["Bea"]}));
    assert!(fails(dir, &["pack", "build", "orig", "-o", "plain.wpak"]).contains("pack sign again"));
    std::fs::remove_file(dir.join("orig/manifest.json.minisig")).unwrap();
    ok(dir, &["pack", "build", "orig", "-o", "plain.wpak"]);
}
