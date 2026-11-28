#![allow(dead_code)]
use reiwand_plugin::{Bundle, Installed, Manifest, Net};
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::rustls::pki_types::{PrivateKeyDer, PrivatePkcs8KeyDer};

pub const HOSTS: [&str; 5] =
    ["api.example.test", "img.example.test", "other.example.test", "api.example.org", "images.example.org"];

pub type Route = fn(&str) -> (u16, Vec<(String, String)>, String);

pub struct Server {
    pub address: SocketAddr,
    pub cert: Vec<u8>,
    pub heard: Arc<Mutex<Vec<String>>>,
}

pub async fn serve(route: Route) -> Server {
    let made = rcgen::generate_simple_self_signed(HOSTS.map(String::from).to_vec()).unwrap();
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(made.signing_key.serialize_der()));
    let chain = vec![made.cert.der().clone()];
    let config = ServerConfig::builder().with_no_client_auth().with_single_cert(chain, key).unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let (address, heard) = (listener.local_addr().unwrap(), Arc::new(Mutex::new(Vec::new())));
    let log = heard.clone();
    tokio::spawn(async move {
        while let Ok((socket, _)) = listener.accept().await {
            let (acceptor, log) = (acceptor.clone(), log.clone());
            tokio::spawn(async move {
                let Ok(mut tls) = acceptor.accept(socket).await else { return };
                let mut asked = Vec::new();
                while !asked.ends_with(b"\r\n\r\n") && tls.read_u8().await.map(|b| asked.push(b)).is_ok() {}
                let head = String::from_utf8_lossy(&asked).into_owned();
                log.lock().unwrap().push(head.clone());
                let (status, headers, body) = route(&head);
                let mut reply =
                    format!("HTTP/1.1 {status} X\r\ncontent-length: {}\r\nconnection: close\r\n", body.len());
                headers.iter().for_each(|(n, v)| reply += &format!("{n}: {v}\r\n"));
                reply += &format!("\r\n{body}");
                drop(tls.write_all(reply.as_bytes()).await);
                drop(tls.shutdown().await);
            });
        }
    });
    Server { address, cert: made.cert.der().to_vec(), heard }
}

impl Server {
    pub fn net(&self, manifest: &Manifest, secret: Option<&str>) -> Arc<Net> {
        let cert = reqwest::Certificate::from_der(&self.cert).unwrap();
        let address = self.address;
        let (manifest, secret) = (manifest.clone(), secret.map(String::from));
        let net = std::thread::spawn(move || {
            Net::with(&manifest, secret.as_deref(), move |builder| {
                HOSTS.iter().fold(builder.tls_certs_only([cert]), |b, host| b.resolve(host, address))
            })
        });
        Arc::new(net.join().unwrap().unwrap())
    }

    pub fn heard(&self) -> Vec<String> {
        self.heard.lock().unwrap().clone()
    }
}

pub fn manifest(hosts: &[&str], secret: &str, settings: &str) -> String {
    let hosts = serde_json::to_string(hosts).unwrap();
    format!(
        r#"{{"schema": "reiwand.provider/1", "id": "sample", "version": "1.0.0", "title": "Sample",
        "license": "CC0-1.0", "hosts": {hosts}, "secret": {secret}, "settings": {settings}}}"#
    )
}

pub fn bundle(manifest: &str, modules: &[(&str, &str)]) -> Bundle {
    let mut files = BTreeMap::new();
    files.insert("reiwand-provider.json".to_owned(), manifest.as_bytes().to_vec());
    modules.iter().for_each(|(n, s)| drop(files.insert(n.to_string(), s.as_bytes().to_vec())));
    Bundle::of(files).unwrap()
}

pub fn installed(manifest: &str, modules: &[(&str, &str)]) -> (tempfile::TempDir, Installed) {
    let root = tempfile::tempdir().unwrap();
    let installed = reiwand_plugin::install(&bundle(manifest, modules), root.path()).unwrap();
    (root, installed)
}
