use crate::grid::Grid;
use std::collections::HashMap;

/// The one request for covers that may be under way, the rows seen while it was, and how many
/// requests in a row failed.
#[derive(Default)]
pub struct Asking {
    pub busy: bool,
    pub pending: Option<(usize, usize)>,
    pub failures: u8,
}

pub struct Answered {
    pub changed: Vec<usize>,
    pub again: Option<(usize, usize)>,
    /// After a failed request, when to ask again by itself: 1, 2, 4… seconds, at most 30, so the
    /// covers come once the library answers again without the shelf being moved.
    pub retry: Option<std::time::Duration>,
}

impl Grid {
    /// The covers to request for the rows shown, or nothing while a request is under way (the rows
    /// are then asked when it answers).
    pub fn request(&mut self, first: usize, last: usize) -> Option<Vec<String>> {
        if self.asking.busy {
            self.asking.pending = Some((first, last));
            return None;
        }
        let wanted = self.ask(first, last)?;
        self.asking.busy = true;
        Some(wanted)
    }

    /// Takes the answer of a request: `None` when it failed, so its covers are asked again.
    pub fn answered(&mut self, found: Option<HashMap<String, String>>) -> Answered {
        self.asking.busy = false;
        let again = self.asking.pending.take();
        let Some(found) = found else {
            self.sent.clear();
            self.asking.failures = self.asking.failures.saturating_add(1);
            let wait = 1u64 << self.asking.failures.saturating_sub(1).min(5);
            let retry = again.is_none().then(|| std::time::Duration::from_secs(wait.min(30)));
            return Answered { changed: Vec::new(), again, retry };
        };
        self.asking.failures = 0;
        let changed = found.iter().flat_map(|(uid, path)| self.thumb(uid, path)).collect();
        Answered { changed, again, retry: None }
    }
}
