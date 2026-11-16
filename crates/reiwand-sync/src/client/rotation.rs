use super::handover::{hand_over, resume};
use super::{Fault, Side, escrow, keys};
use crate::{Api, Secrets, Session, aad, ack, encode, inbox, open_key, rekey};
use base64::Engine;
use base64::prelude::BASE64_STANDARD as B64;
use serde_json::{Value, json};
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

const KEY_POLL: Duration = Duration::from_millis(500);

pub(super) fn marks<S: Side>(side: &S) -> (String, i64, bool, i64) {
    side.read(|f| (f.me(), f.user(), f.encrypted.unwrap_or(true), *f.epoch))
}

pub fn check<S: Side>(side: &S) -> Result<(), S::Error> {
    match side.read(|f| (*f.ready, *f.encrypted)) {
        (false, _) => Err(Fault::NoKey.into()),
        (_, Some(false)) => Err(Fault::PlainRotation.into()),
        _ => Ok(()),
    }
}

pub async fn current<S: Side>(
    side: &S,
    api: &Api,
    secrets: &mut Secrets,
    wait: Duration,
) -> Result<Value, S::Error> {
    let get = async || api.get("/api/v1/sync/state").await.map_err(Fault::Sync);
    let (mut state, encrypted) = (get().await?, side.read(|f| f.encrypted.unwrap_or(true)));
    if state["encrypted"].as_bool() != Some(encrypted) {
        return Err(Fault::EncryptionChanged.into());
    }
    if secrets.key.is_empty() {
        return Err(Fault::NoKey.into());
    }
    if encrypted && !secrets.next.is_empty() {
        rotate(side, api, secrets, state["rotate"] == true).await?;
        state = get().await?;
    }
    let (server, id) = (state["epoch"].as_i64().unwrap_or_default(), state["key_id"].as_str());
    let (mut epoch, mut adopted) = (secrets.epoch.unwrap_or(marks(side).3), None);
    let deadline = Instant::now() + wait;
    while encrypted && server > epoch {
        let next = id.filter(|_| epoch + 1 == server);
        let mut found = adopt(side, api, secrets, epoch + 1, next).await?;
        while found.is_none() && Instant::now() < deadline {
            std::thread::sleep(KEY_POLL);
            found = adopt(side, api, secrets, epoch + 1, next).await?;
        }
        let Some(found) = found else {
            tracing::info!(
                server,
                epoch,
                waited = !wait.is_zero(),
                "no new sync key arrived from a pinned device"
            );
            break;
        };
        (epoch, adopted) = (epoch + 1, Some(found));
    }
    let key = secrets.sync_key().map_err(Fault::Sync)?;
    if server != epoch || id.is_some_and(|id| id != key.id()) {
        side.update(|f| (*f.ready, *f.stale) = (false, true))?;
        return Err(Fault::KeyChanged.into());
    }
    if let Some((from, bound)) = adopted {
        if keys::rekeyed(side, api).await? {
            secrets.lock.clear();
            side.keep(secrets).await?;
        }
        let others = keys::pins(side)?.into_keys().filter(|id| *id != from).collect();
        hand_over(side, api, secrets, epoch, &*bound, Some(others)).await?;
    }
    if id.is_none() {
        api.post("/api/v1/sync/epoch", json!({"epoch": server, "key_id": key.id()}))
            .await
            .map_err(Fault::Sync)?;
    }
    side.update(|f| (*f.ready, *f.stale, *f.epoch) = (true, false, server))?;
    resume(side, api, secrets).await?;
    Ok(state)
}

async fn adopt<S: Side>(
    side: &S,
    api: &Api,
    secrets: &mut Secrets,
    epoch: i64,
    key_id: Option<&str>,
) -> Result<Option<(String, Zeroizing<[u8; 32]>)>, S::Error> {
    let (old, mine) = (secrets.sync_key().map_err(Fault::Sync)?, secrets.device_key().map_err(Fault::Sync)?);
    let pins = keys::pins(side)?;
    let (me, user, encrypted, _) = marks(side);
    let check = old.check(epoch);
    let to = aad(user, &me, epoch, encrypted, &*check);
    for letter in inbox(api).await.map_err(Fault::Sync)?.into_iter().filter(|l| l.rotation()) {
        let at = letter.mail.as_ref().map_or(i64::MIN, |m| m.epoch);
        let theirs = pins.get(&letter.from).and_then(|k| B64.decode(k).ok());
        let (Some(theirs), Ok(mail)) = (theirs.filter(|_| at == epoch), &letter.mail) else {
            if at < epoch {
                ack(api, letter.id).await.map_err(Fault::Sync)?;
            }
            continue;
        };
        let opened = mail.bytes().ok().and_then(|sealed| open_key(&mine, &theirs, &to, &sealed).ok());
        ack(api, letter.id).await.map_err(Fault::Sync)?;
        if let Some(key) = opened.filter(|k| key_id.is_none_or(|id| id == k.id())) {
            secrets.set_key(&key, epoch);
            side.keep(secrets).await?;
            return Ok(Some((letter.from, check)));
        }
    }
    Ok(None)
}

pub async fn rotate<S: Side>(
    side: &S,
    api: &Api,
    secrets: &mut Secrets,
    revoked: bool,
) -> Result<Option<Value>, S::Error> {
    let (old, epoch) = (secrets.sync_key().map_err(Fault::Sync)?, secrets.epoch.unwrap_or(marks(side).3));
    let next = match secrets.next_key().map_err(Fault::Sync)? {
        Some(next) => next,
        None => {
            secrets.next = encode(&old.rotated().bytes());
            side.keep(secrets).await?;
            secrets.next_key().map_err(Fault::Sync)?.ok_or(Fault::KeyNotStored)?
        }
    };
    match api.post("/api/v1/sync/epoch", json!({"epoch": epoch + 1, "key_id": next.id()})).await {
        Err(e) if e.code() == "epoch_mismatch" => {
            secrets.next.clear();
            side.keep(secrets).await?;
            return Ok(None);
        }
        claimed => drop(claimed.map_err(Fault::Sync)?),
    }
    let (_, user, encrypted, _) = marks(side);
    let bound = old.check(epoch + 1);
    let mut session = Session::new(api.clone(), user, epoch + 1, encrypted, next);
    session.old = Some(old);
    let records = rekey(&session, epoch + 1).await.map_err(Fault::Sync)?;
    api.post("/api/v1/sync/epoch", json!({"epoch": epoch + 1, "done": true})).await.map_err(Fault::Sync)?;
    secrets.set_key(&session.key, epoch + 1);
    side.keep(secrets).await?;
    side.update(|f| (*f.epoch, *f.ready, *f.stale) = (epoch + 1, true, false))?;
    match keys::rekeyed(side, api).await? || revoked {
        true => {
            secrets.lock.clear();
            side.keep(secrets).await?;
        }
        false => escrow::reseal(side, api, secrets).await?,
    }
    let shared = hand_over(side, api, secrets, epoch + 1, &*bound, None).await?;
    tracing::info!(epoch = epoch + 1, records, shared = shared.len(), "the sync key was replaced");
    Ok(Some(json!({"epoch": epoch + 1, "records": records, "shared": shared})))
}
