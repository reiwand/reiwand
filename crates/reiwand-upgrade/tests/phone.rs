use reiwand_upgrade::client::run_with;
mod vault;
use vault::Memory;

#[test]
fn old_settings_are_completed_once_and_kept() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("settings.toml");
    std::fs::write(&file, "system_colors = \"always\"\nbackend = \"gl\"\n").unwrap();
    run_with(dir.path(), &mut Memory::default()).unwrap();
    let table: toml::Table = std::fs::read_to_string(&file).unwrap().parse().unwrap();
    assert_eq!(table["format"].as_integer(), Some(2));
    assert_eq!(table["system_colors"].as_str(), Some("always"));
    assert_eq!(table["backend"].as_str(), Some("gl"));
    assert_eq!(table["charger_fps"].as_str(), Some("60"));
    assert_eq!(table["private_mode"].as_str(), Some("off"));
    let first = std::fs::read_to_string(&file).unwrap();
    run_with(dir.path(), &mut Memory::default()).unwrap();
    assert_eq!(std::fs::read_to_string(&file).unwrap(), first);
}
