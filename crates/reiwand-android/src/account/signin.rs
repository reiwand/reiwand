use super::book::{edit, keep_token, random_id};
use crate::state::pause;
use crate::{Error, said};
use reiwand_account::{AGENT, Api, Link, Poll, Rec, clean, origin, poll, same_origin};
use serde_json::{Value, json};
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

struct Pending {
    api: Api,
    site: String,
    link: String,
    code: String,
    interval: u64,
    until: Instant,
}

static PENDING: Mutex<Option<Pending>> = Mutex::new(None);

pub(crate) async fn start(repo_url: &str) -> Result<Value, Error> {
    crate::net::open()?;
    let site = origin(repo_url)
        .ok_or_else(|| said("android.repo_https", "A repository address starts with https://", &[]))?;
    let pin = super::pins::all().get(&site).cloned();
    let api = crate::net::api(&site, pin.as_deref())?;
    let begun =
        api.post("/api/v1/auth/device/start", json!({"platform": "android", "client": AGENT})).await?;
    let text = |field: &str| begun[field].as_str().unwrap_or_default().to_owned();
    let (interval, expires) = (begun["interval"].as_u64(), begun["expires_in"].as_u64());
    let (interval, expires) = (interval.unwrap_or(5).clamp(1, 60), expires.unwrap_or(600).min(1800));
    let until = Instant::now() + Duration::from_secs(expires);
    let shown = Some(text("verification_uri")).filter(|uri| same_origin(uri, &site));
    let uri = shown.unwrap_or_else(|| format!("{site}/link"));
    let pending = Pending { api, site, link: clean(repo_url), code: text("device_code"), interval, until };
    *PENDING.lock().unwrap_or_else(PoisonError::into_inner) = Some(pending);
    Ok(json!({"code": text("user_code"), "uri": uri}))
}

pub(crate) fn cancel() {
    PENDING.lock().unwrap_or_else(PoisonError::into_inner).take();
}

pub(crate) async fn wait() -> Result<Value, Error> {
    let pending = PENDING.lock().unwrap_or_else(PoisonError::into_inner).take();
    let pending = pending
        .ok_or_else(|| said("android.signin_waiting", "No sign-in is waiting; start it again", &[]))?;
    let token = approved(&pending).await?;
    finish(&pending, &token).await
}

async fn approved(pending: &Pending) -> Result<Value, Error> {
    let mut interval = pending.interval;
    while Instant::now() < pending.until {
        pause(interval).await?;
        let answer = pending.api.post("/api/v1/auth/device/token", json!({"device_code": pending.code}));
        match poll(&mut interval, answer.await) {
            Poll::Done(token) => return Ok(token),
            Poll::Wait(_) => {}
            Poll::Stop(e) if e.code() == "access_denied" => {
                return Err(said("android.signin_refused", "The sign-in was refused", &[]));
            }
            Poll::Stop(e) => return Err(e.into()),
        }
    }
    Err(said("android.signin_expired", "The code expired before it was approved; sign in again", &[]))
}

async fn finish(pending: &Pending, token: &Value) -> Result<Value, Error> {
    let text = |field: &str| token[field].as_str().unwrap_or_default().to_owned();
    let (secret, device) = (text("token"), text("device_id"));
    let authed = pending.api.clone().with_token(&secret);
    let done = match authed.get("/api/v1/me").await {
        Ok(me) => {
            let user = me["handle"].as_str().unwrap_or_default().chars().filter(|c| !c.is_control()).take(64);
            keep(&pending.site, &pending.link, me["id"].as_i64().unwrap_or_default(), user.collect(), &secret)
        }
        Err(e) => Err(e.into()),
    };
    if done.is_err() && !device.is_empty() {
        let gone = format!("/api/v1/me/devices/{device}");
        drop(authed.call(reqwest::Method::DELETE, &gone, None).await);
    }
    done
}

pub(crate) fn keep(site: &str, link: &str, user_id: i64, user: String, secret: &str) -> Result<Value, Error> {
    let same = |r: &Rec| r.origin == site && r.user_id == user_id;
    let id = match super::book::file().accounts.into_iter().find(same) {
        Some(found) => found.id,
        None => random_id()?,
    };
    keep_token(&id, secret)?;
    let host = site.split("://").nth(1).unwrap_or(site);
    edit(|file| {
        if !link.is_empty() {
            file.accounts.iter_mut().for_each(|r| r.links.retain(|l| clean(&l.url) != clean(link)));
        }
        let index = match file.accounts.iter().position(|r| r.id == id) {
            Some(index) => index,
            None => {
                file.accounts.push(Rec { id: id.clone(), origin: site.to_owned(), ..Rec::default() });
                file.accounts.len() - 1
            }
        };
        let rec = &mut file.accounts[index];
        (rec.user, rec.user_id, rec.pending) = (user.clone(), user_id, false);
        if rec.label.is_empty() {
            rec.label = format!("{user}@{host}");
        }
        if !link.is_empty() {
            rec.links.push(Link { url: link.to_owned(), repo_id: None });
        }
        Ok(())
    })?;
    super::using::resolve();
    Ok(json!({"account": id, "user": user, "server": site}))
}
