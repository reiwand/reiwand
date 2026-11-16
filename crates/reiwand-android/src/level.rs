use crate::account::{LISTEN, NETWORK};
use crate::side::Phone;
use crate::state::{api, linked, read, update};
use crate::{Error, link, pairing, said, settings};
use reiwand_sync::client::keys;
use reiwand_sync::{Api, Peer, Secrets};
use serde_json::{Value, json};

pub(crate) fn status() -> Value {
    read(|s| {
        let device = |p: &Peer| {
            let current = s.device.as_ref() == Some(&p.id);
            json!({"id": p.id, "name": p.name, "label": s.label_of(&p.id), "platform": p.platform,
                "seen_at": p.seen_at, "current": current})
        };
        json!({
            "url": s.url, "name": s.name, "handle": s.handle, "device": s.device_name, "label": s.label,
            "state": s.state(), "encrypted": s.encrypted, "paused": s.paused, "last_sync": s.last_sync,
            "last_error": s.last_error, "last_reply": s.last_reply, "warning": s.warning.as_ref().filter(|_| !s.sync_off),
            "records": s.replica.bases.len(), "quarantined": s.quarantine.len(), "unapplied": s.unapplied.len(),
            "devices": s.peers.iter().map(device).collect::<Vec<_>>(), "link": link::shown(),
            "pairing": pairing::shown(), "network": settings::chosen(NETWORK),
            "listen": settings::chosen(LISTEN), "active": s.active(),
            "outdated": s.outdated, "signed_out": s.signed_out, "recovery_stale": s.recovery_stale, "pin": s.pin,
            "recovery": match (s.recovery, s.recovery_stale) { (None, _) => "off", (_, true) => "stale", _ => "current" },
            "offers": s.offers.len(), "files": s.files.items.len(), "repository_failed": s.repo_failed,
        })
    })
}

pub(crate) fn syncing() -> Result<(), Error> {
    linked()?;
    match read(|s| s.sync_off) {
        true => Err(said("android.syncing_off", "syncing is off on this phone; turn it on first", &[])),
        false => Ok(()),
    }
}

pub(crate) async fn setup(
    api: &Api,
    secrets: &mut Secrets,
    existing: Option<Value>,
    plaintext: bool,
) -> Result<Value, Error> {
    let state = reiwand_sync::client::level::setup(&Phone, api, secrets, existing, plaintext).await?;
    drop(keys::refresh(&Phone, api).await);
    Ok(state)
}

pub(crate) async fn set(on: bool, plaintext: bool) -> Result<Value, Error> {
    linked()?;
    if !on {
        pairing::cancel();
        update(|s| (s.sync_off, s.last_error, s.last_reply) = (true, None, None))?;
        return Ok(json!({"syncing": false, "active": false}));
    }
    let (api, mut secrets) = api()?;
    let existing = reiwand_sync::client::level::sync_state::<Phone>(&api, plaintext).await?;
    let state = setup(&api, &mut secrets, existing, plaintext).await?;
    let needs_key = secrets.key.is_empty();
    Ok(
        json!({"syncing": true, "encrypted": state["encrypted"], "needs_key": needs_key, "active": crate::state::active()}),
    )
}
