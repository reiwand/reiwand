use crate::{DeviceKey, Error, Lock, Result, SyncKey};
use base64::Engine;
use base64::prelude::BASE64_STANDARD as B64;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use zeroize::{Zeroize, Zeroizing};

#[derive(Default, Serialize, Deserialize)]
pub struct Secrets {
    pub token: String,
    pub key: String,
    pub device: String,
    #[serde(default)]
    pub epoch: Option<i64>,
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub lock: String,
    #[serde(default)]
    pub handover: Option<(i64, String, BTreeSet<String>)>,
}

impl Drop for Secrets {
    fn drop(&mut self) {
        let bound = self.handover.as_mut().map(|(_, bound, _)| bound);
        [&mut self.token, &mut self.key, &mut self.device, &mut self.next, &mut self.lock]
            .into_iter()
            .chain(bound)
            .for_each(Zeroize::zeroize);
    }
}

const BROKEN: Error = Error::Crypto("the stored server secrets are broken; remove the server and link again");

pub fn encode(bytes: &[u8]) -> String {
    B64.encode(bytes)
}

fn decode(text: &str) -> Result<Zeroizing<Vec<u8>>> {
    Ok(Zeroizing::new(B64.decode(text).map_err(|_| BROKEN)?))
}

impl Secrets {
    pub fn sync_key(&self) -> Result<SyncKey> {
        SyncKey::from_bytes(&decode(&self.key)?)
            .map_err(|_| Error::Crypto("this device does not hold the sync key yet"))
    }

    pub fn next_key(&self) -> Result<Option<SyncKey>> {
        Ok(SyncKey::from_bytes(&decode(&self.next)?).ok())
    }

    pub fn set_key(&mut self, key: &SyncKey, epoch: i64) {
        (self.key, self.epoch, self.next) = (encode(&key.bytes()), Some(epoch), String::new());
    }

    pub fn escrow_lock(&self) -> Option<Lock> {
        Lock::from_bytes(&decode(&self.lock).ok()?)
    }

    pub fn device_key(&self) -> Result<DeviceKey> {
        DeviceKey::from_bytes(&decode(&self.device)?).map_err(|_| BROKEN)
    }
}
