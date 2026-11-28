use std::path::Path;
use std::process::Command;

#[test]
fn prints_the_notices_with_the_package_list_and_the_texts() {
    let out = Command::new(env!("CARGO_BIN_EXE_reiwand-ctl"))
        .arg("licenses")
        .env("REIWAND_PRIVATE_BUS_ONLY", "1")
        .env_remove("DBUS_SESSION_BUS_ADDRESS")
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success());
    assert!(text.starts_with("Third-party notices"));
    assert!(text.contains("\nPackages\n"));
    assert!(text.contains("Apache License"));
    assert!(text.contains("SIL OPEN FONT LICENSE"));
    assert!(text.contains("Phosphor"));
}

#[test]
fn the_notice_files_match_the_lock_file() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = Command::new("python3").arg(root.join("scripts/notices.py")).arg("check").output();
    let Ok(out) = out else { return };
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
}
