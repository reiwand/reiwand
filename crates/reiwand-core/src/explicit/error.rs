use crate::message::{Message, said};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fault {
    Missing,
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadError {
    pub file: String,
    pub path: String,
    pub fault: Fault,
}

pub(super) fn join(parent: &str, field: &str) -> String {
    match (parent, field) {
        (_, "") => parent.to_owned(),
        ("" | ".", _) => field.to_owned(),
        _ => format!("{parent}.{field}"),
    }
}

impl ReadError {
    pub fn said(&self) -> Message {
        match &self.fault {
            Fault::Missing => said(
                "explicit.missing",
                "%1: \"%2\" is missing. Reiwand does not fill in values: write it, or open it in the editor, which fills it in.",
                &[&self.file, &self.path],
            ),
            Fault::Invalid(why) => {
                said("explicit.invalid", "%1: \"%2\" is not valid: %3", &[&self.file, &self.path, why])
            }
        }
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.said().local())
    }
}

impl std::error::Error for ReadError {}

impl From<ReadError> for Message {
    fn from(e: ReadError) -> Self {
        e.said()
    }
}
