use reiwand_core::Config;
use std::path::Path;
use std::process::Command;

fn allowed(home: &Path, auto: bool) -> (bool, String) {
    let mut args = vec!["update", "allowed"];
    if auto {
        args.push("--auto");
    }
    let out = Command::new(env!("CARGO_BIN_EXE_reiwand-ctl"))
        .args(args)
        .env("XDG_CONFIG_HOME", home)
        .env("REIWAND_PRIVATE_BUS_ONLY", "1")
        .env("DBUS_SESSION_BUS_ADDRESS", "unix:path=/nonexistent")
        .output()
        .unwrap();
    (out.status.success(), String::from_utf8_lossy(&out.stderr).into_owned())
}

fn store(home: &Path, change: impl FnOnce(&mut Config)) {
    let mut config = Config::default();
    change(&mut config);
    config.save(&home.join("reiwand/config.toml")).unwrap();
}

#[test]
fn the_updater_asks_without_the_service_and_obeys_private_mode_and_the_setting() {
    let home = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(home.path().join("reiwand")).unwrap();
    assert!(allowed(home.path(), true).0, "no configuration yet: the daily check is on");
    store(home.path(), |c| c.set_option("updates.auto", "false").unwrap());
    let (ok, said) = allowed(home.path(), true);
    assert!(!ok && said.contains("update auto on"), "{said}");
    assert!(allowed(home.path(), false).0, "an update asked by hand runs with the daily check off");
    store(home.path(), |c| c.network.private = true);
    for auto in [true, false] {
        let (ok, said) = allowed(home.path(), auto);
        assert!(!ok && said.contains("private mode"), "{said}");
    }
}
