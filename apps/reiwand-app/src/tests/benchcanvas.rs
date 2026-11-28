use crate::bench::{Node, Panel::*};
use crate::benchops::{Side, step};
use crate::tests::benchkit::factory;

fn order(root: &Node) -> Vec<bool> {
    let Node::Split(s) = root else { panic!("the root is not a split: {root:?}") };
    s.nodes.iter().map(|n| n.path_of(Canvas).is_some()).collect()
}

#[test]
fn the_canvas_trades_places_with_its_neighbour_and_takes_its_column_along() {
    let root = factory();
    let left = step(&root, Canvas, Side::Left).unwrap();
    assert_eq!(order(&left), [true, false, false], "the canvas did not go left: {left:?}");
    assert_eq!(left.group_of(Timeline), root.group_of(Timeline), "the timeline did not go along");
    assert_eq!(step(&left, Canvas, Side::Left), None, "the canvas went past the edge");
    let back = step(&left, Canvas, Side::Right).unwrap();
    assert_eq!(back, root, "moving it back did not give the same layout");
    let under = step(&root, Canvas, Side::Down).unwrap();
    assert_eq!(under.path_of(Canvas), Some(vec![1, 1]), "the canvas did not go under the timeline");
    assert_eq!(under.group_of(Canvas).unwrap().panels, [Canvas], "the canvas shared its group");
}
