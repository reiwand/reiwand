use reiwand_account::File;
use reiwand_upgrade::sink::Sink;
use reiwand_upgrade::stored;
use std::fs;

const GRANTS: &str = r#"{"sha256:aa": {"sea": ["audio", "pointer"]}}"#;
const ACCOUNTS: &str = r#"{"accounts": [{"id": "a1", "origin": "https://s.example", "user": "ana",
  "user_id": 4, "label": "", "pending": false, "links": [{"url": "https://s.example/r/main/", "repo_id": null}]}]}"#;

#[test]
fn old_files_are_refused_and_converted_files_load_with_their_content() {
    let dir = tempfile::tempdir().unwrap();
    let (data, copies) = (dir.path().join("data"), dir.path().join("copies"));
    fs::create_dir_all(&data).unwrap();
    fs::write(data.join("inputs.json"), GRANTS).unwrap();
    fs::write(data.join("accounts.json"), ACCOUNTS).unwrap();
    let refused = File::read(&data.join("accounts.json")).unwrap_err().to_string();
    assert!(refused.contains("reiwand-upgrade client"), "{refused}");
    assert_eq!(fs::read_to_string(data.join("accounts.json")).unwrap(), ACCOUNTS);
    assert_eq!(stored::files(&data, &mut Sink::copying(&copies)).unwrap(), 2);
    assert_eq!(fs::read_to_string(copies.join("1-inputs.json")).unwrap(), GRANTS);
    let file = File::read(&data.join("accounts.json")).unwrap();
    assert_eq!((file.accounts[0].user.as_str(), file.accounts[0].user_id), ("ana", 4));
    let grants: std::collections::BTreeMap<String, std::collections::BTreeMap<String, Vec<String>>> =
        reiwand_core::stored::read(&data.join("inputs.json")).unwrap().unwrap();
    assert_eq!(grants["sha256:aa"]["sea"], ["audio", "pointer"]);
}

#[test]
fn converting_twice_changes_nothing() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("inputs.json"), GRANTS).unwrap();
    assert_eq!(stored::files(dir.path(), &mut Sink::copying(&dir.path().join("c"))).unwrap(), 1);
    assert_eq!(stored::files(dir.path(), &mut Sink::copying(&dir.path().join("c"))).unwrap(), 0);
}

#[test]
fn an_account_without_a_value_is_refused_naming_it() {
    let text = r#"{"format": 3, "content": {"accounts": [{"id": "a"}]}}"#;
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("accounts.json"), text).unwrap();
    assert!(File::read(&dir.path().join("accounts.json")).unwrap_err().to_string().contains("missing field"));
}

#[test]
fn the_registry_files_are_converted_and_the_installed_list_names_signer_and_revocation() {
    let dir = tempfile::tempdir().unwrap();
    let (data, copies) = (dir.path(), dir.path().join("copies"));
    let rec = r#"[{"repo_id": "r", "id": "p", "version": "1", "sha256": "s", "size": 3, "path": "/x"}]"#;
    fs::write(data.join("installed.json"), rec).unwrap();
    fs::write(data.join("anchors.json"), "[]").unwrap();
    fs::write(data.join("repo-pins.json"), r#"{"https://s.example": "ab"}"#).unwrap();
    assert!(reiwand_format::installed(data).unwrap_err().to_string().contains("reiwand-upgrade client"));
    assert_eq!(stored::files(data, &mut Sink::copying(&copies)).unwrap(), 3);
    let list = reiwand_format::installed(data).unwrap();
    assert!(list[0].signer.is_none() && list[0].revoked.is_none());
    assert!(reiwand_format::anchors(data).unwrap().is_empty());
    assert_eq!(stored::files(data, &mut Sink::copying(&copies)).unwrap(), 0);
}
