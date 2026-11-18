mod common;
use common::*;
use reiwand_carry::*;
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

fn zip_with(entries: &[(&str, Vec<u8>, bool)]) -> Vec<u8> {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, data, _) in entries {
        let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
        zip.start_file(*name, opts).unwrap();
        zip.write_all(data).unwrap();
    }
    zip.finish().unwrap().into_inner()
}

fn tweak(bytes: &[u8], from: &[u8], to: &[u8]) -> Vec<u8> {
    let at = bytes.windows(from.len()).position(|w| w == from).unwrap();
    let mut out = bytes.to_vec();
    out[at..at + to.len()].copy_from_slice(to);
    out
}

#[test]
fn path_escapes_are_refused() {
    for name in ["../evil", "/abs", "files/../../x", "a\\b"] {
        let bytes = zip_with(&[(name, vec![1], false), ("carry.json", b"{}".to_vec(), false)]);
        assert!(matches!(open_bytes(bytes, None, Limits::default()), Err(Error::Unsafe(_))), "{name}");
    }
}

#[test]
fn symlinks_are_refused() {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    zip.add_symlink("carry.json", "/etc/passwd", SimpleFileOptions::default()).unwrap();
    let bytes = zip.finish().unwrap().into_inner();
    assert!(matches!(open_bytes(bytes, None, Limits::default()), Err(Error::Unsafe(_))));
}

#[test]
fn zip_bombs_are_refused() {
    let bomb = zip_with(&[("carry.json", b"{}".to_vec(), false), ("files/own/x", vec![0u8; 8 << 20], false)]);
    assert!(matches!(open_bytes(bomb.clone(), None, Limits::default()), Err(Error::TooLarge(_))));
    let small = Limits { max_total: 1 << 20, max_ratio: u64::MAX, ..Limits::default() };
    assert!(matches!(open_bytes(bomb, None, small), Err(Error::TooLarge(_))));
    let many = zip_with(&[
        ("carry.json", b"{}".to_vec(), false),
        ("files/a", vec![1], false),
        ("files/b", vec![1], false),
    ]);
    let few = Limits { max_entries: 2, ..Limits::default() };
    assert!(matches!(open_bytes(many, None, few), Err(Error::TooLarge(_))));
}

#[test]
fn a_huge_manifest_is_refused() {
    let big = zip_with(&[("carry.json", vec![b' '; 4 << 20], false)]);
    let limits = Limits { max_json: 1 << 20, ..Limits::default() };
    assert!(matches!(open_bytes(big, None, limits), Err(Error::TooLarge(_))));
}

#[test]
fn newer_formats_give_a_clear_error() {
    let bytes = zip_with(&[("carry.json", br#"{"format": 9, "future": true}"#.to_vec(), false)]);
    match open_bytes(bytes, None, Limits::default()) {
        Err(e @ Error::Future { found: 9, known: 2 }) => assert!(e.to_string().contains("newer")),
        other => panic!("{:?}", other.err()),
    }
}

#[test]
fn the_old_format_is_refused_naming_the_command() {
    let bytes = zip_with(&[("carry.json", br#"{"format": 1}"#.to_vec(), false)]);
    match open_bytes(bytes, None, Limits::default()) {
        Err(e @ Error::Old { found: 1, known: 2 }) => {
            assert!(e.to_string().contains("reiwand-upgrade export"), "{e}")
        }
        other => panic!("{:?}", other.err()),
    }
}

#[test]
fn not_a_zip_and_missing_files() {
    assert!(open_bytes(b"hello".to_vec(), None, Limits::default()).is_err());
    assert!(open_bytes(Vec::new(), None, Limits::default()).is_err());
    let bytes = zip_with(&[("other", vec![1], false)]);
    assert!(open_bytes(bytes, None, Limits::default()).is_err());
}

#[test]
fn a_changed_sum_is_found() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("w.bin");
    std::fs::write(&file, b"AAAAAAAAAAAAAAAA").unwrap();
    let own = vec![(OwnFile { id: "w".into(), name: "W".into() }, file)];
    let bytes = write_bytes(&export(content(), own, None)).unwrap();
    let broken = tweak(&bytes, b"AAAAAAAAAAAAAAAA", b"AAAAAAAAAAAAAAAB");
    let mut opened = open_bytes(broken, None, Limits::default());
    if let Ok(opened) = opened.as_mut() {
        assert!(matches!(opened.extract_own("w", &mut Vec::new()), Err(Error::Sum(_))));
    }
}

#[test]
fn unsafe_ids_are_refused_on_extract_and_export() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("w.bin");
    std::fs::write(&file, b"x").unwrap();
    let own = vec![(OwnFile { id: "../x".into(), name: "W".into() }, file)];
    assert!(write_bytes(&export(content(), own, None)).is_err());
}
