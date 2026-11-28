use crate::bench::{Node, Panel};
use crate::benchops::{Side, put};

/// The canvas moves by trading places with what is next to it the way it goes, at the nearest
/// split of that way where it is not already at the end; its column (the timeline under it) and its
/// share go with it. It never joins a group.
pub fn shift(root: &Node, side: Side) -> Option<Node> {
    let path = root.path_of(Panel::Canvas)?;
    for depth in (1..=path.len()).rev() {
        let (parent, step) = (&path[..depth - 1], path[depth - 1]);
        let Some(Node::Split(s)) = root.at(parent) else { continue };
        let next = if side.forward() { Some(step + 1) } else { step.checked_sub(1) };
        let Some(next) = next.filter(|next| s.dir == side.dir() && *next < s.nodes.len()) else { continue };
        let mut moved = s.clone();
        moved.nodes.swap(step, next);
        moved.shares.swap(step, next);
        return Some(put(root, parent, Node::Split(moved)));
    }
    None
}
