mod common;
use reiwand_core::explicit::{Rule, all_required, declared, omission, read, scan};
use reiwand_plugin::{Bundle, Error, Installed, Manifest, OPTIONAL, PREFILL, schema};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

const SECRET: &str = r#"{"label": "API key", "header": "X-Api-Key", "prefix": "", "hosts": ["api.example.test"], "required": true}"#;
const SETTINGS: &str = r#"[
    {"key": "query", "label": "Search", "kind": "text", "prefill": "", "min": null, "max": null, "values": []},
    {"key": "count", "label": "Count", "kind": "int", "prefill": 10, "min": 1, "max": 50, "values": []},
    {"key": "order", "label": "Order", "kind": "choice", "prefill": "new", "min": null, "max": null, "values": ["new", "top"]}
]"#;

fn full() -> Value {
    serde_json::from_str(&common::manifest(&["api.example.test"], SECRET, SETTINGS)).unwrap()
}

fn refused(edit: impl FnOnce(&mut Value)) -> String {
    let mut value = full();
    edit(&mut value);
    let mut files = BTreeMap::new();
    files.insert("reiwand-provider.json".to_owned(), value.to_string().into_bytes());
    files.insert(
        "main.ts".to_owned(),
        b"export const list = async () => ({ items: [], next: null });".to_vec(),
    );
    match Bundle::of(files) {
        Err(Error::Format(why)) => why,
        other => panic!("not refused: {other:?}"),
    }
}

#[test]
fn every_property_is_required_and_prefilled() {
    let (schema, report) = schema();
    assert!(all_required(&schema).is_empty(), "{:?}", all_required(&schema));
    assert!(report.unknown.is_empty() && report.uncovered.is_empty(), "{report:?}");
    let nulls: Vec<&str> = PREFILL.0.iter().filter(|(_, v)| *v == "null").map(|(r, _)| *r).collect();
    assert_eq!(nulls, OPTIONAL.iter().map(|(r, _)| *r).collect::<Vec<_>>());
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for (row, class) in OPTIONAL {
        let (ty, field) = row.split_once('.').unwrap();
        assert_eq!(declared(&src, Some(ty), field), Some(*class), "{row}");
    }
}

#[test]
fn a_missing_field_is_refused_with_its_path() {
    let load = |v: &Value| read::<Manifest>("reiwand-provider.json", v).map(|_| ());
    load(&full()).unwrap();
    let wrong = omission(&full(), |_| false, load);
    assert!(wrong.is_empty(), "{wrong:#?}");
    let why = refused(|v| drop(v["settings"][1].as_object_mut().unwrap().remove("max")));
    assert!(why.contains("\"settings[1].max\" is missing"), "{why}");
    let why = refused(|v| drop(v.as_object_mut().unwrap().remove("secret")));
    assert!(why.contains("\"secret\" is missing"), "{why}");
}

#[test]
fn wrong_values_are_refused_with_their_path() {
    type Edit = fn(&mut Value);
    let cases: [(&str, Edit); 9] = [
        ("\"schema\"", |v| v["schema"] = json!("reiwand.provider/2")),
        ("\"id\"", |v| v["id"] = json!("Sample")),
        ("\"version\"", |v| v["version"] = json!("one")),
        ("\"hosts[0]\"", |v| v["hosts"] = json!(["https://api.example.test"])),
        ("\"hosts[1]\"", |v| v["hosts"] = json!(["api.example.test", "10.0.0.1"])),
        ("\"secret.hosts[0]\"", |v| v["secret"]["hosts"] = json!(["img.example.test"])),
        ("\"secret.header\"", |v| v["secret"]["header"] = json!("Host")),
        ("\"settings[1].prefill\"", |v| v["settings"][1]["prefill"] = json!(99)),
        ("\"settings[0].key\"", |v| v["settings"][0]["key"] = json!("api_key")),
    ];
    for (path, edit) in cases {
        let why = refused(edit);
        assert!(why.contains(path), "{path}: {why}");
    }
}

#[test]
fn a_bundle_holds_only_its_manifest_modules_and_texts() {
    let manifest = common::manifest(&["api.example.test"], "null", "[]");
    let mut files = BTreeMap::new();
    files.insert("reiwand-provider.json".to_owned(), manifest.into_bytes());
    let fails = |files: &BTreeMap<String, Vec<u8>>| Bundle::of(files.clone()).unwrap_err().to_string();
    assert!(fails(&files).contains("no main.ts"), "{}", fails(&files));
    files.insert("main.ts".into(), b"export async function list() {}".to_vec());
    files.insert("tool.so".into(), vec![0x7f, b'E', b'L', b'F']);
    assert!(fails(&files).contains("tool.so"), "{}", fails(&files));
    files.remove("tool.so");
    files.insert("big.ts".into(), vec![b' '; 300 << 10]);
    let dir = tempfile::tempdir().unwrap();
    for (name, bytes) in &files {
        std::fs::write(dir.path().join(name), bytes).unwrap();
    }
    assert!(Bundle::read(dir.path()).unwrap_err().to_string().contains("at most"));
}

#[test]
fn a_zip_with_one_top_folder_installs_and_opens_again() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("sample.zip");
    let mut zip = zip::ZipWriter::new(std::fs::File::create(&path).unwrap());
    let manifest = common::manifest(&["api.example.test"], "null", "[]");
    let files = [
        ("sample/reiwand-provider.json", manifest.as_str()),
        (
            "sample/main.ts",
            "import { two } from \"./lib/two.ts\";\nexport const list = async (): Promise<unknown> => two;",
        ),
        ("sample/lib/two.ts", "export const two: number = 2;"),
        ("sample/LICENSE", "CC0"),
    ];
    for (name, text) in files {
        zip.start_file(name, zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(text.as_bytes()).unwrap();
    }
    zip.finish().unwrap();
    let bundle = Bundle::read(&path).unwrap();
    assert_eq!(
        bundle.files.keys().collect::<Vec<_>>(),
        ["LICENSE", "lib/two.ts", "main.ts", "reiwand-provider.json"]
    );
    let root = dir.path().join("plugins");
    reiwand_plugin::install(&bundle, &root).unwrap();
    let again = Installed::find(&root, "sample").unwrap();
    assert_eq!(again.modules["lib/two.ts"].trim_end(), "export const two         = 2;");
    assert!(again.source().join("main.ts").exists());
    std::fs::write(root.join("sample/js/lib/two.js"), "export const two = 3;").unwrap();
    let changed = Installed::find(&root, "sample").unwrap_err().to_string();
    assert!(changed.contains("does not match its TypeScript source"), "{changed}");
    reiwand_plugin::install(&bundle, &root).unwrap();
    assert_eq!(Installed::all(&root).len(), 1);
    assert!(reiwand_plugin::remove(&root, "sample").unwrap());
    assert!(Installed::all(&root).is_empty());
}

#[test]
fn the_format_modules_have_no_hidden_values() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let rules = [
        Rule::SerdeDefault,
        Rule::SkipSerializing,
        Rule::UnwrapOr,
        Rule::DeriveDefault,
        Rule::OptionWithoutNull,
    ];
    let found: Vec<_> = scan(&[src.as_path()], &rules)
        .into_iter()
        .filter(|v| {
            ![
                "runner.rs",
                "child.rs",
                "strip.rs",
                "net.rs",
                "files.rs",
                "plugin.rs",
                "bundle.rs",
                "install.rs",
                "modules.rs",
                "provider.rs",
            ]
            .iter()
            .any(|f| v.file.ends_with(f))
        })
        .map(|v| format!("{}:{} {:?}", v.file.display(), v.line, v.rule))
        .collect();
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn an_offered_version_is_placed_against_the_installed_one() {
    let step = reiwand_plugin::step;
    assert_eq!(
        [
            step(None, "1.0.0"),
            step(Some("1.0.0"), "1.1.0"),
            step(Some("1.1.0"), "1.1.0"),
            step(Some("1.1.0"), "1.0.9")
        ],
        ["new", "newer", "same", "older"]
    );
}
