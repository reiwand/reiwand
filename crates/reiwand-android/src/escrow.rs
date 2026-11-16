use crate::Error;
use crate::side::Phone;
use crate::state::{api, read};
use reiwand_sync::client::{Fault, escrow};
use serde_json::Value;
use zeroize::Zeroizing;

pub(crate) async fn set(passphrase: Zeroizing<String>) -> Result<Value, Error> {
    crate::level::syncing()?;
    escrow::check(&Phone, &passphrase)?;
    let (api, mut secrets) = api()?;
    escrow::set(&Phone, &api, &mut secrets, &passphrase).await
}

pub(crate) async fn restore(passphrase: Zeroizing<String>) -> Result<Value, Error> {
    crate::level::syncing()?;
    if read(|s| s.ready) {
        return Err(Fault::HeldKey.into());
    }
    let (api, mut secrets) = api()?;
    escrow::restore(&Phone, &api, &mut secrets, &passphrase).await
}
