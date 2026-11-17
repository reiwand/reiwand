mod mutate;
mod mutated;

use mutate::{EXAMPLES, Rng, capped, example, files};
use mutated::{opened, played, zipped};
use reiwand_format::Manifest;
use reiwand_scene::Scene;
use std::collections::BTreeMap;

fn with(files: &[(String, Vec<u8>)], name: &str, data: Vec<u8>) -> Vec<(String, Vec<u8>)> {
    let mut out = files.to_vec();
    out.iter_mut().filter(|(n, _)| n == name).for_each(|(_, d)| *d = data.clone());
    out
}

fn text(files: &[(String, Vec<u8>)], name: &str) -> Option<String> {
    files.iter().find(|(n, _)| n == name).map(|(_, d)| String::from_utf8(d.clone()).unwrap())
}

#[test]
fn a_damaged_scene_in_a_package_is_refused_or_plays() {
    let mut rng = Rng(0x0bad_5eed_1234_5678);
    for name in EXAMPLES {
        let files = files(&example(name));
        let Some(scene) = text(&files, "scene.json") else { continue };
        let mut accepted = 0;
        for round in 0..120 {
            let changed = if round % 3 == 0 {
                mutate::bytes(&mut rng, scene.as_bytes())
            } else {
                mutate::json(&mut rng, &scene).into_bytes()
            };
            let bytes = zipped(&with(&files, "scene.json", changed));
            let got = capped(format!("{name} scene round {round}"), move || opened(bytes));
            accepted += got.is_ok() as usize;
        }
        assert!(accepted > 3, "{name}: only {accepted} damaged scenes still loaded");
    }
}

#[test]
fn a_damaged_manifest_in_a_package_is_refused_or_read() {
    let mut rng = Rng(0x5151_7a7a_0f0f_3c3c);
    for name in EXAMPLES {
        let files = files(&example(name));
        let manifest = text(&files, "manifest.json").unwrap();
        for round in 0..80 {
            let changed = if round % 3 == 0 {
                mutate::bytes(&mut rng, manifest.as_bytes())
            } else {
                mutate::json(&mut rng, &manifest).into_bytes()
            };
            let alone = changed.clone();
            capped(format!("{name} manifest alone {round}"), move || drop(Manifest::from_json(&alone)));
            let bytes = zipped(&with(&files, "manifest.json", changed));
            let _ = capped(format!("{name} manifest round {round}"), move || opened(bytes));
        }
    }
}

#[test]
fn a_damaged_scene_read_from_a_folder_is_refused_or_plays() {
    let mut rng = Rng(0x1234_dead_beef_0042);
    for name in EXAMPLES {
        let files = files(&example(name));
        let Some(scene) = text(&files, "scene.json") else { continue };
        let manifest = Manifest::from_json(text(&files, "manifest.json").unwrap().as_bytes()).unwrap();
        let mut played_ok = 0;
        for round in 0..200 {
            let changed = mutate::json(&mut rng, &scene);
            let props = manifest.properties.clone();
            let what =
                format!("{name} folder round {round}: {}", changed.chars().take(300).collect::<String>());
            played_ok += capped(what, move || Scene::load(&changed, &props).map(played).is_ok()) as usize;
        }
        assert!(played_ok > 10, "{name}: only {played_ok} damaged scenes still loaded");
    }
}

fn nested(depth: usize) -> String {
    let mut layer =
        r##"{ "blend_space": "canvas", "id":"s","type":"solid","color":"#ffffff","size":[1,1]}"##.to_string();
    for level in 0..depth {
        layer = format!(r#"{{ "blend_space": "canvas", "id":"g{level}","type":"group","layers":[{layer}]}}"#);
    }
    format!(r#"{{"schema":"{}","canvas":{{"size":[100,100]}},"layers":[{layer}]}}"#, reiwand_scene::SCHEMA)
}

#[test]
fn a_scene_nested_too_deep_is_refused() {
    assert!(Scene::load(&nested(3), &BTreeMap::new()).is_ok());
    let scene = nested(2000);
    let loaded = capped("deep groups".into(), move || Scene::load(&scene, &BTreeMap::new()).is_err());
    assert!(loaded, "a scene with 2000 nested groups was accepted");
}

#[test]
fn damaged_settings_to_import_are_refused_or_kept_in_range() {
    let mut rng = Rng(0x7a7a_1b1b_c3c3_0d0d);
    for name in EXAMPLES {
        let files = files(&example(name));
        let manifest = Manifest::from_json(text(&files, "manifest.json").unwrap().as_bytes()).unwrap();
        let values = manifest.properties.iter().map(|(k, p)| (k.clone(), p.default_value()));
        let exported = reiwand_format::values::export(&manifest.id, &manifest.version, values.collect());
        for round in 0..60 {
            let changed = mutate::json(&mut rng, &exported);
            let (props, id) = (manifest.properties.clone(), manifest.id.clone());
            capped(format!("{name} settings {round}: {changed}"), move || {
                let Ok((kept, _)) = reiwand_format::values::import(&changed, &id, &props) else { return };
                for (key, value) in kept {
                    assert_eq!(props[&key].value(&value).as_ref(), Ok(&value), "{key}");
                }
            });
        }
    }
}
