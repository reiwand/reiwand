mod common;
use common::{live_schemas, repo};
use reiwand_upgrade::fill::{Format, tsv};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::PathBuf;

fn table(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("table").join(name)
}

fn specs() -> Vec<(String, Vec<String>)> {
    let mut found = Vec::new();
    for item in std::fs::read_dir(repo().join("docs/specs")).unwrap().flatten() {
        let name = item.file_name().to_string_lossy().to_string();
        if name.ends_with(".md") {
            let lines = std::fs::read_to_string(item.path()).unwrap().lines().map(str::to_string).collect();
            found.push((name, lines));
        }
    }
    found.sort();
    found
}

fn spec_line(specs: &[(String, Vec<String>)], prop: &str) -> String {
    let quoted = format!("`{prop}`");
    specs
        .iter()
        .find_map(|(file, lines)| {
            lines.iter().position(|l| l.contains(&quoted)).map(|i| format!("{file}:{}", i + 1))
        })
        .unwrap_or_else(|| "-".into())
}

fn generated() -> String {
    let specs = specs();
    Format::ALL
        .iter()
        .flat_map(|f| tsv(*f).lines().map(str::to_string).collect::<Vec<_>>())
        .map(|line| {
            let prop = line.split('\t').nth(1).and_then(|k| k.rsplit('.').next()).unwrap_or("").to_string();
            format!("{line}\t{}\n", spec_line(&specs, &prop))
        })
        .collect()
}

#[test]
#[ignore = "rewrites the field table from the frozen schemas"]
fn write_table() {
    std::fs::write(table("fields.tsv"), generated()).unwrap();
}

#[test]
fn table_is_the_one_the_frozen_schemas_give() {
    let (now, kept) = (generated(), std::fs::read_to_string(table("fields.tsv")).unwrap());
    assert!(now == kept, "table/fields.tsv is stale: run write_table (ignored test)");
}

fn each_object<'a>(root: &'a Value, f: &mut dyn FnMut(&str, &'a Value)) {
    let title = root.get("title").and_then(Value::as_str).unwrap_or("");
    let mut visit = |def: &str, schema: &'a Value| {
        f(def, schema);
        for key in ["oneOf", "anyOf"] {
            schema.get(key).and_then(Value::as_array).into_iter().flatten().for_each(|b| f(def, b));
        }
    };
    visit(title, root);
    for (def, schema) in root.get("$defs").and_then(Value::as_object).into_iter().flatten() {
        visit(def, schema);
    }
}

#[test]
fn table_covers_every_optional_property_of_the_live_schemas() {
    let kept = std::fs::read_to_string(table("fields.tsv")).unwrap();
    let keys: BTreeSet<(&str, &str)> = kept
        .lines()
        .filter_map(|l| {
            let mut c = l.split('\t');
            Some((c.next()?, c.next()?))
        })
        .collect();
    let (mut seen, mut missing) = (0, Vec::new());
    for (format, schema) in live_schemas() {
        each_object(&schema, &mut |def, object| {
            let required: Vec<&str> = object
                .get("required")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            for (prop, ps) in object.get("properties").and_then(Value::as_object).into_iter().flatten() {
                if required.contains(&prop.as_str()) || ps.is_boolean() || ps["x-block"] == true {
                    continue;
                }
                seen += 1;
                let key = format!("{def}.{prop}");
                if !keys.contains(&(format, key.as_str())) {
                    missing.push(format!("{format} {key}"));
                }
            }
        });
    }
    println!("optional properties covered: {seen}");
    assert!(missing.is_empty(), "not in the table: {missing:?}");
}

fn rust_files(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    for item in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = item.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n != "tests") {
                rust_files(&path, out);
            }
        } else if path.extension().is_some_and(|e| e == "rs") && !path.to_string_lossy().ends_with("tests.rs")
        {
            out.push(path);
        }
    }
}

fn nullable_names() -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for (format, schema) in live_schemas().into_iter().filter(|(f, _)| matches!(*f, "scene" | "manifest")) {
        let _ = format;
        each_object(&schema, &mut |_, object| {
            for (prop, ps) in object.get("properties").and_then(Value::as_object).into_iter().flatten() {
                let text = ps.to_string();
                if text.contains("\"null\"") {
                    names.insert(prop.clone());
                }
            }
        });
    }
    names
}

fn hits(text: &str, names: &BTreeSet<String>) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for line in text.lines() {
        for (at, _) in line.match_indices(".unwrap_or") {
            let head = &line[..at];
            let name: String = head
                .chars()
                .rev()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            if head[..head.len() - name.len()].ends_with('.') && names.contains(&name) {
                found.insert(name);
            }
        }
    }
    found
}

#[test]
fn every_default_written_in_the_render_code_has_a_row() {
    let names = nullable_names();
    let listed: BTreeSet<String> = std::fs::read_to_string(table("render-defaults.tsv"))
        .unwrap()
        .lines()
        .filter_map(|l| l.split('\t').next())
        .map(str::to_string)
        .collect();
    let mut files = Vec::new();
    for krate in ["reiwand-scene", "reiwand-render", "reiwand-format"] {
        rust_files(&repo().join("crates").join(krate).join("src"), &mut files);
    }
    let found: BTreeSet<String> =
        files.iter().flat_map(|f| hits(&std::fs::read_to_string(f).unwrap(), &names)).collect();
    let missing: Vec<_> = found.difference(&listed).collect();
    println!("render-code defaults: {} names, {} listed", found.len(), listed.len());
    assert!(missing.is_empty(), "add them to table/render-defaults.tsv: {missing:?}");
}
