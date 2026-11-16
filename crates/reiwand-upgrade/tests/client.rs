use reiwand_upgrade::{Outcome, client};
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
    let sync: serde_json::Value = serde_json::from_slice(&fs::read(data.join("sync.json")).unwrap()).unwrap();
    assert_eq!(sync["repo_url"], "https://s.example/r/main/");
    assert_eq!(sync["offers"][0]["url"], "https://c.example/");
    assert_eq!(fs::read_to_string(data.join("format-version")).unwrap(), "2\n");
    assert_eq!(client::run(data).unwrap(), Outcome::Nothing);
}
