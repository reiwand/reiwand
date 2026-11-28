pub mod escrow;
mod handover;
pub mod keys;
pub mod level;
pub mod rotation;

use crate::{Peer, Replica, Secrets};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;

pub enum Fault {
    Sync(crate::Error),
    NoKey,
    HeldKey,
    NeedsEncryption,
    ShortPhrase,
    SamePhrase,
    NoEscrow,
    EscrowReplayed { epoch: i64, seen: i64 },
    EscrowOldKey,
    EncryptionChanged,
    KeyChanged,
    KeyNotStored,
    PinBroken,
    PlainRotation,
    RotatedElsewhere,
    NeedsPlain,
    OtherEncryption,
    NoDevice(String),
    OwnDevice,
}

impl From<crate::Error> for Fault {
    fn from(e: crate::Error) -> Self {
        Fault::Sync(e)
    }
}

pub struct Fields<'a> {
    pub owner: &'a Option<(String, i64)>,
    pub device: &'a Option<String>,
    pub label: &'a str,
    pub encrypted: &'a mut Option<bool>,
    pub plaintext: &'a mut bool,
    pub epoch: &'a mut i64,
    pub ready: &'a mut bool,
    pub stale: &'a mut bool,
    pub sync_off: &'a mut bool,
    pub last_error: &'a mut Option<String>,
    pub last_reply: Option<&'a mut Option<Value>>,
    pub warning: &'a mut Option<String>,
    pub peers: &'a mut Vec<Peer>,
    pub pinned: &'a mut Option<BTreeMap<String, String>>,
    pub members: &'a mut BTreeSet<String>,
    pub replica: &'a mut Replica,
    pub quarantine: &'a mut BTreeMap<String, i64>,
    pub unapplied: &'a mut BTreeMap<String, i64>,
    pub recovery: &'a mut Option<i64>,
    pub recovery_stale: Option<&'a mut bool>,
}

impl Fields<'_> {
    pub fn user(&self) -> i64 {
        self.owner.as_ref().map_or(0, |(_, user)| *user)
    }

    pub fn me(&self) -> String {
        self.device.clone().unwrap_or_default()
    }

    fn recover(&mut self, epoch: Option<i64>, stale: bool) {
        *self.recovery = epoch;
        if let Some(kept) = self.recovery_stale.as_deref_mut() {
            *kept = stale;
        }
    }

    fn quiet(&mut self) {
        *self.last_error = None;
        if let Some(reply) = self.last_reply.as_deref_mut() {
            *reply = None;
        }
    }
}

pub fn label_of(device: Option<&str>, label: &str, replica: &Replica, id: &str) -> Option<String> {
    if device == Some(id) {
        return Some(label.to_owned());
    }
    let (_, doc) = replica.bases.get(&format!("device/{id}"))?;
    doc.view()?.get("name")?.as_str().map(String::from)
}

pub trait Side {
    type Error: From<Fault> + std::fmt::Display;
    const KEEPS_GOING_WITHOUT_ESCROW: bool;
    fn read<T>(&self, look: impl FnOnce(&mut Fields<'_>) -> T) -> T;
    fn update(&self, change: impl FnOnce(&mut Fields<'_>)) -> Result<(), Self::Error>;
    fn keep(&self, secrets: &Secrets) -> impl Future<Output = Result<(), Self::Error>>;
}

#[cfg(test)]
mod tests;
