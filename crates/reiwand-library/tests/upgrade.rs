use reiwand_library::{Error, Library, Rotation, Rules};
use reiwand_schedule::{Order, PlaylistState};
use rusqlite::Connection;
use serde_json::json;

fn old_database(path: &std::path::Path) {
    let mut library = Library::open(path).unwrap();
    let list = library.create_playlist("Sea", None, &Rules::default()).unwrap();
    let none = json!({"fit": null, "background": null});
    library.create_preset(None, "Calm", &none).unwrap();
    library
        .create_profile("Work", &json!({}), &json!({}), &json!({"outputs": null, "on_battery": null}), 0)
        .unwrap();
    let state = PlaylistState::new(5);
    library
        .save_rotation(&Rotation {
            output: "DP-1".into(),
            playlist: list,
            state,
            next_at: None,
            current: None,
        })
        .unwrap();
    drop(library);
    let db = Connection::open(path).unwrap();
    db.execute_batch(
        "UPDATE playlists SET rules = '{\"order\":\"shuffle\"}';
         UPDATE presets SET value = '{\"fit\":\"fit\"}';
         UPDATE profiles SET outputs = '{\"DP-1\":{\"playlist\":\"x\"}}', auto = '{\"on_battery\":true}';
         UPDATE rotation SET state = '{\"seed\":5,\"cycle\":0,\"position\":0}';
         PRAGMA user_version = 8;",
    )
    .unwrap();
}

#[test]
fn an_old_database_is_refused_then_converted_over_a_copy_and_opens() {
    let dir = tempfile::tempdir().unwrap();
    let (data, copies) = (dir.path().join("data"), dir.path().join("copies"));
    std::fs::create_dir_all(&data).unwrap();
    old_database(&data.join("library.db"));
    let refused = Library::open(data.join("library.db")).err().unwrap();
    assert!(matches!(refused, Error::Foreign(8)) && refused.to_string().contains("reiwand-upgrade client"));
    assert!(reiwand_upgrade::library::run(&data, &copies).unwrap());
    let kept: u32 = Connection::open(copies.join("library.db"))
        .unwrap()
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap();
    assert_eq!(kept, 8);
    let library = Library::open(data.join("library.db")).unwrap();
    let rules = library.playlists().unwrap().remove(0).rules;
    assert_eq!((rules.order, rules.resume, rules.transition_ms), (Order::Shuffle, true, None));
    let profile = library.profile("Work").unwrap();
    assert_eq!(profile.outputs["DP-1"]["playlist"], "x");
    assert!(profile.outputs["DP-1"]["filter"].is_null() && profile.auto["on_battery"] == true);
    assert_eq!(library.presets(None).unwrap()[0].value, json!({"fit": "fit", "background": null}));
    assert_eq!(library.rotations().unwrap()[0].state.position(), 0);
    assert!(!reiwand_upgrade::library::run(&data, &copies).unwrap());
}
