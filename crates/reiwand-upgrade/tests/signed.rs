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

fn installed(data: &Path, signed: bool, signer: serde_json::Value) {
    fs::create_dir_all(data.join("files")).unwrap();
    fs::write(data.join("format-version"), "2\n").unwrap();
    let path = data.join("files/old.wpak");
    let bytes = package(&path, signed);
    let sha = reiwand_format::sha256(&bytes[..]).unwrap();
    let record = serde_json::json!([{"repo_id": "main", "id": "org.example.old", "version": "1.0.0",
        "sha256": sha, "size": bytes.len(), "path": path, "signer": signer}]);
    fs::write(data.join("installed.json"), record.to_string()).unwrap();
}

fn options(check: bool) -> Options {
    Options { skip_download: true, check, also: Vec::new(), copies: None }
}

#[test]
fn a_signed_installed_package_becomes_local_with_an_unverified_signature_and_is_named() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    installed(data, true, serde_json::json!("ab12"));
    let said = client::convert(data, &mut NoVault, &options(true)).unwrap();
    assert_eq!(said.unsigned, vec!["org.example.old 1.0.0".to_owned()]);
    let done = client::convert(data, &mut NoVault, &options(false)).unwrap();
    assert_eq!(done.unsigned, vec!["org.example.old 1.0.0".to_owned()]);
    let records: serde_json::Value =
        serde_json::from_slice(&fs::read(data.join("installed.json")).unwrap()).unwrap();
    assert!(records[0]["signer"].is_null());
    let file = fs::File::open(data.join("files/old.wpak")).unwrap();
    let archive = zip::ZipArchive::new(file).unwrap();
    let names: Vec<String> = archive.file_names().map(|n| n.unwrap().into_owned()).collect();
    assert!(!names.iter().any(|n| n == reiwand_format::SIGNATURE), "{names:?}");
    assert!(client::convert(data, &mut NoVault, &options(false)).unwrap().unsigned.is_empty());
}

#[test]
fn an_unsigned_installed_package_is_not_reported() {
    let dir = tempfile::tempdir().unwrap();
    installed(dir.path(), false, serde_json::Value::Null);
    let done = client::convert(dir.path(), &mut NoVault, &options(false)).unwrap();
    assert!(done.unsigned.is_empty());
}
