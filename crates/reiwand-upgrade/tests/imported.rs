use reiwand_upgrade::{Outcome, client};
use rusqlite::{Connection, params};
use std::fs;
use std::io::Write;
use std::path::Path;

const MANIFEST: &str = r#"{"schema": "reiwand.wpak/2", "id": "org.example.old", "version": "1.0.0", "title": "old",
"license": "CC0-1.0", "kind": "image", "entry": "image.png", "photosensitive": false,
"content": {"rating": "general"}, "origin": {"kind": "original"}, "previews": {"thumbnail": "preview.png"},
"requires": {"features": []}}"#;

fn package(path: &Path) {
    let mut zip = zip::ZipWriter::new(fs::File::create(path).unwrap());
    let plain = zip::write::SimpleFileOptions::default();
    let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
    png.extend_from_slice(&[0; 32]);
    for (name, bytes) in [
        ("manifest.json", MANIFEST.as_bytes().to_vec()),
        ("image.png", png.clone()),
        ("preview.png", png),
        (reiwand_format::SIGNATURE, b"signature".to_vec()),
    ] {
        zip.start_file(name, plain).unwrap();
        zip.write_all(&bytes).unwrap();
    }
    zip.finish().unwrap();
}

fn library(data: &Path) -> Connection {
    drop(reiwand_library::Library::open(data.join("library.db")).unwrap());
    Connection::open(data.join("library.db")).unwrap()
}

fn item(db: &Connection, kind: &str, path: &Path, source: Option<i64>, error: Option<&str>) {
    let sql = "INSERT INTO items(uid, source_id, kind, path, title, size, mtime, qhash, added_at, error, pkg_read) \
        VALUES (?1, ?2, ?3, ?4, 't', 1, 1, x'00', 1, ?5, 'old')";
    db.execute(sql, params![path.to_string_lossy(), source, kind, path.to_string_lossy(), error]).unwrap();
}

#[test]
fn an_imported_package_is_converted_and_its_row_is_read_again() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::create_dir_all(data.join("files")).unwrap();
    fs::write(data.join("format-version"), "2\n").unwrap();
    let wpak = data.join("files/old.wpak");
    package(&wpak);
    let db = library(data);
    item(&db, "package", &wpak, None, Some("invalid schema: reiwand.wpak/2 is no longer read"));
    drop(db);
    client::run(data).unwrap();
    let text = fs::read(&wpak).unwrap();
    let manifest = reiwand_format::peek(&wpak).unwrap();
    assert_eq!(manifest.id, "org.example.old");
    let db = Connection::open(data.join("library.db")).unwrap();
    let row = |col: &str| -> rusqlite::types::Value {
        db.query_row(&format!("SELECT {col} FROM items"), [], |r| r.get(0)).unwrap()
    };
    assert_eq!(row("size"), rusqlite::types::Value::Integer(text.len() as i64));
    assert_eq!(row("pkg_read"), rusqlite::types::Value::Null);
    let hash = reiwand_library::qhash(&wpak).unwrap();
    assert_eq!(row("qhash"), rusqlite::types::Value::Blob(hash.to_vec()));
    drop(db);
    assert_eq!(client::run(data).unwrap(), Outcome::Nothing);
    assert_eq!(fs::read(&wpak).unwrap(), text);
}

#[test]
fn edits_next_to_provider_images_are_converted_once() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("format-version"), "2\n").unwrap();
    let cache = data.join("cache/providers/one");
    fs::create_dir_all(&cache).unwrap();
    let image = cache.join("a.png");
    fs::write(&image, b"png").unwrap();
    let sidecar = cache.join("a.png.reiwand.json");
    fs::write(&sidecar, r#"{"version": 1}"#).unwrap();
    let db = library(data);
    db.execute("INSERT INTO sources(uid, kind, uri) VALUES ('s', 'provider', 'provider:one')", []).unwrap();
    item(&db, "image", &image, Some(1), None);
    drop(db);
    client::run(data).unwrap();
    let doc: serde_json::Value = serde_json::from_slice(&fs::read(&sidecar).unwrap()).unwrap();
    assert_eq!(doc["version"], 3);
    let bytes = fs::read(&sidecar).unwrap();
    assert_eq!(client::run(data).unwrap(), Outcome::Nothing);
    assert_eq!(fs::read(&sidecar).unwrap(), bytes);
}
