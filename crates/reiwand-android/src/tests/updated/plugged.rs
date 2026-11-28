use serde_json::{Value, json};
use std::fs;
use std::path::Path;

const KIND: &str = "plugin:example-gallery";

fn account(request: Value) -> Value {
    serde_json::from_str(&crate::account::request(&request.to_string())).unwrap()
}

fn copied() -> tempfile::TempDir {
    let from = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/authoring/examples/provider-plugin");
    let to = tempfile::tempdir().unwrap();
    for name in ["reiwand-provider.json", "main.ts", "lib/shape.ts"] {
        fs::create_dir_all(to.path().join(name).parent().unwrap()).unwrap();
        fs::copy(from.join(name), to.path().join(name)).unwrap();
    }
    to
}

fn path(dir: &tempfile::TempDir) -> String {
    dir.path().to_str().unwrap().to_owned()
}

fn kinds() -> Vec<Value> {
    let listed = account(json!({ "op": "providers" }));
    listed["kinds"].as_array().unwrap().iter().filter(|k| k["kind"] == KIND).cloned().collect()
}

fn source(id: &str) -> Value {
    let listed = account(json!({ "op": "providers" }));
    listed["providers"].as_array().unwrap().iter().find(|p| p["id"] == id).cloned().unwrap_or(Value::Null)
}

fn install(dir: &tempfile::TempDir) -> Value {
    let offer = account(json!({ "op": "plugin_inspect", "path": path(dir) }));
    account(json!({ "op": "plugin_install", "path": path(dir), "token": offer["token"] }))
}

pub(super) fn plugins_add_kinds_of_source_after_a_yes() {
    assert_eq!(account(json!({ "op": "plugins" })), json!([]));
    let plugin = copied();
    let unasked = account(json!({ "op": "plugin_install", "path": path(&plugin), "token": "" }));
    assert_eq!(unasked["id"], json!("android.plugin_changed"), "nothing is installed without the consent");
    let nothing = account(json!({ "op": "plugin_inspect", "path": "/nowhere/at/all" }));
    assert_eq!(nothing["id"], json!("android.plugin_unreadable"));

    let offer = account(json!({ "op": "plugin_inspect", "path": path(&plugin) }));
    assert_eq!(
        (&offer["plugin"]["title"], &offer["installed"], &offer["added_hosts"]),
        (&json!("Example gallery"), &Value::Null, &json!(["api.example.org", "images.example.org"]))
    );
    assert_eq!(
        offer["plugin"]["secret"],
        json!({ "name": "API key", "required": false, "hosts": ["api.example.org"] })
    );
    assert!(kinds().is_empty(), "looking at a plugin installs nothing");
    let token = offer["token"].clone();
    assert_eq!(
        account(json!({ "op": "plugin_install", "path": path(&plugin), "token": token })),
        json!(KIND)
    );

    let kind = &kinds()[0];
    let keys: Vec<&str> =
        kind["options"].as_array().unwrap().iter().map(|o| o["key"].as_str().unwrap()).collect();
    assert_eq!(keys, ["query", "per_page", "videos", "refresh", "keep"]);
    assert_eq!(
        (&kind["title"], &kind["options"][1]["label"]),
        (&json!("Example gallery"), &json!("Photos per page"))
    );
    assert_eq!(
        (&kind["options"][2]["type"], &kind["options"][2]["default"]),
        (&json!("bool"), &json!("false"))
    );

    let add = |options: Value| account(json!({ "op": "provider_add", "kind": KIND, "options": options }));
    assert_eq!(add(json!({ "per_page": "500" }))["id"], json!("android.provider_option"));
    assert_eq!(add(json!({ "nope": "x" }))["id"], json!("android.provider_option_unknown"));
    assert_eq!(
        add(json!({ "query": "lakes" })),
        json!("example-gallery"),
        "the source is named after the plugin"
    );
    let added = source("example-gallery");
    assert_eq!(
        (&added["options"]["query"], &added["options"]["per_page"], &added["options"]["refresh"]),
        (&json!("lakes"), &json!("30"), &json!("1d"))
    );
    assert_eq!(added["hosts"], json!(["api.example.org", "images.example.org"]));
    assert_eq!(added["secret"], json!({ "name": "API key", "required": false, "held": false }));
    let set = json!({ "op": "provider_set", "id": "example-gallery", "key": "videos", "value": "true" });
    assert_eq!(account(set), Value::Null);
    let used = account(json!({ "op": "plugin_remove", "id": "example-gallery" }));
    assert_eq!(used["id"], json!("android.plugin_used"));
    let listed = account(json!({ "op": "plugins" }));
    assert_eq!(
        (&listed[0]["version"], &listed[0]["sources"]),
        (&json!("1.0.0"), &json!(["example-gallery"]))
    );

    let manifest = plugin.path().join("reiwand-provider.json");
    let text = fs::read_to_string(&manifest).unwrap().replace("\"1.0.0\"", "\"1.1.0\"");
    fs::write(
        &manifest,
        text.replace("\"images.example.org\"]", "\"images.example.org\", \"cdn.example.org\"]"),
    )
    .unwrap();
    let stale = account(json!({ "op": "plugin_install", "path": path(&plugin), "token": token }));
    assert_eq!(stale["id"], json!("android.plugin_changed"), "files changed after the consent");
    let update = account(json!({ "op": "plugin_inspect", "path": path(&plugin) }));
    assert_eq!(
        (&update["installed"], &update["added_hosts"], &update["secret_changed"]),
        (&json!("1.0.0"), &json!(["cdn.example.org"]), &json!(false))
    );
    assert_eq!(install(&plugin), json!(KIND));

    let root = crate::store::dir().unwrap().join("plugins");
    fs::write(root.join("example-gallery/source/reiwand-provider.json"), "{").unwrap();
    let broken = source("example-gallery");
    assert_eq!(
        broken["last_error"]["id"],
        json!("android.plugin_missing"),
        "a broken plugin keeps its source listed"
    );
    assert!(kinds().is_empty());
    assert!(account(json!({ "op": "plugins" }))[0]["error"].is_string());
    account(json!({ "op": "provider_remove", "id": "example-gallery", "purge": false }));
    assert_eq!(account(json!({ "op": "plugin_remove", "id": "example-gallery" })), Value::Null);
    assert_eq!(account(json!({ "op": "plugins" })), json!([]));
    assert_eq!(
        account(json!({ "op": "plugin_remove", "id": "example-gallery" }))["id"],
        json!("android.plugin_missing")
    );
}
