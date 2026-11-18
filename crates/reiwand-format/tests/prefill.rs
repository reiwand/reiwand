use std::collections::BTreeMap;
use std::path::Path;

const TABLE: &str = include_str!("../src/prefill.tsv");

fn rows() -> Vec<Vec<&'static str>> {
    TABLE.lines().map(|l| l.split('\t').collect()).collect()
}

#[test]
fn every_row_names_its_format_its_class_and_a_null_prefill() {
    for row in rows() {
        assert_eq!(row.len(), 4, "{row:?}");
        assert!(["bloque", "valor"].contains(&row[2]), "{row:?}");
        assert_eq!(row[3], "null", "{row:?}");
    }
}

const FILES: [&str; 9] =
    ["catalog", "charter", "entry", "featured", "index", "parts", "record", "release", "repo"];

#[test]
fn every_option_of_the_repository_files_has_a_row() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut written: BTreeMap<String, usize> = BTreeMap::new();
    for file in FILES {
        let text = std::fs::read_to_string(src.join(format!("{file}.rs"))).unwrap();
        written.insert(
            file.to_owned(),
            text.matches("explicit::null").count() + text.matches("explicit::block").count(),
        );
    }
    let mut listed: BTreeMap<String, usize> = BTreeMap::new();
    for row in rows().iter().filter(|r| r[0] != "manifest") {
        *listed.entry(row[0].to_owned()).or_insert(0) += 1;
    }
    written.retain(|_, n| *n > 0);
    assert_eq!(written, listed);
}

#[test]
fn every_class_is_the_one_the_source_declares() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut wrong = Vec::new();
    for row in rows() {
        let (scope, ty, field) = if row[0] == "manifest" {
            let (ty, field) = row[1].split_once('.').unwrap();
            (src.clone(), Some(ty), field)
        } else {
            (src.join(format!("{}.rs", row[0])), None, row[1])
        };
        let want = if row[2] == "bloque" { "bloque" } else { "valor" };
        let scene = src.join("../../reiwand-scene/src");
        let field = if field == "loop" { "repeat" } else { field };
        let got = reiwand_core::explicit::declared(&scope, ty, field)
            .or_else(|| ty.and_then(|ty| reiwand_core::explicit::declared(&scene, Some(ty), field)))
            .or_else(|| ty.and_then(|_| reiwand_core::explicit::declared(&scene.join("props"), None, field)))
            .or_else(|| {
                ty.and_then(|_| reiwand_core::explicit::declared(&scene.join("props/choice.rs"), None, field))
            });
        if got != Some(want) {
            wrong.push(format!("{}.{}", row[0], row[1]));
        }
    }
    assert!(wrong.is_empty(), "{wrong:?}");
}
