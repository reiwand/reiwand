use super::{Fault, Side, keys};
use crate::{Api, Lock, MIN_PASSPHRASE, STRONG, Secrets, encode, escrow_epoch, unseal};
use base64::Engine;
use base64::prelude::BASE64_STANDARD as B64;
use reqwest::Method;
use serde_json::{Value, json};
use zeroize::Zeroizing;

const PATH: &str = "/api/v1/sync/escrow";

async fn stored(api: &Api) -> crate::Result<Option<Vec<u8>>> {
    let found = api.get(PATH).await?;
    Ok(found["escrow"].as_str().and_then(|text| B64.decode(text).ok()))
}

async fn put(api: &Api, escrow: Option<Vec<u8>>) -> crate::Result<()> {
    let body = json!({"escrow": escrow.map(|bytes| B64.encode(bytes))});
    api.call(Method::PUT, PATH, Some(body)).await.map(drop)
}

pub fn check<S: Side>(side: &S, passphrase: &str) -> Result<(), S::Error> {
    let (ready, encrypted) = side.read(|f| (*f.ready, *f.encrypted));
    if !ready {
        return Err(Fault::NoKey.into());
    }
    if encrypted != Some(true) {
        return Err(Fault::NeedsEncryption.into());
    }
    if !passphrase.is_empty() && passphrase.trim().chars().count() < MIN_PASSPHRASE {
        return Err(Fault::ShortPhrase.into());
    }
    Ok(())
}

pub async fn set<S: Side>(
    side: &S,
    api: &Api,
    secrets: &mut Secrets,
    passphrase: &Zeroizing<String>,
) -> Result<Value, S::Error> {
    let (user, saved, kept) = side.read(|f| (f.user(), *f.epoch, *f.recovery));
    let epoch = secrets.epoch.unwrap_or(saved);
    let stale = kept.is_some_and(|kept| kept != epoch);
    let old = if stale && !passphrase.is_empty() { stored(api).await.map_err(Fault::Sync)? } else { None };
    if old.is_some_and(|old| unseal(&old, passphrase, user).is_ok()) {
        return Err(Fault::SamePhrase.into());
    }
    let lock = match passphrase.is_empty() {
        true => None,
        false => Some(Lock::new(passphrase, STRONG).map_err(Fault::Sync)?),
    };
    let sealed =
        lock.as_ref().map(|l| l.seal(&secrets.sync_key()?, user, epoch)).transpose().map_err(Fault::Sync)?;
    put(api, sealed).await.map_err(Fault::Sync)?;
    secrets.lock = lock.map_or_else(String::new, |l| encode(&l.bytes()));
    side.keep(secrets).await?;
    side.update(|f| f.recover((!passphrase.is_empty()).then_some(epoch), false))?;
    tracing::info!(set = !passphrase.is_empty(), "the recovery escrow changed");
    Ok(json!({"recovery": if passphrase.is_empty() { "off" } else { "current" }}))
}

pub async fn restore<S: Side>(
    side: &S,
    api: &Api,
    secrets: &mut Secrets,
    passphrase: &Zeroizing<String>,
) -> Result<Value, S::Error> {
    let state = api.get("/api/v1/sync/state").await.map_err(Fault::Sync)?;
    let escrow = stored(api).await.map_err(Fault::Sync)?.ok_or(Fault::NoEscrow)?;
    let (key, epoch, lock) = unseal(&escrow, passphrase, side.read(|f| f.user())).map_err(Fault::Sync)?;
    let seen = side.read(|f| *f.epoch);
    if epoch < seen {
        return Err(Fault::EscrowReplayed { epoch, seen }.into());
    }
    let current =
        state["epoch"].as_i64() == Some(epoch) && state["key_id"].as_str().is_none_or(|id| id == key.id());
    if !current {
        return Err(Fault::EscrowOldKey.into());
    }
    secrets.set_key(&key, epoch);
    secrets.lock = encode(&lock.bytes());
    side.keep(secrets).await?;
    side.update(|f| {
        (*f.ready, *f.stale, *f.epoch) = (true, false, epoch);
        f.recover(Some(epoch), false);
        f.quiet();
        (f.replica.seq, *f.quarantine, *f.unapplied) = Default::default();
        f.pinned.get_or_insert_default();
    })?;
    keys::rekeyed(side, api).await?;
    tracing::info!("the sync key was restored from the recovery escrow");
    Ok(json!({"epoch": epoch}))
}

pub async fn keep<S: Side>(
    side: &S,
    api: &Api,
    secrets: &mut Secrets,
    state: &Value,
) -> Result<(), S::Error> {
    let escrow = match state["escrow"] == true {
        true => match stored(api).await {
            Err(_) if S::KEEPS_GOING_WITHOUT_ESCROW => None,
            found => found.map_err(Fault::Sync)?,
        },
        false => None,
    };
    let other = escrow.as_ref().is_some_and(|e| secrets.escrow_lock().is_some_and(|lock| !lock.fits(e)));
    if other {
        secrets.lock.clear();
        side.keep(secrets).await?;
    }
    let sealed = escrow.as_deref().and_then(escrow_epoch);
    let stale = sealed.is_some_and(|epoch| Some(epoch) < state["epoch"].as_i64());
    side.update(|f| f.recover(sealed, stale))
}

pub async fn reseal<S: Side>(side: &S, api: &Api, secrets: &Secrets) -> Result<(), S::Error> {
    let Some(lock) = secrets.escrow_lock() else { return Ok(()) };
    if !stored(api).await.map_err(Fault::Sync)?.is_some_and(|escrow| lock.fits(&escrow)) {
        return Ok(());
    }
    let (user, saved) = side.read(|f| (f.user(), *f.epoch));
    let epoch = secrets.epoch.unwrap_or(saved);
    let sealed = secrets.sync_key().and_then(|key| lock.seal(&key, user, epoch)).map_err(Fault::Sync)?;
    put(api, Some(sealed)).await.map_err(Fault::Sync)?;
    side.update(|f| f.recover(Some(epoch), false))
}
