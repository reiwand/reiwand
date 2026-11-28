use std::path::Path;

fn read(path: &str) -> String {
    std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap()
}

#[test]
fn after_a_flatpak_update_the_tray_offers_one_restart_that_refreshes_plasma() {
    let ui = "../../integrations/plasma-tray/contents/ui";
    let script = read(&format!("{ui}/flatpak.js"));
    let manifest = read("../../packaging/flatpak/io.github.reiwand.Reiwand.yml");
    assert!(
        script.contains("const APP = \"io.github.reiwand.Reiwand\";")
            && manifest.contains("io.github.reiwand.Reiwand")
    );
    for step in
        ["systemd-run --user", "reiwand-plasma \" + APP + \" | sh", "kill $p", "restart plasma-plasmashell"]
    {
        assert!(script.contains(step), "the guided restart lacks {step}");
    }
    assert!(script.contains("for c in $p $(pgrep -P $p"), "the guided restart does not look past bwrap");
    assert!(
        !script.contains("flatpak kill \" + APP"),
        "flatpak kill also ends the app and its unsaved edits"
    );
    assert!(
        script.contains("flatpak === true")
            && script.contains("(compare(installedVersion, running) ?? 0) > 0")
    );
    let daemon = read(&format!("{ui}/Daemon.qml"));
    assert!(daemon.contains("Flatpak.newer(installed, current.version, current.flatpak_commit != null)"));
    let main = read(&format!("{ui}/main.qml"));
    assert!(main.contains("source: \"FlatpakProbe.qml\"") && !main.contains("plasma5support"));
    let updated = read(&format!("{ui}/Updated.qml"));
    assert!(
        updated.contains("updated.daemon.probe.restart()") && updated.contains("daemon.stale || flatpak")
    );
}
