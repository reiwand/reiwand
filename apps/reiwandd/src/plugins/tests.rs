use super::{PREFIX, build, install, kind, offer};
use crate::kinds::{Options, effective};
use crate::sync::download;
use reiwand_core::dbus::error::CONFLICT;
use reiwand_plugin::Isolation;
use reiwand_providers::Cache;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

mod server;

const EXAMPLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/authoring/examples/provider-plugin");

fn route(head: &str) -> (u16, String) {
    let photo = |id: u32, file: &str| {
        format!(
            r#"{{"id": {id}, "title": "Ridge {id}", "kind": "photo", "file": "{file}", "link": "https://example.org/{id}", "author": "Ann", "license": "CC-BY-4.0"}}"#
        )
    };
    match head.split(' ').nth(1).unwrap_or_default() {
        "/v1/photos?q=high%20peaks&per_page=30&page=1" => {
            (200, format!(r#"{{"photos": [{}], "next_page": 2}}"#, photo(7, "ridge one.jpg")))
        }
        "/v1/photos?q=high%20peaks&per_page=30&page=2" => (
            200,
            format!(
                r#"{{"photos": [{}, {}], "next_page": null}}"#,
                photo(7, "ridge one.jpg"),
                photo(8, "b.jpg")
            ),
        ),
        "/ridge%20one.jpg" => (200, "first picture".into()),
        "/b.jpg" => (200, "second picture".into()),
        _ => (404, String::new()),
    }
}

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        match entry.file_type().unwrap().is_dir() {
            true => copy(&entry.path(), &to.join(entry.file_name())),
            false => drop(std::fs::copy(entry.path(), to.join(entry.file_name())).unwrap()),
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn a_plugin_installed_with_consent_is_the_provider_of_a_source() {
    reiwand_core::net::set_private(false);
    let (root, server) = (tempfile::tempdir().unwrap(), server::serve(route).await);
    let example = PathBuf::from(EXAMPLE).canonicalize().unwrap();
    let shown = offer(root.path(), &example, Vec::new()).unwrap();
    assert_eq!(shown.plugin.kind, "plugin:example-gallery");
    assert_eq!(
        (shown.plugin.hosts.clone(), shown.added_hosts.clone()),
        (server::HOSTS.map(String::from).to_vec(), server::HOSTS.map(String::from).to_vec())
    );
    let secret = shown.plugin.secret.clone().unwrap();
    assert_eq!(
        (secret.label.as_str(), secret.hosts.as_slice(), secret.required),
        ("API key", &["api.example.org".to_owned()][..], false)
    );
    assert_eq!(shown.installed, None);
    install(root.path(), &example, &shown.token).unwrap();
    let known = kind(root.path(), "example-gallery").unwrap();
    assert_eq!(known.kind, format!("{PREFIX}example-gallery"));
    assert_eq!(known.secret.as_ref().map(|s| s.name.as_str()), Some("API key"));
    let given = Options::from([("query".to_owned(), "high peaks".to_owned())]);
    let options = effective(&known, &given).unwrap();
    assert_eq!(
        (options["per_page"].as_str(), options["videos"].as_str(), options["refresh"].as_str()),
        ("30", "false", "1d")
    );
    let provider = build(
        root.path(),
        "example-gallery",
        &options,
        Some("s3cret"),
        server.reach.clone(),
        Isolation::Inline,
    )
    .unwrap();
    let cache = tempfile::tempdir().unwrap();
    let cache = Cache::new(cache.path(), 1 << 20).unwrap();
    let fetched =
        download(provider.as_ref(), &cache, &HashSet::new(), 1 << 20, |_, _| true).await.unwrap().unwrap();
    assert_eq!((fetched.listed, fetched.downloaded, fetched.failed), (2, 2, 0));
    assert_eq!(std::fs::read_to_string(&fetched.files[0].0).unwrap(), "first picture");
    assert_eq!(fetched.files[1].1.attribution.as_deref(), Some("Ann"));
    let heard = server.heard.lock().unwrap().clone();
    let keyed = |h: &String| h.to_ascii_lowercase().contains("authorization: bearer s3cret");
    assert!(heard.iter().filter(|h| h.contains("/v1/photos")).all(keyed), "{heard:?}");
    assert!(!heard.iter().filter(|h| !h.contains("/v1/photos")).any(keyed), "{heard:?}");
}

#[test]
fn a_plugin_is_installed_only_as_it_was_shown() {
    let (root, folder) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    let plugin = folder.path().join("gallery");
    copy(Path::new(EXAMPLE), &plugin);
    let shown = offer(root.path(), &plugin, Vec::new()).unwrap();
    let refused = install(root.path(), &plugin, "").unwrap_err().parts();
    assert_eq!(refused.0, "org.freedesktop.DBus.Error.InvalidArgs", "{refused:?}");
    install(root.path(), &plugin, &shown.token).unwrap();
    let manifest = plugin.join("reiwand-provider.json");
    let text = std::fs::read_to_string(&manifest).unwrap();
    let wider = text
        .replace("\"1.0.0\"", "\"1.1.0\"")
        .replace("\"images.example.org\"]", "\"images.example.org\", \"cdn.example.org\"]");
    std::fs::write(&manifest, wider).unwrap();
    let (name, message) = install(root.path(), &plugin, &shown.token).unwrap_err().parts();
    assert_eq!(name, CONFLICT, "{message}");
    assert_eq!(kind(root.path(), "example-gallery").unwrap().hosts.len(), 2);
    let again = offer(root.path(), &plugin, vec!["art".into()]).unwrap();
    assert_eq!(
        (again.installed.as_deref(), again.added_hosts.as_slice()),
        (Some("1.0.0"), &["cdn.example.org".to_owned()][..])
    );
    assert!(!again.secret_changed && again.plugin.sources == ["art"]);
    install(root.path(), &plugin, &again.token).unwrap();
    assert_eq!(kind(root.path(), "example-gallery").unwrap().hosts.len(), 3);
    let relative = install(root.path(), Path::new("gallery"), &again.token).unwrap_err().parts();
    assert!(relative.1.contains("absolute"), "{relative:?}");
    assert!(kind(root.path(), "elsewhere").unwrap_err().parts().1.contains("no provider plugin"));
}
