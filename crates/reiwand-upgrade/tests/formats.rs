use reiwand_upgrade::fill::{Format, complete, complete_toml};
use serde_json::Value;
use std::path::PathBuf;

fn pair(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/pairs").join(name)
}

fn text(name: &str) -> String {
    std::fs::read_to_string(pair(name)).unwrap()
}

fn expect(name: &str, got: &str) {
    if std::env::var_os("REIWAND_BLESS").is_some() {
        std::fs::write(pair(name), got).unwrap();
    }
    assert_eq!(got, text(name), "{name}");
}

#[test]
fn config_keeps_comments_and_completes_every_value() {
    let original = text("config.original.toml");
    let (done, report) = complete_toml(Format::Config, &original).unwrap();
    assert!(report.written > 0);
    assert!(done.contains("# my settings") && done.contains("# keep it light"));
    expect("config.completed.toml", &done);
    assert_eq!(complete_toml(Format::Config, &done).unwrap().0, done);
}

#[test]
fn carry_reads_the_same() {
    let original = text("carry.original.json");
    let mut doc: Value = serde_json::from_str(&original).unwrap();
    assert!(complete(Format::Carry, &mut doc).written > 0);
    let done = serde_json::to_string_pretty(&doc).unwrap() + "\n";
    let (a, b) = (
        serde_json::from_str::<reiwand_carry::Content>(&original).unwrap(),
        serde_json::from_str::<reiwand_carry::Content>(&done).unwrap(),
    );
    assert!(a == b);
    expect("carry.completed.json", &done);
    assert_eq!(complete(Format::Carry, &mut doc).written, 0);
}

#[test]
fn completion_is_stable() {
    let original = text("edit.original.json");
    let run = || {
        let mut doc: Value = serde_json::from_str(&original).unwrap();
        complete(Format::Edit, &mut doc);
        serde_json::to_string(&doc).unwrap()
    };
    assert_eq!(run(), run());
}
