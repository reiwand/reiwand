use reiwand_upgrade::{Outcome, client};
use std::collections::BTreeMap;
use std::fs;

#[test]
fn stored_repositories_lose_the_share_parameters_once() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    let state = |name: &str, url: &str| {
        let folder = data.join("repos").join(name);
        fs::create_dir_all(&folder).unwrap();
        let body = format!(r#"{{"url":"{url}","bases":["https://a.example/repo/"],"repo_id":"{name}"}}"#);
        fs::write(folder.join("state.json"), body).unwrap();
    };
    state("one", "https://a.example/repo/?fingerprint=ab&repo_id=cd");
    state("two", "https://b.example/r/x/");
    fs::write(
        data.join("sync.json"),
        r#"{"url":"https://s.example","repo_url":"https://s.example/repo/?fingerprint=ab","offers":[{"url":"https://c.example/?repo_id=cd"}]}"#,
    )
    .unwrap();
    assert_eq!(client::run(data).unwrap(), Outcome::Upgraded { from: 1, to: 2 });
    let read = |name: &str| {
        let text = fs::read(data.join("repos").join(name).join("state.json")).unwrap();
        serde_json::from_slice::<serde_json::Value>(&text).unwrap()
    };
    assert_eq!(read("one")["url"], "https://a.example/r/main/");
    assert_eq!(read("one")["bases"][0], "https://a.example/r/main/");
    assert_eq!(read("two")["url"], "https://b.example/r/x/");
    assert!(!data.join("sync.json").exists());
    assert_eq!(fs::read_to_string(data.join("format-version")).unwrap(), "2\n");
    assert_eq!(client::run(data).unwrap(), Outcome::Nothing);
}

mod vault;
use vault::Memory;

const OLD: &str = r#"{"url":"https://srv.example:8443","name":"laptop","pin":"AB12CD",
"repo_url":"https://srv.example:8443/r/shelf/","repo_id":"rid","owner":["ana",7],"label":"home",
"offers":[{"url":"https://c.example/"}],"away":[],"files":{},"replica":{"heads":{}}}"#;

#[test]
fn the_old_account_keeps_its_session_in_the_new_store() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("sync.json"), OLD).unwrap();
    let mut vault = Memory::default();
    vault.items.insert("server".into(), r#"{"token":"tok"}"#.into());
    client::run_with(data, &mut vault).unwrap();
    let file: serde_json::Value =
        serde_json::from_slice(&fs::read(data.join("accounts.json")).unwrap()).unwrap();
    let rec = &file["content"]["accounts"][0];
    assert_eq!((rec["user"].as_str(), rec["pending"].as_bool()), (Some("ana"), Some(false)));
    let id = rec["id"].as_str().unwrap();
    assert_eq!(vault.items.get(&format!("account {id}")).map(String::as_str), Some("tok"));
    assert!(!vault.items.contains_key("server") && !data.join("sync.json").exists());
    let pins: BTreeMap<String, String> =
        reiwand_core::stored::read(&data.join("repo-pins.json")).unwrap().unwrap();
    assert_eq!(pins["https://srv.example:8443"], "ab12cd");
}

#[test]
fn a_locked_keyring_leaves_everything_for_the_next_run() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::write(data.join("sync.json"), OLD).unwrap();
    let mut vault = Memory { unreachable: true, ..Memory::default() };
    assert!(client::run_with(data, &mut vault).is_err());
    assert!(data.join("sync.json").exists() && !data.join("format-version").exists());
    vault.unreachable = false;
    assert_eq!(client::run_with(data, &mut vault).unwrap(), Outcome::Upgraded { from: 1, to: 2 });
}
