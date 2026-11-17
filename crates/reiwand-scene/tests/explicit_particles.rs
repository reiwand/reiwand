#[path = "prefill/mod.rs"]
mod prefill;

use reiwand_core::explicit::{Rule, all_required, omission, read, round_trip, scan};
use reiwand_scene::{System, schema};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;

const TAGS: [&str; 12] = [
    "box",
    "circle",
    "gravity",
    "turbulence",
    "attractor",
    "vortex",
    "orbit",
    "oscillate",
    "slow_near",
    "keep_between",
    "remap",
    "drag",
];
const ALLOWED: [(&str, &str); 4] = [
    ("particles/spread.rs", "pub bound: Option<"),
    ("particles/seed.rs", "audio: Option<Band>,"),
    ("particles/place.rs", "points.get(i).copied().unwrap_or_default()"),
    ("value/emitters.rs", "self.to.unwrap_or(self.from)"),
];
const SAMPLE: &str = include_str!("explicit/system.json");

fn sample() -> Value {
    prefill::completed(serde_json::from_str(SAMPLE).unwrap())
}

fn files() -> Vec<std::path::PathBuf> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found: Vec<_> =
        std::fs::read_dir(src.join("particles")).unwrap().flatten().map(|e| e.path()).collect();
    found.push(src.join("value/emitters.rs"));
    found.retain(|f| std::fs::read_to_string(f).unwrap().contains("Deserialize"));
    found
}

fn names() -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for file in files() {
        for line in std::fs::read_to_string(file).unwrap().lines() {
            let rest = line.strip_prefix("pub struct ").or_else(|| line.strip_prefix("pub enum "));
            found.extend(rest.map(|r| r.split(|c: char| !c.is_alphanumeric()).next().unwrap().to_owned()));
        }
    }
    found
}

#[test]
fn every_particle_property_is_required() {
    let names = names();
    let loose: Vec<String> = all_required(&schema())
        .into_iter()
        .filter(|path| {
            let owner = path.strip_prefix("/$defs/").and_then(|p| p.split('/').next());
            owner.is_some_and(|o| names.contains(o))
        })
        .collect();
    assert!(loose.is_empty(), "{loose:#?}");
}

#[test]
fn the_source_has_no_hidden_values() {
    let rules = [
        Rule::SerdeDefault,
        Rule::SkipSerializing,
        Rule::UnwrapOr,
        Rule::DeriveDefault,
        Rule::OptionWithoutNull,
    ];
    let owned = files();
    let paths: Vec<&Path> = owned.iter().map(|d| d.as_path()).collect();
    let found: Vec<_> = scan(&paths, &rules)
        .into_iter()
        .filter(|v| {
            let text = std::fs::read_to_string(&v.file).unwrap();
            let line = text.lines().nth(v.line - 1).unwrap_or("").trim();
            !ALLOWED.iter().any(|(file, part)| v.file.ends_with(file) && line.contains(part))
        })
        .collect();
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn every_written_key_is_refused_when_removed() {
    let load = |v: &Value| read::<System>("system", v).map(|_| ());
    let tag = |path: &str| {
        let last = path.rsplit('.').next().unwrap_or(path);
        path.ends_with(']') || TAGS.contains(&last)
    };
    let wrong = omission(&sample(), tag, load);
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn writing_the_sample_again_gives_the_same_bytes() {
    let text = serde_json::to_string_pretty(&sample()).unwrap();
    let again = |t: &str| {
        let s: System = serde_json::from_str(t).map_err(|e| e.to_string())?;
        serde_json::to_string_pretty(&s).map_err(|e| e.to_string())
    };
    let canonical = again(&text).unwrap();
    round_trip(&canonical, again).unwrap();
}

#[test]
fn the_prefill_table_matches_the_schema() {
    let table: BTreeSet<&str> = prefill::rows().keys().copied().collect();
    let live = prefill::schema_keys(&schema());
    let live: BTreeSet<&str> = live.iter().map(String::as_str).collect();
    assert_eq!(table.difference(&live).collect::<Vec<_>>(), Vec::<&&str>::new(), "rows without a field");
    assert_eq!(live.difference(&table).collect::<Vec<_>>(), Vec::<&&str>::new(), "fields without a row");
}
