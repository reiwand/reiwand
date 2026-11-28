mod common;

use common::{Who, package, start, upload};
use reiwand_server::{App, Config};
use reqwest::{Method, StatusCode};
use serde_json::{Value, json};
use std::fs;

#[tokio::test(flavor = "multi_thread")]
async fn backup_and_restore_keep_accounts_packages_and_the_signed_index() {
    let server = start(&[("REIWAND_AUTO_PUBLISH", "all")]).await;
    reiwand_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    let status = upload(&server, &ana, &package(server.dir.path(), "org.example.dawn", "1.0.0")).await;
    assert_eq!(status["state"], "published", "{status}");
    let (_, published) = server.json(Method::POST, "/api/v1/admin/repo/publish", &admin, None).await;
    let before = published["entry_version"].as_u64().unwrap();
    let snapshot = reiwand_server::backup(&server.cfg, None).unwrap();
    let manifest: Value = serde_json::from_slice(&fs::read(snapshot.join("manifest.json")).unwrap()).unwrap();
    let files = manifest["files"].as_object().unwrap();
    assert!(
        files.contains_key("reiwand.sqlite")
            && files.contains_key("keys/main.online.key")
            && files.contains_key("keys/main.root.sealed")
            && files.contains_key("keys/seal.key")
            && files.keys().any(|k| k.starts_with("thumbs/"))
    );
    let target = server.dir.path().join("restored");
    let cfg = Config { data: target.clone(), ..server.cfg.clone() };
    reiwand_server::restore(&cfg, &snapshot, false).unwrap();
    assert!(reiwand_server::restore(&cfg, &snapshot, false).is_err());
    let kept = reiwand_server::open_root(&server.cfg.data, "main").unwrap().unwrap();
    assert_eq!(reiwand_server::open_root(&target, "main").unwrap(), Some(kept));
    let app = App::open(cfg.clone()).unwrap();
    assert!(app.publish().await.unwrap().unwrap() > before);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(app.serve(listener));
    let http = reqwest::Client::new();
    let item: Value = http
        .get(format!("{url}/api/v1/r/main/packages/org.example.dawn"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(item["owner"], "ana");
    let login =
        json!({"handle": "ana", "password": "a long enough password", "device": {"platform": "linux"}});
    assert_eq!(
        http.post(format!("{url}/api/v1/auth/login")).json(&login).send().await.unwrap().status(),
        StatusCode::OK
    );
    let latest = http.get(format!("{url}/r/main/entry/latest")).send().await.unwrap();
    assert_eq!(latest.headers()["cache-control"], "no-cache");
    let seen = latest.text().await.unwrap().trim().parse::<u64>().unwrap();
    assert!(seen > before);
    reiwand_server::restore(&cfg, &snapshot, true).unwrap();
    let kept = fs::read_to_string(target.join("repos/main/entry/latest")).unwrap();
    assert!(
        kept.trim() == seen.to_string() && target.join(format!("repos/main/entry/{seen}.json")).is_file()
    );
    let damaged = snapshot.join("reiwand.sqlite");
    fs::write(&damaged, b"broken").unwrap();
    let other = Config { data: server.dir.path().join("other"), ..server.cfg.clone() };
    assert!(reiwand_server::restore(&other, &snapshot, false).unwrap_err().to_string().contains("checksum"));
}

#[tokio::test]
async fn invites_users_and_transparency_need_an_admin() {
    let server = start(&[("REIWAND_REGISTRATION", "closed")]).await;
    let admin = server.admin().await;
    let closed = json!({"handle": "eve", "password": "a long enough password", "invite": "x"});
    assert_eq!(
        server.json(Method::POST, "/api/v1/auth/register", &Who::Nobody, Some(closed)).await.0,
        StatusCode::FORBIDDEN
    );
    let (_, users) = server.json(Method::GET, "/api/v1/admin/users", &admin, None).await;
    let me = users["items"][0]["id"].as_i64().unwrap();
    let own = server
        .call(Method::PATCH, &format!("/api/v1/admin/users/{me}"), &admin)
        .json(&json!({"role": "user"}))
        .send()
        .await
        .unwrap();
    assert_eq!(own.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let csv = server
        .call(Method::GET, "/api/v1/admin/transparency", &admin)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(csv.starts_with("category,name,count"));
    let page = server.call(Method::GET, "/admin", &admin).send().await.unwrap().text().await.unwrap();
    assert!(page.contains("Health") && !page.contains("Index signature"));
    assert_eq!(
        server.call(Method::GET, "/admin", &Who::Nobody).send().await.unwrap().status(),
        StatusCode::SEE_OTHER
    );
    let (status, problem) = server.json(Method::POST, "/api/v1/admin/repo/publish", &admin, None).await;
    assert_eq!((status, problem["code"].as_str()), (StatusCode::CONFLICT, Some("no_repository")));
}
