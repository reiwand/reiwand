mod common;

use common::{Server, Who, package, start, upload_to};
use reqwest::{Method, StatusCode};
use serde_json::json;

const DAWN: &str = "org.example.dawn";
const DUSK: &str = "org.example.dusk";

async fn html(server: &Server, path: &str, who: &Who, lang: Option<&str>) -> (StatusCode, String) {
    let mut request = server.call(Method::GET, path, who);
    if let Some(lang) = lang {
        request = request.header("accept-language", lang);
    }
    let response = request.send().await.unwrap();
    (response.status(), response.text().await.unwrap())
}

async fn setup() -> (Server, Who, Who, Who) {
    let server = start(&[("REIWAND_AUTO_PUBLISH", "all")]).await;
    reiwand_server::init(&server.cfg, "First", false, false).unwrap();
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    let bob = server.user(&admin, "bob", "user").await;
    for (name, private) in [("second", false), ("vault", true)] {
        let make = json!({"name": name, "title": name});
        assert_eq!(
            server.json(Method::POST, "/api/v1/admin/repos", &admin, Some(make)).await.0,
            StatusCode::CREATED
        );
        let set = json!({"private": private});
        let path = format!("/api/v1/admin/repos/{name}");
        assert_eq!(server.json(Method::PATCH, &path, &admin, Some(set)).await.0, StatusCode::OK);
    }
    let db = rusqlite::Connection::open(server.dir.path().join("data/db/reiwand.sqlite")).unwrap();
    let ana_id: i64 = db.query_row("SELECT id FROM users WHERE handle = 'ana'", [], |r| r.get(0)).unwrap();
    let member = json!({"may_read": true});
    let path = format!("/api/v1/admin/repos/vault/members/{ana_id}");
    assert_eq!(server.json(Method::PUT, &path, &admin, Some(member)).await.0, StatusCode::OK);
    upload_to(&server, &admin, &package(server.dir.path(), DAWN, "1.0.0"), Some("main")).await;
    upload_to(&server, &admin, &package(server.dir.path(), DUSK, "1.0.0"), Some("second")).await;
    (server, admin, ana, bob)
}

#[tokio::test(flavor = "multi_thread")]
async fn the_explore_page_lists_public_repositories_with_well_formed_links() {
    let (server, _, _, _) = setup().await;
    for path in ["/"] {
        let (status, page) = html(&server, path, &Who::Nobody, None).await;
        assert_eq!(status, StatusCode::OK, "{path}");
        assert!(page.contains("Explore repositories"));
        for name in ["main", "second"] {
            let address = format!("{}/r/{name}", server.url);
            assert!(page.contains(&format!("<code class=\"dato\">{address}</code>")), "{name}");
            let encoded = address.replace(':', "%3A").replace('/', "%2F").replace('.', "%2E");
            assert!(page.contains(&format!("href=\"reiwand://add?url={encoded}\"")), "{encoded}");
            assert!(page.contains(&format!("href=\"/r/{name}/\"")));
        }
        assert!(page.contains("1 wallpaper") && page.contains("Last update"));
        assert!(!page.contains("vault") && !page.contains("<script>"));
    }
    let (_, spanish) = html(&server, "/", &Who::Nobody, Some("es")).await;
    for word in ["Explorar repositorios", "Abrir en Reiwand", "Copiar la dirección", "Última actualización"]
    {
        assert!(spanish.contains(word), "{word}");
    }
    let (_, found) = server.json(Method::GET, "/.well-known/reiwand-server", &Who::Nobody, None).await;
    assert_eq!(found["repositories_page"], format!("{}/", server.url));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_private_repository_shows_only_to_its_members() {
    let (server, admin, ana, bob) = setup().await;
    for who in [&Who::Nobody, &bob] {
        let (_, page) = html(&server, "/", who, None).await;
        assert!(!page.contains("vault"));
        assert_ne!(html(&server, "/r/vault/", who, None).await.0, StatusCode::OK);
    }
    for who in [&ana, &admin] {
        let (_, page) = html(&server, "/", who, None).await;
        assert!(page.contains("/r/vault") && page.contains("Private"));
        assert_eq!(html(&server, "/r/vault/", who, None).await.0, StatusCode::OK);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn every_repository_has_its_own_catalogue_and_package_pages() {
    let (server, _, _, _) = setup().await;
    let (status, main) = html(&server, "/r/main/", &Who::Nobody, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(main.contains(&format!("href=\"/r/main/packages/{DAWN}\"")) && !main.contains(DUSK));
    let (_, second) = html(&server, "/r/second/", &Who::Nobody, None).await;
    assert!(second.contains(&format!("href=\"/r/second/packages/{DUSK}\"")) && !second.contains(DAWN));
    let (status, package) = html(&server, &format!("/r/second/packages/{DUSK}"), &Who::Nobody, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(package.contains(DUSK) && package.contains("/r/second/packages/"));
    assert_eq!(
        html(&server, &format!("/r/main/packages/{DUSK}"), &Who::Nobody, None).await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        html(&server, &format!("/packages/{DAWN}"), &Who::Nobody, None).await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(html(&server, "/r/nowhere/", &Who::Nobody, None).await.0, StatusCode::NOT_FOUND);
    assert_eq!(html(&server, "/catalog", &Who::Nobody, None).await.0, StatusCode::NOT_FOUND);
}
