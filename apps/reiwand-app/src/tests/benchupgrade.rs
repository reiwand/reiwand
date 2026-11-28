use crate::bench::Panel;
use crate::benchdesk::Chosen;
use crate::benchdesk::Desk;
use crate::benchkits::{Kit, full as kit};
use crate::benchplace::close;
use crate::benchstore::{Loaded, load_from, parse, text};
use serde_json::{Value, json};

const NEWER: [&str; 6] = ["particles", "cost", "scripts", "code", "reference", "console"];

/// A view of a format 6 file: the panels of formats 7 and 8 nowhere, in its tree or among the closed.
fn older(value: &mut Value, scene: &Value) {
    match value {
        Value::Object(map) if map.contains_key("root") && map.contains_key("closed") => {
            map.insert("root".to_owned(), scene.clone());
            if let Some(Value::Array(closed)) = map.get_mut("closed") {
                closed.retain(|panel| !NEWER.iter().any(|newer| panel == newer));
            }
        }
        Value::Object(map) => map.values_mut().for_each(|inner| older(inner, scene)),
        Value::Array(items) => items.iter_mut().for_each(|inner| older(inner, scene)),
        _ => {}
    }
}

#[test]
fn a_format_6_file_is_read_with_the_new_panels_closed() {
    let desk = Desk::from_trees(kit).save_as("Mine").unwrap();
    let mut written: Value = serde_json::from_str(&text(&desk)).unwrap();
    let newer =
        [Panel::Particles, Panel::Cost, Panel::Scripts, Panel::Code, Panel::Reference, Panel::Console];
    let without = newer.into_iter().fold(kit(Kit::Scene), |tree, panel| close(&tree, panel).unwrap());
    let scene = serde_json::to_value(without).unwrap();
    written["format"] = Value::from(6);
    older(&mut written["content"], &scene);
    assert!(!written.to_string().contains("\"cost\"") && !written.to_string().contains("\"console\""));
    let read = parse("workbench.json", &written.to_string()).unwrap();
    let again: Value = serde_json::from_str(&text(&read)).unwrap();
    assert_eq!(again["format"], Value::from(8));
    for view in [
        &again["content"]["factory"]["scene"],
        &again["content"]["factory"]["particles"],
        &again["content"]["own"][0]["saved"],
    ] {
        let closed = view["closed"].as_array().unwrap();
        assert!(NEWER.iter().all(|panel| closed.contains(&Value::from(*panel))), "{view}");
    }
}

#[test]
fn a_file_of_the_first_formats_keeps_its_tree_as_the_scene_layout() {
    let first = [Panel::Canvas, Panel::Layers, Panel::Assets, Panel::Options, Panel::Try];
    let later = [Panel::Effects, Panel::ViewAs, Panel::Review, Panel::Console];
    let old = later.iter().fold(super::factory(), |tree, panel| close(&tree, *panel).unwrap_or(tree));
    let tree = serde_json::to_value(old).unwrap();
    for (format, content) in [(1, json!({ "root": tree })), (2, json!({ "root": tree, "closed": [] }))] {
        let read =
            parse("workbench.json", &json!({ "format": format, "content": content }).to_string()).unwrap();
        assert_eq!(serde_json::to_value(&read.factory.scene.root).unwrap(), tree, "format {format}");
        assert!(
            later.iter().all(|p| read.factory.scene.closed.contains(p)),
            "{:?}",
            read.factory.scene.closed
        );
        assert_eq!(read.factory.particles, Desk::default().factory.particles);
        assert!(first.iter().all(|p| !read.factory.scene.closed.contains(p)));
    }
}

#[test]
fn a_broken_layout_goes_back_to_how_it_comes_and_the_others_are_kept() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workbench.json");
    let desk = Desk::default().put(close(&kit(Kit::Scene), Panel::Timeline).unwrap()).unwrap();
    let desk = desk.save_as("Mine").unwrap();
    let mut written: Value = serde_json::from_str(&text(&desk)).unwrap();
    written["content"]["factory"]["wallpaper"]["root"] = json!({ "group": { "panels": ["canvas"] } });
    written["content"]["own"][0]["view"]["closed"] = json!(["nothing"]);
    std::fs::write(&path, written.to_string()).unwrap();
    let Loaded::Mended(read, said) = load_from(&path) else { panic!("the file was not mended") };
    assert!(said.contains("Wallpaper") && said.contains("Mine"), "{said}");
    assert_eq!(read.factory.scene, desk.factory.scene);
    assert_eq!(read.factory.wallpaper, Desk::default().factory.wallpaper);
    assert!(read.own.is_empty() && read.chosen == Chosen::Factory(Kit::Scene));
}

/// Every value of a saved file replaced by something odd, and the file cut short: each read ends
/// with the layouts kept, mended or refused, and never stops the program.
#[test]
fn an_odd_value_anywhere_is_read_mended_or_refused() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workbench.json");
    let written: Value = serde_json::from_str(&text(&Desk::default().save_as("Mine").unwrap())).unwrap();
    let odd =
        [json!(null), json!(-1), json!(1e308), json!(u64::MAX), json!(""), json!([]), json!({}), json!("x")];
    let mut places = Vec::new();
    walk(&written, String::new(), &mut places);
    let mut tried = 0;
    for place in &places {
        for value in &odd {
            let mut broken = written.clone();
            *broken.pointer_mut(place).unwrap() = value.clone();
            std::fs::write(&path, broken.to_string()).unwrap();
            let _ = load_from(&path);
            tried += 1;
        }
    }
    let whole = written.to_string();
    for cut in (0..whole.len()).step_by(97) {
        std::fs::write(&path, &whole[..cut]).unwrap();
        assert!(matches!(load_from(&path), Loaded::Refused(_) | Loaded::Fresh), "cut at {cut}");
    }
    assert!(tried > 1000, "{tried}");
}

fn walk(value: &Value, here: String, out: &mut Vec<String>) {
    out.push(here.clone());
    match value {
        Value::Object(map) => map.iter().for_each(|(k, v)| walk(v, format!("{here}/{k}"), out)),
        Value::Array(list) => list.iter().enumerate().for_each(|(i, v)| walk(v, format!("{here}/{i}"), out)),
        _ => {}
    }
}
