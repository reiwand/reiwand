use super::benchkit::factory;
use crate::bench::{Dir, Node, PANELS, Panel, Panel::*};
use crate::benchops::{Side, side_of, stack};
use crate::benchplace::{beside, close, closed, edge, reopen};
use crate::benchrule::{closed_fault, fault};

const SIDES: [Side; 4] = [Side::Left, Side::Right, Side::Up, Side::Down];

fn whole(root: &Node, what: &str) {
    let shut = closed(root);
    assert_eq!(fault(root, &shut), None, "{what} left a broken layout: {root:?}");
    assert_eq!(closed_fault(root, &shut), None, "{what} closed a panel twice");
    let mut all: Vec<Panel> = root.groups().into_iter().flat_map(|(_, g)| g.panels.clone()).collect();
    all.extend(&shut);
    all.sort_by_key(|p| PANELS.iter().position(|q| q == p));
    assert_eq!(all, PANELS, "{what} lost or doubled a panel");
}

fn grouped(root: &Node) -> Vec<Vec<Panel>> {
    let mut groups: Vec<Vec<Panel>> = root.groups().into_iter().map(|(_, g)| g.panels.clone()).collect();
    groups.iter_mut().for_each(|g| g.sort_by_key(|p| PANELS.iter().position(|q| q == p)));
    groups
}

#[test]
fn a_side_of_the_cross_splits_the_group_under_it_in_two() {
    let root = factory();
    let next = beside(&root, Assets, Properties, Side::Up).unwrap();
    whole(&next, "putting the assets over the properties");
    let Some(Node::Split(s)) = next.at(&next.path_of(Properties).unwrap()[..1]) else { panic!("{next:?}") };
    assert_eq!((s.dir, s.shares.clone()), (Dir::Column, vec![0.5, 0.5]));
    assert_eq!(next.path_of(Assets).unwrap(), [2, 0]);
    let left = beside(&root, Timeline, Canvas, Side::Left).unwrap();
    whole(&left, "putting the timeline left of the canvas");
    assert_eq!(left.path_of(Timeline).unwrap(), [1, 0, 0]);
    assert_eq!(side_of(&left, Timeline), Some(Side::Left));
    let own = beside(&root, Assets, Layers, Side::Down).unwrap();
    assert_eq!(own.group_of(Assets).unwrap().panels, [Assets]);
    assert_eq!(beside(&root, Properties, Properties, Side::Down), None, "a lone panel split its own group");
    assert_eq!(beside(&root, Canvas, Layers, Side::Down), None, "the canvas moved");
}

#[test]
fn an_edge_target_takes_a_quarter_along_the_whole_side() {
    let root = factory();
    for side in SIDES {
        let next = edge(&root, Layers, side).unwrap();
        whole(&next, &format!("the layers along the {side:?} edge"));
        let Node::Split(s) = &next else { panic!("{next:?}") };
        assert_eq!(s.dir, side.dir());
        let at = if side.forward() { s.nodes.len() - 1 } else { 0 };
        assert_eq!(next.path_of(Layers), Some(vec![at]), "{side:?}");
        assert!((s.shares[at] - 0.25).abs() < 1e-9, "{side:?}: {:?}", s.shares);
    }
    assert_eq!(edge(&root, Canvas, Side::Left), None);
}

#[test]
fn a_closed_panel_leaves_the_tree_and_comes_back_with_its_neighbours() {
    let root = factory();
    let shut = close(&root, Assets).unwrap();
    whole(&shut, "closing the assets");
    assert_eq!(closed(&shut), [Assets]);
    assert_eq!(close(&shut, Assets), None, "a closed panel closed again");
    assert_eq!(close(&root, Canvas), None, "the canvas closed");
    let back = reopen(&shut, &factory(), Assets).unwrap();
    whole(&back, "opening the assets again");
    assert_eq!(back.group_of(Assets).unwrap().panels, [Layers, Options, Try, Effects, Assets]);
    assert_eq!(back.group_of(Assets).unwrap().shown, Assets);
    assert_eq!(reopen(&back, &factory(), Assets), None, "an open panel opened twice");
    let into = stack(&shut, Assets, Properties).unwrap();
    assert_eq!(into.group_of(Assets).unwrap().panels, [Properties, Assets]);
}

#[test]
fn closing_every_panel_and_opening_them_again_gives_the_usual_groups() {
    let mut held = factory();
    for panel in PANELS {
        if let Some(next) = close(&held, panel) {
            whole(&next, &format!("closing {panel:?}"));
            held = next;
        }
    }
    assert_eq!(held, crate::bench::group(&[Canvas]));
    assert_eq!(closed(&held).len(), PANELS.len() - 1);
    for panel in PANELS.into_iter().rev() {
        if let Some(next) = reopen(&held, &factory(), panel) {
            whole(&next, &format!("opening {panel:?}"));
            held = next;
        }
    }
    assert!(closed(&held).is_empty());
    assert_eq!(grouped(&held), grouped(&factory()));
    for panel in [Layers, Properties, Timeline] {
        assert_eq!(side_of(&held, panel), side_of(&factory(), panel), "{panel:?}");
    }
}

#[test]
fn every_target_keeps_the_layout_whole() {
    let mut held = factory();
    for panel in PANELS {
        for target in PANELS {
            for side in SIDES {
                if let Some(next) = beside(&held, panel, target, side) {
                    whole(&next, &format!("{panel:?} {side:?} of {target:?}"));
                    held = next;
                }
            }
        }
        if let Some(next) = close(&held, panel).and_then(|shut| edge(&shut, panel, Side::Down)) {
            whole(&next, &format!("{panel:?} reopened along the bottom"));
            held = next;
        }
    }
}
