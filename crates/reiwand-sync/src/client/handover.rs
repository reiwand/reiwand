use super::rotation::marks;
use super::{Fault, Side, keys};
use crate::{Api, Mail, Secrets, aad, encode, post, seal_key};
use base64::Engine;
use base64::prelude::BASE64_STANDARD as B64;
use std::collections::BTreeSet;
use std::time::Duration;
use zeroize::Zeroizing;

pub(super) async fn hand_over<S: Side>(
    side: &S,
    api: &Api,
    secrets: &mut Secrets,
    epoch: i64,
    bound: &[u8],
    targets: Option<BTreeSet<String>>,
) -> Result<Vec<String>, S::Error> {
    if let Some(ms) = std::env::var("REIWAND_HANDOVER_DELAY_MS").ok().and_then(|v| v.parse().ok()) {
        std::thread::sleep(Duration::from_millis(ms));
    }
    let pins = keys::pins(side)?;
    let (me, user, encrypted, _) = marks(side);
    let peers = side.read(|f| f.peers.clone());
    let (mine, key) = (secrets.device_key().map_err(Fault::Sync)?, secrets.sync_key().map_err(Fault::Sync)?);
    let wanted = |id: &String| *id != me && targets.as_ref().is_none_or(|t| t.contains(id));
    let (mut shared, mut waiting) = (Vec::new(), BTreeSet::new());
    for (id, theirs) in pins.iter().filter(|(id, _)| wanted(id)) {
        let to = aad(user, id, epoch, encrypted, bound);
        let theirs = B64.decode(theirs).map_err(|_| Fault::PinBroken);
        let sealed = theirs.and_then(|theirs| Ok(seal_key(&mine, &theirs, &to, &key)?));
        let sent = match sealed {
            Ok(sealed) => post(api, id, &Mail::body("rotate", "", &sealed, epoch)).await.map_err(Fault::Sync),
            Err(e) => Err(e),
        };
        match sent.map_err(S::Error::from) {
            Ok(()) => shared.push(peers.iter().find(|p| p.id == *id).map_or(id.clone(), |p| p.name.clone())),
            Err(e) => {
                tracing::warn!("the new key waits to reach a device: {e}");
                waiting.insert(id.clone());
            }
        }
    }
    secrets.handover = (!waiting.is_empty()).then(|| (epoch, encode(bound), waiting));
    side.keep(secrets).await?;
    Ok(shared)
}

pub(super) async fn resume<S: Side>(side: &S, api: &Api, secrets: &mut Secrets) -> Result<(), S::Error> {
    let Some((epoch, bound, waiting)) = secrets.handover.clone() else { return Ok(()) };
    let bound = Zeroizing::new(B64.decode(&bound).unwrap_or_default());
    match epoch == secrets.epoch.unwrap_or(marks(side).3) {
        true => hand_over(side, api, secrets, epoch, &bound, Some(waiting)).await.map(drop),
        false => {
            secrets.handover = None;
            side.keep(secrets).await
        }
    }
}
