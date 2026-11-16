use super::{Fault, Side, label_of};
use crate::{Api, Peer, devices};
use serde_json::Value;
use std::collections::BTreeMap;

pub async fn refresh<S: Side>(side: &S, api: &Api) -> Result<Vec<Peer>, S::Error> {
    let peers = devices(api).await.map_err(Fault::Sync)?;
    side.update(|f| {
        f.pinned.iter_mut().for_each(|pins| pins.retain(|id, _| peers.iter().any(|p| p.id == *id)));
        f.members.extend(peers.iter().map(|p| p.id.clone()));
        *f.peers = peers.clone();
    })?;
    Ok(peers)
}

pub fn find(peers: &[Peer], wanted: &str, labels: impl Fn(&str) -> Option<String>) -> Result<Peer, Fault> {
    let wanted = wanted.trim();
    let matches = |p: &&Peer| {
        p.id == wanted || p.name.eq_ignore_ascii_case(wanted) || labels(&p.id).is_some_and(|l| l == wanted)
    };
    peers.iter().find(matches).cloned().ok_or_else(|| Fault::NoDevice(wanted.to_owned()))
}

pub async fn peer<S: Side>(side: &S, api: &Api, wanted: &str) -> Result<Peer, S::Error> {
    let peers = refresh(side, api).await?;
    let labels = |id: &str| side.read(|f| label_of(f.device.as_deref(), f.label, f.replica, id));
    let peer = find(&peers, wanted, labels)?;
    match side.read(|f| f.device.as_ref() == Some(&peer.id)) {
        true => Err(Fault::OwnDevice.into()),
        false => Ok(peer),
    }
}

pub fn pins<S: Side>(side: &S) -> Result<BTreeMap<String, String>, S::Error> {
    let seeded = side.read(|f| {
        if let Some(pins) = &f.pinned {
            return Ok(pins.clone());
        }
        let record = |id: &str| f.replica.bases.get(&format!("device/{id}")).and_then(|(_, doc)| doc.view());
        let vouched = |p: &&Peer| {
            record(&p.id).and_then(|v| v.get("key").cloned()) == p.enc_key.clone().map(Value::from)
        };
        let peers = f.peers.iter().filter(|p| Some(&p.id) != f.device.as_ref()).filter(vouched);
        Err(peers.filter_map(|p| Some((p.id.clone(), p.enc_key.clone()?))).collect::<BTreeMap<_, _>>())
    });
    seeded.or_else(|seeded| side.update(|f| *f.pinned = Some(seeded.clone())).map(|()| seeded))
}

pub fn pin<S: Side>(side: &S, peer: &str, key: &str) -> Result<(), S::Error> {
    side.update(|f| drop(f.pinned.get_or_insert_default().insert(peer.to_owned(), key.to_owned())))
}

pub async fn rekeyed<S: Side>(side: &S, api: &Api) -> Result<bool, S::Error> {
    let before = side.read(|f| f.members.clone());
    let peers = refresh(side, api).await?;
    let left = before.iter().any(|id| !peers.iter().any(|p| p.id == *id));
    side.update(|f| *f.members = peers.iter().map(|p| p.id.clone()).collect())?;
    Ok(left)
}
