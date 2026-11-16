use reiwand_format::{Anchor, DATA_VERSION, Error, check_data};
use std::fs;

#[test]
fn an_empty_folder_is_marked_and_accepted() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path().join("reiwand");
    check_data(&data).unwrap();
    assert_eq!(fs::read_to_string(data.join("format-version")).unwrap(), format!("{DATA_VERSION}\n"));
    check_data(&data).unwrap();
}

#[test]
fn stored_repositories_without_the_mark_are_refused_until_upgraded() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path();
    fs::create_dir_all(data.join("repos/one")).unwrap();
    fs::write(data.join("repos/one/state.json"), "{}").unwrap();
    let refused = check_data(data).unwrap_err();
    assert!(matches!(refused, Error::Upgrade(_)));
    assert!(refused.to_string().contains("reiwand-upgrade client"));
    fs::write(data.join("format-version"), "2\n").unwrap();
    check_data(data).unwrap();
    fs::write(data.join("format-version"), "1\n").unwrap();
    assert!(matches!(check_data(data), Err(Error::Upgrade(_))));
}

#[test]
fn an_address_with_share_parameters_is_refused_with_a_clear_message() {
    for url in ["https://a.example/r/x/?fingerprint=ab", "https://a.example/r/x/?repo_id=cd"] {
        let refused = Anchor::parse(url).unwrap_err().to_string();
        assert!(refused.contains("plain address"), "{refused}");
    }
    assert!(Anchor::parse("https://a.example/r/x/").is_ok());
}
