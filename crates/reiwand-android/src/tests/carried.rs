use super::{ask, open};
use crate::{account, library, settings, store};
use reiwand_carry::{Content, Cost, Export, Item, List, Provider};
use serde_json::{Value, json};
use std::path::Path;

fn call(request: Value) -> Value {
    serde_json::from_str(&account::request(&request.to_string())).unwrap()
}

fn id_of(reply: &Value) -> &str {
    reply["id"].as_str().unwrap_or_default()
}

fn count(report: &Value, section: &str) -> u64 {
    report["added"][section].as_u64().unwrap_or(0)
}

fn row<'a>(preview: &'a Value, section: &str) -> &'a Value {
    preview["rows"].as_array().unwrap().iter().find(|r| r["section"] == section).unwrap()
}

fn arrange(green: &str, bands: &str) {
    library::with(|lib| {
        lib.set_favorite(green, true)?;
        lib.set_rating(bands, 4)?;
        lib.retag(&[bands], &["calm"], &[])?;
        let list = lib.create_playlist("Calm mix", None, &Default::default())?;
        lib.add_to_playlist(&list, &[green, bands])?;
        Ok(())
    })
    .unwrap();
    let options = json!({ "url": "https://example.org/dav/", "user": "me" });
    let added = call(
        json!({ "op": "provider_add", "kind": "webdav", "options": options, "secret": "s3cret", "on": false }),
    );
    assert_eq!(added, json!("webdav"));
    settings::setting("transition.kind", Some("fade")).unwrap();
}

fn forget(green: &str, bands: &str) {
    ask(json!({ "op": "remove", "uids": [bands] }));
    library::with(|lib| {
        lib.set_favorite(green, false)?;
        for list in lib.playlists()?.into_iter().filter(|p| p.name == "Calm mix") {
            lib.remove_playlist(&list.uid)?;
        }
        Ok(())
    })
    .unwrap();
    call(json!({ "op": "provider_remove", "id": "webdav", "purge": false }));
    settings::setting("transition.kind", Some("none")).unwrap();
}

fn exported(dir: &Path) -> String {
    let path = dir.join("all.reiwand").display().to_string();
    let bare = json!({ "op": "carry_export", "path": path, "own": true, "secrets": true });
    assert_eq!(id_of(&call(bare)), "android.carry_secrets_password");
    let sealed =
        json!({ "op": "carry_export", "path": path, "own": true, "secrets": true, "password": "pw" });
    let done = call(sealed.clone());
    assert_eq!((done["encrypted"].clone(), done["own"].as_u64() >= Some(2)), (json!(true), true), "{done}");
    assert_eq!(id_of(&call(sealed)), "android.carry_file", "an existing file is never replaced");
    path
}

fn restored(path: &str, bands_before: usize) {
    let missing = call(json!({ "op": "carry_preview", "path": path }));
    assert_eq!(id_of(&missing), "android.carry_needs_password");
    let wrong = call(json!({ "op": "carry_preview", "path": path, "password": "no" }));
    assert_eq!(id_of(&wrong), "android.carry_wrong_password");
    let preview = call(json!({ "op": "carry_preview", "path": path, "password": "pw" }));
    assert_eq!(preview["file"]["encrypted"], json!(true));
    assert!(row(&preview, "providers")["new"].as_u64() >= Some(1), "{preview}");
    assert!(preview["lists"].as_array().unwrap().iter().any(|l| l == "Calm mix"));
    let only = json!({ "op": "carry_import", "path": path, "password": "pw", "take": ["lists", "bogus"] });
    assert_eq!(id_of(&call(only)), "android.carry_choice");
    let report = call(json!({ "op": "carry_import", "path": path, "password": "pw" }));
    assert_eq!(report["pending"], json!([]), "{report}");
    for section in ["providers", "secrets", "lists", "favourites", "ratings", "tags", "settings", "own"] {
        assert!(count(&report, section) >= 1, "{section}: {report}");
    }
    assert_eq!(settings::chosen("transition.kind"), "fade");
    let listed = call(json!({ "op": "providers" }));
    let held = listed["providers"].as_array().unwrap().iter().find(|p| p["id"] == "webdav").unwrap();
    assert_eq!(held["secret"]["held"], json!(true));
    let items = ask(json!({ "op": "items", "query": "" }));
    assert_eq!(items.as_array().unwrap().len(), bands_before);
    let rated = ask(json!({ "op": "items", "query": "tag:calm" }));
    assert_eq!(rated.as_array().map(Vec::len), Some(1));
    let again = call(json!({ "op": "carry_import", "path": path, "password": "pw" }));
    let added = again["added"].as_object().unwrap();
    assert!(added.keys().all(|k| k == "settings"), "a second import adds nothing but settings: {again}");
}

fn foreign(dir: &Path) {
    let item = Item { repo_id: Some("nowhere".into()), id: "org.example.lost".into() };
    let lonely = Provider {
        kind: "webdav".into(),
        name: "far".into(),
        config: json!({ "url": "https://far.example/", "bogus": 5 }),
    };
    let content = Content {
        lists: vec![List { name: "Far".into(), items: vec![item] }],
        providers: vec![lonely],
        ..Content::default()
    };
    let export = Export {
        program: "reiwand".into(),
        version: "1.0.0".into(),
        created: "0".into(),
        content,
        own: Vec::new(),
        password: None,
        cost: Cost::default(),
    };
    let path = dir.join("far.reiwand");
    reiwand_carry::write(&path, &export).unwrap();
    let both = json!({ "op": "carry_import", "path": path, "lists": "both", "suffix": " (copy)" });
    let report = call(both);
    assert_eq!((count(&report, "lists"), report["unresolved"].clone()), (1, json!(1)), "{report}");
    assert_eq!(report["pending"][0]["section"], json!("providers"), "{report}");
}

pub(super) fn the_configuration_travels_in_both_directions() {
    let (green, _) = store::store(&mut open("green-64x32.png"), "green.png").unwrap();
    let (bands, _) = store::store(&mut open("bands-640x180.png"), "Bands.png").unwrap();
    let before = ask(json!({ "op": "items", "query": "" })).as_array().unwrap().len();
    arrange(&green, &bands);
    let dir = tempfile::tempdir().unwrap();
    let path = exported(dir.path());
    forget(&green, &bands);
    restored(&path, before);
    foreign(dir.path());
}
