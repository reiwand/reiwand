use crate::bench::{Node, Panel::*};
use crate::benchdeskqt::old::{carried, forget, tree, value, without};
use crate::benchkits::{Kit, factory};

#[test]
fn the_arrangement_of_earlier_versions_becomes_the_scene_layout() {
    let conf = "[General]\nx=1\n\n[editor]\narrangement=\"{\\\"places\\\":{\\\"left\\\":[\\\"assets\\\",\\\"layers\\\"],\\\"right\\\":[\\\"properties\\\",\\\"try\\\"],\\\"bottom\\\":[\\\"timeline\\\"]},\\\"sizes\\\":{\\\"left\\\":350}}\"\n";
    let text = value(conf, "editor", "arrangement").unwrap();
    assert!(text.starts_with("{\"places\""), "{text}");
    let root = tree(&text).unwrap();
    assert_eq!(root.group_of(Assets).unwrap().panels, [Assets, Layers]);
    assert_eq!(root.group_of(Try).unwrap().panels, [Properties, Try]);
    assert_eq!(root.group_of(Timeline).unwrap().panels, [Timeline]);
    let Node::Split(top) = &root else { panic!("one group") };
    assert!((top.shares[0] - 350.0 / 1920.0).abs() < 1e-9, "{:?}", top.shares);
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("window.conf"), conf).unwrap();
    let desk = carried(&dir.path().join("window.conf")).unwrap();
    assert_eq!(desk.root(), &root);
    assert!(
        desk.view().closed.contains(&Problems)
            && desk.factory.get(Kit::Particles).root == factory(Kit::Particles)
    );
    assert!(tree("{\"places\": {\"left\": [\"nothing\"]}}").is_none() && tree("not json").is_none());
}

#[test]
fn the_old_arrangement_leaves_window_conf_and_nothing_else_does() {
    let conf =
        "[General]\nx=1\n\n[editor]\nzoom=2\narrangement=\"{}\"\nfold=true\n\n[other]\narrangement=kept\n";
    let rest = without(conf, "editor", "arrangement").unwrap();
    assert_eq!(rest, "[General]\nx=1\n\n[editor]\nzoom=2\nfold=true\n\n[other]\narrangement=kept\n");
    assert!(without(&rest, "editor", "arrangement").is_none());
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("window.conf");
    std::fs::write(&file, conf).unwrap();
    forget(&file).unwrap();
    assert_eq!(std::fs::read_to_string(&file).unwrap(), rest);
    forget(&file).unwrap();
    assert_eq!(std::fs::read_to_string(&file).unwrap(), rest);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1, "no file is left beside it");
}
