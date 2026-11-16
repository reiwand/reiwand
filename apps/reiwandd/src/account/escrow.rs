use super::block_on;
use crate::daemon::Daemon;
use crate::error::Result;
use reiwand_core::model::JobKind;
use reiwand_sync::client::{Fault, escrow};
use std::sync::Arc;
use zeroize::Zeroizing;

impl Daemon {
    pub fn set_escrow(self: &Arc<Self>, passphrase: Zeroizing<String>) -> Result<u64> {
        self.syncing()?;
        escrow::check(&**self, &passphrase)?;
        Ok(self.start_job(JobKind::Account, "escrow", move |d, _| {
            let (api, mut secrets) = d.account_api(true)?;
            block_on(escrow::set(&**d, &api, &mut secrets, &passphrase))
        }))
    }

    pub fn restore(self: &Arc<Self>, passphrase: Zeroizing<String>) -> Result<u64> {
        self.syncing()?;
        if self.saved().ready {
            return Err(Fault::HeldKey.into());
        }
        Ok(self.start_job(JobKind::Link, "restore", move |d, _| {
            let (api, mut secrets) = d.account_api(true)?;
            let restored = block_on(escrow::restore(&**d, &api, &mut secrets, &passphrase))?;
            d.account.wake.notify_one();
            d.account.restart.notify_one();
            Ok(restored)
        }))
    }
}
