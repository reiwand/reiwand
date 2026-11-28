use super::{RETRY, Session, TEST, lock};
use reiwand_rgb::{Client, pattern};
use std::time::Instant;

impl Session {
    pub(super) fn tick(&mut self) {
        if let Some((started, device)) = self.test {
            if started.elapsed() < TEST {
                return self.paint(&pattern(started.elapsed().as_secs_f32() / 2.0), device);
            }
            self.test = None;
            lock(&self.status).testing = false;
            self.repaint();
            return self.idle();
        }
        let changed = self.client.as_mut().map(Client::heard);
        match changed {
            Some(Ok(true)) => self.stale = true,
            Some(Err(e)) => {
                (self.client, self.failed) = (None, Some(Instant::now()));
                self.publish(Some(e.to_string()), Vec::new());
            }
            Some(Ok(false)) => {}
            None => self.retry(),
        }
        self.rebuild();
    }

    pub(super) fn rebuild(&mut self) {
        self.stale &= self.client.is_some();
        if !self.stale || self.rebuilt.is_some_and(|at| at.elapsed() < RETRY) {
            return;
        }
        (self.stale, self.rebuilt, self.client) = (false, Some(Instant::now()), None);
        if self.ready().is_ok() {
            self.repaint();
        }
    }
}
