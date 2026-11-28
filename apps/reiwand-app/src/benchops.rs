use crate::bench::{Dir, Group, Node, Panel, Path, Split, group, split};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Left,
    Right,
    Up,
    Down,
}

impl Side {
    pub fn dir(self) -> Dir {
        if matches!(self, Side::Left | Side::Right) { Dir::Row } else { Dir::Column }
    }

    pub fn forward(self) -> bool {
        matches!(self, Side::Right | Side::Down)
    }
}

pub fn tidy(node: Node) -> Option<Node> {
    match node {
        Node::Group(g) if g.panels.is_empty() => None,
        Node::Group(g) => {
            let shown = if g.panels.contains(&g.shown) { g.shown } else { g.panels[0] };
            Some(Node::Group(Group { panels: g.panels, shown }))
        }
        Node::Split(s) => {
            let mut parts = Vec::new();
            for (share, child) in s.shares.into_iter().zip(s.nodes) {
                match tidy(child) {
                    Some(Node::Split(inner)) if inner.dir == s.dir => {
                        parts.extend(inner.shares.into_iter().map(|one| one * share).zip(inner.nodes));
                    }
                    Some(kept) => parts.push((share, kept)),
                    None => {}
                }
            }
            match parts.len() {
                0 => None,
                1 => parts.pop().map(|(_, only)| only),
                _ => Some(split(s.dir, parts)),
            }
        }
    }
}

pub fn put(root: &Node, path: &[usize], node: Node) -> Node {
    let mut next = root.clone();
    match next.at_mut(path) {
        Some(slot) => *slot = node,
        None => return root.clone(),
    }
    next
}

pub fn emptied(root: &Node, panel: Panel) -> Option<Node> {
    let Some(path) = root.path_of(panel) else { return Some(root.clone()) };
    let held = root.group_of(panel)?;
    let panels: Vec<Panel> = held.panels.iter().copied().filter(|p| *p != panel).collect();
    let shown = if held.shown == panel { panels.first().copied().unwrap_or(panel) } else { held.shown };
    tidy(put(root, &path, Node::Group(Group { panels, shown })))
}

pub fn show(root: &Node, panel: Panel) -> Option<Node> {
    let path = root.path_of(panel)?;
    let held = root.group_of(panel)?;
    Some(put(root, &path, Node::Group(Group { panels: held.panels.clone(), shown: panel })))
}

pub fn stack(root: &Node, panel: Panel, beside: Panel) -> Option<Node> {
    let there = root.group_of(beside)?;
    if panel == Panel::Canvas || beside == Panel::Canvas || there.panels.contains(&panel) {
        return None;
    }
    let next = emptied(root, panel)?;
    let path = next.path_of(beside)?;
    let mut panels = next.group_of(beside)?.panels.clone();
    panels.push(panel);
    Some(put(&next, &path, Node::Group(Group { panels, shown: panel })))
}

pub fn split_off(root: &Node, panel: Panel, side: Side) -> Option<Node> {
    let held = root.group_of(panel)?;
    if panel == Panel::Canvas || held.panels.len() < 2 {
        return None;
    }
    let next = emptied(root, panel)?;
    let path = next.path_of(held.panels.iter().copied().find(|p| *p != panel)?)?;
    let rest = next.at(&path)?.clone();
    let fresh = group(&[panel]);
    let parts =
        if side.forward() { vec![(0.5, rest), (0.5, fresh)] } else { vec![(0.5, fresh), (0.5, rest)] };
    tidy(put(&next, &path, split(side.dir(), parts)))
}

fn descend(root: &Node, mut path: Path, side: Side) -> Path {
    while let Some(Node::Split(Split { dir, nodes, .. })) = root.at(&path) {
        let step = if *dir == side.dir() && !side.forward() { nodes.len() - 1 } else { 0 };
        path.push(step);
    }
    path
}

fn beyond(root: &Node, from: &[usize], side: Side) -> Option<Path> {
    for depth in (1..=from.len()).rev() {
        let (parent, step) = (&from[..depth - 1], from[depth - 1]);
        let Some(Node::Split(s)) = root.at(parent) else { continue };
        if s.dir != side.dir() {
            continue;
        }
        let next = if side.forward() { Some(step + 1) } else { step.checked_sub(1) };
        if let Some(next) = next.filter(|next| *next < s.nodes.len()) {
            return Some(descend(root, [parent, &[next]].concat(), side));
        }
    }
    None
}

fn reach(root: &Node, panel: Panel, side: Side) -> (Option<Path>, bool) {
    let Some(mut from) = root.path_of(panel) else { return (None, false) };
    let mut passed = false;
    while let Some(found) = beyond(root, &from, side) {
        match root.at(&found) {
            Some(Node::Group(g)) if g.panels.contains(&Panel::Canvas) => (from, passed) = (found, true),
            _ => return (Some(found), passed),
        }
    }
    (None, passed)
}

#[cfg(test)]
pub fn neighbour(root: &Node, panel: Panel, side: Side) -> Option<Path> {
    reach(root, panel, side).0
}

pub fn border(root: &Node, panel: Panel, side: Side) -> Option<Node> {
    let rest = emptied(root, panel)?;
    let fresh = group(&[panel]);
    let parts =
        if side.forward() { vec![(0.75, rest), (0.25, fresh)] } else { vec![(0.25, fresh), (0.75, rest)] };
    tidy(split(side.dir(), parts))
}

pub fn step(root: &Node, panel: Panel, side: Side) -> Option<Node> {
    if panel == Panel::Canvas {
        return crate::benchcanvas::shift(root, side);
    }
    let (found, passed) = reach(root, panel, side);
    match found.and_then(|path| root.at(&path)) {
        Some(Node::Group(g)) => stack(root, panel, g.panels[0]),
        _ if passed => border(root, panel, side),
        _ => split_off(root, panel, side),
    }
}

pub fn side_of(root: &Node, panel: Panel) -> Option<Side> {
    let (mine, base) = (root.path_of(panel)?, root.path_of(Panel::Canvas)?);
    let fork = mine.iter().zip(&base).position(|(a, b)| a != b)?;
    let Some(Node::Split(s)) = root.at(&mine[..fork]) else { return None };
    let before = mine[fork] < base[fork];
    Some(match (s.dir, before) {
        (Dir::Row, true) => Side::Left,
        (Dir::Row, false) => Side::Right,
        (Dir::Column, true) => Side::Up,
        (Dir::Column, false) => Side::Down,
    })
}
