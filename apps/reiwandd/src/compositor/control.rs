use super::{Command, Compositor, StripSpec};
use std::sync::Arc;

impl Compositor {
    pub fn set_power(&self, on_battery: bool) {
        self.send(Command::Power(on_battery))
    }

    pub fn set_lowest(&self, lowest: bool) {
        self.send(Command::Lowest(lowest))
    }

    pub fn set_saver(&self, saver: bool) {
        self.send(Command::Saver(saver))
    }

    pub fn set_locked(&self, locked: bool) {
        self.send(Command::Locked(locked))
    }

    pub fn retry_effects(&self) {
        self.send(Command::Retry)
    }

    pub fn strip(&self, output: &str, spec: Option<Arc<StripSpec>>) {
        self.send(Command::Strip(output.into(), spec))
    }

    pub fn restart_particles(&self, output: &str) {
        self.send(Command::Again(output.into()))
    }
}
