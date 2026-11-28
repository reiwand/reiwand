use super::benchkit::factory;
use crate::bench::{Dir, Node, PANELS, Panel, Panel::*, group, split};
use crate::benchops::{Side, neighbour, show, side_of, split_off, stack, step, tidy};
use crate::benchrule::fault;

fn whole(root: &Node, what: &str) {
    assert_eq!(fault(root, &[]), None, "{what} left a broken layout: {root:?}");
    let mut all = root.panels();
    all.sort_by_key(|p| PANELS.iter().position(|q| q == p));
    assert_eq!(all, PANELS, "{what} lost or doubled a panel");
}

#[test]
fn the_usual_layout_holds_the_panels_where_they_were() {
    let root = factory();
    whole(&root, "the usual layout");
    let groups: Vec<Vec<Panel>> = root.groups().into_iter().map(|(_, g)| g.panels.clone()).collect();
    let usual = [
        vec![Layers, Assets, Options, Try, Effects],
        vec![Canvas],
        vec![
            Timeline, Curves, Problems, ViewAs, Inputs, Screens, Sheet, Texts, Cover, Review, Particles,
            Cost, Scripts, Code, Reference, Console,
        ],
        vec![Properties],
    ];
    assert_eq!(groups, usual);
    assert_eq!(root.path_of(Properties), Some(vec![2]));
    assert_eq!(root.path_of(Timeline), Some(vec![1, 1]));
}

#[test]
fn the_menu_moves_a_panel_to_the_group_on_that_side_past_the_canvas() {
    let root = factory();
    let moved = step(&root, Properties, Side::Left).unwrap();
    whole(&moved, "moving the properties to the left");
    assert_eq!(
        moved.group_of(Properties).unwrap().panels,
        [Layers, Assets, Options, Try, Effects, Properties]
    );
    assert_eq!(moved.group_of(Properties).unwrap().shown, Properties);
    assert_eq!(moved.groups().len(), 3, "the empty right side stayed");
    let back = step(&moved, Properties, Side::Right).unwrap();
    whole(&back, "moving it back");
    assert_eq!(back.groups().len(), 4);
    assert_eq!(side_of(&back, Properties), Some(Side::Right));
    let low = step(&root, Problems, Side::Up).unwrap();
    assert_eq!(low.group_of(Problems).unwrap().panels, [Problems]);
    assert_eq!(low.path_of(Problems), Some(vec![0]), "past the canvas it did not open at the top");
    assert_eq!(side_of(&low, Problems), Some(Side::Up));
}

#[test]
fn a_panel_alone_at_an_edge_cannot_move_out_of_it() {
    let root = factory();
    assert_eq!(step(&root, Properties, Side::Right), None);
    assert_eq!(step(&root, Canvas, Side::Up), None);
    assert_eq!(neighbour(&root, Layers, Side::Right), Some(vec![2]));
    assert_eq!(neighbour(&root, Canvas, Side::Down), Some(vec![1, 1]));
    let down = step(&root, Layers, Side::Down).unwrap();
    whole(&down, "moving the layers down");
    let Some(Node::Split(left)) = down.at(&[0]) else { panic!("the left side did not split: {down:?}") };
    assert_eq!(left.dir, Dir::Column);
    assert_eq!(down.group_of(Layers).unwrap().panels, [Layers]);
}

#[test]
fn every_move_keeps_the_layout_whole() {
    let sides = [Side::Left, Side::Right, Side::Up, Side::Down];
    let mut held = factory();
    for panel in PANELS {
        for beside in PANELS {
            if let Some(next) = stack(&held, panel, beside) {
                whole(&next, &format!("{panel:?} stacked with {beside:?}"));
                assert_eq!(next.path_of(panel), next.path_of(beside));
                held = next;
            }
            for side in sides {
                if let Some(next) = step(&held, panel, side) {
                    whole(&next, &format!("{panel:?} moved {side:?}"));
                    held = next;
                }
            }
        }
    }
    assert_eq!(stack(&held, Layers, Canvas), None, "a panel joined the canvas");
    assert_eq!(stack(&held, Canvas, Layers), None, "the canvas left its place");
}

#[test]
fn showing_a_tab_and_splitting_a_group_keep_the_rest() {
    let root = factory();
    let shown = show(&root, Try).unwrap();
    assert_eq!(shown.group_of(Try).unwrap().shown, Try);
    assert_eq!(shown.groups().len(), root.groups().len());
    let off = split_off(&root, Assets, Side::Right).unwrap();
    whole(&off, "splitting the assets off");
    assert_eq!(off.groups().len(), 5);
    assert_eq!(split_off(&root, Properties, Side::Left), None, "a lone panel split off itself");
}

#[test]
fn tidying_folds_splits_of_one_node_and_of_the_same_way() {
    let inner = split(Dir::Row, vec![(1.0, group(&[Layers])), (1.0, group(&[Assets]))]);
    let empty = Node::Group(crate::bench::Group { panels: vec![], shown: Try });
    let root = split(Dir::Row, vec![(2.0, inner), (1.0, empty), (1.0, group(&[Canvas]))]);
    let Some(Node::Split(s)) = tidy(root) else { panic!("the split went away") };
    assert_eq!(s.nodes.len(), 3);
    let shares: Vec<f64> = s.shares.iter().map(|x| (x * 1000.0).round() / 1000.0).collect();
    assert_eq!(shares, [0.333, 0.333, 0.333]);
    assert_eq!(tidy(split(Dir::Column, vec![(1.0, group(&[Canvas]))])), Some(group(&[Canvas])));
}

#[test]
fn a_broken_layout_is_named_by_its_place() {
    let mut root = factory();
    let Some(Node::Split(s)) = root.at_mut(&[1]) else { unreachable!() };
    s.shares = vec![0.5, -0.5];
    assert_eq!(fault(&root, &[]).unwrap().0, "split.nodes[1].split.shares[1]");
    let mut root = factory();
    let Some(Node::Group(g)) = root.at_mut(&[0]) else { unreachable!() };
    g.shown = Properties;
    assert_eq!(fault(&root, &[]).unwrap().0, "split.nodes[0].group.shown");
    let Some(Node::Group(g)) = root.at_mut(&[0]) else { unreachable!() };
    g.panels.push(Properties);
    assert_eq!(fault(&root, &[]).unwrap().0, "split.nodes[2].group.panels[0]");
    let lonely = split(Dir::Row, vec![(1.0, group(&[Canvas])), (1.0, group(&[Layers]))]);
    assert_eq!(fault(&lonely, &[]).unwrap().1, "the panel \"assets\" is in no group and is not closed");
    let same = split(Dir::Row, vec![(1.0, lonely.clone()), (1.0, group(&[Assets]))]);
    assert_eq!(fault(&same, &[]).unwrap().0, "split.nodes[0].split.dir");
}

#[path = "benchcanvas.rs"]
mod canvas;
