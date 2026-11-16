use super::block_on;
use super::secrets::Secrets;
use crate::daemon::Daemon;
use crate::error::{Error, Result, failed};
use crate::jobs::now;
use reiwand_core::dbus::error::SIGNED_OUT;
use reiwand_core::model::JobKind;
use reiwand_sync::Api;
use reiwand_sync::client::rotation;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;

mod turn;

const KEY_WAIT: Duration = Duration::from_secs(30);

impl Daemon {
    pub fn sync_now(self: &Arc<Self>) -> Result<u64> {
        self.syncing()?;
        let keyed = {
            let s = self.saved();
            s.ready || s.stale
        };
        if !keyed {
            return Err(failed("this computer has no sync key yet; join from a linked device"));
        }
        Ok(self.account_sync_job(true))
    }

    pub fn account_sync_job(self: &Arc<Self>, prompt: bool) -> u64 {
        self.start_job(JobKind::AccountSync, "", move |d, _| {
            let result = d.cycle(prompt);
            d.retry_later(result.as_ref().map_or(None, |v| v["unsent"].as_u64()));
            let error = result.as_ref().err().map(ToString::to_string);
            let out = result.as_ref().err().is_some_and(|e| e.parts().0 == SIGNED_OUT);
            d.update(|s| {
                s.last_sync = if error.is_none() { Some(now()) } else { s.last_sync };
                s.last_error = error;
                s.signed_out = out;
            })?;
            result
        })
    }

    pub fn track_access(&self, e: &Error) {
        if e.parts().0 == SIGNED_OUT {
            let note = e.to_string();
            self.update(|s| (s.signed_out, s.last_error) = (true, Some(note))).ok();
        }
    }

    fn current_key(&self, api: &Api, secrets: &mut Secrets, patient: bool) -> Result<Value> {
        block_on(rotation::current(self, api, secrets, if patient { KEY_WAIT } else { Duration::ZERO }))
    }
}
