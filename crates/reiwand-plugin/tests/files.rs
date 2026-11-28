mod common;
use reiwand_plugin::{Bundle, Error, Files};
use std::path::Path;

const EXAMPLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/authoring/examples/provider-plugin");

fn route(head: &str) -> (u16, Vec<(String, String)>, String) {
    match head.split(' ').nth(1).unwrap_or_default() {
        "/ridge.jpg" => (200, Vec::new(), "picture".into()),
        "/moved.jpg" => {
            (302, vec![("location".into(), "https://other.example.test/x.jpg".into())], String::new())
        }
        "/hop.jpg" => {
            (302, vec![("location".into(), "https://images.example.org/ridge.jpg".into())], String::new())
        }
        _ => (404, Vec::new(), String::new()),
    }
}

fn files(server: &common::Server, secret: Option<&str>) -> Files {
    let manifest = Bundle::read(Path::new(EXAMPLE)).unwrap().manifest;
    let cert = reqwest::Certificate::from_der(&server.cert).unwrap();
    let address = server.address;
    Files::with(&manifest, secret, move |builder| {
        common::HOSTS.iter().fold(builder.tls_certs_only([cert]), |b, host| b.resolve(host, address))
    })
    .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn files_come_only_from_declared_hosts_and_the_secret_only_goes_to_its_hosts() {
    reiwand_core::net::set_private(false);
    let server = common::serve(route).await;
    let files = files(&server, Some("s3cret"));
    let answer = files.get("https://images.example.org/ridge.jpg").unwrap().send().await.unwrap();
    assert_eq!(answer.text().await.unwrap(), "picture");
    assert!(!server.heard()[0].to_ascii_lowercase().contains("s3cret"), "{:?}", server.heard());
    let hopped = files.get("https://api.example.org/hop.jpg").unwrap().send().await.unwrap();
    assert_eq!(hopped.status(), 302, "a redirect off the hosts of the secret is not followed");
    let plain = self::files(&server, None);
    let hopped = plain.get("https://api.example.org/hop.jpg").unwrap().send().await.unwrap();
    assert_eq!(hopped.text().await.unwrap(), "picture");
    let moved = plain.get("https://images.example.org/moved.jpg").unwrap().send().await.unwrap();
    assert_eq!(moved.status(), 302);
    for url in [
        "https://other.example.test/x.jpg",
        "http://images.example.org/x.jpg",
        "https://images.example.org:8443/x",
    ] {
        assert!(matches!(plain.get(url), Err(Error::Refused(_))), "{url}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn files_on_private_addresses_are_refused() {
    reiwand_core::net::set_private(false);
    let manifest =
        common::bundle(&common::manifest(&["localhost"], "null", "[]"), &[("main.ts", "")]).manifest;
    let refused = Files::new(&manifest, None).unwrap().get("https://localhost/x.jpg").unwrap().send().await;
    let chain =
        std::iter::successors(refused.as_ref().err().map(|e| e as &dyn std::error::Error), |e| e.source());
    let said: Vec<String> = chain.map(ToString::to_string).collect();
    assert!(said.iter().any(|s| s.contains("a plugin reaches only public hosts")), "{said:?}");
}
