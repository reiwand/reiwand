use crate::Error;
use crate::said;
use crate::state::{read, update};
use reiwand_sync::{same_origin, valid_pin};
use serde_json::{Value, json};

pub(crate) async fn add(url: &str, fingerprint: Option<&str>) -> Result<Value, Error> {
    let url = crate::address::address(url)?;
    if read(|s| s.device.is_some()) {
        return Err(said(
            "android.phone_linked_server",
            "this phone is linked to a server; remove it first",
            &[],
        ));
    }
    let pin = fingerprint
        .map(|f| f.chars().filter(char::is_ascii_hexdigit).collect::<String>().to_ascii_lowercase());
    if pin.as_deref().is_some_and(|p| !valid_pin(p)) {
        return Err(said("android.fingerprint_digits", "a certificate fingerprint has 64 hex digits", &[]));
    }
    let found = match crate::net::api(&url, pin.as_deref())?.get("/.well-known/reiwand-server").await {
        Err(reiwand_sync::Error::Untrusted) if pin.is_none() => {
            let seen = crate::net::peek(&url).await?;
            return Err(said(
                "android.server_own_certificate",
                "this server uses its own certificate with the fingerprint %1; add it only if the server shows the same one",
                &[&seen],
            ));
        }
        Err(reiwand_sync::Error::Untrusted) => {
            return Err(said(
                "android.server_other_certificate",
                "the server's certificate is not the one with the fingerprint given",
                &[],
            ));
        }
        Err(reiwand_sync::Error::Network(_)) => {
            let host = reqwest::Url::parse(&url).ok().and_then(|u| u.host_str().map(str::to_owned));
            return Err(said(
                "android.server_unreachable",
                "cannot reach %1: check the address and the connection",
                &[&host.unwrap_or(url)],
            ));
        }
        found => found?,
    };
    if !found["api"].as_array().is_some_and(|api| api.contains(&json!("v1"))) {
        return Err(said("android.address_does_serve", "this address does not serve a Reiwand server", &[]));
    }
    let name: String =
        found["name"].as_str().unwrap_or_default().chars().filter(|c| !c.is_control()).take(128).collect();
    let repo_id = Some(announced(&found, &url));
    let repo_url = repo_id
        .as_ref()
        .filter(|id| !id.is_empty())
        .and(found["repository"]["url"].as_str())
        .map(String::from);
    update(|s| {
        (s.url, s.name, s.pin, s.repo_id, s.repo_url) = (url, name.clone(), pin, repo_id, repo_url);
        s.repo_failed = None;
    })?;
    Ok(json!({"name": name, "registration": found["registration"]}))
}

pub(crate) fn announced(found: &Value, url: &str) -> String {
    let repo = found["repository"]["url"].as_str().filter(|r| same_origin(r, url));
    let id = found["repository"]["repo_id"].as_str().filter(|_| repo.is_some()).unwrap_or_default();
    id.chars().take(128).filter(|c| !c.is_control()).collect()
}

pub(crate) async fn repository() -> Result<Option<String>, Error> {
    if let Some(known) = read(|s| s.repo_id.clone()) {
        return Ok(Some(known).filter(|r| !r.is_empty()));
    }
    let (url, pin) = read(|s| (s.url.clone(), s.pin.clone()));
    let found = crate::net::api(&url, pin.as_deref())?.get("/.well-known/reiwand-server").await?;
    let id = announced(&found, &url);
    update(|s| s.repo_id = Some(id.clone()))?;
    Ok(Some(id).filter(|r| !r.is_empty()))
}
