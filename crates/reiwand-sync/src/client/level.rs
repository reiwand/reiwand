use super::{Fault, Side};
use crate::{Api, Secrets, SyncKey};
use serde_json::{Value, json};

pub async fn sync_state<S: Side>(api: &Api, plaintext: bool) -> Result<Option<Value>, S::Error> {
    let existing = match api.get("/api/v1/sync/state").await {
        Err(e) if e.code() == "sync_not_set_up" => None,
        state => Some(state.map_err(Fault::Sync)?),
    };
    if existing.as_ref().is_some_and(|state| state["encrypted"] != true) && !plaintext {
        return Err(Fault::NeedsPlain.into());
    }
    Ok(existing)
}

pub async fn setup<S: Side>(
    side: &S,
    api: &Api,
    secrets: &mut Secrets,
    existing: Option<Value>,
    plaintext: bool,
) -> Result<Value, S::Error> {
    let state = match existing {
        Some(state) => state,
        None => {
            let sync = SyncKey::generate();
            secrets.set_key(&sync, 0);
            side.keep(secrets).await?;
            let body = json!({"encrypted": !plaintext, "confirm_plaintext": plaintext, "key_id": sync.id()});
            let state = api.post("/api/v1/sync/setup", body).await.map_err(Fault::Sync)?;
            if state["encrypted"].as_bool() != Some(!plaintext) {
                return Err(Fault::OtherEncryption.into());
            }
            secrets.set_key(&sync, state["epoch"].as_i64().unwrap_or(0));
            side.keep(secrets).await?;
            side.update(|f| (*f.ready, *f.stale) = (false, false))?;
            state
        }
    };
    let fresh = side.read(|f| !*f.ready && !*f.stale);
    side.update(|f| {
        if fresh {
            (*f.epoch, *f.ready) = (state["epoch"].as_i64().unwrap_or(0), !secrets.key.is_empty());
        }
        (*f.encrypted, *f.warning) =
            (state["encrypted"].as_bool(), state["warning"].as_str().map(String::from));
        (*f.plaintext, *f.sync_off) = (plaintext, false);
        f.quiet();
    })?;
    Ok(state)
}
