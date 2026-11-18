use reiwand_core::explicit::declared;
use std::path::Path;

fn check(table: &str, dir: &Path, column: usize) {
    let mut wrong = Vec::new();
    for line in table.lines() {
        let row: Vec<&str> = line.split('\t').collect();
        let (ty, field) = row[0].split_once('.').unwrap();
        let want = row[column];
        if declared(dir, Some(ty), field) != Some(if want == "bloque" { "bloque" } else { "valor" }) {
            wrong.push(row[0].to_owned());
        }
    }
    assert!(wrong.is_empty(), "declared in the source with another class than the table says: {wrong:?}");
}

#[test]
fn every_class_in_the_tables_is_the_one_the_source_declares() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    check(include_str!("../prefill/model.tsv"), &src, 1);
    check(include_str!("../prefill/edit.tsv"), &src, 1);
    let config: String = include_str!("../prefill/config.tsv")
        .lines()
        .filter(|l| l.split('\t').nth(1).is_some_and(|c| c == "bloque" || c == "valor"))
        .filter(|l| !l.starts_with("Config.") && !l.starts_with("Surfaces."))
        .map(|l| format!("{l}\n"))
        .collect();
    check(&config, &src, 1);
}
