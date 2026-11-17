use super::*;

#[test]
fn released_scenes_keep_their_meaning() {
    let look = json!({ "properties": { "look": { "type": "combo", "options": ["media", "audio", "cursor"], "default": "media" } } });
    let mapped = json!({ "prop": "look", "map": { "media": "sky", "audio": "sky", "cursor": "sky" } });
    let layers = json!([{ "blend_space": "canvas", "id": "sky", "type": "image", "image": mapped },
        { "blend_space": "canvas", "id": "time", "type": "text", "text": { "clock": "HH:mm Sydney, Madrid" } }]);
    let package = load(&manifest(&["properties2"], look.clone()), &scene(layers, json!({})), &[]).unwrap();
    assert!(package.facts.inputs.is_empty() && !package.facts.features.contains("clock_dates"));
    let dated =
        json!([{ "blend_space": "canvas", "id": "day", "type": "text", "text": { "date": "EEEE d MMMM" } }]);
    let fault = load(&manifest(&["properties2"], look), &scene(dated, json!({})), &[]).map(drop).unwrap_err();
    assert!(fault.to_string().contains("add \"clock_dates\""), "{fault}");
}

#[test]
fn released_scenes_keep_their_own_locale_files() {
    let plain =
        scene(json!([{ "blend_space": "canvas", "id": "sky", "type": "image", "image": "sky" }]), json!({}));
    let files: [(&str, &[u8]); 2] =
        [("locale/data.json", br#"{"stars": 120}"#), ("locale/es.json", br#"{"hello": "x"}"#)];
    let package = load(&manifest(&[], json!({})), &plain, &files).unwrap();
    assert!(!package.facts.features.contains("locale_files") && package.manifest.string_files.is_empty());
    let declared = load(&manifest(&["locale_files"], json!({})), &plain, &files).map(drop).unwrap_err();
    assert!(declared.to_string().contains("locale/data.json"), "{declared}");
    let texts = json!({ "default_locale": "en", "title": "@title",
        "strings": { "en": { "title": "Dusk" }, "es": { "title": "Atardecer" } } });
    let source = tempfile::tempdir().unwrap();
    std::fs::create_dir(source.path().join("locale")).unwrap();
    let png = image(16, 16, ImageFormat::Png);
    let text = (manifest(&[], texts).to_json().unwrap(), plain.to_string());
    let written: [(&str, &[u8]); 7] = [
        ("manifest.json", text.0.as_bytes()),
        ("scene.json", text.1.as_bytes()),
        ("preview.png", &png),
        ("sky.png", &png),
        ("locale/data.json", br#"{"stars": 120}"#),
        ("locale/fr.json", br#"{"Good morning": "Bonjour"}"#),
        ("locale/es.json", br#"{"hello": "hola"}"#),
    ];
    written.iter().for_each(|(name, data)| std::fs::write(source.path().join(name), data).unwrap());
    let (manifest, facts) = reiwand_format::inspect(source.path()).unwrap();
    features::check(&manifest, &facts).unwrap();
    assert!(manifest.string_files.is_empty() && !facts.features.contains("locale_files"));
    let noted: Vec<&str> = facts.notes.iter().filter_map(|n| n.split(' ').next()).collect();
    assert_eq!(noted, ["locale/es.json", "locale/fr.json"]);
    assert!(reiwand_format::fill(source.path()).unwrap().is_empty());
    let out = source.path().join("dusk.wpak");
    reiwand_format::build(source.path(), &out, &Limits::default()).unwrap();
    let package = reiwand_format::open(&out, &Limits::default()).unwrap();
    assert!(package.manifest.features().is_empty() && package.manifest.string_files.is_empty());
    assert_eq!(package.manifest.title_in(&["fr".into()]), "Dusk");
}
