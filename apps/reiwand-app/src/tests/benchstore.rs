use super::benchkit::factory;
use crate::bench::{Node, Panel};
use crate::benchdesk::{Chosen, Desk};
use crate::benchkits::{Kit, full as kit};
use crate::benchops::{Side, step};
use crate::benchplace::close;
use crate::benchsize::{Measure, even};
use crate::benchstore::{Loaded, load_from, parse, save_to, text};
use reiwand_core::explicit::round_trip;
use serde_json::{Value, json};
use std::fs;
fn measure() -> Measure {
    serde_json::from_value(json!({
        "floors": { "canvas": [320, 160], "properties": [320, 88], "layers": [280, 88], "assets": [280, 88],
            "options": [280, 88], "try": [280, 88], "timeline": [280, 88], "problems": [280, 88] },
        "fixed": {}
    }))
    .unwrap()
}

fn mine() -> Desk {
    let moved = step(&kit(Kit::Scene), Panel::Assets, Side::Right).unwrap();
    Desk::from_trees(kit).put(moved).unwrap().save_as("Mine").unwrap()
}

#[test]
fn saved_layouts_read_back_the_same_bytes() {
    let desk = mine();
    assert_eq!(parse("workbench.json", &text(&desk)).unwrap(), desk);
    round_trip(&text(&desk), |again| parse("workbench.json", again).map(|d| text(&d))).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("reiwand/workbench.json");
    assert_eq!(load_from(&path), Loaded::Fresh);
    save_to(&path, &desk).unwrap();
    assert_eq!(load_from(&path), Loaded::Kept(Box::new(desk)));
}

#[test]
fn a_closed_panel_is_written_in_its_list_and_read_back() {
    let desk = Desk::from_trees(kit).put(close(&kit(Kit::Scene), Panel::Timeline).unwrap()).unwrap();
    let written: Value = serde_json::from_str(&text(&desk)).unwrap();
    assert_eq!(written["content"]["factory"]["scene"]["closed"], json!(["timeline"]));
    assert_eq!(written["content"]["factory"]["particles"]["closed"], json!([]));
    assert_eq!(parse("workbench.json", &text(&desk)).unwrap(), desk);
    let mut twice = written.clone();
    twice["content"]["factory"]["scene"]["closed"] = json!(["timeline", "problems"]);
    let said = parse("workbench.json", &twice.to_string()).unwrap_err();
    assert!(said.contains("\"content.factory.scene.closed[1]\" is not valid"), "{said}");
    twice["content"]["factory"]["scene"]["closed"] = json!(["canvas", "timeline"]);
    let said = parse("workbench.json", &twice.to_string()).unwrap_err();
    assert!(said.contains("\"content.factory.scene.closed[0]\" is not valid"), "{said}");
    twice["content"]["factory"]["scene"]["closed"] = json!([]);
    let said = parse("workbench.json", &twice.to_string()).unwrap_err();
    assert!(said.contains("\"timeline\" is in no group and is not closed"), "{said}");
}

#[test]
fn every_missing_key_is_refused_with_its_path() {
    let written: Value = serde_json::from_str(&text(&mine())).unwrap();
    let mut paths = Vec::new();
    gather(&written, String::new(), &mut paths);
    assert!(paths.len() > 100, "{paths:?}");
    for path in paths {
        let mut cut = written.clone();
        remove(&mut cut, &path);
        let said = parse("workbench.json", &cut.to_string()).unwrap_err();
        let shown = (path.split('/').skip(1))
            .map(|step| if step.parse::<usize>().is_ok() { format!("[{step}]") } else { format!(".{step}") })
            .collect::<String>();
        let shown = shown.trim_start_matches('.');
        assert!(said.contains(&format!("\"{shown}\" is missing")), "{path}: {said}");
    }
}

fn gather(value: &Value, here: String, out: &mut Vec<String>) {
    if let Value::Object(map) = value {
        for (key, child) in map {
            let at = format!("{here}/{key}");
            if !matches!(key.as_str(), "split" | "group" | "format") && here != "/content/chosen" {
                out.push(at.clone());
            }
            gather(child, at, out);
        }
    }
    if let Value::Array(list) = value {
        for (at, child) in list.iter().enumerate() {
            gather(child, format!("{here}/{at}"), out);
        }
    }
}

fn remove(value: &mut Value, path: &str) {
    let (parent, key) = path.rsplit_once('/').unwrap();
    value.pointer_mut(parent).unwrap().as_object_mut().unwrap().remove(key);
}

#[test]
fn another_format_or_a_broken_layout_is_refused_and_left_as_it_is() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workbench.json");
    let tree = serde_json::to_value(factory()).unwrap();
    let newer = json!({ "format": 9, "content": { "root": tree, "closed": [] } }).to_string();
    let old = newer.as_str();
    fs::write(&path, old).unwrap();
    let Loaded::Refused(said) = load_from(&path) else { panic!("a newer file was read") };
    assert!(said.contains("(format 9)") && said.contains("only format 8"), "{said}");
    assert_eq!(fs::read_to_string(&path).unwrap(), old);
    let plain = Desk::from_trees(kit).save_as("Mine").unwrap();
    let written: Value = serde_json::from_str(&text(&plain)).unwrap();
    let mut doubled = written.clone();
    let middle = &mut doubled["content"]["factory"]["scene"]["root"]["split"]["nodes"][1];
    middle["split"]["nodes"][1]["group"]["panels"] = json!(["timeline", "assets"]);
    let said = parse("workbench.json", &doubled.to_string()).unwrap_err();
    let place = "content.factory.scene.root.split.nodes[1].split.nodes[1].group.panels[1]";
    assert!(said.contains(&format!("\"{place}\"")), "{said}");
    let mut unknown = written.clone();
    unknown["content"]["factory"]["wallpaper"]["root"]["split"]["nodes"][1]["group"]["panels"] =
        json!(["inspector"]);
    let said = parse("workbench.json", &unknown.to_string()).unwrap_err();
    assert!(said.contains("\"content.factory.wallpaper.root.split.nodes[1].group.panels[0]\""), "{said}");
    let mut same = written.clone();
    same["content"]["own"] = json!([written["content"]["own"][0], written["content"]["own"][0]]);
    let said = parse("workbench.json", &same.to_string()).unwrap_err();
    assert!(said.contains("\"content.own[1].name\" is not valid"), "{said}");
    let mut lost = written.clone();
    lost["content"]["chosen"] = json!({ "own": "Gone" });
    let said = parse("workbench.json", &lost.to_string()).unwrap_err();
    assert!(said.contains("\"content.chosen.own\" is not valid"), "{said}");
    for broken in ["{not json", "[]"] {
        assert!(parse("workbench.json", broken).is_err(), "{broken}");
    }
    let bad: Desk = serde_json::from_value(doubled["content"].clone()).unwrap();
    assert!(save_to(&path, &bad).is_err(), "a broken layout was written");
    assert_eq!(fs::read_to_string(&path).unwrap(), old);
    assert_eq!(mine().chosen, Chosen::Own("Mine".to_owned()));
}

#[test]
fn sizes_fill_the_window_and_keep_every_floor() {
    let m = measure();
    for (width, height) in [(1600.0, 900.0), (1024.0, 640.0), (700.0, 400.0)] {
        let (boxes, edges) = m.boxes(&factory(), width, height);
        let groups: Vec<_> =
            boxes.iter().filter(|(p, _)| matches!(factory().at(p), Some(Node::Group(_)))).collect();
        let area: f64 = groups.iter().map(|(_, at)| at.width * at.height).sum();
        assert!((area - width * height).abs() < 1.0, "{width}: {area}");
        let canvas = boxes.iter().find(|(p, _)| *p == vec![1, 0]).unwrap().1;
        assert!(
            canvas.width >= 320f64.min(width / 2.0) - 1.0 && canvas.height >= 160.0,
            "{width}: {canvas:?}"
        );
        assert_eq!(edges.len(), 3);
    }
}

#[test]
fn an_edge_moves_between_its_floors_and_goes_back_to_its_place() {
    let m = measure();
    let root = factory();
    let wide = m.resize(&root, (1600.0, 900.0), &[], 0, 200.0).unwrap();
    let left = |r: &Node| m.boxes(r, 1600.0, 900.0).0.iter().find(|(p, _)| *p == vec![0]).unwrap().1.width;
    assert_eq!(left(&wide), left(&root) + 200.0);
    assert_eq!(left(&m.resize(&root, (1600.0, 900.0), &[], 0, -5000.0).unwrap()), 280.0);
    let most = m.resize(&root, (1600.0, 900.0), &[], 0, 5000.0).unwrap();
    let middle = m.boxes(&most, 1600.0, 900.0).0.iter().find(|(p, _)| *p == vec![1]).unwrap().1.width;
    assert_eq!(middle, 320.0, "the canvas went below its floor");
    assert_eq!(left(&even(&wide, &root, &[], 0).unwrap()), left(&root));
    let mut folded = measure();
    folded.fixed.insert(Panel::Timeline, 60.0);
    let (boxes, edges) = folded.boxes(&root, 1600.0, 900.0);
    assert_eq!(boxes.iter().find(|(p, _)| *p == vec![1, 1]).unwrap().1.height, 60.0);
    assert!(edges.iter().any(|e| e.fixed) && folded.resize(&root, (1600.0, 900.0), &[1], 0, 10.0).is_none());
}

#[path = "benchrails.rs"]
mod rails;

#[path = "benchold.rs"]
mod old;
#[path = "benchupgrade.rs"]
mod upgrade;

#[test]
fn a_package_kind_keeps_its_layouts_apart_from_the_scene() {
    use crate::benchstore::file_for;
    let scene = file_for("");
    assert!(scene.ends_with("workbench.json"));
    assert!(file_for("web").ends_with("workbench-web.json"));
    assert!(file_for("preset").ends_with("workbench-preset.json"));
    assert_eq!(file_for("../web"), scene);
    assert_eq!(file_for("Web"), scene);
}
