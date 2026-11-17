use super::*;

#[test]
fn scripts_declare_their_inputs_by_what_they_import() {
    let source = r#"
        // import { audio } from "reiwand";
        const text = "import { input } from 'reiwand'";
        import { scene, particles as p, setTimeout } from "reiwand";
        import * as math from "reiwand/math";
    "#;
    assert_eq!(imported(source), [ScriptApi::Timers, ScriptApi::Particles, ScriptApi::Math]);
    assert_eq!(imported("import * as w from 'reiwand'"), [ScriptApi::Audio, ScriptApi::Pointer]);
    assert_eq!(imported("import { f } from './lib.js'"), [ScriptApi::Shared]);
    assert_eq!(imported("import './lib.js'; import('./x.js');"), [ScriptApi::Shared]);
    assert_eq!(
        imported("import { audio as a, input } from \"reiwand\""),
        [ScriptApi::Audio, ScriptApi::Pointer]
    );
    let scene = Scene::from_json(&reiwand_upgrade::fill::whole(
        &&json!({ "schema": SCHEMA, "canvas": { "size": [10, 10] },
        "scripts": [{ "path": "a.js" }, { "path": "b.js", "uses": ["timers"] }] })
        .to_string(),
    ))
    .unwrap();
    let files = [
        ("a.js".to_string(), "/* audio cursor */ export const x = 1;".to_string()),
        ("b.js".into(), "import { audio, input } from 'reiwand';".into()),
    ];
    assert_eq!(script_inputs(&scene, &files), [false, false]);
    let files = [("a.js".to_string(), "import { audio } from 'reiwand';".to_string())];
    assert_eq!(script_inputs(&scene, &files), [true, false]);
}
