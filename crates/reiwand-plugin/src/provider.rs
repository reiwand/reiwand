use crate::{Files, Installed, Isolation, Net, Plugin};
use reiwand_providers::{Cache, Error, Item, Media, Provider, Result, Secret};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, HashSet};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

const PAGES: usize = 5;
const ITEMS: usize = 500;
const COMMON: [&str; 2] = ["refresh", "cache_mb"];

/// Where the plugin's requests may go beyond the public web: trusted certificates and fixed addresses
/// (a local test server).
#[derive(Clone, Default)]
pub struct Reach {
    pub certificates: Vec<reqwest::Certificate>,
    pub addresses: Vec<(String, SocketAddr)>,
}

impl Reach {
    fn blocking(&self, builder: reqwest::blocking::ClientBuilder) -> reqwest::blocking::ClientBuilder {
        let builder = match self.certificates.is_empty() {
            true => builder,
            false => builder.tls_certs_only(self.certificates.clone()),
        };
        self.addresses.iter().fold(builder, |b, (host, at)| b.resolve(host, *at))
    }

    fn client(&self, builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        let builder = match self.certificates.is_empty() {
            true => builder,
            false => builder.tls_certs_only(self.certificates.clone()),
        };
        self.addresses.iter().fold(builder, |b, (host, at)| b.resolve(host, *at))
    }
}

struct PluginProvider {
    plugin: Arc<Plugin>,
    settings: Map<String, Value>,
    secret: Option<Secret>,
    reach: Reach,
    isolation: Isolation,
    files: Files,
}

fn fault(e: crate::Error) -> Error {
    match e {
        crate::Error::Setting(message) => Error::Setting(message),
        crate::Error::Refused(m) if m.contains(reiwand_core::net::REFUSED) => Error::Private,
        e => Error::Plugin(e.to_string()),
    }
}

/// A source that lists through the installed plugin `id` with the source's options (its own settings
/// and the ones every source has) and the key the user gave it.
pub fn build(
    root: &Path,
    id: &str,
    options: &BTreeMap<String, String>,
    secret: Option<&str>,
    reach: Reach,
    isolation: Isolation,
) -> Result<Box<dyn Provider>> {
    let plugin = Plugin::new(Installed::find(root, id).map_err(fault)?);
    let given: BTreeMap<String, String> = options
        .iter()
        .filter(|(k, _)| !COMMON.contains(&k.as_str()))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let settings = plugin.settings(&given).map_err(fault)?;
    let files = Files::with(&plugin.installed.manifest, secret, |b| reach.client(b)).map_err(fault)?;
    let (plugin, secret) = (Arc::new(plugin), secret.map(Secret::from));
    Ok(Box::new(PluginProvider { plugin, settings, secret, reach, isolation, files }))
}

fn pages(
    plugin: &Plugin,
    settings: Map<String, Value>,
    secret: Option<&str>,
    reach: &Reach,
    isolation: &Isolation,
) -> Result<Vec<Item>> {
    let net = Net::with(&plugin.installed.manifest, secret, |b| reach.blocking(b)).map_err(fault)?;
    let (net, mut items, mut seen, mut page) = (Arc::new(net), Vec::new(), HashSet::new(), None);
    for _ in 0..PAGES {
        let found = plugin.list(net.clone(), settings.clone(), page.take(), isolation).map_err(fault)?;
        for item in found.items.into_iter().filter(|i| seen.insert(i.id.clone())).take(ITEMS - items.len()) {
            let media = if item.media == crate::Media::Video { Media::Video } else { Media::Image };
            let (id, title, url, attribution, license) =
                (item.id, item.title, item.url, item.author, item.license);
            items.push(Item { id, title, url, attribution, license, media });
        }
        match found.next {
            Some(next) if items.len() < ITEMS => page = Some(next),
            _ => break,
        }
    }
    Ok(items)
}

#[async_trait::async_trait]
impl Provider for PluginProvider {
    async fn list(&self) -> Result<Vec<Item>> {
        let (plugin, settings, reach) = (self.plugin.clone(), self.settings.clone(), self.reach.clone());
        let (secret, isolation) = (self.secret.clone(), self.isolation.clone());
        let listed = tokio::task::spawn_blocking(move || {
            pages(&plugin, settings, secret.as_ref().map(|s| s.0.as_str()), &reach, &isolation)
        });
        listed.await.map_err(|e| Error::Plugin(e.to_string()))?
    }

    async fn fetch(&self, item: &Item, cache: &Cache) -> Result<PathBuf> {
        cache.download(item, self.files.get(&item.url).map_err(fault)?).await
    }
}
