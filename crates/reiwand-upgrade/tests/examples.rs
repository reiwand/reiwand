use reiwand_upgrade::{Outcome, client};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

fn read(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn old_repo(data: &Path) -> String {
    let (repo, keys) = (data.join("examples-repo"), data.join("examples-keys"));
    reiwand_format::bundled(&repo, &keys, "Example wallpapers", &[], 1_700_000_000).unwrap();
    let root = repo.join("root/1.json");
    let mut doc = read(&root);
    doc["policy"].as_object_mut().unwrap().remove("authors").unwrap();
    let bytes = serde_json::to_vec(&doc).unwrap();
    fs::write(&root, &bytes).unwrap();
    reiwand_format::sha256(&bytes[..]).unwrap()
}

#[test]
fn a_repository_of_examples_with_an_old_root_is_made_again_once() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    let old = old_repo(data);
    assert!(reiwand_format::status(&data.join("examples-repo")).is_err());
    fs::write(data.join("examples-repo.json"), json!({"repo_id": old, "declined": true}).to_string())
        .unwrap();
    fs::write(
        data.join("installed.json"),
        json!([{"repo_id": old, "id": "a", "version": "1.0.0", "path": "gone.wpak"}]).to_string(),
    )
    .unwrap();
    fs::create_dir_all(data.join("repos").join(&old)).unwrap();
    client::run(data).unwrap();
    let status = reiwand_format::status(&data.join("examples-repo")).unwrap();
    assert_ne!(status.repo_id, old);
    let record = read(&data.join("examples-repo.json"));
    assert_eq!(record["content"]["repo_id"], status.repo_id);
    assert_eq!(record["content"]["declined"], true);
    let installed = read(&data.join("installed.json"));
    assert_eq!(installed["content"][0]["repo_id"], status.repo_id);
    assert!(!data.join("repos").join(&old).exists());
    assert_eq!(client::run(data).unwrap(), Outcome::Nothing);
    assert_eq!(reiwand_format::status(&data.join("examples-repo")).unwrap().repo_id, status.repo_id);
}

#[test]
fn the_phone_record_stays_plain_and_asks_for_the_examples_again() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    let old = old_repo(data);
    fs::write(
        data.join("examples-repo.json"),
        json!({"repo_id": old, "declined": false, "stamp": 9}).to_string(),
    )
    .unwrap();
    client::run(data).unwrap();
    let record = read(&data.join("examples-repo.json"));
    assert_ne!(record["repo_id"], old);
    assert_eq!(record["stamp"], 0);
    assert!(record.get("content").is_none());
}

#[test]
fn a_record_without_the_format_wrapper_is_converted_and_read_by_the_daemon() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    let file = data.join("examples-repo.json");
    fs::write(&file, json!({"repo_id": "ab", "declined": true}).to_string()).unwrap();
    assert!(reiwand_core::stored::read::<Value>(&file).is_err());
    client::run(data).unwrap();
    let record: Value = reiwand_core::stored::read(&file).unwrap().unwrap();
    assert_eq!(record, json!({"repo_id": "ab", "declined": true}));
    assert_eq!(client::run(data).unwrap(), Outcome::Nothing);
}
