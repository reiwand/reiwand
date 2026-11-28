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

#[test]
fn an_account_is_chosen_by_repository_and_never_by_order() {
    let links = [("https://a.example/r/one/", Some("f1")), ("https://b.example/r/two", None)];
    assert_eq!(super::publish::chosen(&links, "https://b.example/r/two/").unwrap(), 1);
    assert_eq!(super::publish::chosen(&links, "f1").unwrap(), 0);
    let several = super::publish::chosen(&links, "").unwrap_err().to_string();
    assert!(several.contains("https://a.example/r/one/") && several.contains("https://b.example/r/two"));
    assert!(super::publish::chosen(&links, "https://c.example/r/x").is_err());
    assert_eq!(super::publish::chosen(&links[..1], "").unwrap(), 0);
    assert!(super::publish::chosen(&[], "").is_err());
}
