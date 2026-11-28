use super::Runtime;
use crate::BANDS;
use crate::input::{AudioFrame, Usual, usual_key};

impl Runtime {
    pub fn heard(&self) -> &[[f32; BANDS]] {
        &self.slots
    }

    pub(super) fn listen_slots(&mut self, audio: &AudioFrame) {
        self.slots.clear();
        let usual = self.usual.get_mut();
        for l in &self.scene.listens {
            let usual = l
                .surging()
                .then(|| &*usual.entry(usual_key(&l.binding())).or_insert_with(|| Usual::at(audio)));
            self.slots.push(l.bands(audio, usual));
        }
    }
}
