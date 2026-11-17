use reiwand_format::Fetch;
use reiwand_providers::HttpFetch;
use std::io::ErrorKind;
use std::sync::Arc;
use wiremock::matchers::path;
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn authorization(server: &MockServer) -> Vec<Option<String>> {
    let seen = server.received_requests().await.unwrap_or_default();
    let header = |r: &wiremock::Request| {
        r.headers.get("authorization").and_then(|v| v.to_str().ok()).map(String::from)
    };
    seen.iter().map(header).collect()
}

#[tokio::test]
async fn the_token_reaches_only_the_repository_it_belongs_to() {
    let home = MockServer::start().await;
    let mirror = MockServer::start().await;
    let moved = ResponseTemplate::new(302).insert_header("location", format!("{}/r/shelf/data", mirror.uri()));
    Mock::given(path("/r/shelf/data"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"ok"))
        .mount(&mirror)
        .await;
    Mock::given(path("/r/shelf/moved")).respond_with(moved).mount(&home).await;
    Mock::given(path("/r/shelf/data"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"ok"))
        .mount(&home)
        .await;
    Mock::given(path("/r/y/data"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"ok"))
        .mount(&home)
        .await;
    let (base, mirror_base, other) =
        (format!("{}/r/shelf/", home.uri()), format!("{}/r/shelf/", mirror.uri()), format!("{}/r/y/", home.uri()));
    tokio::task::spawn_blocking(move || {
        let fetch = HttpFetch::new(Arc::default()).unwrap().within(&[base.clone(), mirror_base.clone()]);
        let fetch = fetch.account(&base, "secret").unwrap();
        assert!(fetch.get(&base, "data", 100).unwrap().is_some());
        assert!(fetch.get(&base, "moved", 100).unwrap().is_some());
        assert!(fetch.get(&mirror_base, "data", 100).unwrap().is_some());
        assert!(fetch.get(&other, "data", 100).unwrap().is_some());
    })
    .await
    .unwrap();
    let (home_seen, mirror_seen) = (authorization(&home).await, authorization(&mirror).await);
    assert_eq!(home_seen.iter().filter(|h| h.as_deref() == Some("Bearer secret")).count(), 2);
    assert_eq!(home_seen.iter().filter(|h| h.is_none()).count(), 1);
    assert!(mirror_seen.iter().all(Option::is_none), "{mirror_seen:?}");
}

#[tokio::test]
async fn a_private_repository_without_its_token_answers_permission_denied() {
    let server = MockServer::start().await;
    Mock::given(path("/r/shelf/entry.json")).respond_with(ResponseTemplate::new(401)).mount(&server).await;
    let base = format!("{}/r/shelf/", server.uri());
    let kind = tokio::task::spawn_blocking(move || {
        let fetch = HttpFetch::new(Arc::default()).unwrap();
        fetch.get(&base, "entry.json", 100).unwrap_err().kind()
    })
    .await
    .unwrap();
    assert_eq!(kind, ErrorKind::PermissionDenied);
}
