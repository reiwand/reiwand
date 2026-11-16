mod common;

use common::start;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn closed_within(stream: &mut TcpStream, limit: Duration) -> bool {
    let mut buf = [0u8; 256];
    tokio::time::timeout(limit, async {
        loop {
            match stream.read(&mut buf).await {
                Ok(0) | Err(_) => return,
                Ok(_) => {}
            }
        }
    })
    .await
    .is_ok()
}

#[tokio::test]
async fn a_connection_whose_headers_never_finish_is_closed() {
    let server = start(&[]).await;
    let addr = server.url.trim_start_matches("http://").to_string();
    let mut slow = TcpStream::connect(&addr).await.unwrap();
    slow.write_all(b"GET / HTTP/1.1\r\nHost: x\r\n").await.unwrap();
    assert!(closed_within(&mut slow, Duration::from_secs(20)).await);
    let reply = server.http.get(format!("{}/", server.url)).send().await.unwrap();
    assert!(reply.status().as_u16() < 500);
}

#[tokio::test]
async fn a_finished_request_is_still_answered_while_a_slow_one_waits() {
    let server = start(&[]).await;
    let addr = server.url.trim_start_matches("http://").to_string();
    let mut slow = TcpStream::connect(&addr).await.unwrap();
    slow.write_all(b"GET / HTTP/1.1\r\n").await.unwrap();
    let reply = server.http.get(format!("{}/", server.url)).send().await.unwrap();
    assert!(reply.status().as_u16() < 500);
    drop(slow);
}
