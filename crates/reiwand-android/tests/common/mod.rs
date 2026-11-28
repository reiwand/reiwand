#![allow(dead_code)]
use reiwand_android::host;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

pub fn items() -> usize {
    let reply = host::library(&json!({ "op": "items", "query": "" }).to_string());
    serde_json::from_str::<Value>(&reply).unwrap().as_array().map_or(0, Vec::len)
}

pub fn leftovers(dir: &Path) -> usize {
    fs::read_dir(dir.join("media")).map_or(0, |d| d.flatten().count())
}

pub fn attempt(file: PathBuf) -> (String, bool) {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let path = file.to_str().unwrap().to_owned();
        let _ = tx.send(std::panic::catch_unwind(|| host::import(&path)).map_err(|_| ()));
    });
    match rx.recv_timeout(Duration::from_secs(60)) {
        Ok(Ok(reply)) => (reply, true),
        Ok(Err(())) => ("panic".into(), false),
        Err(_) => ("hang".into(), false),
    }
}

pub fn pack(name: &str, dir: &Path) -> PathBuf {
    let out = dir.join(format!("{name}.wpak"));
    let source = Path::new(ROOT).join("../../scenes").join(name);
    reiwand_format::build(&source, &out, &reiwand_format::Limits::default()).unwrap();
    out
}
