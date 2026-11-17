use reiwand_carry::Opened;
use reiwand_sync::{Api, Doc, Hlc, Session, SyncKey, encode, hex, seal_segment};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

mod vault;
use vault::Memory;

fn doc(fields: &[(&str, Value)]) -> Doc {
    Doc {
        fields: fields.iter().map(|(k, v)| (k.to_string(), (v.clone(), Hlc(1, 0, "a".into())))).collect(),
        ..Doc::default()
    }
}

struct Old {
    url: String,
    up: Arc<AtomicBool>,
}

fn serve(changes: Vec<Value>, blob: (String, Vec<u8>)) -> Old {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let up = Arc::new(AtomicBool::new(true));
    let alive = up.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let mut stream = stream;
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).unwrap_or(0);
            let head = String::from_utf8_lossy(&buf[..n]).to_string();
            let line = head.lines().next().unwrap_or_default().to_owned();
            let authorised = head.to_ascii_lowercase().contains("bearer tok");
            let (status, body): (&str, Vec<u8>) = if !alive.load(Ordering::SeqCst) || !authorised {
                ("503 Unavailable", b"{}".to_vec())
            } else if line.contains("/api/v1/sync/changes") {
                let page = json!({"seq": 9, "epoch": 3, "more": false, "changes": changes});
                ("200 OK", page.to_string().into_bytes())
            } else if line.contains(&format!("/api/v1/sync/blobs/{}", blob.0)) {
                ("200 OK", blob.1.clone())
            } else {
                ("404 Not Found", b"{}".to_vec())
            };
            let head =
                format!("HTTP/1.1 {status}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", body.len());
            let _ = stream.write_all(head.as_bytes());
            let _ = stream.write_all(&body);
        }
    });
    Old { url, up }
}

fn record(session: &Session, key: &str, doc: &Doc, seq: i64) -> Value {
    let (collection, item) = key.split_once('/').unwrap();
    json!({"collection": collection, "item": item, "seq": seq, "epoch": 3, "payload": session.encode(key, doc).unwrap()})
}

fn setup(data: &Path, up: bool) -> (Old, Memory) {
    let key = SyncKey::generate();
    let ids = key.ids();
    let session = Session::new(
        Api::new("http://127.0.0.1:9").unwrap(),
        7,
        3,
        true,
        SyncKey::from_bytes(&key.bytes()).unwrap(),
    );
    let plain = b"private file bytes".to_vec();
    let hash = hex(&Sha256::digest(&plain));
    let stored = ids.stored(hash.as_bytes());
    let sealed = seal_segment(&ids.file_key(hash.as_bytes()), 0, true, &plain).unwrap();
    let blob = json!({"id": stored, "hash": hash, "size": plain.len(), "name": "notes.png"});
    let (mine, other) = (ids.blob(b"x"), ids.library("r".repeat(64).as_str(), "pack"));
    let list = "p1";
    let changes = vec![
        record(
            &session,
            &format!("library/{mine}"),
            &doc(&[("favorite", json!(true)), ("rating", json!(4)), ("blob", blob)]),
            1,
        ),
        record(&session, &format!("library/{other}"), &doc(&[("favorite", json!(true))]), 2),
        record(
            &session,
            &format!("playlists/{list}"),
            &doc(&[("name", json!("Mine")), ("e:a-0", json!({"p": 1.0, "i": mine}))]),
            3,
        ),
        record(
            &session,
            "repos/z",
            &doc(&[
                ("url", json!("https://x.example/r/main/")),
                ("repo_id", json!("f".repeat(64))),
                ("name", json!("Shelf")),
            ]),
            4,
        ),
    ];
    let old = serve(changes, (ids.stored(hash.as_bytes()), sealed));
    old.up.store(up, Ordering::SeqCst);
    let saved = json!({"url": old.url, "owner": ["ana", 7], "epoch": 3, "encrypted": true, "repo_url": "", "offers": []});
    fs::write(data.join("sync.json"), saved.to_string()).unwrap();
    let secrets = json!({"token": "tok", "key": encode(&key.bytes()), "device": "", "epoch": 3});
    let mut vault = Memory::default();
    vault.items.insert("server".into(), secrets.to_string());
    (old, vault)
}

#[test]
fn the_last_download_becomes_an_export_before_the_old_data_is_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    let (_old, mut vault) = setup(data, true);
    reiwand_upgrade::client::run_with(data, &mut vault).unwrap();
    let opened =
        reiwand_carry::open(&data.join("from-server.reiwand"), None, reiwand_carry::Limits::default())
            .unwrap();
    let Opened { content, .. } = opened;
    assert_eq!(content.lists.len(), 1);
    assert_eq!(content.lists[0].name, "Mine");
    assert_eq!(content.lists[0].items.len(), 1);
    assert_eq!(content.favourites.len(), 1);
    assert_eq!(content.ratings[0].rating, 4);
    assert_eq!(content.repositories[0].name, "Shelf");
    assert_eq!(content.own[0].name, "notes.png");
    assert!(!data.join("sync.json").exists() && !data.join("sync-download").exists());
    assert!(!data.join("sync-download-pending").exists());
}

#[test]
fn an_unreachable_server_keeps_everything_and_retries_on_the_next_run() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    let (old, mut vault) = setup(data, false);
    let failed = reiwand_upgrade::client::run_with(data, &mut vault).unwrap_err().to_string();
    assert!(failed.contains("--skip-download"), "{failed}");
    assert!(data.join("sync.json").exists() && data.join("sync-download-pending").exists());
    assert!(!data.join("format-version").exists() && vault.items.contains_key("server"));
    old.up.store(true, Ordering::SeqCst);
    reiwand_upgrade::client::run_with(data, &mut vault).unwrap();
    assert!(data.join("from-server.reiwand").exists() && !data.join("sync-download-pending").exists());
    assert!(data.join("format-version").exists());
}

#[test]
fn skipping_the_download_goes_on_without_it() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    let (_old, mut vault) = setup(data, false);
    assert!(reiwand_upgrade::client::run_with(data, &mut vault).is_err());
    reiwand_upgrade::client::run_options(data, &mut vault, true).unwrap();
    assert!(!data.join("from-server.reiwand").exists() && !data.join("sync.json").exists());
    assert!(!data.join("sync-download-pending").exists());
}
