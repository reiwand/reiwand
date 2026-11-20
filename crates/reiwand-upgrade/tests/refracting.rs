use reiwand_upgrade::NoVault;
use reiwand_upgrade::client::{self, Options};
use serde_json::{Value, json};
use std::fs;

fn options(also: std::path::PathBuf) -> Options {
    Options { skip_download: true, check: false, also: vec![also], copies: None }
}

fn old_rain() -> Value {
    let render = json!({ "refract": { "normal": "drop", "units": "pixels", "shift": 30 } });
    json!({ "canvas": { "size": [64, 64] }, "particles": { "rain": { "capacity": 4, "emitters": [], "render": render } } })
}

#[test]
fn from_5_every_refraction_gains_its_color_once() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("format-version"), "5\n").unwrap();
    let folder = data.join("scenes").join("rain");
    fs::create_dir_all(&folder).unwrap();
    fs::write(folder.join("scene.json"), old_rain().to_string()).unwrap();
    let opts = options(data.join("scenes"));
    let done = client::convert(data, &mut NoVault, &opts).unwrap();
    assert_eq!(done.outcome, reiwand_upgrade::Outcome::Upgraded { from: 5, to: 6 });
    let scene: Value = serde_json::from_slice(&fs::read(folder.join("scene.json")).unwrap()).unwrap();
    let bent = &scene["particles"]["rain"]["render"]["refract"];
    assert_eq!((bent["color"].as_str(), bent["shift"].as_f64()), (Some("none"), Some(30.0)));
    fs::write(folder.join("scene.json"), old_rain().to_string()).unwrap();
    let done = client::convert(data, &mut NoVault, &opts).unwrap();
    assert_eq!(done.outcome, reiwand_upgrade::Outcome::Nothing);
    assert_eq!(fs::read_to_string(folder.join("scene.json")).unwrap(), old_rain().to_string());
}

#[test]
fn from_5_every_perspective_gains_the_near_plane_it_had() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("format-version"), "5\n").unwrap();
    let folder = data.join("scenes").join("stars");
    fs::create_dir_all(&folder).unwrap();
    let old = json!({ "canvas": { "size": [64, 64] },
        "particles": { "stars": { "capacity": 4, "emitters": [], "perspective": { "focal": 300 } } } });
    fs::write(folder.join("scene.json"), old.to_string()).unwrap();
    let opts = options(data.join("scenes"));
    let done = client::convert(data, &mut NoVault, &opts).unwrap();
    assert_eq!(done.outcome, reiwand_upgrade::Outcome::Upgraded { from: 5, to: 6 });
    let scene: Value = serde_json::from_slice(&fs::read(folder.join("scene.json")).unwrap()).unwrap();
    let seen = &scene["particles"]["stars"]["perspective"];
    assert_eq!((seen["focal"].as_f64(), seen["near"].as_f64()), (Some(300.0), Some(1.0)));
}
