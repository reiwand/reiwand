mod apply;
mod author;
mod away;
mod cycle;
mod devices;
mod escrow;
mod fetch;
mod files;
mod gone;
mod keys;
mod level;
mod link;
#[cfg(test)]
mod linktests;
mod local;
mod mail;
mod manage;
mod objects;
mod offers;
mod pending;
mod presets;
mod rotate;
mod saved;
mod secrets;

impl Account {
    pub fn server_ring(&self) -> Option<&Keyring> {
        self.keyring.get()
    }
}
mod server;
mod share;
mod side;
mod status;
#[cfg(test)]
mod tests;
mod transfer;
mod upload;
mod votes;
mod watch;

pub use away::away_title;
pub use reiwand_sync::DOWNLOADS;
pub use reiwand_sync::{removed_blob, removed_package};
pub use watch::start;

use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::{Error, Result, failed, not_found};
use crate::keyring::Keyring;
use reiwand_core::dbus::error::SIGNED_OUT;
use reiwand_core::model::JobKind;
use reiwand_sync::Api;
use reiwand_sync::client::keys as client;
use saved::Peer;
use saved::Saved;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::{Mutex, MutexGuard, OnceLock};
use tokio::sync::Notify;

pub const KINDS: [JobKind; 4] = [JobKind::Account, JobKind::Link, JobKind::AccountSync, JobKind::Upload];

#[derive(Default)]
pub struct Account {
    saved: Mutex<Saved>,
    keyring: OnceLock<Keyring>,
    wake: Notify,
    pub(crate) restart: Notify,
    confirmed: Mutex<Option<(String, String)>>,
    edits: AtomicBool,
    tries: AtomicU32,
    ids: Mutex<Option<reiwand_sync::Ids>>,
    pruned: std::sync::atomic::AtomicU64,
}

pub fn block_on<T>(work: impl Future<Output = T>) -> T {
    crate::outside::block_on(work)
}

pub fn sync_error(e: reiwand_sync::Error) -> Error {
    match e.code() {
        reiwand_sync::TOO_OLD => failed(crate::words::too_old()),
        "unauthorized" => Error::Named(SIGNED_OUT, crate::words::signed_out()),
        _ => e.into(),
    }
}

impl Daemon {
    fn saved(&self) -> MutexGuard<'_, Saved> {
        lock(&self.account.saved)
    }

    pub(crate) fn server_pin(&self) -> Option<(String, String)> {
        let saved = self.saved();
        saved.pin.clone().filter(|p| !p.is_empty() && !saved.url.is_empty()).map(|p| (saved.url.clone(), p))
    }

    fn sync_path(&self) -> PathBuf {
        self.store.data.join("sync.json")
    }

    fn update(&self, change: impl FnOnce(&mut Saved)) -> Result<()> {
        let mut saved = self.saved();
        change(&mut saved);
        saved::save(&self.sync_path(), &saved)?;
        drop(saved);
        self.changes.changed("account", Vec::new());
        Ok(())
    }

    fn jobs_of(&self, kind: JobKind) -> Vec<u64> {
        let jobs = self.jobs.list().into_iter().filter(|j| j.kind == kind && j.finished_at.is_none());
        jobs.map(|j| j.id).collect()
    }

    fn server_url(&self) -> Result<String> {
        let url = self.saved().url.clone();
        if url.is_empty() { Err(not_found("no server is added; add one first")) } else { Ok(url) }
    }

    fn linked(&self) -> Result<()> {
        self.server_url()?;
        let linked = self.saved().device.is_some();
        if linked { Ok(()) } else { Err(failed("link this computer to the server first")) }
    }

    fn refresh_peers(&self, api: &Api) -> Result<Vec<Peer>> {
        block_on(client::refresh(self, api))
    }

    fn pins(&self) -> Result<BTreeMap<String, String>> {
        client::pins(self)
    }

    fn pin(&self, peer: &str, key: &str) -> Result<()> {
        client::pin(self, peer, key)
    }

    fn rekeyed(&self, api: &Api) -> Result<bool> {
        block_on(client::rekeyed(self, api))
    }
}

pub(crate) fn hostname() -> String {
    let name = rustix::system::uname().nodename().to_string_lossy().into_owned();
    if name.trim().is_empty() { "Linux computer".into() } else { name }
}
