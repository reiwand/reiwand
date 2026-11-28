use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::rustls::pki_types::{PrivateKeyDer, PrivatePkcs8KeyDer};

pub const HOSTS: [&str; 2] = ["api.example.org", "images.example.org"];

pub type Route = fn(&str) -> (u16, String);

pub struct Server {
    pub reach: reiwand_plugin::Reach,
    pub heard: Arc<Mutex<Vec<String>>>,
}

pub async fn serve(route: Route) -> Server {
    let made = rcgen::generate_simple_self_signed(HOSTS.map(String::from).to_vec()).unwrap();
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(made.signing_key.serialize_der()));
    let config =
        ServerConfig::builder().with_no_client_auth().with_single_cert(vec![made.cert.der().clone()], key);
    let acceptor = TlsAcceptor::from(Arc::new(config.unwrap()));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let (address, heard): (SocketAddr, _) =
        (listener.local_addr().unwrap(), Arc::new(Mutex::new(Vec::new())));
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
                let (status, body) = route(&head);
                let reply = format!(
                    "HTTP/1.1 {status} X\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                    body.len()
                );
                drop(tls.write_all(reply.as_bytes()).await);
                drop(tls.shutdown().await);
            });
        }
    });
    let certificates = vec![reqwest::Certificate::from_der(made.cert.der()).unwrap()];
    let addresses = HOSTS.iter().map(|h| (h.to_string(), address)).collect();
    Server { reach: reiwand_plugin::Reach { certificates, addresses }, heard }
}
