use crate::error::Error;
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Report {
    added: BTreeMap<&'static str, usize>,
    pending: Vec<Value>,
    notes: Vec<String>,
    pub unresolved: usize,
}

impl Report {
    pub fn add(&mut self, section: &'static str) {
        *self.added.entry(section).or_default() += 1;
    }

    pub fn pend(&mut self, section: &'static str, name: &str, reason: &str) {
        self.pending.push(json!({"section": section, "name": name, "reason": reason}));
    }

    pub fn fail(&mut self, section: &'static str, name: &str, error: &Error) {
        self.pend(section, name, &error.parts().1);
    }

    pub fn note(&mut self, text: &str) {
        self.notes.push(text.to_owned());
    }

    pub fn value(self) -> Value {
        json!({"added": self.added, "pending": self.pending, "unresolved": self.unresolved, "notes": self.notes})
    }
}
