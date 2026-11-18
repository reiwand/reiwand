mod accounts;
pub mod client;
pub mod descriptors;
#[cfg(feature = "client")]
mod download;
pub mod fill;
pub mod gain;
pub mod manifests;
pub mod scenes;
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "server")]
mod server_export;
pub mod settings;
#[cfg(feature = "client")]
pub mod vault;

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

pub use accounts::{Moved, NoVault, Vault, accounts_step, apply};
