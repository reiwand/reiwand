use reiwand_core::explicit::{Rule, scan};
use std::collections::BTreeMap;
use std::path::Path;

const ALLOWED: &str = include_str!("explicit_scan/allowed.tsv");
const RULES: [Rule; 5] =
    [Rule::SerdeDefault, Rule::SkipSerializing, Rule::UnwrapOr, Rule::DeriveDefault, Rule::OptionWithoutNull];

fn allowed() -> BTreeMap<String, (usize, String)> {
    ALLOWED
        .lines()
        .map(|l| {
            let mut c = l.splitn(3, '\t');
            let file = c.next().unwrap().to_owned();
            let count = c.next().unwrap().parse().unwrap();
            (file, (count, c.next().unwrap().to_owned()))
        })
        .collect()
}

fn found() -> BTreeMap<String, usize> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scene = root.join("src");
    let render = root.join("../reiwand-render/src");
    let mut counts = BTreeMap::new();
    for v in scan(&[scene.as_path(), render.as_path()], &RULES) {
        let name = match v.file.strip_prefix(&scene) {
            Ok(rest) => rest.display().to_string(),
            Err(_) => format!("render/{}", v.file.strip_prefix(&render).unwrap().display()),
        };
        *counts.entry(name).or_insert(0) += 1;
    }
    counts
}

#[test]
fn every_hit_of_the_scan_is_classified_as_runtime_or_internal() {
    let allowed = allowed();
    let found = found();
    let mut wrong = Vec::new();
    for (file, count) in &found {
        match allowed.get(file) {
            Some((n, _)) if n == count => {}
            Some((n, _)) => wrong.push(format!("{file}: {count} hits, classified {n}")),
            None => wrong.push(format!("{file}: {count} hits, not classified")),
        }
    }
    wrong.extend(
        allowed.keys().filter(|f| !found.contains_key(*f)).map(|f| format!("{f}: classified but clean")),
    );
    assert!(
        wrong.is_empty(),
        "classify in tests/explicit_scan/allowed.tsv or remove the hidden value:\n{}",
        wrong.join("\n")
    );
}

#[test]
#[ignore = "lists the hidden values that are still pending"]
fn no_hidden_value_of_a_saved_format_is_pending() {
    let pending: Vec<_> = allowed().into_iter().filter(|(_, (_, why))| why.starts_with("pending")).collect();
    assert!(pending.is_empty(), "{pending:#?}");
}
