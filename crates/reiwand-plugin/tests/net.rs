mod common;
use reiwand_plugin::{Error, Fault, Isolation, Media, Net, Plugin, Request};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

const SECRET: &str = r#"{"label": "API key", "header": "X-Api-Key", "prefix": "Key ", "hosts": ["api.example.test"], "required": true}"#;
const SETTINGS: &str =
    r#"[{"key": "count", "label": "Count", "kind": "int", "prefill": 2, "min": 1, "max": 10, "values": []}]"#;
const LISTING: &str = r#"
interface Photo { id: number; name: string; file: string; by: string | null }
type Args = { settings: { count: number }; page: string | null };
export async function list({ settings, page }: Args) {
  const at = page ?? "1";
  const answer = await fetch(`https://api.example.test/photos?page=${at}&count=${settings.count}`);
  const photos = (await answer.json()) as Photo[];
  const items = photos.map((p: Photo) => ({
    id: String(p.id), title: p.name, media: "image", url: `https://img.example.test/${p.file}`,
    page: `https://example.test/photo/${p.id}`, author: p.by, license: "CC-BY-4.0",
  }));
  const seen = JSON.stringify([answer, Object.getOwnPropertyNames(globalThis), await answer.text()]);
  return { items, next: seen.includes("hunter2") ? "leak" : String(Number(at) + 1) };
}
"#;

fn route(head: &str) -> (u16, Vec<(String, String)>, String) {
    let path = head.split(' ').nth(1).unwrap_or_default();
    let found = |body: &str| (200, vec![("content-type".into(), "application/json".into())], body.to_owned());
    match path {
        p if p.starts_with("/photos") => found(
            r#"[{"id": 1, "name": "One", "file": "1.jpg", "by": "Ann"}, {"id": 2, "name": "Two", "file": "2.jpg", "by": null}]"#,
        ),
        "/away" => {
            (302, vec![("location".into(), "https://other.example.test/stolen".into())], String::new())
        }
        "/hop" => (302, vec![("location".into(), "https://img.example.test/landed".into())], String::new()),
        "/landed" => found("\"landed\""),
        "/huge" => found(&"x".repeat(5 << 20)),
        _ => (404, Vec::new(), String::new()),
    }
}

fn plugin(secret: &str, main: &str) -> (tempfile::TempDir, Plugin) {
    let manifest = common::manifest(&["api.example.test", "img.example.test"], secret, SETTINGS);
    let (root, installed) = common::installed(&manifest, &[("main.ts", main)]);
    (root, Plugin::new(installed))
}

fn ask(url: &str) -> Request {
    Request { url: url.into(), method: "GET".into(), headers: Vec::new(), body: None }
}

#[tokio::test(flavor = "multi_thread")]
async fn a_plugin_lists_items_from_its_hosts_and_never_sees_its_secret() {
    reiwand_core::net::set_private(false);
    let server = common::serve(route).await;
    let (_root, plugin) = plugin(SECRET, LISTING);
    let net = server.net(&plugin.installed.manifest, Some("hunter2"));
    let given = BTreeMap::from([("count".to_owned(), "2".to_owned())]);
    let settings = plugin.settings(&given).unwrap();
    let page = tokio::task::spawn_blocking(move || plugin.list(net, settings, None, &Isolation::Inline));
    let page = page.await.unwrap().unwrap();
    assert_eq!(page.next.as_deref(), Some("2"));
    assert_eq!(page.items.len(), 2);
    let first = &page.items[0];
    assert_eq!((first.id.as_str(), first.title.as_str(), first.media), ("1", "One", Media::Image));
    assert_eq!(first.url, "https://img.example.test/1.jpg");
    assert_eq!((first.author.as_deref(), page.items[1].author.as_deref()), (Some("Ann"), None));
    let heard = server.heard();
    assert!(heard[0].starts_with("GET /photos?page=1&count=2 "), "{heard:?}");
    assert!(heard[0].to_ascii_lowercase().contains("x-api-key: key hunter2"), "{heard:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn fetch_stays_on_the_declared_hosts() {
    reiwand_core::net::set_private(false);
    let server = common::serve(route).await;
    let (_root, plugin) = plugin(SECRET, LISTING);
    let net = server.net(&plugin.installed.manifest, Some("hunter2"));
    let done = tokio::task::spawn_blocking(move || {
        let until = Instant::now() + Duration::from_secs(10);
        let urls = [
            "https://other.example.test/",
            "http://api.example.test/",
            "https://api.example.test:8443/",
            "https://me@api.example.test/",
            "https://api.example.test/away",
            "https://api.example.test/hop",
            "https://api.example.test/huge",
        ];
        urls.map(|url| net.fetch(&ask(url), until, 4 << 20))
    });
    let [other, plain, port, user, away, hop, huge] = done.await.unwrap();
    for refused in [other, plain, port, user, away] {
        assert!(matches!(&refused, Err(Fault::Refused(why)) if why.contains("declares")), "{refused:?}");
    }
    assert_eq!(hop.unwrap().body, "\"landed\"");
    assert!(matches!(&huge, Err(Fault::Limit(_))), "{:?}", huge.map(|a| a.status));
    let heard = server.heard();
    assert!(heard.iter().all(|h| !h.contains("/stolen")), "{heard:?}");
    let landed = heard.iter().find(|h| h.starts_with("GET /landed")).unwrap();
    assert!(!landed.to_ascii_lowercase().contains("hunter2"), "the secret went to another host: {landed}");
}

#[tokio::test(flavor = "multi_thread")]
async fn private_addresses_and_forbidden_headers_are_refused() {
    reiwand_core::net::set_private(false);
    let manifest = common::manifest(&["localhost"], "null", "[]");
    let manifest = common::bundle(&manifest, &[("main.ts", "")]).manifest;
    let done = tokio::task::spawn_blocking(move || {
        let net = Net::new(&manifest, None).unwrap();
        let until = Instant::now() + Duration::from_secs(10);
        let header = Request { headers: vec![("Host".into(), "x".into())], ..ask("https://localhost/") };
        (net.fetch(&ask("https://localhost/"), until, 1024), net.fetch(&header, until, 1024))
    });
    let (private, header) = done.await.unwrap();
    assert!(matches!(&private, Err(Fault::Refused(why)) if why.contains("private")), "{private:?}");
    assert!(matches!(&header, Err(Fault::Refused(why)) if why.contains("header Host")), "{header:?}");
    let (_root, plugin) = plugin(SECRET, LISTING);
    let missing = Net::new(&plugin.installed.manifest, None).err();
    assert!(matches!(&missing, Some(Error::Refused(why)) if why.contains("API key")), "{missing:?}");
}

#[test]
fn a_page_with_media_on_another_host_is_refused() {
    let main = "export const list = async () => ({ items: [{ id: 'a', title: 'A', media: 'image', \
        url: 'https://cdn.elsewhere.test/a.jpg', page: null, author: null, license: null }], next: null });";
    let (_root, plugin) = plugin("null", main);
    let net = std::sync::Arc::new(Net::new(&plugin.installed.manifest, None).unwrap());
    let settings = plugin.settings(&BTreeMap::from([("count".into(), "3".into())])).unwrap();
    let out = plugin.list(net, settings, None, &Isolation::Inline);
    assert!(matches!(&out, Err(Error::Page(why)) if why.contains("items[0].url")), "{out:?}");
    let wrong = plugin.settings(&BTreeMap::from([("count".into(), "30".into())]));
    assert!(matches!(&wrong, Err(Error::Setting(why)) if why.contains("count")), "{wrong:?}");
}
