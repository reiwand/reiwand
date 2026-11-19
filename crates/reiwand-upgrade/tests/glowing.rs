use reiwand_upgrade::NoVault;
use reiwand_upgrade::client::{self, Options};
use std::fs;
use std::path::{Path, PathBuf};

fn options() -> Options {
    Options { skip_download: true, check: false, also: Vec::new(), copies: None }
}

const OLD_GLOW: &str = r##"{"canvas": {"size": [64, 64], "bloom": {"threshold": 0.8, "strength": 1, "radius": 16, "tint": "#ffffff", "hdr": {"threshold": 2, "strength": 1}}}}"##;

fn glow_folder(data: &Path) -> PathBuf {
    let folder = data.join("scenes").join("glow");
    fs::create_dir_all(&folder).unwrap();
    fs::write(folder.join("scene.json"), OLD_GLOW).unwrap();
    folder
}

#[test]
fn from_4_the_glow_fields_are_written_once_and_from_5_nothing_is_read() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("format-version"), "4\n").unwrap();
    let folder = glow_folder(data);
    let mut opts = options();
    opts.also = vec![data.join("scenes")];
    let done = client::convert(data, &mut NoVault, &opts).unwrap();
    assert_eq!(done.outcome, reiwand_upgrade::Outcome::Upgraded { from: 4, to: 5 });
    let scene: serde_json::Value =
        serde_json::from_slice(&fs::read(folder.join("scene.json")).unwrap()).unwrap();
    let bloom = &scene["canvas"]["bloom"];
    assert_eq!(bloom["saturation"], 1.0);
    assert_eq!(bloom["knee"], 0.5);
    assert_eq!(bloom["radius_unit"], "canvas");
    assert!(bloom["limit"].is_null() && bloom.as_object().unwrap().contains_key("limit"));
    assert_eq!(bloom["hdr"]["knee"], 0.5);
    assert_eq!(bloom["hdr"]["levels"], 1);
    assert!(bloom["hdr"]["scatter"].is_number());
    let again = fs::read(folder.join("scene.json")).unwrap();
    fs::write(folder.join("scene.json"), OLD_GLOW).unwrap();
    let done = client::convert(data, &mut NoVault, &opts).unwrap();
    assert_eq!(done.outcome, reiwand_upgrade::Outcome::Nothing);
    assert_eq!(fs::read_to_string(folder.join("scene.json")).unwrap(), OLD_GLOW);
    assert!(!again.is_empty());
}
