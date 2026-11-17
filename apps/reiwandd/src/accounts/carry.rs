use super::store::{Link, Rec};
use super::{block_on, hostname, random, sync_error};
use crate::daemon::Daemon;
use crate::error::Result;
use reiwand_account::{Api, hex};
use reiwand_carry::Session;
use serde_json::json;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Imported {
    pub account: String,
    pub origin: String,
    pub repos: Vec<String>,
    pub pending: Option<String>,
}

impl Daemon {
    pub fn export_sessions(&self) -> Vec<Session> {
        let mut out = Vec::new();
        for rec in self.account_list().into_iter().filter(|r| !r.pending) {
            let Some(token) = self.account_token(&rec.id) else { continue };
            for id in rec.links.iter().filter_map(|l| l.repo_id.clone()) {
                let (origin, account, token) = (rec.origin.clone(), rec.user.clone(), token.to_string());
                out.push(Session { repo_id: id, origin, account, token });
            }
        }
        out
    }

    pub fn import_sessions(&self, sessions: &[Session]) -> Vec<Imported> {
        let linked: Vec<String> = self
            .account_list()
            .iter()
            .flat_map(|r| r.links.iter().filter_map(|l| l.repo_id.clone()))
            .collect();
        let mut groups: BTreeMap<(String, String, String), Vec<String>> = BTreeMap::new();
        for s in sessions.iter().filter(|s| !linked.contains(&s.repo_id)) {
            let key = (s.origin.clone(), s.token.clone(), s.account.clone());
            groups.entry(key).or_default().push(s.repo_id.clone());
        }
        let mut done = Vec::new();
        for ((origin, token, account), repos) in groups {
            let copied = self.copy_session(&origin, &token);
            let pending = copied.as_ref().err().map(|e| e.parts().1);
            let kept = copied
                .and_then(|(user_id, user, secret)| self.keep_account(&origin, "", user_id, user, &secret));
            let id = kept.as_ref().ok().and_then(|v| v["account"].as_str().map(String::from));
            let pending = pending.or_else(|| kept.err().map(|e| e.parts().1));
            if let Err(e) = self.attach(&origin, &account, &repos, id.as_deref()) {
                tracing::warn!("an imported account could not be linked: {}", e.parts().1);
            }
            done.push(Imported { account, origin, repos, pending });
        }
        self.changes.changed("repos", Vec::new());
        done
    }

    fn copy_session(&self, origin: &str, token: &str) -> Result<(i64, String, String)> {
        let pin = self.repo_pins().get(origin).cloned();
        let api = Api::pinned(origin, pin.as_deref()).map_err(sync_error)?.with_token(token);
        let made = block_on(api.post("/api/v1/auth/session/copy", json!({"name": hostname()})));
        let copy = made.map_err(sync_error)?;
        let secret = copy["token"].as_str().unwrap_or_default().to_owned();
        let me = block_on(api.clone().with_token(&secret).get("/api/v1/me")).map_err(sync_error)?;
        let user = me["handle"].as_str().unwrap_or_default().chars().filter(|c| !c.is_control()).take(64);
        Ok((me["id"].as_i64().unwrap_or_default(), user.collect(), secret))
    }

    fn attach(&self, origin: &str, account: &str, repos: &[String], id: Option<&str>) -> Result<()> {
        let links: Vec<Link> =
            repos.iter().map(|id| Link { url: String::new(), repo_id: Some(id.clone()) }).collect();
        self.edit_accounts(|file| {
            let wanted = |r: &Rec| match id {
                Some(id) => r.id == id,
                None => r.origin == origin && r.pending && r.user == account,
            };
            let index = match (file.accounts.iter().position(wanted), id) {
                (Some(index), _) => index,
                (None, Some(_)) => return Ok(()),
                (None, None) => {
                    let host = origin.split("://").nth(1).unwrap_or(origin);
                    file.accounts.push(Rec {
                        id: format!("a{}", hex(&random::<5>())),
                        origin: origin.into(),
                        user: account.into(),
                        label: format!("{account}@{host}"),
                        pending: true,
                        ..Rec::default()
                    });
                    file.accounts.len() - 1
                }
            };
            file.accounts[index].links.retain(|l| !links.iter().any(|n| n.repo_id == l.repo_id));
            file.accounts[index].links.extend(links.iter().cloned());
            Ok(())
        })
    }
}
