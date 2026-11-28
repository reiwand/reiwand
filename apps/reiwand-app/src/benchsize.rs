use crate::bench::{Dir, Node, Panel, Path, Split};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Measure {
    pub floors: HashMap<Panel, [f64; 2]>,
    pub fixed: HashMap<Panel, f64>,
    /// The sides folded into rails: the node that holds exactly the panels of one takes
    /// `rail_width` along the split of its `dir` (a side across a row, the foot down a column),
    /// and the edge beside it does not move.
    #[serde(default)]
    pub rails: Vec<Rail>,
    #[serde(default)]
    pub rail_width: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Rail {
    pub panels: Vec<Panel>,
    pub dir: Dir,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Edge {
    pub path: Path,
    pub index: usize,
    pub dir: Dir,
    pub x: f64,
    pub y: f64,
    pub length: f64,
    pub fixed: bool,
}

impl Measure {
    /// A side folded into a rail keeps its strip even when its panel has a fixed height.
    fn fixed(&self, node: &Node, dir: Dir) -> Option<f64> {
        let held: Vec<Panel> = node.groups().into_iter().flat_map(|(_, g)| g.panels.clone()).collect();
        let exactly = |rail: &Rail| {
            rail.dir == dir && held.len() == rail.panels.len() && held.iter().all(|p| rail.panels.contains(p))
        };
        match node {
            _ if self.rails.iter().any(exactly) => Some(self.rail_width),
            Node::Group(g) if dir == Dir::Column => self.fixed.get(&g.shown).copied(),
            _ => None,
        }
    }

    pub fn floor(&self, node: &Node, dir: Dir) -> f64 {
        if let Some(fixed) = self.fixed(node, dir) {
            return fixed;
        }
        let across = usize::from(dir == Dir::Column);
        match node {
            Node::Group(g) => {
                g.panels.iter().map(|p| self.floors.get(p).map_or(0.0, |f| f[across])).fold(0.0, f64::max)
            }
            Node::Split(s) if s.dir == dir => s.nodes.iter().map(|n| self.floor(n, dir)).sum(),
            Node::Split(s) => s.nodes.iter().map(|n| self.floor(n, dir)).fold(0.0, f64::max),
        }
    }

    pub fn lengths(&self, s: &Split, length: f64) -> Vec<f64> {
        let fixed: Vec<Option<f64>> = s.nodes.iter().map(|n| self.fixed(n, s.dir)).collect();
        let floors: Vec<f64> = s.nodes.iter().map(|n| self.floor(n, s.dir)).collect();
        let rest = (length - fixed.iter().flatten().sum::<f64>()).max(0.0);
        let free: f64 =
            s.shares.iter().zip(&fixed).filter(|(_, f)| f.is_none()).map(|(share, _)| share).sum();
        let want: Vec<f64> = (s.shares.iter().zip(&fixed).zip(&floors))
            .map(|((share, f), floor)| f.unwrap_or_else(|| (share / free * rest).max(*floor)))
            .collect();
        let over = want.iter().sum::<f64>() - length;
        let room: f64 =
            want.iter().zip(&floors).zip(&fixed).filter(|(_, f)| f.is_none()).map(|((w, f), _)| w - f).sum();
        if over <= 0.0 {
            return want;
        }
        if room < over {
            return squeezed(s, &floors, &fixed, rest);
        }
        (want.iter().zip(&floors).zip(&fixed))
            .map(|((w, floor), f)| if f.is_some() { *w } else { w - over * (w - floor) / room })
            .collect()
    }

    fn frame(
        &self,
        node: &Node,
        at: Rect,
        here: &mut Path,
        out: &mut Vec<(Path, Rect)>,
        edges: &mut Vec<Edge>,
    ) {
        out.push((here.clone(), at));
        let Node::Split(s) = node else { return };
        let row = s.dir == Dir::Row;
        let (start, length) = if row { (at.x, at.width) } else { (at.y, at.height) };
        let sizes = self.lengths(s, length);
        let mut from = start;
        for (step, child) in s.nodes.iter().enumerate() {
            let (a, b) = (from.round(), (from + sizes[step]).round());
            from += sizes[step];
            let part = if row {
                Rect { x: a, y: at.y, width: b - a, height: at.height }
            } else {
                Rect { x: at.x, y: a, width: at.width, height: b - a }
            };
            if step + 1 < s.nodes.len() {
                let fixed =
                    self.fixed(child, s.dir).or_else(|| self.fixed(&s.nodes[step + 1], s.dir)).is_some();
                let (x, y, length) = if row { (b, at.y, at.height) } else { (at.x, b, at.width) };
                edges.push(Edge { path: here.clone(), index: step, dir: s.dir, x, y, length, fixed });
            }
            here.push(step);
            self.frame(child, part, here, out, edges);
            here.pop();
        }
    }

    pub fn boxes(&self, root: &Node, width: f64, height: f64) -> (Vec<(Path, Rect)>, Vec<Edge>) {
        let (mut out, mut edges) = (Vec::new(), Vec::new());
        self.frame(root, Rect { x: 0.0, y: 0.0, width, height }, &mut Vec::new(), &mut out, &mut edges);
        (out, edges)
    }

    pub fn resize(
        &self,
        root: &Node,
        size: (f64, f64),
        path: &[usize],
        index: usize,
        by: f64,
    ) -> Option<Node> {
        let (boxes, _) = self.boxes(root, size.0, size.1);
        let at = boxes.iter().find(|(p, _)| p == path)?.1;
        let Some(Node::Split(s)) = root.at(path) else { return None };
        let next = s.nodes.get(index + 1)?;
        if self.fixed(&s.nodes[index], s.dir).or_else(|| self.fixed(next, s.dir)).is_some() || !by.is_finite()
        {
            return None;
        }
        let mut sizes = self.lengths(s, if s.dir == Dir::Row { at.width } else { at.height });
        let pair = sizes[index] + sizes[index + 1];
        let (low, high) = (self.floor(&s.nodes[index], s.dir), pair - self.floor(next, s.dir));
        let one = (sizes[index] + by).min(high).max(low.min(high));
        sizes[index] = one.max(1.0);
        sizes[index + 1] = (pair - one).max(1.0);
        let total: f64 = sizes.iter().sum();
        let shares = sizes.iter().map(|size| size / total).collect();
        reshare(root, path, shares)
    }
}

fn squeezed(s: &Split, floors: &[f64], fixed: &[Option<f64>], rest: f64) -> Vec<f64> {
    let canvas =
        s.nodes.iter().position(|n| n.path_of(Panel::Canvas).is_some()).filter(|at| fixed[*at].is_none());
    let others: f64 = (floors.iter().zip(fixed).enumerate())
        .filter(|(at, (_, f))| f.is_none() && Some(*at) != canvas)
        .map(|(_, (floor, _))| floor)
        .sum();
    let kept = canvas.map_or(0.0, |at| (rest - others).max(floors[at].min(rest / 2.0)));
    let scale = if others > rest - kept && others > 0.0 { (rest - kept) / others } else { 1.0 };
    let mut out: Vec<f64> = floors.iter().zip(fixed).map(|(floor, f)| f.unwrap_or(floor * scale)).collect();
    if let Some(at) = canvas {
        out[at] = kept;
    }
    out
}

fn reshare(root: &Node, path: &[usize], shares: Vec<f64>) -> Option<Node> {
    let mut next = root.clone();
    let Some(Node::Split(s)) = next.at_mut(path) else { return None };
    s.shares = shares;
    Some(next)
}

pub fn even(root: &Node, usual: &Node, path: &[usize], index: usize) -> Option<Node> {
    let Some(Node::Split(s)) = root.at(path) else { return None };
    let pair = s.shares.get(index)? + s.shares.get(index + 1)?;
    let first = match usual.at(path) {
        Some(Node::Split(f)) if f.dir == s.dir && f.nodes.len() == s.nodes.len() => {
            f.shares[index] / (f.shares[index] + f.shares[index + 1])
        }
        _ => 0.5,
    };
    let mut shares = s.shares.clone();
    shares[index] = pair * first;
    shares[index + 1] = pair - shares[index];
    reshare(root, path, shares)
}
