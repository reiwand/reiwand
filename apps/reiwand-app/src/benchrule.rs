use crate::bench::{Node, PANELS, Panel};
use crate::tr::tr;

pub fn spelled(path: &[usize], root: &Node, tail: &str) -> String {
    let mut out = String::new();
    let mut node = Some(root);
    for step in path {
        if let Some(Node::Split(s)) = node {
            out.push_str(&format!("split.nodes[{step}]."));
            node = s.nodes.get(*step);
        }
    }
    out.push_str(match node {
        Some(Node::Split(_)) => "split",
        _ => "group",
    });
    if tail.is_empty() { out } else { format!("{out}.{tail}") }
}

pub fn closed_fault(root: &Node, closed: &[Panel]) -> Option<(String, String)> {
    for (at, panel) in closed.iter().enumerate() {
        let here = format!("closed[{at}]");
        if *panel == Panel::Canvas {
            return Some((here, tr("the canvas is never closed")));
        }
        if closed[..at].contains(panel) || root.path_of(*panel).is_some() {
            return Some((here, tr("a panel is in one place only")));
        }
    }
    None
}

pub fn fault(root: &Node, closed: &[Panel]) -> Option<(String, String)> {
    let mut seen: Vec<Panel> = Vec::new();
    let mut pending = vec![(Vec::new(), root, None)];
    while let Some((path, node, outer)) = pending.pop() {
        let at = |tail: &str| spelled(&path, root, tail);
        match node {
            Node::Split(s) => {
                if s.nodes.len() < 2 {
                    return Some((at("nodes"), tr("a split holds at least two nodes")));
                }
                if s.shares.len() != s.nodes.len() {
                    return Some((at("shares"), tr("there is one share for each node")));
                }
                if let Some(bad) = s.shares.iter().position(|share| !share.is_finite() || *share <= 0.0) {
                    return Some((at(&format!("shares[{bad}]")), tr("a share is a number above 0")));
                }
                if (s.shares.iter().sum::<f64>() - 1.0).abs() > 1e-6 {
                    return Some((at("shares"), tr("the shares add up to 1")));
                }
                if outer == Some(s.dir) {
                    return Some((
                        at("dir"),
                        tr("a split does not go the same way as the split that holds it"),
                    ));
                }
                for (step, child) in s.nodes.iter().enumerate().rev() {
                    pending.push(([path.clone(), vec![step]].concat(), child, Some(s.dir)));
                }
            }
            Node::Group(g) => {
                if g.panels.is_empty() {
                    return Some((at("panels"), tr("a group holds at least one panel")));
                }
                if let Some(twice) = g.panels.iter().position(|panel| seen.contains(panel)) {
                    return Some((at(&format!("panels[{twice}]")), tr("a panel is in one place only")));
                }
                if g.panels.contains(&Panel::Canvas) && g.panels.len() > 1 {
                    return Some((at("panels"), tr("the canvas is alone in its group")));
                }
                if !g.panels.contains(&g.shown) {
                    return Some((at("shown"), tr("the shown panel is one of the group")));
                }
                seen.extend(&g.panels);
            }
        }
    }
    let lost = PANELS.iter().find(|panel| !seen.contains(panel) && !closed.contains(panel))?;
    let name = serde_json::to_value(lost).ok()?.as_str()?.to_owned();
    Some((
        String::new(),
        reiwand_core::message::render(&tr("the panel \"%1\" is in no group and is not closed"), &[name]),
    ))
}
