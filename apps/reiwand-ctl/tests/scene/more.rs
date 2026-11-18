use super::*;

pub fn ok_lines(dir: &Path) -> String {
    String::from_utf8(run(dir, &["pack", "check", "snow"]).stdout).unwrap()
}

#[test]
fn renaming_a_particle_system_warns_about_the_scripts_that_name_it() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "snow", "--from", "particles"]);
    std::fs::create_dir_all(dir.join("snow/scripts")).unwrap();
    std::fs::write(dir.join("snow/scripts/flakes.js"), "export const update = () => system('flakes');")
        .unwrap();
    let scripts = r#"{"pointer": "/scene/scripts", "value": [{"path": "scripts/flakes.js"}]}"#;
    ok(dir, &["scene", "act", "snow", "set", scripts]);
    let renamed =
        run(dir, &["scene", "act", "snow", "particles", r#"{"rename":{"system":"flakes","to":"embers"}}"#]);
    let warned = String::from_utf8(renamed.stderr).unwrap();
    assert!(
        renamed.status.success()
            && warned.contains("scripts/flakes.js:1 names \"flakes\"")
            && warned.contains("\"scripts\":{\"from\":\"flakes\",\"to\":\"embers\"}"),
        "{warned}"
    );
    assert_eq!(show(dir, "/scene/layers/1/system"), json!("embers"));
}

fn words(line: &str) -> Vec<String> {
    let (mut out, mut word, mut quote, mut started) = (Vec::new(), String::new(), None, false);
    for c in line.trim().chars() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), c) => word.push(c),
            (None, '\'' | '"') => (quote, started) = (Some(c), true),
            (None, ' ') => {
                if started {
                    out.push(std::mem::take(&mut word));
                }
                started = false;
            }
            (None, c) => {
                word.push(c);
                started = true;
            }
        }
    }
    out.extend(started.then_some(word));
    out
}

#[test]
fn the_scene_commands_of_the_docs_work_as_written() {
    let pages = [
        "usar/terminal.md",
        "usar/servidor.md",
        "crear/funciones-del-editor.md",
        "crear/otros-paquetes-del-editor.md",
    ];
    let docs = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs");
    let text: String = pages.iter().map(|p| std::fs::read_to_string(docs.join(p)).unwrap()).collect();
    let elsewhere = ["open", "import", "thumbnail", "preview", "apply", "bench", "add_layer"];
    let tmp = tempfile::tempdir().unwrap();
    let lines = text.lines().filter(|l| l.starts_with("reiwand-ctl scene "));
    let commands = lines.flat_map(|l| l.split("  #").next().unwrap_or_default().split(" && ")).map(words);
    let mut ran = 0;
    for args in commands
        .filter(|a| a[3] == "mar" && !elsewhere.contains(&a[2].as_str()))
        .filter(|a| a[2] != "act" || !elsewhere.contains(&a[4].as_str()))
    {
        ok(tmp.path(), &args[1..].iter().map(String::as_str).collect::<Vec<_>>());
        ran += 1;
    }
    assert!(ran >= 18, "only {ran} commands of the docs ran");
    assert_eq!(
        show_in(tmp.path(), "mar", "/manifest/properties/velocidad/label"),
        json!({"en": "Speed", "es": "Velocidad"})
    );
}

#[test]
fn scripts_are_imported_with_what_they_declare() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "snow", "--from", "empty", "--title", "Snow"]);
    let module = r#"import { scene, audio, setInterval, sound } from "reiwand";
        import { Vec2 } from "reiwand/math";
        export function init(ctx) { scene.spawn("bar"); scene.layer("bar_1").on("click", () => {}); }
        export const spin = (value, ctx) => value + 1;
        export function update() {}"#;
    std::fs::write(dir.join("bars.js"), module).unwrap();
    ok(dir, &["scene", "act", "snow", "import_script", &script(dir, "bars.js")]);
    let entry = json!({ "path": "scripts/bars.js", "events": ["init", "update", "click"], "uses": ["audio", "timers", "layers", "sounds", "math"], "params": {} });
    assert_eq!(show(dir, "/scene/scripts"), json!([entry]));
    assert!(dir.join("snow/scripts/bars.js").is_file());
    std::fs::write(dir.join("big.js"), " ".repeat((1 << 20) + 1)).unwrap();
    assert!(fails(dir, &["scene", "act", "snow", "import_script", &script(dir, "big.js")]).contains("1 MiB"));
    assert!(
        fails(dir, &["scene", "act", "snow", "import_script", &script(dir, "missing.js")])
            .contains("cannot read")
    );
}

fn script(dir: &Path, name: &str) -> String {
    json!({ "file": dir.join(name) }).to_string()
}

#[test]
fn importing_a_module_again_says_what_changed() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "snow", "--from", "empty"]);
    std::fs::write(dir.join("bars.js"), "export function update() {}").unwrap();
    ok(dir, &["scene", "act", "snow", "import_script", &script(dir, "bars.js")]);
    let params =
        json!({"pointer": "/scene/scripts/0/params", "value": {"speed": {"type": "number", "default": 1}}});
    ok(dir, &["scene", "act", "snow", "set", &params.to_string()]);
    std::fs::write(dir.join("bars.js"), "export function click() {}").unwrap();
    let again = run(dir, &["scene", "act", "snow", "import_script", &script(dir, "bars.js")]);
    let said = String::from_utf8_lossy(&again.stderr).into_owned();
    assert!(said.contains("scripts/bars.js now handles click"), "{said}");
    assert!(said.contains("scripts/bars.js kept its 1 settings"), "{said}");
    assert_eq!(show(dir, "/scene/scripts/0/params/speed/default"), 1);
}
