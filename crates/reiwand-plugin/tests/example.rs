mod common;
use reiwand_plugin::{Bundle, Isolation, Media, Plugin};
use std::collections::BTreeMap;
use std::path::Path;

const EXAMPLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/authoring/examples/provider-plugin");

fn route(head: &str) -> (u16, Vec<(String, String)>, String) {
    let photos = r#"{"photos": [
        {"id": 7, "title": "Ridge", "kind": "photo", "file": "ridge one.jpg", "link": "https://example.org/7", "author": "Ann", "license": "CC-BY-4.0"},
        {"id": 8, "title": "Falls", "kind": "video", "file": "falls.webm", "link": "http://example.org/8", "author": null, "license": null}
    ], "next_page": 2}"#;
    match head.starts_with("GET /v1/photos?q=high%20peaks&per_page=30&page=1 ") {
        true => (200, Vec::new(), photos.into()),
        false => (404, Vec::new(), String::new()),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn the_authoring_example_installs_and_lists() {
    reiwand_core::net::set_private(false);
    let server = common::serve(route).await;
    let root = tempfile::tempdir().unwrap();
    let installed = reiwand_plugin::install(&Bundle::read(Path::new(EXAMPLE)).unwrap(), root.path()).unwrap();
    assert_eq!(installed.manifest.hosts, ["api.example.org", "images.example.org"]);
    let net = server.net(&installed.manifest, Some("s3cret"));
    let plugin = Plugin::new(installed);
    let given = [("query", "high peaks"), ("per_page", "30"), ("videos", "false")];
    let given: BTreeMap<String, String> = given.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    let settings = plugin.settings(&given).unwrap();
    let page = tokio::task::spawn_blocking(move || plugin.list(net, settings, None, &Isolation::Inline));
    let page = page.await.unwrap().unwrap();
    assert_eq!(page.next.as_deref(), Some("2"));
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].url, "https://images.example.org/ridge%20one.jpg");
    assert_eq!(
        (page.items[0].media, page.items[0].page.as_deref()),
        (Media::Image, Some("https://example.org/7"))
    );
    assert!(server.heard()[0].contains("authorization: Bearer s3cret"), "{:?}", server.heard());
}
