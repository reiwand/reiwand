mod common;

use common::{HttpFetch, package, start, upload};
use reiwand_format::{Keypair, Signer, add_repo, install, keygen, sign_source, update_repo};
use reqwest::Method;
use serde_json::json;
use std::path::Path;

const DAWN: &str = "org.example.dawn";

fn signed(dir: &Path, version: &str, key: &Keypair) -> std::path::PathBuf {
    let wpak = package(dir, DAWN, version);
    let src = dir.join(format!("src-{DAWN}-{version}"));
    sign_source(&src, &Signer::new(&key.secret, None).unwrap()).unwrap();
    reiwand_format::build(&src, &wpak, &reiwand_format::Limits::default()).unwrap();
    wpak
}

fn later(seconds: i64) -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64 + seconds
}

#[tokio::test(flavor = "multi_thread")]
async fn an_unsigned_update_of_a_signed_online_package_is_never_served() {
    let server = start(&[("REIWAND_AUTO_PUBLISH", "trusted")]).await;
    reiwand_server::init(&server.cfg, "Example wallpapers", false, true).unwrap();
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "trusted").await;
    let key = keygen(None).unwrap();
    server.json(Method::POST, "/api/v1/me/author-keys", &ana, Some(json!({"public_key": key.public}))).await;
    let published = upload(&server, &ana, &signed(server.dir.path(), "1.0.0", &key)).await;
    assert_eq!(published["state"], "published", "{published}");
    server.json(Method::POST, "/api/v1/admin/repo/publish", &admin, None).await;
    let url = reiwand_server::status(&server.cfg).unwrap()["url"].as_str().unwrap().to_string();
    let (data, fresh) = (server.dir.path().join("client"), server.dir.path().join("fresh"));
    let repo_id = {
        let (data, url) = (data.clone(), url.clone());
        let done = tokio::task::spawn_blocking(move || {
            let info = add_repo(&data, &HttpFetch, &url, later(0)).unwrap();
            install(&data, &HttpFetch, DAWN, later(0), false, false).unwrap();
            info.repo_id
        });
        done.await.unwrap()
    };
    let bare = package(server.dir.path(), DAWN, "1.1.0");
    let held = upload(&server, &ana, &bare).await;
    assert_eq!(held["state"], "pending_review", "{held}");
    let repo = server.dir.path().join("data/repos/main");
    let refused = reiwand_format::add(&repo, &bare, None, later(1));
    assert!(matches!(&refused, Err(e) if e.to_string().contains("differs from the author")), "{refused:?}");
    server.json(Method::POST, "/api/v1/admin/repo/publish", &admin, None).await;
    let (kept, first) = tokio::task::spawn_blocking(move || {
        update_repo(&data, &HttpFetch, &repo_id, later(2)).unwrap();
        let kept = install(&data, &HttpFetch, DAWN, later(2), false, false);
        add_repo(&fresh, &HttpFetch, &url, later(2)).unwrap();
        (kept, install(&fresh, &HttpFetch, DAWN, later(2), false, false))
    })
    .await
    .unwrap();
    assert_eq!(kept.unwrap().version, "1.0.0");
    let first = first.unwrap();
    assert_eq!((first.version.as_str(), first.signer), ("1.0.0", Some(key.fingerprint)));
}
