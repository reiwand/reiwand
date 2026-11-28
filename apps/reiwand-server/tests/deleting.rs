mod common;

use common::{Server, Who, package, start, upload};
use reqwest::{Method, StatusCode};
use serde_json::{Value, json};
use std::fs;

async fn page(server: &Server, path: &str, who: &Who) -> String {
    server.call(Method::GET, path, who).send().await.unwrap().text().await.unwrap()
}

async fn audit(server: &Server, admin: &Who) -> Value {
    server.json(Method::GET, "/api/v1/mod/audit", admin, None).await.1
}

fn catalog(server: &Server) -> String {
    fs::read_to_string(server.dir.path().join("data/repos/main/catalog.json")).unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn each_role_sees_only_the_tools_it_may_use_on_the_package_page() {
    let server = start(&[("REIWAND_AUTO_PUBLISH", "all")]).await;
    reiwand_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    let (ana, ben) = (server.user(&admin, "ana", "user").await, server.user(&admin, "ben", "user").await);
    let mia = server.user(&admin, "mia", "moderator").await;
    upload(&server, &ana, &package(server.dir.path(), "org.example.dawn", "1.0.0")).await;
    let path = "/r/main/packages/org.example.dawn";
    let tools = [
        "Withdraw this wallpaper for good",
        "Withdraw from the catalog",
        "Withdraw from the repository",
        "Delete this wallpaper",
    ];
    for (who, shown) in [
        (&Who::Nobody, [false; 4]),
        (&ben, [false; 4]),
        (&ana, [true, false, false, false]),
        (&mia, [false, true, true, false]),
        (&admin, [false, true, true, true]),
    ] {
        let html = page(&server, path, who).await;
        let seen: Vec<bool> = tools.iter().map(|t| html.contains(t)).collect();
        assert_eq!(seen, shown, "{html}");
    }
    let asked = page(&server, &format!("{path}?ask=withdraw"), &ana).await;
    assert!(
        asked.contains("class=\"confirm\"") && asked.contains("name=\"confirm\" value=\"true\""),
        "{asked}"
    );
    assert!(asked.contains("/api/v1/r/main/me/packages/org.example.dawn?_method=DELETE"));
    let asked = page(&server, &format!("{path}?ask=delete"), &admin).await;
    assert!(asked.contains("/api/v1/r/main/admin/packages/org.example.dawn?_method=DELETE"), "{asked}");
}

#[tokio::test(flavor = "multi_thread")]
async fn authors_withdraw_and_administrators_delete_only_with_a_confirmation() {
    let server = start(&[("REIWAND_AUTO_PUBLISH", "all")]).await;
    reiwand_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    let (ana, ben) = (server.user(&admin, "ana", "user").await, server.user(&admin, "ben", "user").await);
    let mia = server.user(&admin, "mia", "moderator").await;
    upload(&server, &ana, &package(server.dir.path(), "org.example.dawn", "1.0.0")).await;
    upload(&server, &ben, &package(server.dir.path(), "org.example.dusk", "1.0.0")).await;
    let data = server.dir.path().join("data");
    let tree = |id: &str| data.join(format!("repos/main/packages/{id}/1.0.0.wpak"));
    assert!(tree("org.example.dawn").is_file() && tree("org.example.dusk").is_file());
    let mine = "/api/v1/r/main/me/packages/org.example.dawn";
    let sure = Some(json!({"confirm": true}));
    assert_eq!(server.json(Method::DELETE, mine, &ben, sure.clone()).await.0, StatusCode::NOT_FOUND);
    assert_eq!(
        server.json(Method::DELETE, mine, &Who::Nobody, sure.clone()).await.0,
        StatusCode::UNAUTHORIZED
    );
    let (status, refused) = server.json(Method::DELETE, mine, &ana, Some(json!({}))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{refused}");
    let form = format!("{mine}?_method=DELETE&back=/account");
    let cross = server.call(Method::POST, &form, &ana).header("sec-fetch-site", "cross-site");
    assert_eq!(cross.form(&[("confirm", "true")]).send().await.unwrap().status(), StatusCode::FORBIDDEN);
    assert!(tree("org.example.dawn").is_file());
    let done = server.form(&form, &ana, &[("confirm", "true")]).await;
    assert_eq!(done.status(), StatusCode::OK);
    let gone = server.call(Method::GET, "/r/main/packages/org.example.dawn/1.0.0.wpak", &Who::Nobody);
    assert_eq!(gone.send().await.unwrap().status(), StatusCode::GONE);
    let api = "/api/v1/r/main/packages/org.example.dawn";
    assert_eq!(server.json(Method::GET, api, &Who::Nobody, None).await.0, StatusCode::NOT_FOUND);
    assert!(!tree("org.example.dawn").exists() && catalog(&server).contains("Withdrawn by its author."));
    assert_eq!(fs::read_dir(data.join("held")).unwrap().count(), 0);
    let admin_path = "/api/v1/r/main/admin/packages/org.example.dusk";
    for who in [&ben, &mia, &ana] {
        assert_eq!(server.json(Method::DELETE, admin_path, who, sure.clone()).await.0, StatusCode::FORBIDDEN);
    }
    let (status, _) = server.json(Method::DELETE, admin_path, &admin, None).await;
    assert!(status.is_client_error() && tree("org.example.dusk").is_file());
    let (status, deleted) = server.json(Method::DELETE, admin_path, &admin, sure.clone()).await;
    assert_eq!((status, deleted["deleted"].as_bool()), (StatusCode::OK, Some(true)), "{deleted}");
    assert!(!tree("org.example.dusk").exists() && catalog(&server).contains("Deleted by the administrator"));
    assert_eq!(server.json(Method::DELETE, admin_path, &admin, sure).await.0, StatusCode::NOT_FOUND);
    let actions = audit(&server, &admin).await.to_string();
    assert!(actions.contains("\"withdraw\"") && actions.contains("\"delete-package\""), "{actions}");
    let taken = upload(&server, &ben, &package(server.dir.path(), "org.example.dawn", "2.0.0")).await;
    assert_ne!(taken["state"], "published", "{taken}");
    let again = upload(&server, &ana, &package(server.dir.path(), "org.example.dawn", "1.0.1")).await;
    assert_eq!(again["state"], "published", "{again}");
}

#[tokio::test(flavor = "multi_thread")]
async fn moderators_hide_and_remove_from_the_package_page() {
    let server = start(&[("REIWAND_AUTO_PUBLISH", "all")]).await;
    reiwand_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    let (ana, mia) =
        (server.user(&admin, "ana", "user").await, server.user(&admin, "mia", "moderator").await);
    upload(&server, &ana, &package(server.dir.path(), "org.example.dawn", "1.0.0")).await;
    let back = "?back=/r/main/packages/org.example.dawn";
    let hide = format!("/api/v1/r/main/mod/hidden{back}");
    let reason = [("id", "org.example.dawn"), ("reason", "Waiting for the licence.")];
    assert_eq!(server.form(&hide, &ana, &reason).await.status(), StatusCode::FORBIDDEN);
    assert_eq!(server.form(&hide, &mia, &reason).await.status(), StatusCode::OK);
    let html = page(&server, "/r/main/packages/org.example.dawn", &mia).await;
    assert!(html.contains("Return to the catalog") && html.contains("Waiting for the licence."), "{html}");
    let remove = "/api/v1/r/main/mod/removed?back=/r/main/";
    let short = [("id", "org.example.dawn"), ("statement", "short")];
    assert_eq!(server.form(remove, &mia, &short).await.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let statement = [("id", "org.example.dawn"), ("statement", "The licence does not allow sharing.")];
    assert_eq!(server.form(remove, &ana, &statement).await.status(), StatusCode::FORBIDDEN);
    assert_eq!(server.form(remove, &mia, &statement).await.status(), StatusCode::OK);
    let held = fs::read_dir(server.dir.path().join("data/held")).unwrap().count();
    assert!(held >= 1);
    let (_, works) = server.json(Method::GET, "/api/v1/me/packages", &ana, None).await;
    assert_eq!(works["items"][0]["state"], "removed", "{works}");
    assert_eq!(works["items"][0]["statement"], "The licence does not allow sharing.");
    assert_eq!(server.form(remove, &mia, &statement).await.status(), StatusCode::NOT_FOUND);
}

#[tokio::test(flavor = "multi_thread")]
async fn administrators_delete_a_repository_after_confirming_it() {
    let server = start(&[("REIWAND_AUTO_PUBLISH", "all")]).await;
    reiwand_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    let mia = server.user(&admin, "mia", "moderator").await;
    let make = Some(json!({"title": "Second"}));
    let (_, made) = server.json(Method::POST, "/api/v1/admin/repos", &admin, make.clone()).await;
    let first_id = made["repository"]["repo_id"].clone();
    common::upload_to(
        &server,
        &admin,
        &package(server.dir.path(), "org.example.dawn", "1.0.0"),
        Some("second"),
    )
    .await;
    let path = "/api/v1/admin/repos/second";
    let sure = Some(json!({"confirm": true}));
    assert_eq!(server.json(Method::DELETE, path, &mia, sure.clone()).await.0, StatusCode::FORBIDDEN);
    assert!(server.json(Method::DELETE, path, &admin, Some(json!({}))).await.0.is_client_error());
    let listed = page(&server, "/admin/repos", &admin).await;
    assert!(listed.contains("/admin/repos?delete=second#second") && !listed.contains("class=\"confirm\""));
    let asked = page(&server, "/admin/repos?delete=second", &admin).await;
    assert!(
        asked.contains("class=\"confirm\"") && asked.contains("/api/v1/admin/repos/second?_method=DELETE")
    );
    let done = server.form(
        "/api/v1/admin/repos/second?_method=DELETE&back=/admin/repos",
        &admin,
        &[("confirm", "true")],
    );
    assert_eq!(done.await.status(), StatusCode::OK);
    let data = server.dir.path().join("data");
    assert!(!data.join("repos/second").exists() && !data.join("keys/second.online.key").exists());
    assert!(!data.join("keys/second.root.sealed").exists());
    let latest = server.call(Method::GET, "/r/second/entry/latest", &Who::Nobody).send().await.unwrap();
    assert_eq!(latest.status(), StatusCode::NOT_FOUND);
    let (_, found) = server.json(Method::GET, "/.well-known/reiwand-server", &Who::Nobody, None).await;
    assert_eq!(found["repositories"].as_array().unwrap().len(), 1, "{found}");
    let main = server.json(Method::GET, "/api/v1/r/main/catalog", &Who::Nobody, None).await.0;
    assert_eq!(main, StatusCode::OK);
    assert!(audit(&server, &admin).await.to_string().contains("\"repo-delete\""));
    assert_eq!(server.json(Method::DELETE, path, &admin, sure).await.0, StatusCode::NOT_FOUND);
    let (status, again) = server.json(Method::POST, "/api/v1/admin/repos", &admin, make).await;
    assert_eq!(status, StatusCode::CREATED, "{again}");
    assert_ne!(again["repository"]["repo_id"], first_id);
}
