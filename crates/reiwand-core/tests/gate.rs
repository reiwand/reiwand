use std::fs;
use std::path::{Path, PathBuf};

const RAW: [&str; 9] = [
    "Client::builder(",
    "Client::new()",
    "ClientBuilder::new(",
    "TcpStream::connect",
    "to_socket_addrs(",
    "lookup_host(",
    "TcpListener::bind(",
    "UdpSocket::bind(",
    "ServiceDaemon::new(",
];
const OUTSIDE: [&str; 3] = ["apps/reiwand-server", "apps/reiwand-web", "crates/reiwand-android"];
const GATE: [&str; 3] = [
    "crates/reiwand-core/src/net.rs",
    "crates/reiwand-core/src/net/gate.rs",
    "crates/reiwand-plugin/src/guard.rs",
];

fn sources(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        match path.is_dir() {
            true if path.file_name().is_some_and(|n| n != "target" && n != "tests") => sources(&path, found),
            false if path.extension().is_some_and(|e| e == "rs") => found.push(path),
            _ => {}
        }
    }
}

#[test]
fn every_network_client_is_built_through_the_gate() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut found = Vec::new();
    ["crates", "apps"].iter().for_each(|top| sources(&root.join(top), &mut found));
    let mut beside = Vec::new();
    for path in found {
        let name = path.strip_prefix(&root).unwrap().to_string_lossy().into_owned();
        if OUTSIDE.iter().any(|o| name.starts_with(o)) || GATE.contains(&name.as_str()) {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap_or_default();
        let lines = text.lines().enumerate().filter(|(_, l)| RAW.iter().any(|r| l.contains(r)));
        let raw = lines.filter(|(_, l)| !l.contains("\"127.0.0.1:0\""));
        beside.extend(raw.map(|(n, l)| format!("{name}:{}: {}", n + 1, l.trim())));
    }
    assert!(beside.is_empty(), "network clients built beside reiwand_core::net:\n{}", beside.join("\n"));
}
