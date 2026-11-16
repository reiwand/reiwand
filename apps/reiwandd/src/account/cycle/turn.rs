use super::super::secrets::encode;
use super::super::{block_on, sync_error};
use crate::daemon::Daemon;
use crate::error::{Result, failed};
use reiwand_sync::client::{escrow, rotation};
use reiwand_sync::{Clock, Ids, Replica, Session, View, arrived, carry, hold, now_ms, rules};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

impl Daemon {
    pub fn cycle(self: &Arc<Self>, prompt: bool) -> Result<Value> {
        self.linked()?;
        let (api, mut secrets) = self.account_api(prompt)?;
        let state = self.current_key(&api, &mut secrets, prompt || !self.saved().stale)?;
        if state["format"].as_i64() > Some(reiwand_sync::FORMAT) {
            return Err(failed(crate::words::too_old()));
        }
        let (owner, user, device, label, encrypted, epoch, before, mut kept) = {
            let s = self.saved();
            let before: BTreeSet<String> = match s.members.is_empty() {
                true => s.peers.iter().map(|p| p.id.clone()).collect(),
                false => s.members.clone(),
            };
            let device = s.device.clone().unwrap_or_default();
            (
                s.owner.clone(),
                s.user(),
                device,
                s.label.clone(),
                s.encrypted.unwrap_or(true),
                s.epoch,
                before,
                s.kept.clone(),
            )
        };
        let key = secrets.sync_key()?;
        let ids = key.ids();
        *crate::compositor::lock(&self.account.ids) = Some(ids.clone());
        let session = Session::new(api.clone(), user, epoch, encrypted, key);
        let (mut replica, mut links, mut clock, present) = self.replica_for(&ids);
        let removed = self.saved().removed.clone();
        self.removals(&ids, &mut replica);
        let mut local = self.snapshot(&ids, &replica, &mut links, &present, &mut kept)?;
        if encrypted {
            let key = encode(&secrets.device_key()?.public());
            let fields = [("name", json!(label)), ("platform", json!("linux")), ("key", json!(key))];
            let (key, mut own): (_, View) =
                (format!("device/{device}"), fields.map(|(k, v)| (k.to_owned(), v)).into());
            carry(&key, &mut own, &replica, &BTreeSet::new());
            local.views.insert(key, own);
        }
        let now = clock.tick(now_ms(), &device);
        let start = replica.bases.clone();
        let synced = block_on(reiwand_sync::sync(&session, &mut replica, &local.views, rules, &now));
        let synced = synced.map_err(sync_error)?;
        let (mut applied, unsent) = (synced.applied, synced.left);
        applied.extend(arrived(&local, &replica, &applied));
        let count = applied.len();
        let errors = self.apply_remote(&local, applied, &mut links, &replica, &mut kept);
        self.copy_playlists(&local, synced.copies, &mut links, &ids);
        let unapplied = hold(&mut replica, &start, &local, &errors, &now);
        let away =
            reiwand_sync::away(&replica, &local, &links, &self.offered_keys(&ids), &self.added_repos());
        replica.bases.values().for_each(|(_, doc)| clock.observe(&doc.clock()));
        let (rejected, records, offers) =
            (session.rejected(), replica.bases.len(), self.offers(&local, &replica));
        let left = self
            .refresh_peers(&api)
            .is_ok_and(|now| before.iter().any(|id| !now.iter().any(|p| &p.id == id)));
        self.pins()?;
        let (mut fresh, mut broken) = (0, 0);
        let stored = |key: &String| {
            replica.bases.get(&format!("library/{key}")).and_then(|(_, d)| d.view()?.get("blob").cloned())
        };
        let here: BTreeMap<String, String> = local
            .items
            .iter()
            .filter(|(key, _)| stored(key).is_some())
            .filter_map(|(key, uids)| Some((key.clone(), uids.first()?.clone())))
            .collect();
        let mut present: BTreeSet<String> = local.items.into_keys().chain(local.repos).collect();
        present.retain(|key| !unapplied.contains_key(&format!("library/{key}")));
        let number = |field: &str| state[field].as_u64().unwrap_or(0);
        self.update(|s| {
            if s.owner != owner {
                return;
            }
            fresh = rejected.iter().filter(|(k, seq)| s.quarantine.get(*k) != Some(seq)).count();
            broken = unapplied.iter().filter(|(k, seq)| s.unapplied.get(*k) != Some(seq)).count();
            s.quarantine.extend(rejected);
            s.quarantine.retain(|k, seq| replica.bases.get(k).is_none_or(|(base, _)| base < seq));
            (s.replica, s.links, s.clock, s.kept) = (replica, links, clock, kept);
            s.removed.retain(|key, _| !removed.contains_key(key));
            (s.present, s.offers, s.unapplied, s.away) = (present, offers, unapplied, away);
            s.warning = state["warning"].as_str().map(String::from);
            (s.used_bytes, s.quota_bytes) = (number("used_bytes"), number("quota_bytes"));
            (s.files_bytes, s.files_quota) = (number("files_bytes"), number("files_quota_bytes"));
            s.files.here = here;
        })?;
        if encrypted {
            block_on(escrow::keep(&**self, &api, &mut secrets, &state))?;
        }
        if encrypted && (state["rotate"] == true || left) {
            block_on(rotation::rotate(&**self, &api, &mut secrets, true))?;
        }
        let idle = {
            let f = &self.saved().files;
            f.folders.is_empty() && f.items.is_empty() && f.stored.is_empty() && f.dropped.is_empty()
        };
        if encrypted && !idle {
            self.files_job();
        }
        if fresh > 0 {
            return Err(failed(format!(
                "{fresh} records cannot be decrypted with this sync key; they are skipped from now on"
            )));
        }
        if let Some((_, error)) = errors.first().filter(|_| broken > 0) {
            return Err(failed(format!(
                "{broken} synced changes cannot be applied on this computer and are tried again at every sync: {error}"
            )));
        }
        let applied = count - errors.len();
        let quarantined = self.saved().quarantine.len();
        Ok(json!({"applied": applied, "records": records, "quarantined": quarantined, "unsent": unsent}))
    }

    fn replica_for(&self, ids: &Ids) -> (Replica, BTreeMap<String, String>, Clock, BTreeSet<String>) {
        let names = ids.named("names", "");
        let mut s = self.saved();
        if !s.names.is_empty() && s.names != names {
            (s.replica, s.links, s.present, s.quarantine) = Default::default();
        }
        s.names = names;
        (s.replica.clone(), s.links.clone(), s.clock.clone(), s.present.clone())
    }
}
