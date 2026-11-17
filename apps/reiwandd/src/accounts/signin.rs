use super::store::{Link, Rec, clean};
use crate::account::{block_on, sync_error};
use crate::daemon::Daemon;
use crate::error::{Result, failed, invalid};
use crate::repos::pins::origin;
use reiwand_account::{AGENT, Api, hex, same_origin};
use reiwand_core::model::JobKind;
use reqwest::Method;
use serde_json::{Value, json};
use std::sync::Arc;

impl Daemon {
    pub fn sign_in(self: &Arc<Self>, repo_url: &str) -> Result<(String, String, u64)> {
        let site = origin(repo_url).ok_or_else(|| invalid("give the address of a repository"))?;
        let link = clean(repo_url);
        self.keyring_ready()?;
        let pin = self.repo_pins().get(&site).cloned();
        let api = Api::pinned(&site, pin.as_deref()).map_err(sync_error)?;
        let body = json!({"platform": "linux", "client": AGENT});
        let start = block_on(api.post("/api/v1/auth/device/start", body)).map_err(sync_error)?;
        let text = |field: &str| start[field].as_str().unwrap_or_default().to_owned();
        let (code, interval, expires) =
            (text("device_code"), start["interval"].as_u64(), start["expires_in"].as_u64());
        let (interval, expires) = (interval.unwrap_or(5).clamp(1, 60), expires.unwrap_or(600).min(1800));
        let target = format!("sign-in {link}");
        let (shown, home) = (text("verification_uri"), site.clone());
        let job = self.start_job(JobKind::Account, &target, move |d, ctl| {
            let token = d.enroll(&api, &code, interval, expires, ctl)?;
            d.finish_sign_in(&api, &home, &link, &token)
        });
        let uri = Some(shown).filter(|uri| same_origin(uri, &site));
        Ok((text("user_code"), uri.unwrap_or_else(|| format!("{site}/link")), job))
    }

    fn finish_sign_in(&self, api: &Api, site: &str, link: &str, token: &Value) -> Result<Value> {
        let text = |field: &str| token[field].as_str().unwrap_or_default().to_owned();
        let (secret, device) = (text("token"), text("device_id"));
        let authed = api.clone().with_token(&secret);
        let done = block_on(authed.get("/api/v1/me")).map_err(sync_error).and_then(|me| {
            let user = me["handle"].as_str().unwrap_or_default().chars().filter(|c| !c.is_control()).take(64);
            self.keep_account(site, link, me["id"].as_i64().unwrap_or_default(), user.collect(), &secret)
        });
        if done.is_err() && !device.is_empty() {
            let gone = format!("/api/v1/me/devices/{device}");
            let _ = block_on(authed.call(Method::DELETE, &gone, None));
        }
        done
    }

    pub(super) fn keep_account(
        &self,
        site: &str,
        link: &str,
        user_id: i64,
        user: String,
        secret: &str,
    ) -> Result<Value> {
        let same = |r: &Rec| r.origin == site && (r.user_id == user_id || (r.pending && r.user == user));
        let id = match self.account_list().into_iter().find(same) {
            Some(rec) => rec.id,
            None => format!("a{}", hex(&reiwand_sync::random::<5>())),
        };
        self.save_token(&id, secret)?;
        let host = site.split("://").nth(1).unwrap_or(site);
        let wanted = Link { url: link.to_owned(), repo_id: None };
        self.edit_accounts(|file| {
            if !link.is_empty() {
                file.accounts.iter_mut().for_each(|r| r.links.retain(|l| clean(&l.url) != clean(link)));
            }
            let rec = match file.accounts.iter().position(|r| r.id == id) {
                Some(index) => &mut file.accounts[index],
                None => {
                    file.accounts.push(Rec { id: id.clone(), origin: site.to_owned(), ..Rec::default() });
                    file.accounts.last_mut().ok_or_else(|| failed("no account"))?
                }
            };
            (rec.user, rec.user_id, rec.pending) = (user.clone(), user_id, false);
            if rec.label.is_empty() {
                rec.label = format!("{user}@{host}");
            }
            if !link.is_empty() {
                rec.links.push(wanted);
            }
            Ok(())
        })?;
        self.resolve_links();
        self.changes.changed("repos", Vec::new());
        Ok(json!({"account": id, "user": user, "server": site}))
    }
}
