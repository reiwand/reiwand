use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Panel {
    Canvas,
    Layers,
    Assets,
    Options,
    Try,
    Properties,
    Effects,
    Timeline,
    Curves,
    Problems,
    ViewAs,
    Inputs,
    Screens,
    Sheet,
    Texts,
    Cover,
    Review,
    Particles,
    Cost,
    Scripts,
    Code,
    Reference,
    Console,
}

pub const PANELS: [Panel; 23] = [
    Panel::Canvas,
    Panel::Layers,
    Panel::Assets,
    Panel::Options,
    Panel::Try,
    Panel::Properties,
    Panel::Effects,
    Panel::Timeline,
    Panel::Curves,
    Panel::Problems,
    Panel::ViewAs,
    Panel::Inputs,
    Panel::Screens,
    Panel::Sheet,
    Panel::Texts,
    Panel::Cover,
    Panel::Review,
    Panel::Particles,
    Panel::Cost,
    Panel::Scripts,
    Panel::Code,
    Panel::Reference,
    Panel::Console,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dir {
    Row,
    Column,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Node {
    Split(Split),
    Group(Group),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Split {
    pub dir: Dir,
    pub shares: Vec<f64>,
    pub nodes: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Group {
    pub panels: Vec<Panel>,
    pub shown: Panel,
}

pub type Path = Vec<usize>;

pub fn group(panels: &[Panel]) -> Node {
    Node::Group(Group { panels: panels.to_vec(), shown: panels[0] })
}

pub fn split(dir: Dir, parts: Vec<(f64, Node)>) -> Node {
    let total: f64 = parts.iter().map(|(share, _)| share).sum();
    let (shares, nodes) = parts.into_iter().map(|(share, node)| (share / total, node)).unzip();
    Node::Split(Split { dir, shares, nodes })
}

impl Node {
    pub fn at(&self, path: &[usize]) -> Option<&Node> {
        path.iter().try_fold(self, |node, step| match node {
            Node::Split(s) => s.nodes.get(*step),
            Node::Group(_) => None,
        })
    }

    pub fn at_mut(&mut self, path: &[usize]) -> Option<&mut Node> {
        path.iter().try_fold(self, |node, step| match node {
            Node::Split(s) => s.nodes.get_mut(*step),
            Node::Group(_) => None,
        })
    }

    pub fn groups(&self) -> Vec<(Path, &Group)> {
        let mut found = Vec::new();
        self.walk(&mut Vec::new(), &mut found);
        found
    }

    fn walk<'a>(&'a self, here: &mut Path, found: &mut Vec<(Path, &'a Group)>) {
        match self {
            Node::Group(g) => found.push((here.clone(), g)),
            Node::Split(s) => {
                for (at, child) in s.nodes.iter().enumerate() {
                    here.push(at);
                    child.walk(here, found);
                    here.pop();
                }
            }
        }
    }

    #[cfg(test)]
    pub fn panels(&self) -> Vec<Panel> {
        self.groups().into_iter().flat_map(|(_, g)| g.panels.clone()).collect()
    }

    pub fn path_of(&self, panel: Panel) -> Option<Path> {
        self.groups().into_iter().find(|(_, g)| g.panels.contains(&panel)).map(|(path, _)| path)
    }

    pub fn group_of(&self, panel: Panel) -> Option<&Group> {
        self.groups().into_iter().find(|(_, g)| g.panels.contains(&panel)).map(|(_, g)| g)
    }
}
