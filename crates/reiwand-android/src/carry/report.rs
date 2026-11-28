use crate::Error;
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct Report {
    added: BTreeMap<&'static str, usize>,
    pending: Vec<Value>,
    notes: Vec<&'static str>,
    pub(super) unresolved: usize,
}

impl Report {
    pub(super) fn add(&mut self, section: &'static str) {
        *self.added.entry(section).or_default() += 1;
    }

    pub(super) fn pend(&mut self, section: &'static str, name: &str, reason: &str) {
        self.pending.push(json!({"section": section, "name": name, "reason": reason}));
    }

    pub(super) fn fail(&mut self, section: &'static str, name: &str, error: &Error) {
        let reply = error.reply();
        self.pend(section, name, reply["error"].as_str().unwrap_or_default());
    }

    pub(super) fn note(&mut self, id: &'static str) {
        if !self.notes.contains(&id) {
            self.notes.push(id);
        }
    }

    pub(super) fn value(self) -> Value {
        json!({"added": self.added, "pending": self.pending, "unresolved": self.unresolved, "notes": self.notes})
    }
}
