use super::fetching::needed_text;
use super::store::{File, Rec, clean};

#[test]
fn addresses_compare_without_query_fragment_or_trailing_slash() {
    assert_eq!(clean("https://h.example/r/a?anchor=1#x"), "https://h.example/r/a/");
    assert_eq!(clean("https://h.example/r/a/"), clean("https://h.example/r/a"));
}

#[test]
fn the_accounts_file_keeps_no_secret_and_reads_back() {
    let rec =
        Rec { id: "a1".into(), origin: "https://h.example".into(), user: "ana".into(), ..Rec::default() };
    let file = File { accounts: vec![rec.clone()] };
    let text = serde_json::to_string(&file).unwrap();
    assert!(!text.contains("token") && !text.contains("secret"));
    let back: File = serde_json::from_str(&text).unwrap();
    assert!(back.accounts == [rec]);
}

#[test]
fn a_private_repository_lists_the_accounts_already_known_on_its_server() {
    let none = needed_text("https://h.example", &[]);
    assert!(none.contains("sign in with an account of h.example"));
    let some = needed_text("https://h.example", &["ana".into(), "bo".into()]);
    assert!(some.contains("ana, bo") && some.contains("another"));
}
