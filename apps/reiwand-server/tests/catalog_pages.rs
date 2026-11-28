mod common;

use common::{Who, package, start, upload};
use reqwest::header::LINK;
use reqwest::{Method, StatusCode};
use serde_json::json;

#[tokio::test(flavor = "multi_thread")]
async fn the_catalog_pages_and_sorts_with_query_keys_that_keep_the_filters() {
    let server = start(&[("REIWAND_AUTO_PUBLISH", "all")]).await;
    reiwand_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    for (id, version) in [("org.example.e", "1.0.0"), ("org.example.d", "1.0.1"), ("org.example.c", "1.0.2")]
    {
        upload(&server, &admin, &package(server.dir.path(), id, version)).await;
    }
    for (id, version) in [("org.example.b", "1.0.3"), ("org.example.a", "1.0.4")] {
        upload(&server, &admin, &package(server.dir.path(), id, version)).await;
    }
    let ids = |v: &serde_json::Value| -> Vec<String> {
        v["items"].as_array().unwrap().iter().map(|i| i["id"].as_str().unwrap().to_string()).collect()
    };
    let (_, newest) = server.json(Method::GET, "/api/v1/r/main/catalog?limit=2", &Who::Nobody, None).await;
    assert_eq!(ids(&newest), ["org.example.a", "org.example.b"]);
    assert_eq!(
        (newest["page"].as_u64(), newest["pages"].as_u64(), newest["total"].as_u64()),
        (Some(1), Some(3), Some(5))
    );
    let response = server
        .call(Method::GET, "/api/v1/r/main/catalog?sort=title&limit=2&page=2", &Who::Nobody)
        .send()
        .await
        .unwrap();
    let link = response.headers().get(LINK).unwrap().to_str().unwrap().to_string();
    assert!(
        link.contains("sort=title&limit=2&page=3>; rel=\"next\"")
            && link.contains("sort=title&limit=2>; rel=\"prev\""),
        "{link}"
    );
    let titled: serde_json::Value = response.json().await.unwrap();
    assert_eq!(ids(&titled), ["org.example.c", "org.example.b"]);
    let vote = server
        .json(Method::POST, "/api/v1/r/main/packages/org.example.d/vote", &admin, Some(json!({"vote": "up"})))
        .await;
    assert_eq!(vote.0, StatusCode::OK);
    let (_, voted) =
        server.json(Method::GET, "/api/v1/r/main/catalog?sort=votes&limit=1", &Who::Nobody, None).await;
    assert_eq!(ids(&voted), ["org.example.d"]);
    let (_, beyond) =
        server.json(Method::GET, "/api/v1/r/main/catalog?limit=2&page=9", &Who::Nobody, None).await;
    assert!(ids(&beyond).is_empty());
    let (status, _) = server.json(Method::GET, "/api/v1/r/main/catalog?sort=price", &Who::Nobody, None).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let html = server
        .call(Method::GET, "/r/main/?sort=title&q=dawn&limit=2&page=2", &Who::Nobody)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    for part in [
        "aria-label=\"Pages\"",
        "aria-current=\"page\"",
        "Page 2 of 3",
        "rel=\"prev\" href=\"/r/main/?q=dawn&amp;sort=title&amp;limit=2\"",
        "rel=\"next\" href=\"/r/main/?q=dawn&amp;sort=title&amp;limit=2&amp;page=3\"",
        "<option value=\"title\" selected>",
    ] {
        assert!(html.contains(part), "{part}\n{html}");
    }
    let spanish = server.call(Method::GET, "/r/main/?limit=2", &Who::Nobody).header("accept-language", "es");
    let spanish = spanish.send().await.unwrap().text().await.unwrap();
    assert!(spanish.contains("Página 1 de 3") && spanish.contains("Siguiente"), "{spanish}");
}
