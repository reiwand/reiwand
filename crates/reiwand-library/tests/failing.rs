use reiwand_library::{Library, Rules};
use rusqlite::Connection;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

const WRITING: &str = "REIWAND_TEST_WRITING";
const FULL: &str = "REIWAND_TEST_FULL";

fn fresh(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    drop(std::fs::remove_dir_all(&dir));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn whole(path: &Path) -> usize {
    let check: String = Connection::open(path)
        .and_then(|raw| raw.query_row("PRAGMA integrity_check", [], |r| r.get(0)))
        .unwrap();
    assert_eq!(check, "ok", "the library database was left damaged");
    let library = Library::open(path).unwrap_or_else(|e| panic!("the library does not open again: {e}"));
    let mut names: Vec<usize> =
        library.playlists().unwrap().iter().map(|p| p.name[1..].parse().unwrap()).collect();
    names.sort_unstable();
    assert!(names.iter().enumerate().all(|(n, name)| n == *name), "a playlist was lost: {names:?}");
    names.len()
}

fn create(library: &mut Library, n: usize) -> reiwand_library::Result<String> {
    library.create_playlist(&format!("P{n}"), Some("kind:image"), &Rules::default())
}

#[test]
fn writing_child() {
    let Some(path) = std::env::var_os(WRITING).map(PathBuf::from) else { return };
    let mut library = Library::open(&path).unwrap();
    let start = library.playlists().unwrap().len();
    for n in start..usize::MAX {
        create(&mut library, n).unwrap();
    }
}

#[test]
fn a_library_killed_while_it_writes_keeps_every_finished_change() {
    let dir = fresh("killed");
    let path = dir.join("data/library.db");
    let mut kept = 0;
    for round in 0..10u64 {
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "writing_child", "--nocapture"])
            .env(WRITING, &path)
            .stdout(Stdio::null())
            .spawn()
            .unwrap();
        std::thread::sleep(Duration::from_millis(60 + round * 11));
        child.kill().unwrap();
        child.wait().unwrap();
        let now = whole(&path);
        assert!(now >= kept, "the library went back from {kept} to {now} playlists");
        kept = now;
    }
    assert!(kept > 0, "the child never wrote");
}

#[test]
fn full_child() {
    let Some(dir) = std::env::var_os(FULL).map(PathBuf::from) else { return };
    let path = dir.join("data/library.db");
    let mut library = Library::open(&path).unwrap();
    let made = (0..100_000).take_while(|n| create(&mut library, *n).is_ok()).count();
    let error = create(&mut library, made).expect_err("the disk never filled");
    assert!(made > 0 && library.playlists().unwrap().len() == made, "{made}: {error}");
    drop(library);
    assert_eq!(whole(&path), made);
    println!("FULL-OK {error}");
}

#[test]
fn a_library_on_a_full_disk_refuses_the_change_and_keeps_the_rest() {
    let dir = fresh("full");
    let script = r#"mount -t tmpfs -o size=400k tmpfs "$1" && exec "$2" --exact full_child --nocapture"#;
    let out = Command::new("unshare")
        .args(["-Urm", "sh", "-c", script, "sh"])
        .arg(&dir)
        .arg(std::env::current_exe().unwrap())
        .env(FULL, &dir)
        .output();
    let Ok(out) = out else { return eprintln!("no unshare here: the full disk is not tried") };
    let (said, errors) = (String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    if !said.contains("full_child") {
        return eprintln!("no private namespace here: the full disk is not tried: {errors}");
    }
    assert!(out.status.success() && said.contains("FULL-OK"), "{said}\n{errors}");
}
