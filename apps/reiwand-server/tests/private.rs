mod common;

use common::{Server, Who, package, start, upload_to};
use reqwest::{Method, StatusCode};
use serde_json::{Value, json};

const DAWN: &str = "org.example.dawn";

fn id(server: &Server, handle: &str) -> i64 {
    let db = rusqlite::Connection::open(server.dir.path().join("data/db/reiwand.sqlite")).unwrap();
    db.query_row("SELECT id FROM users WHERE handle = ?1", [handle], |r| r.get(0)).unwrap()
}

async fn status(server: &Server, path: &str, who: &Who) -> (StatusCode, Value) {
    server.json(Method::GET, path, who, None).await
}

async fn member(server: &Server, admin: &Who, handle: &str, rights: Value) {
    let path = format!("/api/v1/admin/repos/vault/members/{}", id(server, handle));
    assert_eq!(server.json(Method::PUT, &path, admin, Some(rights)).await.0, StatusCode::OK);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_private_repository_answers_only_its_members() {
    let server = start(&[("REIWAND_AUTO_PUBLISH", "all")]).await;
    reiwand_server::init(&server.cfg, "First", false, false).unwrap();
    let admin = server.admin().await;
    for name in ["ana", "bob", "cyd"] {
        server.user(&admin, name, "user").await;
    }
    let (ana, _) = server.device("ana").await;
    let (bob, _) = server.device("bob").await;
    let (cy, _) = server.device("cyd").await;
    let make = json!({"name": "vault", "title": "Vault"});
    assert_eq!(
        server.json(Method::POST, "/api/v1/admin/repos", &admin, Some(make)).await.0,
        StatusCode::CREATED
    );
    let set = json!({"private": true});
    assert_eq!(
        server.json(Method::PATCH, "/api/v1/admin/repos/vault", &admin, Some(set)).await.0,
        StatusCode::OK
    );
    member(&server, &admin, "ana", json!({"may_read": true})).await;
    member(&server, &admin, "cyd", json!({"may_publish": true})).await;
    let path = format!("/api/v1/admin/repos/vault/members/{}", id(&server, "bob"));
    let denied = server.json(Method::PUT, &path, &ana, Some(json!({"may_read": true}))).await.0;
    assert_eq!(denied, StatusCode::FORBIDDEN);
    let refused = upload_refusal(&server, &ana).await;
    assert_eq!(refused, (StatusCode::FORBIDDEN, "not_allowed".to_string()));
    upload_to(&server, &cy, &package(server.dir.path(), DAWN, "1.0.0"), Some("vault")).await;
    assert_eq!(server.json(Method::POST, "/api/v1/admin/repo/publish", &admin, None).await.0, StatusCode::OK);
    for path in [
        "/r/vault/entry/latest",
        "/api/v1/r/vault/catalog",
        "/api/v1/r/vault/votes?ids=x",
        "/api/v1/r/vault/packages/x",
    ] {
        for who in [&Who::Nobody, &bob] {
            let (code, body) = status(&server, path, who).await;
            assert_eq!(code, StatusCode::UNAUTHORIZED, "{path}");
            if path.contains("catalog") {
                assert_eq!(body["code"], "account_needed");
            }
        }
        for who in [&ana, &cy, &admin] {
            assert_ne!(status(&server, path, who).await.0, StatusCode::UNAUTHORIZED, "{path}");
        }
    }
    let (_, found) = status(&server, "/api/v1/r/vault/catalog", &ana).await;
    assert_eq!(found["items"].as_array().unwrap().len(), 1, "{found}");
    assert_eq!(status(&server, "/r/main/entry/latest", &Who::Nobody).await.0, StatusCode::OK);
    let names = |v: &Value| {
        v["repositories"].as_array().unwrap().iter().map(|r| r["name"].clone()).collect::<Vec<_>>()
    };
    let (_, anonymous) = status(&server, "/.well-known/reiwand-server", &Who::Nobody).await;
    assert_eq!(names(&anonymous), [json!("main")]);
    assert!(!anonymous.to_string().contains("vault"));
    let (_, member_view) = status(&server, "/.well-known/reiwand-server", &ana).await;
    assert_eq!(names(&member_view), [json!("main"), json!("vault")]);
    let (_, one) = status(&server, "/.well-known/reiwand-server?repo=vault", &Who::Nobody).await;
    assert_eq!((one["private"].clone(), one["publishing"].clone()), (json!(true), json!("members")));
    assert!(one.get("url").is_none() && one.get("repo_id").is_none(), "{one}");
    let (_, full) = status(&server, "/.well-known/reiwand-server?repo=/r/vault/", &ana).await;
    assert!(full["url"].as_str().unwrap().contains("/r/vault/") && full["repo_id"].is_string(), "{full}");
    let (_, public) = status(&server, "/.well-known/reiwand-server?repo=main", &Who::Nobody).await;
    assert_eq!((public["private"].clone(), public["publishing"].clone()), (json!(false), json!("open")));
    assert_eq!(
        status(&server, "/.well-known/reiwand-server?repo=nowhere", &Who::Nobody).await.0,
        StatusCode::NOT_FOUND
    );
}

async fn upload_refusal(server: &Server, who: &Who) -> (StatusCode, String) {
    let body = json!({"size": 10, "sha256": "0".repeat(64), "repo": "vault"});
    let (code, found) = server.json(Method::POST, "/api/v1/uploads", who, Some(body)).await;
    (code, found["code"].as_str().unwrap_or_default().to_string())
}

#[tokio::test(flavor = "multi_thread")]
async fn publishing_into_a_public_repository_can_be_limited_to_members() {
    let server = start(&[]).await;
    reiwand_server::init(&server.cfg, "First", false, false).unwrap();
    let admin = server.admin().await;
    server.user(&admin, "ana", "user").await;
    let (ana, _) = server.device("ana").await;
    let body = json!({"size": 10, "sha256": "0".repeat(64), "repo": "main"});
    assert_eq!(
        server.json(Method::POST, "/api/v1/uploads", &ana, Some(body.clone())).await.0,
        StatusCode::CREATED
    );
    let set = json!({"publishing": "members"});
    assert_eq!(
        server.json(Method::PATCH, "/api/v1/admin/repos/main", &admin, Some(set)).await.0,
        StatusCode::OK
    );
    let path = "/api/v1/uploads";
    let (code, found) = server.json(Method::POST, path, &ana, Some(body.clone())).await;
    assert_eq!((code, found["code"].clone()), (StatusCode::FORBIDDEN, json!("not_allowed")));
    assert_eq!(status(&server, "/api/v1/r/main/catalog", &Who::Nobody).await.0, StatusCode::OK);
    let put = format!("/api/v1/admin/repos/main/members/{}", id(&server, "ana"));
    server.json(Method::PUT, &put, &admin, Some(json!({"may_publish": true}))).await;
    assert_eq!(server.json(Method::POST, path, &ana, Some(body)).await.0, StatusCode::CREATED);
}

#[tokio::test(flavor = "multi_thread")]
async fn each_copy_of_a_session_is_revoked_on_its_own() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let session = server.user(&admin, "ana", "user").await;
    let (phone, _) = server.device("ana").await;
    let mut copies = Vec::new();
    for name in ["tablet", "laptop"] {
        let (code, made) =
            server.json(Method::POST, "/api/v1/auth/session/copy", &phone, Some(json!({"name": name}))).await;
        assert_eq!(code, StatusCode::CREATED, "{made}");
        assert!(!made["scopes"].as_str().unwrap().contains("admin"));
        copies.push((made["id"].as_i64().unwrap(), Who::Bearer(made["token"].as_str().unwrap().into())));
    }
    assert_ne!(copies[0].0, copies[1].0);
    assert_eq!(
        server.json(Method::POST, "/api/v1/auth/session/copy", &Who::Nobody, None).await.0,
        StatusCode::UNAUTHORIZED
    );
    for (_, who) in &copies {
        assert_eq!(status(&server, "/api/v1/me", who).await.0, StatusCode::OK);
    }
    let gone = format!("/api/v1/me/tokens/{}", copies[0].0);
    assert_eq!(server.json(Method::DELETE, &gone, &session, None).await.0, StatusCode::NO_CONTENT);
    assert_eq!(status(&server, "/api/v1/me", &copies[0].1).await.0, StatusCode::UNAUTHORIZED);
    assert_eq!(status(&server, "/api/v1/me", &copies[1].1).await.0, StatusCode::OK);
    assert_eq!(status(&server, "/api/v1/me", &phone).await.0, StatusCode::OK);
}

#[tokio::test(flavor = "multi_thread")]
async fn the_people_page_gives_rights_per_repository() {
    let server = start(&[]).await;
    reiwand_server::init(&server.cfg, "First", false, false).unwrap();
    let admin = server.admin().await;
    server.user(&admin, "ana", "user").await;
    let path = format!("/admin/personas/accounts/{}/rights", id(&server, "ana"));
    let done = server.form(&path, &admin, &[("repo_main", "publish")]).await;
    assert_eq!(done.status(), StatusCode::SEE_OTHER);
    let (_, found) = status(&server, "/api/v1/admin/repos/main/members", &admin).await;
    assert_eq!(found["items"][0]["handle"], "ana");
    assert_eq!(found["items"][0]["may_publish"], 1);
    let page =
        server.call(Method::GET, "/admin/personas", &admin).send().await.unwrap().text().await.unwrap();
    assert!(page.contains("repo_main"));
    server.form(&path, &admin, &[("repo_main", "none")]).await;
    let (_, found) = status(&server, "/api/v1/admin/repos/main/members", &admin).await;
    assert!(found["items"].as_array().unwrap().is_empty());
    let page = server.call(Method::GET, "/admin/repos", &admin).send().await.unwrap().text().await.unwrap();
    assert!(page.contains("name=\"private\""));
}
