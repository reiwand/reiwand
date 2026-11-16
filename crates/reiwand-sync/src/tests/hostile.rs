use super::engine::{Device, Server};
use super::{at, view};
use crate::*;
use crate::{Api, Replica, Session, SyncKey};
use serde_json::json;

const FAR: u64 = u64::MAX - 1;

#[test]
fn the_clock_counter_never_overflows() {
    let mut clock = Clock { ms: 5, n: u32::MAX };
    let first = clock.tick(3, "a");
    let second = clock.tick(3, "a");
    assert!(second >= first);
    clock.observe(&Hlc(FAR, u32::MAX, "b".into()));
    let last = clock.tick(1, "a");
    assert_eq!((last.0, last.1), (FAR, u32::MAX));
}

#[tokio::test]
async fn records_with_extreme_clocks_sync_without_a_panic() {
    let server = Server::default();
    let mut doc = Doc::default();
    doc.fields.insert("name".into(), (json!("Far"), Hlc(FAR, u32::MAX, "z".into())));
    doc.gone = Some(Hlc(u64::MAX, u32::MAX, "z".into()));
    server.records.borrow_mut().insert("lists/p".into(), (1, doc));
    server
        .records
        .borrow_mut()
        .insert("library/q".into(), (2, Doc { gone: Some(Hlc(u64::MAX, 0, "z".into())), ..Doc::default() }));
    let mut a = Device::new("a", &[("lists/p", view(&[("name", json!("Mine"))]))]);
    for ms in [10, 20, u64::MAX] {
        a.sync_by(&server, ms, |_| Rules { clash: |_, _, _| Some(Winner::Mine), ..Rules::default() }).await;
    }
    let mut b = Device::new("b", &[("lists/p", view(&[("name", json!("Other"))]))]);
    b.sync(&server, 30).await;
}

struct Rng(u64);

impl Rng {
    fn next(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n
    }
}

#[tokio::test]
async fn devices_with_jumping_clocks_and_random_edits_still_converge() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for round in 0..300 {
        let server = Server::default();
        let keys = ["lists/a", "lists/b", "library/c"];
        let mut devices = [Device::new("a", &[]), Device::new("b", &[]), Device::new("c", &[])];
        let mut clock = [1_000u64; 3];
        for step in 0..30 {
            let who = rng.next(3) as usize;
            clock[who] = match rng.next(4) {
                0 => clock[who].saturating_sub(rng.next(5_000)),
                1 => clock[who] + rng.next(1 << 26),
                _ => clock[who] + 1 + rng.next(50),
            };
            let key = keys[rng.next(3) as usize];
            match rng.next(3) {
                0 => drop(devices[who].local.remove(key)),
                _ => {
                    let value = json!(format!("v{round}-{step}"));
                    devices[who].local.insert(key.into(), view(&[("name", value)]));
                }
            }
            if rng.next(2) == 0 {
                devices[who].sync_by(&server, clock[who], |_| Rules::default()).await;
            }
        }
        for _ in 0..3 {
            for who in 0..3 {
                clock[who] += 1;
                devices[who].sync_by(&server, clock[who], |_| Rules::default()).await;
            }
        }
        assert!(
            devices[0].local == devices[1].local && devices[1].local == devices[2].local,
            "round {round}: {:?} {:?} {:?} server {:?} bases {:?}",
            devices[0].local,
            devices[1].local,
            devices[2].local,
            server.records.borrow().get("library/c"),
            devices.iter().map(|d| d.replica.bases.get("library/c").cloned()).collect::<Vec<_>>()
        );
    }
}

#[tokio::test]
async fn an_answer_that_is_not_a_list_of_changes_leaves_the_replica_untouched() {
    for junk in
        [json!({"hello": "portal"}), json!([]), json!("text"), json!(null), json!({"seq": "x", "changes": 3})]
    {
        let url = super::protocol::pages(vec![junk.clone(), junk.clone()]);
        let session = Session::new(Api::new(&url).unwrap(), 3, 1, false, SyncKey::generate());
        let kept = super::doc(&[("title", json!("Dawn"), 5)]);
        let mut replica = Replica { seq: 5, ..Replica::default() };
        replica.bases.insert("library/ab".into(), (4, kept));
        let before = (replica.seq, replica.bases.clone());
        let local = Default::default();
        let result = crate::sync(&session, &mut replica, &local, |_| Rules::default(), &at(9, "a")).await;
        assert!(result.is_err(), "{junk}: the sync went on with an answer that is not a page");
        assert_eq!((replica.seq, replica.bases.clone()), before, "{junk}");
    }
}
