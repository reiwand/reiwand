use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const EXAMPLES: [&str; 3] = ["lights", "orbits", "solids"];

fn copied(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        let target = to.join(path.file_name().unwrap());
        match path.is_dir() {
            true => copied(&path, &target),
            false => drop(std::fs::copy(&path, &target).unwrap()),
        }
    }
}

fn refused_or_done(args: &[&Path], what: &str) -> bool {
    let mut command = Command::new(env!("CARGO_BIN_EXE_reiwand-ctl"));
    command.arg("pack").args(args).stdout(Stdio::null()).stderr(Stdio::piped());
    let mut child = command.env("LC_ALL", "C").spawn().unwrap();
    let start = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if start.elapsed() > Duration::from_secs(60) {
            child.kill().unwrap();
            panic!("{what}: pack {args:?} hung");
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let said = std::io::read_to_string(child.stderr.take().unwrap()).unwrap();
    assert!(matches!(status.code(), Some(0..=2)), "{what}: pack {args:?} ended with {status}: {said}");
    assert!(status.success() || !said.trim().is_empty(), "{what}: pack {args:?} failed without a word");
    assert!(!said.contains("panicked"), "{what}: {said}");
    status.success()
}

fn damages(name: &str, dir: &Path) -> Vec<(String, PathBuf, Vec<u8>)> {
    let mut out = Vec::new();
    for (file, kind) in [
        ("scene.json", 0),
        ("scene.json", 2),
        ("manifest.json", 1),
        ("manifest.json", 2),
        ("index.html", 2),
        ("preview.jpg", 0),
        ("preview.png", 1),
        ("assets/crate.glb", 0),
        ("scripts/orbits.js", 2),
    ] {
        let Ok(data) = std::fs::read(dir.join(file)) else { continue };
        let broken = match kind {
            0 => data[..data.len() / 2].to_vec(),
            1 => Vec::new(),
            _ => [&data[..data.len() / 3], &[0xc3, 0x28, 0xff][..], &data[data.len() / 3..]].concat(),
        };
        out.push((format!("{name}: {file} damage {kind}"), dir.join(file), broken));
    }
    out
}

#[test]
fn check_and_build_refuse_a_damaged_folder_or_package_with_a_message() {
    let tmp = tempfile::tempdir().unwrap();
    let scenes = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenes");
    for name in EXAMPLES {
        let dir = tmp.path().join(name);
        copied(&scenes.join(name), &dir);
        let built = tmp.path().join(format!("{name}.wpak"));
        assert!(refused_or_done(&["build".as_ref(), &dir, "-o".as_ref(), &built], name));
        let whole = std::fs::read(&built).unwrap();
        for (what, path, broken) in damages(name, &dir) {
            let kept = std::fs::read(&path).unwrap();
            std::fs::write(&path, &broken).unwrap();
            let manifest = std::fs::read(dir.join("manifest.json")).unwrap();
            refused_or_done(&["check".as_ref(), &dir], &what);
            let out = tmp.path().join("out.wpak");
            if !refused_or_done(&["build".as_ref(), &dir, "-o".as_ref(), &out], &what) {
                assert_eq!(
                    std::fs::read(dir.join("manifest.json")).unwrap(),
                    manifest,
                    "{what}: manifest changed"
                );
            }
            std::fs::write(&path, kept).unwrap();
        }
        for cut in [30, whole.len() / 2, whole.len() - 1] {
            std::fs::write(&built, &whole[..cut]).unwrap();
            assert!(!refused_or_done(&["check".as_ref(), &built], &format!("{name}.wpak cut at {cut}")));
        }
    }
}
