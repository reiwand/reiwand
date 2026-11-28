use crate::bench::{Dir, Node, Panel};
use serde::Serialize;

/// One group of a layout drawn in its blueprint: a box in whole pixels of the drawing, and whether
/// it holds the canvas (drawn filled).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Plan {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub canvas: bool,
}

/// The blueprint of a layout: the boxes of its groups in a drawing of `width` x `height` pixels,
/// from the shares of the tree alone (no floors), so the same tree always gives the same drawing.
/// Each box is cut on whole pixels from the running sum of the shares, so the boxes tile the
/// drawing with no gap and no overlap, and a group never gets less than one pixel each way.
pub fn blueprint(root: &Node, width: i32, height: i32) -> Vec<Plan> {
    let mut found = Vec::new();
    draw(root, 0, 0, width.max(1), height.max(1), &mut found);
    found
}

fn draw(node: &Node, x: i32, y: i32, width: i32, height: i32, found: &mut Vec<Plan>) {
    match node {
        Node::Group(g) => found.push(Plan { x, y, width, height, canvas: g.panels.contains(&Panel::Canvas) }),
        Node::Split(s) => {
            let length = if s.dir == Dir::Row { width } else { height };
            let count = s.nodes.len() as i32;
            let mut sum = 0.0;
            let mut start = 0;
            for (at, (share, child)) in s.shares.iter().zip(&s.nodes).enumerate() {
                sum += share;
                let left = count - at as i32 - 1;
                let want = (sum * f64::from(length)).round() as i32;
                let end = if left == 0 { length } else { want.min(length - left).max(start + 1) };
                let size = (end - start).max(1);
                match s.dir {
                    Dir::Row => draw(child, x + start, y, size, height, found),
                    Dir::Column => draw(child, x, y + start, width, size, found),
                }
                start = end;
            }
        }
    }
}
