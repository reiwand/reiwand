use super::Context;
use crate::input::{Usual, read, usual_key};
use crate::value::{AudioKind, Band, Binding};

impl Context<'_> {
    pub(crate) fn heard(&self, b: &Binding<impl Sized>, band: Band) -> f32 {
        if b.kind != Some(AudioKind::Surge) {
            return read(self.audio, None, b, band);
        }
        let mut usual = self.usual.borrow_mut();
        let usual = usual.entry(usual_key(b)).or_insert_with(|| Usual::at(self.audio));
        read(self.audio, Some(usual), b, band)
    }
}

impl Context<'_> {
    pub(crate) fn listen(&self, l: &crate::value::Listen) -> [f32; crate::BANDS] {
        if !l.surging() {
            return l.bands(self.audio, None);
        }
        let mut usual = self.usual.borrow_mut();
        let usual = usual.entry(usual_key(&l.binding())).or_insert_with(|| Usual::at(self.audio));
        l.bands(self.audio, Some(usual))
    }

    pub(crate) fn listened(&self, listens: &[crate::value::Listen]) -> [[f32; crate::BANDS]; 2] {
        let mut out = [[0.0; crate::BANDS]; 2];
        for (slot, l) in out.iter_mut().zip(listens) {
            *slot = self.listen(l);
        }
        out
    }
}
