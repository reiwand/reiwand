use crate::{Error, net, said};
use reiwand_account::origin;
use reiwand_format::Fetch;
use serde_json::{Value, json};

pub(crate) fn https(url: &str) -> Result<(), Error> {
    if super::shared(url) {
        return Ok(());
    }
    Err(said(
        "android.repo_https",
        "A repository address starts with https://; http:// works only for an address on this same phone (localhost or 127.0.0.1)",
        &[],
    ))
}

pub(crate) fn listed(url: &str, trust: Option<&str>) -> Result<Value, Error> {
    https(url)?;
    net::reach(url)?;
    let site = origin(url)
        .ok_or_else(|| said("android.repo_https", "A repository address starts with https://", &[]))?;
    let extra = Some(site.as_str()).zip(trust.map(str::trim).filter(|t| !t.is_empty()));
    let fetch = net::fetch_with(extra)?;
    let body = fetch.get(&format!("{site}/"), ".well-known/reiwand-server", 256 << 10)?;
    let body = body.ok_or_else(|| {
        said("android.address_does_serve", "This address does not serve a Reiwand server", &[])
    })?;
    let found: Value = serde_json::from_slice(&body)?;
    let listed = found["repositories"].as_array().into_iter().flatten().filter_map(|r| {
        let url = r["url"].as_str().filter(|u| super::shared(u))?;
        Some(json!({"name": r["name"].as_str()?, "url": url, "private": r["private"].as_bool().unwrap_or(false)}))
    });
    let name: String =
        found["name"].as_str().unwrap_or(&site).chars().filter(|c| !c.is_control()).take(128).collect();
    Ok(json!({"name": name, "repos": listed.take(200).collect::<Vec<_>>()}))
}
