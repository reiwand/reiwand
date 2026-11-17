mod common;

use common::start;
use reqwest::Method;

#[tokio::test]
async fn the_account_page_counts_only_packages_and_offers_no_synced_storage() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    let page = server.call(Method::GET, "/account", &ana).send().await.unwrap().text().await.unwrap();
    assert!(page.contains("you use 0\u{a0}B of"), "{page}");
    assert!(!page.contains("Synced data") && !page.contains("Private files"));
}
