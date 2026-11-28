mod common;

use common::{Who, start};
use reqwest::Method;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

fn routes() -> Vec<String> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap() {
        let text = std::fs::read_to_string(entry.unwrap().path()).unwrap_or_default();
        for part in text.split(".route(\"").skip(1) {
            if let Some((path, _)) = part.split_once('"') {
                let path = path.replace("{*path}", "a/b").replace("{*rest}", "a/b");
                let mut out = String::new();
                let mut skip = false;
                for c in path.chars() {
                    match c {
                        '{' => {
                            skip = true;
                            out.push('x');
                        }
                        '}' => skip = false,
                        _ if !skip => out.push(c),
                        _ => {}
                    }
                }
                found.push(out);
            }
        }
    }
    found.sort();
    found.dedup();
    found
}

fn bodies() -> Vec<(&'static str, Vec<u8>)> {
    let deep = format!("{}{}", "[".repeat(100_000), "]".repeat(100_000));
    vec![
        ("empty", Vec::new()),
        ("brace", b"{".to_vec()),
        ("null", b"null".to_vec()),
        ("array", b"[]".to_vec()),
        ("bytes", (0..=255u8).cycle().take(4096).collect()),
        ("deep", deep.into_bytes()),
        ("types", br#"{"handle":1,"password":[],"size":-1,"sha256":null,"role":{},"offset":1e999}"#.to_vec()),
        ("big", vec![b'a'; 3 << 20]),
    ]
}

#[tokio::test]
async fn every_route_answers_junk_without_a_server_error() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let (device, _) = {
        server.user(&admin, "ana", "user").await;
        server.device("ana").await
    };
    let methods = [Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE];
    assert!(routes().len() > 50);
    let mut broken = Vec::new();
    for path in routes() {
        for who in [&Who::Nobody, &admin, &device] {
            for method in &methods {
                for (name, body) in bodies() {
                    let request = server.call(method.clone(), &path, who).body(body);
                    let sent = timeout(Duration::from_secs(20), request.send()).await;
                    match sent {
                        Ok(Ok(answer)) if answer.status().as_u16() < 500 => {}
                        other => broken.push(format!(
                            "{method} {path} {name}: {:?}",
                            other.map(|r| r.map(|a| a.status()))
                        )),
                    }
                }
            }
        }
    }
    assert!(broken.is_empty(), "{} failures, first: {:?}", broken.len(), &broken[..broken.len().min(10)]);
    assert_eq!(server.http.get(format!("{}/healthz", server.url)).send().await.unwrap().status(), 200);
}

#[tokio::test]
async fn broken_http_on_the_wire_never_stops_the_server() {
    let server = start(&[]).await;
    let host = server.url.trim_start_matches("http://").to_string();
    let long = format!("GET /{} HTTP/1.1\r\nHost: x\r\n\r\n", "a".repeat(200_000));
    let heads = format!("GET / HTTP/1.1\r\nHost: x\r\n{}\r\n", "X-A: b\r\n".repeat(50_000));
    let cases: Vec<Vec<u8>> = vec![
        b"\x00\x01\x02garbage\r\n\r\n".to_vec(),
        b"GET / HTTP/9.9\r\n\r\n".to_vec(),
        b"POST /api/v1/uploads HTTP/1.1\r\nHost: x\r\nContent-Length: 99999999999999999999\r\n\r\n".to_vec(),
        b"POST /api/v1/uploads HTTP/1.1\r\nHost: x\r\nContent-Length: 10\r\nContent-Length: 11\r\n\r\nabc"
            .to_vec(),
        b"POST /api/v1/uploads HTTP/1.1\r\nHost: x\r\nTransfer-Encoding: chunked\r\n\r\nzz\r\n".to_vec(),
        b"PUT /api/v1/uploads/x HTTP/1.1\r\nHost: x\r\nContent-Length: 1000\r\n\r\nshort".to_vec(),
        b"GET /%ff%fe%00 HTTP/1.1\r\nHost: x\r\nCookie: \xff\xfe\r\n\r\n".to_vec(),
        b"GET / HTTP/1.1\r\nHost: x\r\nAuthorization: Bearer \xff\r\nAccept-Language: \xff\r\n\r\n".to_vec(),
        long.into_bytes(),
        heads.into_bytes(),
    ];
    for case in cases {
        let mut stream = TcpStream::connect(&host).await.unwrap();
        let _ = stream.write_all(&case).await;
        let _ = stream.shutdown().await;
        let mut sink = Vec::new();
        let _ = timeout(Duration::from_secs(10), stream.read_to_end(&mut sink)).await;
        let alive =
            timeout(Duration::from_secs(10), server.http.get(format!("{}/healthz", server.url)).send()).await;
        assert!(alive.is_ok_and(|r| r.is_ok_and(|a| a.status() == 200)));
    }
}

fn parts(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut found = Vec::new();
    let mut todo = vec![root.to_path_buf()];
    while let Some(dir) = todo.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                todo.push(path);
            } else if path.to_string_lossy().contains(".part-") {
                found.push(path);
            }
        }
    }
    found
}

#[tokio::test]
async fn an_upload_cut_short_leaves_no_partial_file_and_can_be_retried() {
    use reqwest::StatusCode;
    use serde_json::json;
    let server = start(&[]).await;
    let admin = server.admin().await;
    server.user(&admin, "ana", "user").await;
    let (device, _) = server.device("ana").await;
    let data: Vec<u8> = (0..400_000u32).map(|n| (n % 251) as u8).collect();
    let sha = reiwand_format::sha256(&data[..]).unwrap();
    let opened = json!({"size": data.len(), "sha256": sha, "repo": "main"});
    let (status, open) = server.json(Method::POST, "/api/v1/uploads", &device, Some(opened)).await;
    assert_eq!(status, StatusCode::CREATED, "{open}");
    let id = open["id"].as_str().unwrap().to_string();
    let Who::Bearer(token) = &device else { unreachable!() };
    let host = server.url.trim_start_matches("http://").to_string();
    let head = format!(
        "PUT /api/v1/uploads/{id} HTTP/1.1\r\nHost: x\r\nAuthorization: Bearer {token}\r\nContent-Length: {}\r\n\r\n",
        data.len()
    );
    let mut stream = TcpStream::connect(&host).await.unwrap();
    stream.write_all(head.as_bytes()).await.unwrap();
    stream.write_all(&data[..150_000]).await.unwrap();
    drop(stream);
    let mut left = Vec::new();
    for _ in 0..50 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        left = parts(server.dir.path());
        if left.is_empty() {
            break;
        }
    }
    assert!(left.is_empty(), "partial files left: {left:?}");
    let path = format!("/api/v1/uploads/{id}");
    let (_, state) = server.json(Method::GET, &path, &device, None).await;
    let retry = server.call(Method::PUT, &path, &device).body(data).send().await.unwrap();
    assert!(
        retry.status() == StatusCode::OK || state["state"] == "failed",
        "after a cut the upload is neither retriable nor reported failed: {state} {}",
        retry.status()
    );
}
