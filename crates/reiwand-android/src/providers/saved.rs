use super::kinds::Options;
use crate::{Error, said, store, vault};
use reiwand_core::explicit;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard, PoisonError};

const FILE: &str = "providers.json";
const SEEN: usize = 2000;

static LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Credit {
    pub(super) uid: String,
    pub(super) title: String,
    #[serde(deserialize_with = "explicit::null")]
    pub(super) attribution: Option<String>,
    #[serde(deserialize_with = "explicit::null")]
    pub(super) license: Option<String>,
    #[serde(deserialize_with = "explicit::null")]
    pub(super) link: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Entry {
    pub(super) id: String,
    pub(super) kind: String,
    pub(super) options: Options,
    pub(super) on: bool,
    #[serde(deserialize_with = "explicit::null")]
    pub(super) last_sync: Option<i64>,
    #[serde(deserialize_with = "explicit::null")]
    pub(super) last_error: Option<serde_json::Value>,
    pub(super) seen: Vec<String>,
    pub(super) credits: Vec<Credit>,
}

impl Entry {
    pub(super) fn new(id: String, kind: String, options: Options, on: bool) -> Self {
        Self {
            id,
            kind,
            options,
            on,
            last_sync: None,
            last_error: None,
            seen: Vec::new(),
            credits: Vec::new(),
        }
    }

    pub(super) fn remember(&mut self, id: String) {
        self.seen.push(id);
        let extra = self.seen.len().saturating_sub(SEEN);
        self.seen.drain(..extra);
    }
}

pub(super) fn held() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(PoisonError::into_inner)
}

pub(super) fn load() -> Result<Vec<Entry>, Error> {
    match std::fs::read_to_string(store::dir()?.join(FILE)) {
        Ok(text) => Ok(explicit::read_str(FILE, &text)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e.into()),
    }
}

pub(super) fn save(entries: &[Entry]) -> Result<(), Error> {
    store::write(&store::dir()?.join(FILE), &serde_json::to_vec_pretty(entries)?)
}

pub(super) fn find<'a>(entries: &'a mut [Entry], id: &str) -> Result<&'a mut Entry, Error> {
    let found = entries.iter_mut().find(|e| e.id == id);
    found.ok_or_else(|| said("android.provider_missing", "there is no picture source %1", &[&id]))
}

fn name(id: &str) -> String {
    format!("provider/{id}")
}

pub(super) fn secret(id: &str) -> Option<String> {
    vault::kept().remove(&name(id)).filter(|s| !s.is_empty())
}

pub(super) fn keep(id: &str, secret: &str) -> Result<(), Error> {
    vault::keep(&name(id), secret)
}
