use crate::bench::{Node, Panel};
use crate::benchplace::closed;
use crate::benchrule::{closed_fault, fault};
use serde::{Deserialize, Serialize};

/// The six layouts the editor comes with, in the order of the top bar and of Ctrl+1 to Ctrl+6.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kit {
    Scene,
    Particles,
    Scripts,
    Simulation,
    Animation,
    Wallpaper,
}

pub use crate::benchfactory::factory;
pub use crate::benchfull::full;

pub const KITS: [Kit; 6] =
    [Kit::Scene, Kit::Particles, Kit::Scripts, Kit::Simulation, Kit::Animation, Kit::Wallpaper];

/// The layout a scene opens in: a scene just made opens in Scene; a package of another kind (web,
/// image, video or preset) opens in Wallpaper; any other keeps the layout chosen last.
pub fn opening(kind: &str, fresh: bool) -> Option<Kit> {
    match kind {
        "web" | "image" | "video" | "preset" => Some(Kit::Wallpaper),
        "scene" if fresh => Some(Kit::Scene),
        _ => None,
    }
}

/// A workbench as it is now: its tree and the panels closed in it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct View {
    pub root: Node,
    pub closed: Vec<Panel>,
}

impl View {
    pub fn of(root: Node) -> View {
        View { closed: closed(&root), root }
    }

    pub fn fault(&self) -> Option<(String, String)> {
        if let Some(found) = closed_fault(&self.root, &self.closed) {
            return Some(found);
        }
        let (path, why) = fault(&self.root, &self.closed)?;
        Some((if path.is_empty() { "root".to_owned() } else { format!("root.{path}") }, why))
    }
}

/// The six layouts the editor comes with, each with the author's changes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Factory {
    pub scene: View,
    pub particles: View,
    pub scripts: View,
    pub simulation: View,
    pub animation: View,
    pub wallpaper: View,
}

impl Factory {
    pub fn get(&self, kit: Kit) -> &View {
        match kit {
            Kit::Scene => &self.scene,
            Kit::Particles => &self.particles,
            Kit::Scripts => &self.scripts,
            Kit::Simulation => &self.simulation,
            Kit::Animation => &self.animation,
            Kit::Wallpaper => &self.wallpaper,
        }
    }

    pub fn get_mut(&mut self, kit: Kit) -> &mut View {
        match kit {
            Kit::Scene => &mut self.scene,
            Kit::Particles => &mut self.particles,
            Kit::Scripts => &mut self.scripts,
            Kit::Simulation => &mut self.simulation,
            Kit::Animation => &mut self.animation,
            Kit::Wallpaper => &mut self.wallpaper,
        }
    }
}
