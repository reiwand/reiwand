use super::super::Src;
use super::{Painter, Target};

const KEPT: usize = usize::MAX;

impl Painter<'_, '_, '_> {
    /// Where the layers go and where the layers the engine draws over the whole scene (the glow of
    /// the canvas and the veil) begin. A scene that keeps its canvas between frames draws its
    /// layers into a texture of its own, which the canvas copies before those are drawn over it, so
    /// they reach the screen each frame without piling up in what is kept.
    pub(crate) fn kept(&mut self, canvas: Src) -> (Target, usize) {
        let draws = &self.frame.draws;
        let (mut i, mut over) = (0, None);
        while i < draws.len() {
            over = if draws[i].layer.id.is_empty() { over.or(Some(i)) } else { None };
            i = draws[i].end;
        }
        let root = Target { src: canvas, rect: self.full };
        let Some(over) = over.filter(|_| self.frame.keep) else { return (root, draws.len()) };
        self.cache(KEPT, 0, self.full.size());
        self.kept.push(KEPT);
        (Target { src: Src::Cache(KEPT), rect: self.full }, over)
    }

    pub(crate) fn over(&mut self, root: Target, canvas: Src, over: usize) {
        if root.src == canvas {
            return;
        }
        self.blit(root.src, canvas);
        let target = Target { src: canvas, rect: self.full };
        self.list(over..self.frame.draws.len(), target, false);
    }
}
