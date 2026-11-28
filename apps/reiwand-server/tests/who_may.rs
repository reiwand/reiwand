mod common;

use common::{Server, Who, start};
use reqwest::{Method, StatusCode};
use serde_json::json;

fn read(server: &Server, sql: &str) -> String {
    let db = rusqlite::Connection::open(server.dir.path().join("data/db/reiwand.sqlite")).unwrap();
    db.query_row(sql, [], |r| r.get(0)).unwrap()
}

fn role(server: &Server, handle: &str) -> String {
    read(server, &format!("SELECT role FROM users WHERE handle = '{handle}'"))
}

fn id(server: &Server, handle: &str) -> String {
    read(server, &format!("SELECT CAST(id AS TEXT) FROM users WHERE handle = '{handle}'"))
}

async fn token(server: &Server, who: &Who, scopes: &str) -> Who {
    let body = json!({"name": "script", "scopes": scopes});
    let (status, made) = server.json(Method::POST, "/api/v1/me/tokens", who, Some(body)).await;
    assert_eq!(status, StatusCode::CREATED, "{made}");
    Who::Bearer(made["token"].as_str().unwrap().into())
}

#[tokio::test]
async fn a_token_reaches_the_web_pages_only_as_far_as_its_scopes() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let boss = server.user(&admin, "boss", "admin").await;
    let _ = server.user(&admin, "judge", "moderator").await;
    let _ = server.user(&admin, "eve", "user").await;
    let (boss_phone, _) = server.device("boss").await;
    let (judge_phone, _) = server.device("judge").await;
    let reader = token(&server, &boss, "catalog:read").await;
    let moderating = token(&server, &boss, "mod").await;
    let eve = id(&server, "eve");
    let path = format!("/admin/people/accounts/{eve}");
    for who in [&boss_phone, &reader, &moderating, &judge_phone] {
        let _ = server.form(&path, who, &[("role", "admin")]).await;
        assert_eq!(role(&server, "eve"), "user");
        let page = server.call(Method::GET, "/admin/people", who).send().await.unwrap();
        assert_ne!(page.status(), StatusCode::OK);
        let page = server.call(Method::GET, "/admin", who).send().await.unwrap();
        assert_ne!(page.status(), StatusCode::OK);
    }
    let page = server.call(Method::GET, "/mod", &judge_phone).send().await.unwrap();
    assert_ne!(page.status(), StatusCode::OK);
    assert_eq!(server.call(Method::GET, "/mod", &moderating).send().await.unwrap().status(), StatusCode::OK);
    let full = token(&server, &boss, "admin").await;
    let _ = server.form(&path, &full, &[("role", "trusted")]).await;
    assert_eq!(role(&server, "eve"), "trusted");
}

#[tokio::test]
async fn roles_below_the_route_are_refused_on_the_api_and_the_web() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let judge = server.user(&admin, "judge", "moderator").await;
    let eve = server.user(&admin, "eve", "user").await;
    let target = id(&server, "eve");
    for (who, path) in [
        (&eve, "/api/v1/mod/queue"),
        (&eve, "/api/v1/mod/audit"),
        (&eve, "/api/v1/admin/users"),
        (&judge, "/api/v1/admin/users"),
        (&judge, "/api/v1/admin/invites"),
        (&judge, "/api/v1/admin/settings"),
    ] {
        assert_eq!(server.json(Method::GET, path, who, None).await.0, StatusCode::FORBIDDEN, "{path}");
    }
    let edit = format!("/api/v1/admin/users/{target}");
    let promote = Some(json!({"role": "admin"}));
    assert_eq!(server.json(Method::PATCH, &edit, &judge, promote).await.0, StatusCode::FORBIDDEN);
    let path = format!("/admin/people/accounts/{target}");
    for who in [&judge, &eve] {
        let _ = server.form(&path, who, &[("role", "admin")]).await;
        let _ = server.form("/admin/people/invites", who, &[("role", "admin")]).await;
    }
    assert_eq!(role(&server, "eve"), "user");
    assert_eq!(read(&server, "SELECT CAST(COUNT(*) AS TEXT) FROM invites WHERE role = 'admin'"), "0");
    let invite = Some(json!({"role": "admin"}));
    let (status, _) = server.json(Method::POST, "/api/v1/admin/invites", &judge, invite).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn one_account_cannot_reach_the_devices_of_another() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let _ = server.user(&admin, "ana", "user").await;
    let bob = server.user(&admin, "bob", "user").await;
    let (_, ana_device) = server.device("ana").await;
    let (bob_phone, _) = server.device("bob").await;
    let gone = format!("/api/v1/me/devices/{ana_device}");
    assert_eq!(server.json(Method::DELETE, &gone, &bob_phone, None).await.0, StatusCode::FORBIDDEN);
    assert_eq!(server.json(Method::DELETE, &gone, &bob, None).await.0, StatusCode::NOT_FOUND);
    let revoke = format!("/api/v1/me/devices/{ana_device}/revoke");
    let password = Some(json!({"password": "a long enough password"}));
    assert_eq!(server.json(Method::POST, &revoke, &bob_phone, password).await.0, StatusCode::NOT_FOUND);
    let kept = format!("SELECT CAST(COUNT(*) AS TEXT) FROM devices WHERE id = '{ana_device}'");
    assert_eq!(read(&server, &kept), "1");
    let (_, mine) = server.json(Method::GET, "/api/v1/me/devices", &bob_phone, None).await;
    assert!(!mine.to_string().contains(&ana_device));
}

#[tokio::test]
async fn revoking_a_device_shares_the_sign_in_limits_and_keeps_the_password_out_of_the_files() {
    let server = start(&[("REIWAND_RATE_LIMITS", "true"), ("REIWAND_LOG_CLIENTS", "full")]).await;
    let admin = server.admin().await;
    let _ = server.user(&admin, "ana", "user").await;
    let (phone, device) = server.device("ana").await;
    let revoke = format!("/api/v1/me/devices/{device}/revoke");
    let (guess, mut statuses) = ("a guessed password that is wrong", Vec::new());
    for body in std::iter::repeat_n(json!({"password": "a guessed password that is wrong"}), 8) {
        statuses.push(server.json(Method::POST, &revoke, &phone, Some(body)).await.0);
    }
    assert!(statuses.contains(&StatusCode::UNAUTHORIZED), "{statuses:?}");
    assert!(statuses.ends_with(&[StatusCode::TOO_MANY_REQUESTS]), "{statuses:?}");
    let body =
        json!({"handle": "ana", "password": "a long enough password", "device": {"platform": "linux"}});
    let (status, _) = server.json(Method::POST, "/api/v1/auth/login", &Who::Nobody, Some(body)).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    let (mut files, mut logged) = (vec![server.dir.path().to_path_buf()], false);
    while let Some(at) = files.pop() {
        for item in std::fs::read_dir(&at).unwrap().flatten() {
            logged |= item.file_name().to_string_lossy().starts_with("clients-");
            match item.file_type().unwrap().is_dir() {
                true => files.push(item.path()),
                false => {
                    assert!(!String::from_utf8_lossy(&std::fs::read(item.path()).unwrap()).contains(guess))
                }
            }
        }
    }
    assert!(logged);
}
