mod common;
use common::*;
use reiwand_android::host;
use std::fs;
use std::path::Path;

fn media_files(files: &Path) -> usize {
    fs::read_dir(files.join("media")).map_or(0, |d| d.flatten().filter(|e| e.path().is_file()).count())
}

fn fill(path: &Path) {
    use std::io::Write;
    let mut file = fs::File::create(path).unwrap();
    let block = vec![7u8; 4096];
    while file.write_all(&block).is_ok() {}
    let _ = file.sync_all();
}

fn full_disk_child(dir: &Path) {
    host::prepare(dir.to_str().unwrap()).unwrap();
    let fixtures = Path::new(ROOT).join("../../tests/fixtures");
    let ok = host::import(fixtures.join("green-64x32.png").to_str().unwrap());
    assert!(!ok.contains("error"), "{ok}");
    let scratch = tempfile::tempdir_in(dir.parent().unwrap()).unwrap();
    let pack = pack("orbits", scratch.path());
    let mut sources = vec![pack, fixtures.join("bands-640x180.png"), fixtures.join("magenta-64x32.webm")];
    let filler = dir.join("filler");
    fill(&filler);
    let (known, held) = (items(), media_files(dir));
    for source in &sources {
        let (reply, alive) = attempt(source.clone());
        assert!(alive, "{reply}");
        assert!(reply.contains("error"), "a full disk must be reported: {reply}");
    }
    assert_eq!((items(), media_files(dir)), (known, held), "a failed install leaves nothing behind");
    fs::remove_file(&filler).unwrap();
    for source in sources.drain(..) {
        let reply = host::import(source.to_str().unwrap());
        assert!(!reply.contains("error"), "{reply}");
    }
    assert_eq!(items(), known + 3);
    assert_eq!(items(), media_files(dir));
}

#[test]
fn a_full_disk_is_reported_and_leaves_nothing_half_installed() {
    if let Ok(dir) = std::env::var("REIWAND_FULL_DISK_DIR") {
        return full_disk_child(Path::new(&dir));
    }
    let work = tempfile::tempdir().unwrap();
    let mount = work.path().join("small");
    fs::create_dir(&mount).unwrap();
    let script = format!(
        "mount -t tmpfs -o size=3m none '{0}' && mkdir '{0}/files' && exec \"$0\" --exact a_full_disk_is_reported_and_leaves_nothing_half_installed",
        mount.display()
    );
    // Some machines forbid a private namespace with its own mounts: there is no small disk to fill.
    let probe = format!("mount -t tmpfs -o size=1m none '{}'", mount.display());
    let able = std::process::Command::new("unshare").args(["-Urm", "sh", "-c", &probe]).status();
    if !able.is_ok_and(|s| s.success()) {
        return eprintln!("no user namespaces with mounts here: skipped");
    }
    let exe = std::env::current_exe().unwrap();
    let status = std::process::Command::new("unshare")
        .args(["-Urm", "sh", "-c", &script])
        .arg(exe)
        .env("REIWAND_FULL_DISK_DIR", mount.join("files"))
        .status();
    match status {
        Ok(s) => assert!(s.success(), "full disk attack failed: {s}"),
        Err(_) => eprintln!("no user namespaces here: skipped"),
    }
}
