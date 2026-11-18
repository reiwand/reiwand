use reiwand_core::explicit::declared;
use std::path::Path;

const SAVED: [(&str, &str); 5] = [
    ("Credit", "attribution"),
    ("Credit", "license"),
    ("Credit", "link"),
    ("Entry", "last_sync"),
    ("Entry", "last_error"),
];

#[test]
fn every_optional_field_of_the_saved_providers_is_a_block_the_source_declares() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/providers/saved.rs");
    let text = std::fs::read_to_string(&file).unwrap();
    assert_eq!(text.matches(": Option<").count(), SAVED.len());
    for (ty, field) in SAVED {
        assert_eq!(declared(&file, Some(ty), field), Some("bloque"), "{ty}.{field}");
    }
}
