use super::repository::{operate, serve, uid_of, wpak};
use crate::tests::ask;
use crate::{repos, store};
use reiwand_format::{Settings, Signer, init, keygen, sign_root};
use serde_json::json;
use std::fs;
use std::path::Path;

fn create(repo: &Path, packages: &[&Path]) -> String {
    drop(fs::remove_dir_all(repo));
    let (root, online) = (keygen(None).unwrap(), keygen(Some("example".into())).unwrap());
    let settings = Settings {
        name: "Example".into(),
        root_keys: vec![root.public.clone()],
        online_keys: vec![online.public.clone()],
        ..Settings::default()
    };
    init(repo, &settings, 1_800_000_000).unwrap();
    sign_root(repo, &Signer::new(&root.secret, None).unwrap()).unwrap();
    operate(repo, json!({ "op": "add", "files": packages }));
    online.secret
}

fn publish(repo: &Path, key: &str) {
    operate(repo, json!({ "op": "publish", "key": key, "password": "example", "days": 7 }));
}

pub(super) fn installing_reads_the_repository_as_it_is_now() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path().join("repo");
    let first = wpak(tmp.path(), "org.example.first", "1.0.0", "image", &[]);
    let key = create(&repo, &[&first]);
    publish(&repo, &key);
    let url = format!("http://127.0.0.1:{}/", serve(repo.clone()));
    let repo_id = repos::add(&url, None).unwrap()["repo_id"].as_str().unwrap().to_owned();

    let late = wpak(tmp.path(), "org.example.late", "1.0.0", "image", &[]);
    operate(&repo, json!({ "op": "add", "files": [late] }));
    publish(&repo, &key);
    repos::install(&repo_id, "org.example.late")
        .expect("a package published after the last refresh installs");

    let other = wpak(tmp.path(), "org.example.other", "1.0.0", "image", &[]);
    let key = create(&repo, &[&first, &other]);
    publish(&repo, &key);
    let told = repos::install(&repo_id, "org.example.other").unwrap_err().to_string();
    assert!(told.contains("signature of the repository"), "a repository created again is named: {told}");

    let all: Vec<String> = ["org.example.late"].iter().filter_map(|id| uid_of(id)).collect();
    assert_eq!(ask(json!({ "op": "remove", "uids": all })), serde_json::Value::Null);
    repos::remove(&repo_id).unwrap();
    store::changed();
}
