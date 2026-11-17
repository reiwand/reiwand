use crate::{account, net, repos, settings};
use serde_json::{Value, json};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::time::{Duration, Instant};

fn listener() -> (String, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let seen = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&seen);
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            counted.fetch_add(1, Relaxed);
            drop(stream);
        }
    });
    (url, seen)
}

fn asked(request: Value) -> Value {
    serde_json::from_str(&account::request(&request.to_string())).unwrap()
}

fn every_action(url: &str) -> Vec<Value> {
    let anchored = format!("{url}/");
    let added = repos::add(&anchored).unwrap_or_else(|e| e.reply());
    let offered = asked(json!({ "op": "add", "url": url }));
    let api = net::api(url, None).map_or_else(|e| e.reply(), |_| json!({}));
    let fetched = net::fetch().map(|f| reiwand_format::Fetch::get(&f, url, "index.json", 64).is_err());
    vec![added, offered, api, json!(fetched.unwrap_or(true))]
}

pub(super) fn nothing_leaves_in_private_mode() {
    let (url, seen) = listener();
    assert_eq!(settings::setting(net::PRIVATE, None).unwrap(), "off");
    settings::setting(net::PRIVATE, Some("on")).unwrap();
    let cut = every_action(&url);
    assert!(cut[..3].iter().all(|v| v["id"] == "android.private_mode"), "{cut:?}");
    assert_eq!(cut[3], json!(true));
    assert!(asked(json!({ "op": "refresh_repos", "force": true })).get("error").is_none());
    std::thread::sleep(Duration::from_millis(300));
    assert_eq!(seen.load(Relaxed), 0);
    settings::setting(net::PRIVATE, Some("off")).unwrap();
    drop(every_action(&url));
    let until = Instant::now() + Duration::from_secs(5);
    while seen.load(Relaxed) == 0 && Instant::now() < until {
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(seen.load(Relaxed) > 0);
}
