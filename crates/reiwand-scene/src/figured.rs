use crate::Layer;
use crate::rig::Rig;

impl Layer {
    pub fn rig(&self) -> Option<&Rig> {
        match &self.kind {
            crate::Kind::Mesh(m) => Some(&m.rig),
            crate::Kind::Model(m) => Some(&m.rig),
            _ => None,
        }
    }

    pub fn figure(&self) -> Option<&str> {
        match &self.kind {
            crate::Kind::Mesh(m) => Some(&m.mesh),
            crate::Kind::Model(m) => Some(&m.model),
            _ => None,
        }
    }
}
