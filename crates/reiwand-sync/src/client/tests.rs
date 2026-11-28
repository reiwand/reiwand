use super::{Fault, Fields, Side, escrow, keys, label_of, rotation};
use crate::{Peer, Replica, Secrets};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
struct Saved {
    owner: Option<(String, i64)>,
    device: Option<String>,
    encrypted: Option<bool>,
    plaintext: bool,
    epoch: i64,
    ready: bool,
    stale: bool,
    sync_off: bool,
    last_error: Option<String>,
    warning: Option<String>,
    peers: Vec<Peer>,
    pinned: Option<BTreeMap<String, String>>,
    members: BTreeSet<String>,
    replica: Replica,
    quarantine: BTreeMap<String, i64>,
    unapplied: BTreeMap<String, i64>,
    recovery: Option<i64>,
}

#[derive(Default)]
struct Fake(RefCell<Saved>, RefCell<usize>);

#[derive(Debug, PartialEq)]
struct Told(&'static str);

impl std::fmt::Display for Told {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl From<Fault> for Told {
    fn from(fault: Fault) -> Self {
        Told(match fault {
            Fault::NoKey => "no key",
            Fault::NeedsEncryption => "needs encryption",
            Fault::ShortPhrase => "short",
            Fault::PlainRotation => "plain",
            Fault::NoDevice(_) => "no device",
            _ => "other",
        })
    }
}

impl Side for Fake {
    type Error = Told;
    const KEEPS_GOING_WITHOUT_ESCROW: bool = false;

    fn read<T>(&self, look: impl FnOnce(&mut Fields<'_>) -> T) -> T {
        let s = &mut *self.0.borrow_mut();
        look(&mut Fields {
            owner: &s.owner,
            device: &s.device,
            label: "here",
            encrypted: &mut s.encrypted,
            plaintext: &mut s.plaintext,
            epoch: &mut s.epoch,
            ready: &mut s.ready,
            stale: &mut s.stale,
            sync_off: &mut s.sync_off,
            last_error: &mut s.last_error,
            last_reply: None,
            warning: &mut s.warning,
            peers: &mut s.peers,
            pinned: &mut s.pinned,
            members: &mut s.members,
            replica: &mut s.replica,
            quarantine: &mut s.quarantine,
            unapplied: &mut s.unapplied,
            recovery: &mut s.recovery,
            recovery_stale: None,
        })
    }

    fn update(&self, change: impl FnOnce(&mut Fields<'_>)) -> Result<(), Told> {
        *self.1.borrow_mut() += 1;
        self.read(change);
        Ok(())
    }

    async fn keep(&self, _: &Secrets) -> Result<(), Told> {
        Ok(())
    }
}

fn peer(id: &str, name: &str, key: Option<&str>) -> Peer {
    Peer { id: id.into(), name: name.into(), enc_key: key.map(String::from), ..Peer::default() }
}

#[test]
fn the_checks_before_a_key_change_come_in_order() {
    let side = Fake::default();
    let long = "a".repeat(crate::MIN_PASSPHRASE);
    assert_eq!(escrow::check(&side, &long), Err(Told("no key")));
    side.0.borrow_mut().ready = true;
    assert_eq!(escrow::check(&side, &long), Err(Told("needs encryption")));
    assert_eq!(rotation::check(&side), Ok(()));
    side.0.borrow_mut().encrypted = Some(true);
    assert_eq!(escrow::check(&side, "  short  "), Err(Told("short")));
    assert_eq!(escrow::check(&side, ""), Ok(()));
    assert_eq!(escrow::check(&side, &long), Ok(()));
    side.0.borrow_mut().encrypted = Some(false);
    assert_eq!(rotation::check(&side), Err(Told("plain")));
}

#[test]
fn a_device_is_found_by_id_name_or_label_and_pins_seed_once() {
    let peers = [peer("d1", "Desk", Some("k1")), peer("d2", "Phone", Some("k2")), peer("d3", "Old", None)];
    let labels = |id: &str| (id == "d2").then(|| "pocket".to_owned());
    assert_eq!(keys::find(&peers, " desk ", labels).map(|p| p.id).ok(), Some("d1".into()));
    assert_eq!(keys::find(&peers, "pocket", labels).map(|p| p.id).ok(), Some("d2".into()));
    assert!(matches!(keys::find(&peers, "none", labels), Err(Fault::NoDevice(name)) if name == "none"));
    let side = Fake::default();
    side.0.borrow_mut().peers = peers.to_vec();
    side.0.borrow_mut().device = Some("d2".into());
    assert_eq!(keys::pins(&side), Ok(BTreeMap::new()));
    assert_eq!(*side.1.borrow(), 1);
    keys::pin(&side, "d1", "k1").unwrap();
    assert_eq!(keys::pins(&side).unwrap().into_iter().collect::<Vec<_>>(), [("d1".into(), "k1".into())]);
    assert_eq!(*side.1.borrow(), 2);
    let replica = Replica::default();
    assert_eq!(label_of(Some("d2"), "here", &replica, "d2").as_deref(), Some("here"));
    assert_eq!(label_of(Some("d2"), "here", &replica, "d1"), None);
}
