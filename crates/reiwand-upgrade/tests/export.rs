use reiwand_carry::{Cost, Error, Limits, Opened, is_sealed, open, seal};
use reiwand_upgrade::Outcome;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

const OWN: &[u8] = b"own wallpaper bytes";
const KEY: &str = "pass-phrase";

fn old_file() -> Vec<u8> {
    let content = json!({
        "repositories": [{"address": "https://example.org/r/one", "repo_id": "r1", "origin": "https://example.org"}],
        "lists": [{"name": "Calm", "items": [{"id": "a"}, {"repo_id": "r1", "id": "b"}]}],
        "favourites": [{"id": "a"}],
        "own": [{"id": "mine", "name": "Mine"}]
    })
    .to_string();
    let hex = |b: &[u8]| b.iter().map(|x| format!("{x:02x}")).collect::<String>();
    let head = json!({
        "format": 1, "program": "reiwand", "version": "1.4.0", "created": "2026-01-01",
        "sections": ["repositories", "lists", "favourites", "own"], "encrypted": false,
        "sums": {
            "files/content.json": hex(&Sha256::digest(content.as_bytes())),
            "files/own/mine": hex(&Sha256::digest(OWN))
        }
    });
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    let packed = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    zip.start_file("files/content.json", packed).unwrap();
    zip.write_all(content.as_bytes()).unwrap();
    zip.start_file(
        "files/own/mine",
        SimpleFileOptions::default().compression_method(CompressionMethod::Stored),
    )
    .unwrap();
    zip.write_all(OWN).unwrap();
    zip.start_file("carry.json", packed).unwrap();
    zip.write_all(head.to_string().as_bytes()).unwrap();
    zip.finish().unwrap().into_inner()
}

fn check(opened: &mut Opened) {
    assert_eq!(opened.manifest.format, 2);
    assert_eq!(opened.content.repositories[0].name, "");
    assert_eq!(opened.content.repositories[0].pin, None);
    assert_eq!(opened.content.lists[0].query, None);
    assert_eq!(opened.content.lists[0].items.len(), 2);
    assert_eq!(opened.content.favourites[0].repo_id, None);
    assert!(opened.content.secrets.is_none());
    let mut out = Vec::new();
    opened.extract_own("mine", &mut out).unwrap();
    assert_eq!(out, OWN);
}

#[test]
fn a_plain_old_file_is_refused_then_converted_and_reads_back() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("old.reiwand");
    std::fs::write(&path, old_file()).unwrap();
    let refused = open(&path, None, Limits::default()).err().unwrap();
    assert!(matches!(refused, Error::Old { found: 1, .. }));
    assert!(refused.to_string().contains("reiwand-upgrade export"));
    let before = std::fs::read(&path).unwrap();
    assert_eq!(
        reiwand_upgrade::carry::file(&path, None, true).unwrap(),
        Outcome::Upgraded { from: 1, to: 2 }
    );
    assert_eq!(std::fs::read(&path).unwrap(), before);
    assert_eq!(
        reiwand_upgrade::carry::file(&path, None, false).unwrap(),
        Outcome::Upgraded { from: 1, to: 2 }
    );
    check(&mut open(&path, None, Limits::default()).unwrap());
    let converted = std::fs::read(&path).unwrap();
    assert_eq!(reiwand_upgrade::carry::file(&path, None, false).unwrap(), Outcome::Nothing);
    assert_eq!(std::fs::read(&path).unwrap(), converted);
}

#[test]
fn an_encrypted_old_file_stays_encrypted_and_needs_its_password() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("old.reiwand");
    let cost = Cost { memory_kib: 64, passes: 1 };
    std::fs::write(&path, seal(&old_file(), KEY, cost).unwrap()).unwrap();
    assert!(reiwand_upgrade::carry::file(&path, None, false).is_err());
    assert!(reiwand_upgrade::carry::file(&path, Some("wrong"), false).is_err());
    assert!(is_sealed(&std::fs::read(&path).unwrap()));
    reiwand_upgrade::carry::file(&path, Some(KEY), false).unwrap();
    assert!(is_sealed(&std::fs::read(&path).unwrap()));
    assert!(matches!(open(&path, None, Limits::default()), Err(Error::NeedsPassword)));
    check(&mut open(&path, Some(KEY), Limits::default()).unwrap());
}

#[test]
fn a_damaged_conversion_leaves_the_original() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("old.reiwand");
    std::fs::write(&path, b"not an archive").unwrap();
    assert!(reiwand_upgrade::carry::file(&path, None, false).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), b"not an archive");
}
