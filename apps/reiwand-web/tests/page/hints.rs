use super::helper::{Page, available, with};
use reiwand_web::{Isolation, Start};
use serde_json::json;
use std::net::TcpListener;
use std::time::{Duration, Instant};

fn listener() -> (TcpListener, String) {
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    server.set_nonblocking(true).unwrap();
    let origin = format!("http://127.0.0.1:{}", server.local_addr().unwrap().port());
    (server, origin)
}

fn accepted(server: &TcpListener) -> usize {
    std::iter::from_fn(|| server.accept().ok()).count()
}

#[test]
fn connection_hints_reach_only_the_granted_origins() {
    if !available() {
        return eprintln!("skipped: no Qt WebEngine 6.5 or newer");
    }
    let dir = tempfile::tempdir().unwrap();
    let (granted, a) = listener();
    let rels = ["preconnect", "dns-prefetch", "preload\" as=\"fetch", "prefetch", "prerender"];
    let others: Vec<(TcpListener, String)> = rels.iter().map(|_| listener()).collect();
    let hints: String = rels
        .iter()
        .zip(&others)
        .map(|(rel, (_, b))| format!("<link rel=\"{rel}\" href=\"{b}/\" crossorigin>"))
        .collect();
    let html = format!(
        "<!doctype html><html><head>{hints}</head><body><script>fetch(\"{a}/own\").catch(() => {{}});\
         console.log(\"probe asked true\");</script></body></html>"
    );
    let start = with(dir.path(), 9, &[("index.html", html.as_bytes())]);
    let start = Start { network: vec![a.clone()], ..start };
    let mut page = Page::open(start, Isolation { network: false, devices: false, limits: false });
    assert_eq!(page.wait("asked", 20), json!(true));
    let (mut own, mut outside) = (0, vec![0; rels.len()]);
    let end = Instant::now() + Duration::from_secs(4);
    while Instant::now() < end {
        page.pump(Instant::now() + Duration::from_millis(100));
        own += accepted(&granted);
        outside.iter_mut().zip(&others).for_each(|(n, (server, _))| *n += accepted(server));
    }
    assert!(own > 0, "the granted origin was not reached");
    let reached: Vec<_> = rels.iter().zip(&outside).filter(|(_, n)| **n > 0).collect();
    assert!(reached.is_empty(), "connection hints reached origins outside the grant: {reached:?}");
}
