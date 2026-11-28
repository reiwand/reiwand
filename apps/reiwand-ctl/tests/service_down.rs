use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn run(address: &str, args: &[&str]) -> (bool, String, Duration) {
    let started = Instant::now();
    let mut child = Command::new(env!("CARGO_BIN_EXE_reiwand-ctl"))
        .args(args)
        .env("REIWAND_PRIVATE_BUS_ONLY", "1")
        .env("DBUS_SESSION_BUS_ADDRESS", address)
        .env("REIWAND_PRIVATE_BUS_ADDRESS", address)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    while child.try_wait().unwrap().is_none() {
        assert!(started.elapsed() < Duration::from_secs(20), "reiwand-ctl {args:?} hangs without a bus");
        std::thread::sleep(Duration::from_millis(20));
    }
    let out = child.wait_with_output().unwrap();
    let text = String::from_utf8_lossy(&out.stderr).into_owned() + &String::from_utf8_lossy(&out.stdout);
    (out.status.success(), text, started.elapsed())
}

#[test]
fn every_kind_of_call_fails_with_a_message_when_the_bus_is_gone() {
    let dir = tempfile::tempdir().unwrap();
    let address = format!("unix:path={}", dir.path().join("none").display());
    for args in [
        &["state"][..],
        &["state", "--json"],
        &["config"],
        &["wait", "--timeout", "100"],
        &["watch"],
        &["jobs"],
        &["next"],
    ] {
        let (ok, text, _) = run(&address, args);
        assert!(!ok && !text.trim().is_empty(), "{args:?}: {text}");
        assert!(!text.contains("panicked"), "{args:?}: {text}");
    }
}

#[test]
fn odd_arguments_never_panic_even_without_a_service() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    let dir = tempfile::tempdir().unwrap();
    let address = format!("unix:path={}", dir.path().join("none").display());
    let huge = "x".repeat(100_000);
    let odd = OsString::from_vec(vec![0xff, 0xfe, b'a']);
    let cases: Vec<Vec<OsString>> = vec![
        vec!["set".into(), odd.clone()],
        vec!["set".into(), huge.clone().into()],
        vec!["config".into(), huge.clone().into(), huge.clone().into()],
        vec!["config".into(), "playback.max_fps".into(), odd.clone()],
        vec!["scene".into(), "act".into(), dir.path().into(), "nothing".into(), "{".into()],
        vec!["scene".into(), "act".into(), dir.path().into(), odd.clone(), "[".repeat(100_000).into()],
        vec!["scene".into(), "actions".into()],
        vec!["volume".into(), "99999999999999999999".into()],
        vec!["wait".into(), "--timeout".into(), "-1".into()],
        vec![odd.clone()],
    ];
    for args in cases {
        let out = Command::new(env!("CARGO_BIN_EXE_reiwand-ctl"))
            .args(&args)
            .env("REIWAND_PRIVATE_BUS_ONLY", "1")
            .env("DBUS_SESSION_BUS_ADDRESS", &address)
            .env("REIWAND_PRIVATE_BUS_ADDRESS", &address)
            .output()
            .unwrap();
        let text = String::from_utf8_lossy(&out.stderr);
        assert!(!text.contains("panicked") && out.status.code().is_some(), "{args:?}: {text}");
    }
}
