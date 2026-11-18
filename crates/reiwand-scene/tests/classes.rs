use reiwand_scene::schema;
use std::collections::BTreeMap;
use std::path::Path;

const TABLE: &str = include_str!("../src/classes.tsv");

fn sources(dir: &Path, found: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            sources(&path, found);
        } else if path.extension().is_some_and(|e| e == "rs") {
            found.push(path);
        }
    }
}

fn scanned() -> BTreeMap<String, &'static str> {
    let mut files = Vec::new();
    sources(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut files);
    let mut rows = BTreeMap::new();
    for file in files {
        let text = std::fs::read_to_string(&file).unwrap();
        let (mut owner, mut class, mut renamed) = (String::new(), "", None::<String>);
        for line in text.lines() {
            let t = line.trim();
            if t.starts_with("#[cfg(test)]") {
                break;
            }
            if let Some(rest) = t.strip_prefix("pub struct ").or_else(|| t.strip_prefix("pub enum ")) {
                owner = rest.split(['<', ' ', '{', '(']).next().unwrap().to_owned();
            }
            if let Some(rest) = t.strip_prefix("#[serde(rename = \"") {
                renamed = rest.split('"').next().map(str::to_owned);
            }
            if !t.starts_with("#[") && class.is_empty() {
                renamed = None;
            }
            if t.contains("deserialize_with = \"block\"") {
                class = "block";
            } else if t.contains("deserialize_with = \"null\"") {
                class = "value";
            } else if !class.is_empty() && !t.starts_with("#[") {
                let name = t.trim_start_matches("pub ").split(':').next().unwrap();
                let name = renamed.take().unwrap_or_else(|| name.to_owned());
                rows.insert(format!("{owner}.{name}"), class);
                class = "";
            }
        }
    }
    rows
}

fn table() -> BTreeMap<String, &'static str> {
    TABLE
        .lines()
        .map(|l| {
            let (row, class) = l.split_once('\t').unwrap();
            (row.to_owned(), class)
        })
        .collect()
}

#[test]
fn the_class_table_lists_every_optional_field_with_its_class() {
    let found = scanned();
    let text: String = found.iter().map(|(row, class)| format!("{row}\t{class}\n")).collect();
    if std::env::var_os("REIWAND_WRITE_CLASSES").is_some() {
        std::fs::write(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/classes.tsv"), &text).unwrap();
    }
    assert_eq!(table(), found, "rerun with REIWAND_WRITE_CLASSES=1");
    assert!(found.values().any(|c| *c == "block") && found.values().any(|c| *c == "value"));
}

#[test]
fn the_schema_marks_exactly_the_blocks_of_the_table() {
    let rows = table();
    let root = schema();
    let mut marked = 0;
    for (def, node) in root["$defs"].as_object().unwrap() {
        let owner = def.split("_for_").next().unwrap();
        for (name, prop) in node["properties"].as_object().into_iter().flatten() {
            let block = prop["x-block"] == true;
            marked += usize::from(block);
            let class = rows.get(&format!("{owner}.{name}")).copied();
            assert_eq!(block, class == Some("block"), "{def}.{name} is {class:?}");
        }
    }
    assert!(marked > 0);
}
