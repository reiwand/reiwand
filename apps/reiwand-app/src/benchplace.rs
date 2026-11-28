use crate::bench::{Node, PANELS, Panel, group, split};
use crate::benchkits::{Kit, full};
use crate::benchops::{Side, border, emptied, put, side_of, split_off, stack, tidy};

pub fn closed(root: &Node) -> Vec<Panel> {
    PANELS.into_iter().filter(|panel| root.path_of(*panel).is_none()).collect()
}

pub fn close(root: &Node, panel: Panel) -> Option<Node> {
    root.path_of(panel)?;
    if panel == Panel::Canvas {
        return None;
    }
    emptied(root, panel)
}

fn attach(root: &Node, panel: Panel, beside: Panel, side: Side, share: f64) -> Option<Node> {
    let next = emptied(root, panel)?;
    let path = next.path_of(beside)?;
    let there = next.at(&path)?.clone();
    let fresh = group(&[panel]);
    let parts = if side.forward() {
        vec![(1.0 - share, there), (share, fresh)]
    } else {
        vec![(share, fresh), (1.0 - share, there)]
    };
    tidy(put(&next, &path, split(side.dir(), parts)))
}

pub fn beside(root: &Node, panel: Panel, target: Panel, side: Side) -> Option<Node> {
    let there = root.group_of(target)?;
    if panel == Panel::Canvas {
        return None;
    }
    if there.panels.contains(&panel) {
        return split_off(root, panel, side);
    }
    attach(root, panel, target, side, 0.5)
}

pub fn edge(root: &Node, panel: Panel, side: Side) -> Option<Node> {
    if panel == Panel::Canvas {
        return None;
    }
    border(root, panel, side)
}

/// Opens a closed panel again in the place `usual` (the tree its layout resets to) gives it, or the
/// place of the Scene layout when `usual` has it closed too.
pub fn reopen(root: &Node, usual: &Node, panel: Panel) -> Option<Node> {
    if root.path_of(panel).is_some() {
        return None;
    }
    let scene = full(Kit::Scene);
    let usual = if usual.path_of(panel).is_some() { usual } else { &scene };
    let mates = usual.group_of(panel)?.panels.clone();
    if let Some(mate) = mates.into_iter().find(|mate| root.path_of(*mate).is_some()) {
        return stack(root, panel, mate);
    }
    match side_of(usual, panel)? {
        side @ (Side::Left | Side::Right) => border(root, panel, side),
        side => attach(root, panel, Panel::Canvas, side, 0.25),
    }
}
