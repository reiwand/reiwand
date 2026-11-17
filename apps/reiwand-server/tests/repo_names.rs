mod common;

use common::{Who, start};
use reqwest::{Method, StatusCode};
use serde_json::json;

#[tokio::test(flavor = "multi_thread")]
async fn only_admins_see_the_repositories_entry_in_the_header() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    let page = |who: &Who| server.call(Method::GET, "/account", who).send();
    let seen = page(&admin).await.unwrap().text().await.unwrap();
    assert!(seen.contains("href=\"/admin/repos\""));
    let hidden = page(&ana).await.unwrap().text().await.unwrap();
    assert!(!hidden.contains("href=\"/admin/repos\""));
}

#[tokio::test(flavor = "multi_thread")]
async fn the_form_takes_one_name_and_lands_on_the_list_with_the_new_repository_shown() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let listed = server.call(Method::GET, "/admin/repos", &admin).send().await.unwrap().text().await.unwrap();
    assert!(listed.contains("name=\"name\"") && !listed.contains("name=\"title\""), "{listed}");
    let sent = server.form("/admin/repos", &admin, &[("name", "Mis fondos")]).await;
    assert_eq!(sent.status(), StatusCode::SEE_OTHER);
    let to = sent.headers()["location"].to_str().unwrap().to_string();
    assert_eq!(to, "/admin/repos?new=mis-fondos");
    let listed = server.call(Method::GET, &to, &admin).send().await.unwrap().text().await.unwrap();
    assert!(listed.contains("Repository created") && listed.contains("/r/mis-fondos/"), "{listed}");
    assert!(!listed.contains("no backup root key"), "{listed}");
    let second = server.form("/admin/repos", &admin, &[("name", "Mis fondos")]).await;
    assert_eq!(second.headers()["location"], "/admin/repos?new=mis-fondos-2");
    let empty = server.form("/admin/repos", &admin, &[("name", "   ")]).await;
    assert_eq!(empty.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(empty.text().await.unwrap().contains("The repository was not created"));
    let long = "x".repeat(81);
    assert_eq!(
        server.form("/admin/repos", &admin, &[("name", &long)]).await.status(),
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert!(reiwand_server::open_root(&server.cfg.data, "mis-fondos").unwrap().is_some());
}

#[tokio::test(flavor = "multi_thread")]
async fn the_api_derives_the_address_from_the_title_and_never_answers_with_a_secret() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let path = "/api/v1/admin/repos";
    let (status, made) = server.json(Method::POST, path, &admin, Some(json!({"title": "Año Nuevo"}))).await;
    assert_eq!(status, StatusCode::CREATED, "{made}");
    assert_eq!(made["repository"]["name"], "ano-nuevo");
    assert_eq!(made["repository"]["root_key_kept"], true);
    assert!(made.get("root_key").is_none() && !made.to_string().contains("RW"), "{made}");
    let (status, _) = server.json(Method::POST, path, &admin, Some(json!({}))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _) = server.json(Method::POST, path, &admin, Some(json!({"name": "Bad Name"}))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_repository_without_a_kept_root_key_says_so() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    reiwand_server::init(&server.cfg, "Old", false, false).unwrap();
    std::fs::remove_file(server.cfg.data.join("keys/main.root.sealed")).unwrap();
    let page = server.call(Method::GET, "/admin/repos", &admin).send().await.unwrap().text().await.unwrap();
    assert!(page.contains("no backup root key"), "{page}");
}
