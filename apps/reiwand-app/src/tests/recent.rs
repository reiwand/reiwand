use crate::recentstore::{Known, Store, drop, hide, order, parse, put};
use crate::scenerecent::{full_path, short_path};
use serde_json::json;
use std::path::{Path, PathBuf};

fn folders(store: &Store) -> Vec<String> {
    store.known.iter().map(|k| k.folder.to_string_lossy().into_owned()).collect()
}

#[test]
fn the_last_project_opened_goes_first_and_is_not_repeated() {
    let start = put(put(Store::default(), Path::new("/p/b"), "b"), Path::new("/p/a"), "a");
    let store = put(start, Path::new("/p/b"), "B");
    assert_eq!(folders(&store), ["/p/b", "/p/a"]);
    assert_eq!(store.known[0].title, "B");
}

#[test]
fn the_list_keeps_at_most_two_hundred_projects() {
    let mut store = Store::default();
    for n in 0..250 {
        store = put(store, Path::new(&format!("/p/{n}")), "x");
    }
    assert_eq!(store.known.len(), 200);
    assert_eq!(store.known[0].folder, Path::new("/p/249"));
}

#[test]
fn a_project_left_out_of_the_recents_stays_known_and_comes_back_when_edited() {
    let store = put(Store::default(), Path::new("/p/a"), "a");
    let hidden = hide(store, &[PathBuf::from("/p/a"), PathBuf::from("/p/b")]);
    assert_eq!(folders(&hidden), ["/p/a", "/p/b"]);
    assert!(hidden.known.iter().all(|k| !k.recent));
    let back = put(hidden, Path::new("/p/b"), "b");
    assert!(back.known[0].recent && back.known[0].folder == Path::new("/p/b"));
}

#[test]
fn a_forgotten_project_leaves_the_list_until_it_is_edited_again() {
    let store = drop(put(Store::default(), Path::new("/p/a"), "a"), Path::new("/p/a"));
    assert!(store.known.is_empty() && store.forgotten == [PathBuf::from("/p/a")]);
    let back = put(store, Path::new("/p/a"), "a");
    assert!(back.forgotten.is_empty() && back.known.len() == 1);
}

#[test]
fn a_list_saved_before_the_recents_had_a_switch_reads_as_all_recent() {
    let old = parse(r#"[{"folder": "/p/a", "title": "A"}]"#);
    assert_eq!(old.known, [Known { folder: "/p/a".into(), title: "A".into(), recent: true }]);
    assert_eq!(parse(&serde_json::to_string(&old).unwrap()), old);
    assert_eq!(parse("not json"), Store::default());
}

#[test]
fn present_projects_come_first_and_the_newest_edit_leads() {
    let mut rows = vec![
        json!({"folder": "gone", "missing": true, "edited": 0}),
        json!({"folder": "old", "missing": false, "edited": 10}),
        json!({"folder": "new", "missing": false, "edited": 20}),
    ];
    order(&mut rows);
    let folders: Vec<_> = rows.iter().map(|r| r["folder"].as_str().unwrap()).collect();
    assert_eq!(folders, ["new", "old", "gone"]);
}

#[test]
fn a_path_shows_the_home_folder_as_a_tilde_and_shortens_in_the_middle() {
    assert_eq!(short_path("/h/me/Documents/reiwand/a", "/h/me", 56), "~/Documents/reiwand/a");
    assert_eq!(short_path("/h/meow/a", "/h/me", 56), "/h/meow/a");
    assert_eq!(short_path("/srv/a", "", 56), "/srv/a");
    let long = short_path("/h/me/one/two/three/four/five/six/seven/my-scene", "/h/me/", 20);
    assert_eq!(long.chars().count(), 20);
    assert!(long.starts_with("~/one/") && long.ends_with("my-scene") && long.contains('…'), "{long}");
}

#[test]
fn a_typed_folder_starting_with_a_tilde_lands_in_the_home_folder() {
    assert_eq!(full_path("~/Documents/a", "/h/me/"), "/h/me/Documents/a");
    assert_eq!(full_path("~", "/h/me"), "/h/me");
    assert_eq!(full_path("~other/a", "/h/me"), "~other/a");
    assert_eq!(full_path("/srv/a", "/h/me"), "/srv/a");
    assert_eq!(full_path(&short_path("/h/me/x/y", "/h/me", usize::MAX), "/h/me"), "/h/me/x/y");
}
