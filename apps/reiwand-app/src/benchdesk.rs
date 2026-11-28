use crate::bench::Node;
use crate::benchkits::{Factory, KITS, Kit, View, factory, opening};
use crate::tr::tr;
use serde::{Deserialize, Serialize};

#[path = "benchdeskown.rs"]
mod own;

/// The layout in use: one of the six the editor comes with, or one the author saved by its name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Chosen {
    Factory(Kit),
    Own(String),
}

/// A layout the author saved: its name, the tree it was saved with (what "Reset" goes back to)
/// and the tree as it is now.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Own {
    pub name: String,
    pub saved: View,
    pub view: View,
}

/// Every layout of the editor and the one in use.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Desk {
    pub chosen: Chosen,
    pub factory: Factory,
    pub own: Vec<Own>,
}

impl Default for Desk {
    fn default() -> Desk {
        Desk::from_trees(factory)
    }
}

/// The trees a set of layouts comes with: a scene's (`factory`) or a package's (`full`).
pub type Trees = fn(Kit) -> Node;

impl Desk {
    pub fn from_trees(trees: Trees) -> Desk {
        let view = |kit| View::of(trees(kit));
        Desk {
            chosen: Chosen::Factory(Kit::Scene),
            factory: Factory {
                scene: view(Kit::Scene),
                particles: view(Kit::Particles),
                scripts: view(Kit::Scripts),
                simulation: view(Kit::Simulation),
                animation: view(Kit::Animation),
                wallpaper: view(Kit::Wallpaper),
            },
            own: Vec::new(),
        }
    }

    pub(crate) fn own_at(&self, name: &str) -> Option<usize> {
        self.own.iter().position(|own| own.name == name)
    }

    pub fn view(&self) -> &View {
        match &self.chosen {
            Chosen::Factory(kit) => self.factory.get(*kit),
            Chosen::Own(name) => self.own_at(name).map_or(&self.factory.scene, |at| &self.own[at].view),
        }
    }

    fn view_mut(&mut self) -> &mut View {
        match self.chosen.clone() {
            Chosen::Factory(kit) => self.factory.get_mut(kit),
            Chosen::Own(name) => match self.own_at(&name) {
                Some(at) => &mut self.own[at].view,
                None => &mut self.factory.scene,
            },
        }
    }

    pub fn root(&self) -> &Node {
        &self.view().root
    }

    /// The tree the layout in use goes back to with "Reset this layout".
    #[cfg(test)]
    pub fn usual(&self) -> Node {
        self.usual_in(factory)
    }

    /// The same, for a set of layouts that comes with `trees`.
    pub fn usual_in(&self, trees: Trees) -> Node {
        match &self.chosen {
            Chosen::Factory(kit) => trees(*kit),
            Chosen::Own(name) => {
                self.own_at(name).map_or(trees(Kit::Scene), |at| self.own[at].saved.root.clone())
            }
        }
    }

    pub fn put(&self, root: Node) -> Option<Desk> {
        let view = View::of(root);
        if view.fault().is_some() {
            return None;
        }
        let mut next = self.clone();
        *next.view_mut() = view;
        Some(next)
    }

    pub fn choose(&self, chosen: Chosen) -> Option<Desk> {
        if let Chosen::Own(name) = &chosen {
            self.own_at(name)?;
        }
        Some(Desk { chosen, ..self.clone() })
    }

    /// The layout under a key: Ctrl+1 to Ctrl+6 the six of the editor, Ctrl+7 to Ctrl+9 the first
    /// three of the author's.
    pub fn keyed(&self, number: usize) -> Option<Chosen> {
        match number {
            1..=6 => Some(Chosen::Factory(KITS[number - 1])),
            7..=9 => self.own.get(number - 7).map(|own| Chosen::Own(own.name.clone())),
            _ => None,
        }
    }

    pub fn remove(&self, at: usize) -> Option<Desk> {
        let mut next = self.clone();
        let gone = next.own.get(at)?.name.clone();
        next.own.remove(at);
        if next.chosen == Chosen::Own(gone) {
            next.chosen = Chosen::Factory(Kit::Scene);
        }
        Some(next)
    }

    #[cfg(test)]
    pub fn reset(&self) -> Desk {
        self.reset_in(factory)
    }

    pub fn reset_in(&self, trees: Trees) -> Desk {
        let mut next = self.clone();
        *next.view_mut() = View::of(self.usual_in(trees));
        next
    }

    pub fn opening(&self, kind: &str, fresh: bool) -> Desk {
        let chosen = opening(kind, fresh).map_or_else(|| self.chosen.clone(), Chosen::Factory);
        Desk { chosen, ..self.clone() }
    }

    /// The first place that breaks a rule, as a path inside `content`, and the rule it breaks.
    pub fn fault(&self) -> Option<(String, String)> {
        for kit in KITS {
            let name = serde_json::to_value(kit).ok()?.as_str()?.to_owned();
            if let Some((path, why)) = self.factory.get(kit).fault() {
                return Some((format!("factory.{name}.{path}"), why));
            }
        }
        for (at, own) in self.own.iter().enumerate() {
            if let Some(why) = self.name_fault(&own.name, Some(at)) {
                return Some((format!("own[{at}].name"), why));
            }
            for (part, view) in [("saved", &own.saved), ("view", &own.view)] {
                if let Some((path, why)) = view.fault() {
                    return Some((format!("own[{at}].{part}.{path}"), why));
                }
            }
        }
        match &self.chosen {
            Chosen::Own(name) if self.own_at(name).is_none() => {
                Some(("chosen.own".to_owned(), tr("the chosen layout is one of yours that is not saved")))
            }
            _ => None,
        }
    }
}
