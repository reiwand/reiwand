use super::block_on;
use super::server::exists;
use crate::daemon::Daemon;
use crate::error::{Result, failed};
use reiwand_core::model::JobKind;
use reiwand_sync::client::level;
use serde_json::json;
use std::sync::Arc;

pub const OFF: &str = "syncing is off on this computer; turn it on first";

impl Daemon {
    pub fn syncing(&self) -> Result<()> {
        self.linked()?;
        if self.saved().sync_off { Err(failed(OFF)) } else { Ok(()) }
    }

    pub fn set_syncing(self: &Arc<Self>, on: bool, plaintext: bool) -> Result<u64> {
        self.linked()?;
        if !self.jobs_of(JobKind::Link).is_empty() {
            return Err(exists("this computer is linking, joining or sharing the key; wait for it to end"));
        }
        if !on {
            self.update(|s| (s.sync_off, s.last_error, s.pairing) = (true, None, None))?;
            for job in
                self.jobs_of(JobKind::AccountSync).into_iter().chain(self.jobs.find(JobKind::Upload, "files"))
            {
                self.cancel_job(job)?;
            }
            self.account.restart.notify_one();
            tracing::info!("syncing turned off");
        }
        Ok(self.start_job(JobKind::Link, "syncing", move |d, _| {
            if !on {
                return Ok(json!({"syncing": false}));
            }
            let (api, mut secrets) = d.account_api(true)?;
            let existing = block_on(level::sync_state::<Daemon>(&api, plaintext))?;
            let state = block_on(level::setup(&**d, &api, &mut secrets, existing, plaintext))?;
            let _ = d.refresh_peers(&api);
            d.account.wake.notify_one();
            d.account.restart.notify_one();
            tracing::info!("syncing turned on");
            Ok(json!({"syncing": true, "encrypted": state["encrypted"], "needs_key": secrets.key.is_empty()}))
        }))
    }
}
