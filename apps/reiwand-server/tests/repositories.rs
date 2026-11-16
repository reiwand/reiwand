mod common;

use common::{HttpFetch, Who, package, start, upload_to};
use reiwand_format::{add_repo, install};
use reqwest::{Method, StatusCode};
use serde_json::json;

const DAWN: &str = "org.example.dawn";
const DUSK: &str = "org.example.dusk";

fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64
}

async fn follow(server: &common::Server, name: &str, url: &str) -> reiwand_format::RepoInfo {
    let client = server.dir.path().join(format!("client-{name}"));
    let url = url.to_string();
    tokio::task::spawn_blocking(move || add_repo(&client, &HttpFetch, &url, now())).await.unwrap().unwrap()
}

async fn fetch(server: &common::Server, path: &str) -> (StatusCode, Vec<u8>) {
    let got = server.call(Method::GET, path, &Who::Nobody).send().await.unwrap();
    (got.status(), got.bytes().await.unwrap().to_vec())
}

#[tokio::test(flavor = "multi_thread")]
async fn two_repositories_have_their_own_keys_catalogue_and_votes() {
    let server = start(&[("REIWAND_AUTO_PUBLISH", "all")]).await;
    let first = reiwand_server::init(&server.cfg, "First", false, false).unwrap();
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    let bad = json!({"name": "Not Valid"});
    assert_eq!(
        server.json(Method::POST, "/api/v1/admin/repos", &admin, Some(bad)).await.0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(server.json(Method::GET, "/api/v1/admin/repos", &ana, None).await.0, StatusCode::FORBIDDEN);
    let make = json!({"name": "second", "title": "Second"});
    let (status, made) = server.json(Method::POST, "/api/v1/admin/repos", &admin, Some(make.clone())).await;
    assert_eq!(status, StatusCode::CREATED, "{made}");
    let second = made["root_key"]["fingerprint"].as_str().unwrap().to_string();
    assert_ne!(second, first.fingerprint);
    assert!(made["root_key"]["secret"].as_str().unwrap().len() > 20);
    assert_eq!(
        server.json(Method::POST, "/api/v1/admin/repos", &admin, Some(make)).await.0,
        StatusCode::CONFLICT
    );
    let (_, listed) = server.json(Method::GET, "/api/v1/admin/repos", &admin, None).await;
    assert_eq!(listed["items"].as_array().unwrap().len(), 2, "{listed}");
    assert!(!listed.to_string().contains("secret"));
    upload_to(&server, &ana, &package(server.dir.path(), DAWN, "1.0.0"), None).await;
    upload_to(&server, &ana, &package(server.dir.path(), DUSK, "1.0.0"), Some("second")).await;
    assert_eq!(server.json(Method::POST, "/api/v1/admin/repo/publish", &admin, None).await.0, StatusCode::OK);
    let (_, found) = server.json(Method::GET, "/.well-known/reiwand-server", &Who::Nobody, None).await;
    assert_eq!(found["repositories"].as_array().unwrap().len(), 2, "{found}");
    assert_eq!(found["repository"]["repo_id"], found["repositories"][0]["repo_id"]);
    assert_eq!(found["repositories"][0]["name"], "main");
    assert_eq!(found["repositories"][1]["name"], "second");
    let one = reiwand_server::status_repo(&server.cfg, "main").unwrap();
    let two = reiwand_server::status_repo(&server.cfg, "second").unwrap();
    assert!(one["url"].as_str().unwrap().contains(&first.fingerprint));
    assert!(two["url"].as_str().unwrap().contains(&second));
    assert!(two["url"].as_str().unwrap().contains("/r/second/"));
    let (a, b) = (
        follow(&server, "main", one["url"].as_str().unwrap()).await,
        follow(&server, "second", two["url"].as_str().unwrap()).await,
    );
    assert_eq!((a.packages, b.packages), (1, 1));
    let client = server.dir.path().join("client-second");
    let (c, d) = (client.clone(), client);
    tokio::task::spawn_blocking(move || install(&c, &HttpFetch, DUSK, now(), false, false))
        .await
        .unwrap()
        .unwrap();
    let missing = tokio::task::spawn_blocking(move || install(&d, &HttpFetch, DAWN, now(), false, false))
        .await
        .unwrap();
    assert!(missing.is_err());
    let wrong = format!(
        "{}?{}",
        one["url"].as_str().unwrap().split('?').next().unwrap().replace("/repo/", "/r/second/"),
        one["query"].as_str().unwrap()
    );
    let client = server.dir.path().join("client-crossed");
    let crossed =
        tokio::task::spawn_blocking(move || add_repo(&client, &HttpFetch, &wrong, now())).await.unwrap();
    assert!(crossed.is_err(), "a key of one repository must not verify the other");
    let (_, main_catalog) = server.json(Method::GET, "/api/v1/catalog", &Who::Nobody, None).await;
    let (_, second_catalog) = server.json(Method::GET, "/api/v1/r/second/catalog", &Who::Nobody, None).await;
    assert_eq!(
        main_catalog["items"].as_array().unwrap().iter().map(|i| i["id"].clone()).collect::<Vec<_>>(),
        [json!(DAWN)]
    );
    assert_eq!(
        second_catalog["items"].as_array().unwrap().iter().map(|i| i["id"].clone()).collect::<Vec<_>>(),
        [json!(DUSK)]
    );
    let (alias, bytes_a) = fetch(&server, "/repo/entry/latest").await;
    let (named, bytes_b) = fetch(&server, "/r/main/entry/latest").await;
    assert_eq!((alias, named), (StatusCode::OK, StatusCode::OK));
    assert_eq!(bytes_a, bytes_b);
    assert_eq!(fetch(&server, "/r/nowhere/entry/latest").await.0, StatusCode::NOT_FOUND);
    let vote = json!({"vote": "up"});
    let path = format!("/api/v1/r/second/packages/{DUSK}/vote");
    assert_eq!(server.json(Method::POST, &path, &ana, Some(vote.clone())).await.0, StatusCode::OK);
    let (_, here) =
        server.json(Method::GET, &format!("/api/v1/r/second/votes?ids={DUSK}"), &Who::Nobody, None).await;
    assert_eq!(here["items"][DUSK], json!({"up": 1, "down": 0}));
    let there = server.json(Method::POST, &format!("/api/v1/packages/{DUSK}/vote"), &ana, Some(vote)).await.0;
    assert_eq!(there, StatusCode::NOT_FOUND);
    let (_, none) = server.json(Method::GET, &format!("/api/v1/votes?ids={DUSK}"), &Who::Nobody, None).await;
    assert!(
        none["items"].get(DUSK).is_none() || none["items"][DUSK] == json!({"up": 0, "down": 0}),
        "{none}"
    );
    let feature = json!({"id": DUSK});
    let (status, _) =
        server.json(Method::POST, "/api/v1/r/second/admin/featured", &admin, Some(feature)).await;
    assert_eq!(status, StatusCode::OK);
    let (_, main_featured) = server.json(Method::GET, "/api/v1/admin/featured", &admin, None).await;
    assert!(!main_featured.to_string().contains(DUSK), "{main_featured}");
}
