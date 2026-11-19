use reiwand_upgrade::client::{self, Options};
use reiwand_upgrade::{NoVault, Outcome};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const MANIFEST: &str = r#"{"schema": "reiwand.wpak/2", "id": "org.example.old", "version": "1.0.0", "title": "old",
"license": "CC0-1.0", "kind": "image", "entry": "image.png", "photosensitive": false,
"content": {"rating": "general"}, "origin": {"kind": "original"}, "previews": {"thumbnail": "preview.png"},
"requires": {"features": []}}"#;

const EDIT: &str = include_str!("pairs/edit.original.json");
const CONFIG: &str = include_str!("pairs/config.realistic.toml");

fn png() -> Vec<u8> {
    let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
    data.extend_from_slice(&[0; 32]);
    data
}

fn package(path: &Path) -> Vec<u8> {
    let mut zip = zip::ZipWriter::new(fs::File::create(path).unwrap());
    let plain = zip::write::SimpleFileOptions::default();
    for (name, bytes) in
        [("manifest.json", MANIFEST.as_bytes().to_vec()), ("image.png", png()), ("preview.png", png())]
    {
        zip.start_file(name, plain).unwrap();
        zip.write_all(&bytes).unwrap();
    }
    zip.finish().unwrap();
    fs::read(path).unwrap()
}

fn six(root: &Path) -> (PathBuf, PathBuf) {
    let (data, config) = (root.join("data"), root.join("config"));
    fs::create_dir_all(data.join("files")).unwrap();
    fs::create_dir_all(data.join("edits")).unwrap();
    fs::create_dir_all(&config).unwrap();
    fs::write(data.join("format-version"), "2\n").unwrap();
    let path = data.join("files/old.wpak");
    let bytes = package(&path);
    let sha = reiwand_format::sha256(&bytes[..]).unwrap();
    let record = serde_json::json!([{"repo_id": "main", "id": "org.example.old", "version": "1.0.0",
        "sha256": sha, "size": bytes.len(), "path": path}]);
    fs::write(data.join("installed.json"), record.to_string()).unwrap();
    fs::write(data.join("inputs.json"), r#"{"sha256:aa": {"sea": ["audio"]}}"#).unwrap();
    fs::write(data.join("edits/a.json"), EDIT).unwrap();
    fs::write(config.join("config.toml"), CONFIG).unwrap();
    (data, config)
}

fn tree(dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
    let mut items: Vec<_> = fs::read_dir(dir).unwrap().flatten().map(|e| e.path()).collect();
    items.sort();
    for path in items.into_iter().filter(|p| !p.to_string_lossy().contains("upgrade-copias")) {
        if path.is_dir() {
            tree(&path, out);
        } else {
            out.push((path.clone(), fs::read(&path).unwrap()));
        }
    }
}

fn options(check: bool) -> Options {
    Options { skip_download: true, check, also: Vec::new(), copies: None }
}

#[test]
fn a_six_client_folder_converts_loads_strictly_and_converts_again_to_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let (data, config) = six(dir.path());
    let (mut before, mut after) = (Vec::new(), Vec::new());
    tree(dir.path(), &mut before);
    let said = client::convert(&data, &mut NoVault, &options(true)).unwrap();
    tree(dir.path(), &mut after);
    assert_eq!(before, after, "--check wrote something");
    assert!(said.changed.iter().any(|p| p.ends_with("files/old.wpak")));
    assert!(said.changed.iter().any(|p| p.ends_with("installed.json")));
    assert!(said.changed.iter().any(|p| p.ends_with("edits/a.json")));
    assert!(!data.join("upgrade-copias").exists());

    let mut sink = reiwand_upgrade::sink::Sink::copying(&dir.path().join("config-copies"));
    reiwand_upgrade::settings::run(&config, &mut sink).unwrap();
    let done = client::convert(&data, &mut NoVault, &options(false)).unwrap();
    assert_eq!(done.outcome, Outcome::Upgraded { from: 2, to: 5 });
    assert_eq!(fs::read_to_string(data.join("format-version")).unwrap(), "5\n");
    let index = fs::read_to_string(done.copies.join("index.txt")).unwrap();
    assert!(index.contains("old.wpak") && index.contains("installed.json") && index.contains("inputs.json"));

    let manifest = reiwand_format::peek(data.join("files/old.wpak")).unwrap();
    assert_eq!(manifest.id, "org.example.old");
    let records = reiwand_format::installed(&data).unwrap();
    let bytes = fs::read(&records[0].path).unwrap();
    assert_eq!(records[0].sha256, reiwand_format::sha256(&bytes[..]).unwrap());
    assert_eq!(records[0].size, bytes.len() as u64);
    reiwand_core::edit::Edit::parse(&fs::read_to_string(data.join("edits/a.json")).unwrap()).unwrap();
    reiwand_core::Config::parse(&fs::read_to_string(config.join("config.toml")).unwrap()).unwrap();
    reiwand_format::check_data(&data).unwrap();

    let (mut once, mut twice) = (Vec::new(), Vec::new());
    tree(dir.path(), &mut once);
    let again = client::convert(&data, &mut NoVault, &options(false)).unwrap();
    tree(dir.path(), &mut twice);
    assert_eq!((again.outcome, again.changed.len()), (Outcome::Nothing, 0));
    assert_eq!(once, twice, "the second run changed something");
    assert!(client::convert(&data, &mut NoVault, &options(true)).unwrap().changed.is_empty());
}

#[test]
fn a_file_command_converts_a_package_and_a_check_changes_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("a.wpak");
    package(&path);
    let was = fs::read(&path).unwrap();
    let mut check = reiwand_upgrade::sink::Sink::checking();
    assert_eq!(reiwand_upgrade::file::convert(&path, None, &mut check).unwrap(), 1);
    assert_eq!(fs::read(&path).unwrap(), was);
    let mut real = reiwand_upgrade::sink::Sink::copying(&dir.path().join("copies"));
    assert_eq!(reiwand_upgrade::file::convert(&path, None, &mut real).unwrap(), 1);
    assert_eq!(reiwand_upgrade::file::convert(&path, None, &mut real).unwrap(), 0);
    assert_eq!(fs::read(dir.path().join("copies/1-a.wpak")).unwrap(), was);
}
