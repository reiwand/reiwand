pub mod client;
#[cfg(feature = "server")]
pub mod server;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Nothing,
    Upgraded { from: i64, to: i64 },
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Outcome::Nothing => write!(out, "nothing to upgrade"),
            Outcome::Upgraded { from, to } => write!(out, "upgraded from version {from} to {to}"),
        }
    }
}
