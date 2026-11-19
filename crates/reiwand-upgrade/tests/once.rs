use reiwand_upgrade::NoVault;
use reiwand_upgrade::client::{self, Options};
use std::fs;
use std::io::Write;
use std::path::Path;

const MANIFEST: &str = r#"{"schema": "reiwand.wpak/2", "id": "org.example.old", "version": "1.0.0", "title": "old",
"license": "CC0-1.0", "kind": "image", "entry": "image.png", "photosensitive": false,
"content": {"rating": "general"}, "origin": {"kind": "original"}, "previews": {"thumbnail": "preview.png"},
"requires": {"features": []}}"#;

fn png() -> Vec<u8> {
    let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
    data.extend_from_slice(&[0; 32]);
    data
}

fn package(path: &Path, signed: bool) -> Vec<u8> {
    let mut zip = zip::ZipWriter::new(fs::File::create(path).unwrap());
    let plain = zip::write::SimpleFileOptions::default();
    let mut entries = vec![("manifest.json", MANIFEST.as_bytes().to_vec()), ("image.png", png())];
    entries.push(("preview.png", png()));
    if signed {
        entries.push((reiwand_format::SIGNATURE, b"signature".to_vec()));
    }
    for (name, bytes) in entries {
        zip.start_file(name, plain).unwrap();
        zip.write_all(&bytes).unwrap();
    }
    zip.finish().unwrap();
    fs::read(path).unwrap()
}

use std::path::PathBuf;

fn options() -> Options {
    Options { skip_download: true, check: false, also: Vec::new(), copies: None }
}

fn records(data: &Path, paths: &[PathBuf]) {
    let list: Vec<_> = paths
        .iter()
        .map(|p| {
            serde_json::json!({"repo_id": "main", "id": "org.example.old", "version": "1.0.0",
                "sha256": "00", "size": 1, "path": p, "signer": "ab12"})
        })
        .collect();
    fs::write(data.join("installed.json"), serde_json::Value::from(list).to_string()).unwrap();
}

fn state(data: &Path, name: &str, body: &str) -> PathBuf {
    let folder = data.join("repos").join(name);
    fs::create_dir_all(&folder).unwrap();
    fs::write(folder.join("state.json"), body).unwrap();
    folder.join("state.json")
}

const OLD_STATE: &str = r#"{"url":"https://a.example/repo/?fingerprint=ab","bases":[],"repo_id":"x"}"#;

#[test]
fn a_run_that_stops_midway_keeps_the_records_of_what_was_rewritten() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("format-version"), "2\n").unwrap();
    let (good, bad) = (data.join("good.wpak"), data.join("bad.wpak"));
    let bytes = package(&good, true);
    fs::write(&bad, b"not a package").unwrap();
    records(data, &[good.clone(), bad.clone()]);
    let done = client::convert(data, &mut NoVault, &options()).unwrap();
    assert_eq!(done.skipped.len(), 1);
    assert_eq!(done.skipped[0].0, bad);
    assert_eq!(done.unsigned.len(), 1);
    let saved: serde_json::Value =
        serde_json::from_slice(&fs::read(data.join("installed.json")).unwrap()).unwrap();
    let first = &saved["content"][0];
    assert!(first["signer"].is_null());
    assert_ne!(first["sha256"], reiwand_format::sha256(&bytes[..]).unwrap());
    assert_eq!(first["size"], fs::metadata(&good).unwrap().len());
    assert_eq!(fs::read(&bad).unwrap(), b"not a package");
}

#[test]
fn damaged_files_are_listed_and_the_rest_is_converted() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("format-version"), "2\n").unwrap();
    let empty = state(data, "empty", "");
    let fine = state(data, "fine", OLD_STATE);
    let done = client::convert(data, &mut NoVault, &options()).unwrap();
    assert_eq!(done.skipped.iter().map(|(p, _)| p.clone()).collect::<Vec<_>>(), vec![empty]);
    assert!(!fs::read_to_string(fine).unwrap().contains('?'));
    assert_eq!(fs::read_to_string(data.join("format-version")).unwrap(), "4\n");
    let listed = fs::read_to_string(done.copies.join("not-converted.txt")).unwrap();
    assert!(listed.contains("empty"));
}

#[test]
fn a_link_to_an_ancestor_is_not_followed() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path().join("data");
    let library = dir.path().join("library");
    fs::create_dir_all(&data).unwrap();
    fs::create_dir_all(library.join("inner")).unwrap();
    std::os::unix::fs::symlink(&library, library.join("inner/back")).unwrap();
    fs::write(data.join("format-version"), "2\n").unwrap();
    let options = Options { also: vec![library], ..options() };
    client::convert(&data, &mut NoVault, &options).unwrap();
}

#[test]
fn a_second_run_does_nothing_and_does_not_read_the_packages() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("format-version"), "2\n").unwrap();
    let fine = state(data, "fine", OLD_STATE);
    client::convert(data, &mut NoVault, &options()).unwrap();
    let bad = data.join("bad.wpak");
    fs::write(&bad, b"not a package").unwrap();
    records(data, std::slice::from_ref(&bad));
    fs::write(&fine, OLD_STATE).unwrap();
    let done = client::convert(data, &mut NoVault, &options()).unwrap();
    assert_eq!(done.outcome, reiwand_upgrade::Outcome::Nothing);
    assert!(done.skipped.is_empty() && done.changed.is_empty());
    assert_eq!(fs::read_to_string(fine).unwrap(), OLD_STATE);
}

#[test]
fn from_3_to_4_only_the_newer_steps_run() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("format-version"), "3\n").unwrap();
    let fine = state(data, "fine", OLD_STATE);
    let done = client::convert(data, &mut NoVault, &options()).unwrap();
    assert_eq!(done.outcome, reiwand_upgrade::Outcome::Upgraded { from: 3, to: 4 });
    assert_eq!(fs::read_to_string(fine).unwrap(), OLD_STATE);
    assert_eq!(fs::read_to_string(data.join("format-version")).unwrap(), "4\n");
}

#[test]
fn an_empty_mark_does_not_block_and_the_mark_is_replaced_whole() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("format-version"), "").unwrap();
    client::convert(data, &mut NoVault, &options()).unwrap();
    assert_eq!(fs::read_to_string(data.join("format-version")).unwrap(), "4\n");
}

#[test]
fn saving_keeps_the_mode_and_writes_through_a_link() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let real = dir.path().join("real.json");
    let link = dir.path().join("link.json");
    fs::write(&real, "old").unwrap();
    fs::set_permissions(&real, fs::Permissions::from_mode(0o600)).unwrap();
    std::os::unix::fs::symlink(&real, &link).unwrap();
    let mut sink = reiwand_upgrade::sink::Sink::copying(&dir.path().join("copies"));
    sink.save(&link, b"new").unwrap();
    assert!(fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
    assert_eq!(fs::read_to_string(&real).unwrap(), "new");
    assert_eq!(fs::metadata(&real).unwrap().permissions().mode() & 0o777, 0o600);
}

#[test]
fn an_interrupted_examples_swap_is_finished_by_the_next_run() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("format-version"), "3\n").unwrap();
    let (staged, keys) = (data.join("examples-repo.upgrading"), data.join("examples-keys"));
    reiwand_format::bundled(&staged, &keys, "Example wallpapers", &[], 1_700_000_000).unwrap();
    client::convert(data, &mut NoVault, &options()).unwrap();
    assert!(!staged.exists());
    assert!(reiwand_format::status(&data.join("examples-repo")).is_ok());
}

#[test]
fn two_sinks_of_one_run_number_their_copies_one_after_the_other() {
    let dir = tempfile::tempdir().unwrap();
    let copies = dir.path().join("copies");
    let (a, b) = (dir.path().join("a"), dir.path().join("b"));
    for (folder, text) in [(&a, "first"), (&b, "second")] {
        fs::create_dir_all(folder).unwrap();
        fs::write(folder.join("state.json"), text).unwrap();
    }
    reiwand_upgrade::sink::Sink::copying(&copies).keep(&a.join("state.json")).unwrap();
    reiwand_upgrade::sink::Sink::copying(&copies).keep(&b.join("state.json")).unwrap();
    let index = fs::read_to_string(copies.join("index.txt")).unwrap();
    let numbers: Vec<&str> = index.lines().filter_map(|l| l.split('\t').next()).collect();
    assert_eq!(numbers, ["1", "2"], "{index}");
    assert_eq!(fs::read_to_string(copies.join("1-state.json")).unwrap(), "first");
    assert_eq!(fs::read_to_string(copies.join("2-state.json")).unwrap(), "second");
}
