mod common;
use common::*;
use reiwand_android::host;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn writer(dir: &Path) {
    host::prepare(dir.to_str().unwrap()).unwrap();
    let fixtures = Path::new(ROOT).join("../../tests/fixtures");
    let sources: Vec<PathBuf> =
        ["green-64x32.png", "bands-640x180.png", "magenta-64x32.webm", "blink-16x8.gif"]
            .iter()
            .map(|n| fixtures.join(n))
            .collect();
    let mut n = 0u64;
    loop {
        n += 1;
        let _ = host::setting("fit", Some(["fit", "fill", "stretch"][(n % 3) as usize]));
        let _ = host::setting("transition.kind", Some(["none", "fade"][(n % 2) as usize]));
        host::import(sources[(n % 4) as usize].to_str().unwrap());
        host::library(&json!({ "op": "create", "name": format!("list {n}") }).to_string());
    }
}

fn verify(dir: &Path) {
    host::prepare(dir.to_str().unwrap()).unwrap();
    assert!(["fit", "fill", "stretch"].contains(&host::setting("fit", None).as_str()), "fit");
    assert!(["none", "fade"].contains(&host::setting("transition.kind", None).as_str()), "transition");
    let listed = host::library(&json!({ "op": "items", "query": "" }).to_string());
    let listed: Value = serde_json::from_str(&listed).unwrap();
    assert!(listed.is_array(), "{listed}");
    let media = dir.join("media");
    for item in listed.as_array().unwrap() {
        let path = item["path"].as_str().unwrap();
        assert!(Path::new(path).is_file(), "an item lost its file: {item}");
    }
    let stored = fs::read_dir(&media).map_or(0, |d| d.flatten().filter(|e| e.path().is_file()).count());
    let names: Vec<String> = fs::read_dir(&media)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        stored <= listed.as_array().unwrap().len() + 1,
        "orphans after a kill: {stored} files for {} items: {names:?}",
        listed.as_array().unwrap().len()
    );
    let reply = host::import(Path::new(ROOT).join("../../tests/fixtures/blue-8x8.png").to_str().unwrap());
    assert!(!reply.contains("error"), "the library does not work after a kill: {reply}");
}

#[test]
fn killing_the_phone_library_while_it_saves_loses_nothing() {
    if let Ok(dir) = std::env::var("REIWAND_KILL_DIR") {
        let dir = Path::new(&dir);
        return match std::env::var("REIWAND_ROLE").as_deref() {
            Ok("writer") => writer(dir),
            _ => verify(dir),
        };
    }
    let work = tempfile::tempdir().unwrap();
    let files = work.path().join("files");
    fs::create_dir(&files).unwrap();
    let exe = std::env::current_exe().unwrap();
    let run = |role: &str| {
        let mut command = std::process::Command::new(&exe);
        command
            .args(["--exact", "killing_the_phone_library_while_it_saves_loses_nothing"])
            .env("REIWAND_KILL_DIR", &files)
            .env("REIWAND_ROLE", role);
        command
    };
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    for round in 0..12 {
        let mut child = run("writer")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        std::thread::sleep(Duration::from_millis(150 + rng.next() % 600));
        child.kill().unwrap();
        child.wait().unwrap();
        let checked = run("verify").output().unwrap();
        assert!(
            checked.status.success(),
            "round {round}: {}{}",
            String::from_utf8_lossy(&checked.stdout),
            String::from_utf8_lossy(&checked.stderr)
        );
    }
}
