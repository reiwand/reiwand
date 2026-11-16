use super::{block_on, sync_error};
use crate::daemon::Daemon;
use crate::error::{Result, failed};
use reiwand_core::model::JobKind;
use reiwand_sync::SyncKey;
use reiwand_sync::client::{Fault, rotation};
use serde_json::json;
use std::sync::Arc;

impl Daemon {
    pub fn rotate_now(self: &Arc<Self>) -> Result<u64> {
        self.syncing()?;
        rotation::check(&**self)?;
        Ok(self.start_job(JobKind::Account, "rotate", |d, _| {
            let (api, mut secrets) = d.account_api(true)?;
            block_on(rotation::rotate(&**d, &api, &mut secrets, false))?
                .ok_or_else(|| Fault::RotatedElsewhere.into())
        }))
    }

    pub fn reset_sync(self: &Arc<Self>) -> Result<u64> {
        self.syncing()?;
        Ok(self.start_job(JobKind::Account, "reset", |d, _| {
            let (api, mut secrets) = d.account_api(true)?;
            let encrypted = {
                let s = d.saved();
                !(s.plaintext && s.encrypted == Some(false))
            };
            block_on(api.post("/api/v1/sync/reset", json!({"confirm": "reset"}))).map_err(sync_error)?;
            let key = secrets.sync_key().map_or_else(|_| SyncKey::generate(), |old| old.rotated());
            (secrets.lock, secrets.handover) = Default::default();
            let body = json!({"encrypted": encrypted, "confirm_plaintext": !encrypted, "key_id": key.id()});
            let state = block_on(api.post("/api/v1/sync/setup", body)).map_err(sync_error)?;
            if state["encrypted"].as_bool() != Some(encrypted) {
                return Err(failed("the server set up sync with another encryption than asked"));
            }
            let epoch = state["epoch"].as_i64().unwrap_or(0);
            secrets.set_key(&key, epoch);
            d.store_secrets(&secrets)?;
            d.update(|s| {
                (s.replica, s.present, s.quarantine) = Default::default();
                (s.pairing, s.pinned) = (None, Some(Default::default()));
                (s.kept, s.unapplied, s.offers, s.recovery) = Default::default();
                (s.files.stored, s.files.dropped, s.files.uploaded) = Default::default();
                (s.ready, s.stale, s.epoch, s.last_error, s.encrypted) =
                    (true, false, epoch, None, Some(encrypted));
            })?;
            d.rekeyed(&api)?;
            d.cycle(true)
        }))
    }
}
