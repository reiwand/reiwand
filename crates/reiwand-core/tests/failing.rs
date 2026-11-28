use reiwand_core::OutputConfig;
use reiwand_core::config::Config;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const SAVING: &str = "REIWAND_TEST_SAVING";
const FULL: &str = "REIWAND_TEST_FULL";

fn config(outputs: usize, tag: usize) -> Config {
    let mut config = Config::default();
    for n in 0..outputs {
        let image = PathBuf::from(format!("/pictures/{tag}/picture-{n}.png"));
        config
            .outputs
            .insert(format!("screen-{n}"), OutputConfig { image: Some(image), ..Default::default() });
    }
    config
}

fn fresh(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    drop(std::fs::remove_dir_all(&dir));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn whole(path: &Path) -> usize {
    let config = Config::load(path).unwrap_or_else(|e| panic!("the configuration was left broken: {e}"));
    assert_eq!(config.outputs.len(), 300, "a saved configuration lost screens");
    let tags: Vec<_> = config.outputs.values().filter_map(|o| o.image.as_ref()?.parent()).collect();
    assert!(tags.windows(2).all(|t| t[0] == t[1]), "a configuration mixes two saves");
    std::fs::read_dir(path.parent().unwrap()).unwrap().count()
}

#[test]
fn saving_child() {
    let Some(path) = std::env::var_os(SAVING).map(PathBuf::from) else { return };
    for tag in 0..usize::MAX {
        config(300, tag).save(&path).unwrap();
    }
}

#[test]
fn a_save_killed_at_any_moment_leaves_the_last_whole_configuration() {
    let dir = fresh("killed");
    let path = dir.join("config.toml");
    config(300, 0).save(&path).unwrap();
    let started = Instant::now();
    for round in 0..16u64 {
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "saving_child", "--nocapture", "--test-threads=1"])
            .env(SAVING, &path)
            .stdout(Stdio::null())
            .spawn()
            .unwrap();
        std::thread::sleep(Duration::from_millis(20 + round * 7));
        child.kill().unwrap();
        child.wait().unwrap();
        let files = whole(&path);
        assert!(files <= 1 + round as usize + 1, "{files} files left after {round} kills");
        assert!(started.elapsed() < Duration::from_secs(20), "saving under kills took too long");
    }
}

#[test]
fn full_child() {
    let Some(dir) = std::env::var_os(FULL).map(PathBuf::from) else { return };
    let path = dir.join("config.toml");
    config(300, 1).save(&path).unwrap();
    let error = config(3000, 2).save(&path).expect_err("a save on a full disk was reported as done");
    assert!(error.to_string().contains("cannot write"), "{error}");
    whole(&path);
    let left: Vec<_> = std::fs::read_dir(&dir).unwrap().flatten().map(|e| e.file_name()).collect();
    assert_eq!(left.len(), 1, "a failed save left files behind: {left:?}");
    println!("FULL-OK");
}

#[test]
fn a_save_on_a_full_disk_fails_cleanly_and_keeps_the_last_configuration() {
    let dir = fresh("full");
    let script = r#"mount -t tmpfs -o size=96k tmpfs "$1" && exec "$2" --exact full_child --nocapture"#;
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
