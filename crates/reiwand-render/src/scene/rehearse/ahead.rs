use super::Rehearsal;

impl Rehearsal {
    /// The longest stretch of the scene clock a render plays through before its first frame.
    pub const MAX_SIMULATE: f64 = 600.0;

    /// The second of the first frame: a seek, or with `simulate` the end of the frames played from 0.
    pub fn start(&self) -> f64 {
        let most = if self.simulate { Self::MAX_SIMULATE } else { f64::from(reiwand_scene::MAX_POSTER_TIME) };
        self.time.clamp(0.0, most)
    }

    /// The frames played from second 0 at the frame step before the first frame, which comes at
    /// `start` after a last shorter step when `start` is no whole number of frames.
    pub fn ahead(&self) -> usize {
        match self.simulate {
            true => (self.start() * f64::from(self.rate()) - 1e-6).ceil().max(0.0) as usize,
            false => 0,
        }
    }

    /// The second of the scene clock of the frame `n`, counting the frames played ahead.
    pub fn moment(&self, n: usize) -> f64 {
        let (ahead, step) = (self.ahead(), 1.0 / f64::from(self.rate()));
        match n < ahead {
            true => n as f64 * step,
            false => self.start() + (n - ahead) as f64 * step,
        }
    }
}

impl crate::Renderer {
    /// Waits for a frame played ahead, so that a long stretch does not pile up work on the device.
    pub(super) fn ahead_done(&self) -> Result<(), crate::RenderError> {
        let timeout = Some(crate::readback::GPU_TIMEOUT);
        self.polled(wgpu::PollType::Wait { submission_index: None, timeout })
    }
}
