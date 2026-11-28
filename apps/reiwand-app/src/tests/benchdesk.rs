use crate::bench::{Node, PANELS, Panel::*};
use crate::benchdesk::{Chosen, Desk};
use crate::benchkits::{KITS, Kit, View, factory, full};
use crate::benchops::{Side, side_of, step};
use crate::benchplace::close;
use crate::benchplan::blueprint;

#[test]
fn every_full_layout_holds_every_panel_once() {
    for kit in KITS {
        let root = full(kit);
        assert_eq!(View::of(root.clone()).fault(), None, "{kit:?}");
        let mut panels = root.panels();
        panels.sort_by_key(|panel| PANELS.iter().position(|p| p == panel));
        assert_eq!(panels, PANELS, "{kit:?}");
    }
}

#[test]
fn every_layout_the_editor_comes_with_holds_its_own_panels_and_closes_the_rest() {
    for kit in KITS {
        let root = factory(kit);
        let view = View::of(root.clone());
        assert_eq!(view.fault(), None, "{kit:?}");
        let panels = root.panels();
        assert!(panels.contains(&Canvas) && panels.len() <= 8, "{kit:?}: {panels:?}");
        assert_eq!(panels.len() + view.closed.len(), PANELS.len(), "{kit:?}");
        let plan = blueprint(&root, 30, 20);
        assert_eq!(plan.len(), root.groups().len(), "{kit:?}");
        assert_eq!(plan.iter().filter(|p| p.canvas).count(), 1, "{kit:?}");
        let area: i32 = plan.iter().map(|p| p.width * p.height).sum();
        assert_eq!(area, 600, "{kit:?}: {plan:?}");
    }
}

#[test]
fn the_scene_layout_has_assets_left_the_canvas_in_the_middle_and_layers_right() {
    let root = factory(Kit::Scene);
    assert_eq!(side_of(&root, Assets), Some(Side::Left));
    assert_eq!(side_of(&root, Properties), Some(Side::Right));
    assert_eq!(side_of(&root, Layers), Some(Side::Right));
    assert_eq!(root.path_of(Canvas), Some(vec![1, 0]));
    assert_eq!(root.path_of(Timeline), Some(vec![1, 1]));
    assert_eq!(root.group_of(Problems).unwrap().panels, [Timeline, Problems]);
    assert_eq!(root.path_of(Properties), Some(vec![2, 0]));
    assert_eq!(root.path_of(Layers), Some(vec![2, 1]));
    let Node::Split(top) = &root else { panic!("one group") };
    assert!(top.shares[1] > top.shares[0] + top.shares[2], "the canvas is not the largest area");
}

#[test]
fn each_layout_keeps_its_own_changes_and_resets_alone() {
    let desk = Desk::default();
    let moved = step(desk.root(), Assets, Side::Right).unwrap();
    let changed = desk.put(moved.clone()).unwrap();
    let particles = changed.choose(Chosen::Factory(Kit::Particles)).unwrap();
    assert_eq!(particles.root(), &factory(Kit::Particles));
    let back = particles.choose(Chosen::Factory(Kit::Scene)).unwrap();
    assert_eq!(back.root(), &moved);
    assert_eq!(back.reset().root(), &factory(Kit::Scene));
    assert_eq!(back.reset().factory.get(Kit::Particles).root, factory(Kit::Particles));
    let broken = crate::bench::group(&[Canvas]);
    assert!(desk.put(crate::bench::split(crate::bench::Dir::Row, vec![(1.0, broken)])).is_none());
}

#[test]
fn a_layout_of_the_author_is_saved_renamed_reset_and_removed() {
    let desk = Desk::default();
    let shut = close(desk.root(), Timeline).unwrap();
    let mine = desk.put(shut.clone()).unwrap().save_as("Grading").unwrap();
    assert_eq!(mine.chosen, Chosen::Own("Grading".to_owned()));
    assert_eq!(mine.root(), &shut);
    assert_eq!(mine.usual(), shut);
    assert!(mine.name_fault("Grading", None).is_some() && mine.save_as("Grading").is_none());
    assert!(mine.name_fault(" ", None).is_some() && mine.name_fault(" Grading", None).is_some());
    assert_eq!(mine.name_fault("Grading", Some(0)), None);
    let moved = mine.put(step(&shut, Assets, Side::Right).unwrap()).unwrap();
    assert_eq!(moved.reset().root(), &shut, "reset does not go back to the saved layout");
    let renamed = moved.rename(0, "Color").unwrap();
    assert_eq!(renamed.chosen, Chosen::Own("Color".to_owned()));
    assert!(renamed.rename(1, "Other").is_none());
    assert_eq!(renamed.keyed(7), Some(Chosen::Own("Color".to_owned())));
    assert_eq!(renamed.keyed(8), None);
    assert_eq!(renamed.keyed(2), Some(Chosen::Factory(Kit::Particles)));
    assert!(renamed.choose(Chosen::Own("Gone".to_owned())).is_none());
    let removed = renamed.remove(0).unwrap();
    assert!(removed.own.is_empty());
    assert_eq!(removed.chosen, Chosen::Factory(Kit::Scene));
    assert_eq!(removed.root(), &shut, "removing an own layout changed the Scene layout");
}

#[test]
fn a_new_scene_opens_in_scene_and_other_packages_in_wallpaper() {
    let desk = Desk::default().choose(Chosen::Factory(Kit::Animation)).unwrap();
    assert_eq!(desk.opening("scene", true).chosen, Chosen::Factory(Kit::Scene));
    assert_eq!(desk.opening("scene", false).chosen, Chosen::Factory(Kit::Animation));
    assert_eq!(desk.opening("assets", true).chosen, Chosen::Factory(Kit::Animation));
    for kind in ["web", "image", "video", "preset"] {
        assert_eq!(desk.opening(kind, false).chosen, Chosen::Factory(Kit::Wallpaper), "{kind}");
    }
}
