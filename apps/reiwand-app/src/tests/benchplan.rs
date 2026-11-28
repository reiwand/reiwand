use super::benchkit::factory;
use crate::bench::{Dir, Node, Panel::*, group, split};
use crate::benchops::Side;
use crate::benchplace::beside;
use crate::benchplan::{Plan, blueprint};

fn tiled(plan: &[Plan], width: i32, height: i32) {
    let mut cells = vec![0u8; (width * height) as usize];
    for p in plan {
        assert!(p.width >= 1 && p.height >= 1, "an empty box: {p:?}");
        assert!(p.x >= 0 && p.y >= 0 && p.x + p.width <= width && p.y + p.height <= height, "outside: {p:?}");
        for y in p.y..p.y + p.height {
            for x in p.x..p.x + p.width {
                cells[(y * width + x) as usize] += 1;
            }
        }
    }
    assert!(cells.iter().all(|c| *c == 1), "the boxes do not tile {width} x {height}: {plan:?}");
}

#[test]
fn the_usual_layout_draws_its_four_groups_with_the_canvas_filled() {
    let plan = blueprint(&factory(), 18, 12);
    tiled(&plan, 18, 12);
    let boxes: Vec<_> = plan.iter().map(|p| (p.x, p.y, p.width, p.height, p.canvas)).collect();
    assert_eq!(
        boxes,
        vec![(0, 0, 4, 12, false), (4, 0, 10, 9, true), (4, 9, 10, 3, false), (14, 0, 4, 12, false)]
    );
}

#[test]
fn the_same_tree_gives_the_same_drawing_at_every_size_it_is_drawn() {
    for (width, height) in [(18, 12), (30, 20), (1, 1), (3, 2), (200, 120)] {
        let plan = blueprint(&factory(), width, height);
        assert_eq!(plan.len(), 4);
        assert_eq!(plan.iter().filter(|p| p.canvas).count(), 1);
        if width >= 3 && height >= 2 {
            tiled(&plan, width, height);
        }
    }
}

#[test]
fn a_moved_panel_changes_the_drawing_the_way_it_changes_the_tree() {
    let next = beside(&factory(), Assets, Properties, Side::Up).unwrap();
    let plan = blueprint(&next, 30, 20);
    tiled(&plan, 30, 20);
    assert_eq!(plan.len(), 5);
    let right: Vec<_> = plan.iter().filter(|p| p.x + p.width == 30).map(|p| (p.y, p.height)).collect();
    assert_eq!(right, vec![(0, 10), (10, 10)]);
}

#[test]
fn a_share_too_small_for_a_pixel_still_gets_one() {
    let thin =
        split(Dir::Row, vec![(0.98, group(&[Canvas])), (0.01, group(&[Layers])), (0.01, group(&[Assets]))]);
    let plan = blueprint(&thin, 18, 12);
    tiled(&plan, 18, 12);
    assert_eq!(plan.iter().map(|p| p.width).collect::<Vec<_>>(), vec![16, 1, 1]);
    assert!(matches!(thin, Node::Split(_)));
}
