use super::{PREFIX, info, root};
use crate::daemon::Daemon;
use crate::error::{Result, invalid, not_found};
use crate::items::reply;
use reiwand_core::model::{MODEL, PluginInfo, PluginsReply};
use reiwand_plugin::Installed;
use std::path::PathBuf;
use std::sync::Arc;

impl Daemon {
    fn plugin_sources(&self, id: &str) -> Vec<String> {
        let kind = format!("{PREFIX}{id}");
        let providers = self.state().config.providers.clone();
        let uses =
            |value: &toml::Value| value.get("kind").and_then(toml::Value::as_str) == Some(kind.as_str());
        providers.into_iter().filter(|(_, value)| uses(value)).map(|(source, _)| source).collect()
    }

    pub async fn inspect_plugin(self: &Arc<Self>, path: String) -> Result<String> {
        self.run(move |d| {
            let path = PathBuf::from(path);
            let id = reiwand_plugin::Bundle::read(&path).map(|b| b.manifest.id).unwrap_or_default();
            reply(super::offer(&root(), &path, d.plugin_sources(&id))?)
        })
        .await
    }

    pub async fn install_plugin(self: &Arc<Self>, path: String, token: String) -> Result<String> {
        self.run(move |d| {
            let installed = super::install(&root(), &PathBuf::from(path), &token)?;
            let manifest = &installed.manifest;
            tracing::info!(plugin = manifest.id, version = manifest.version, "provider plugin installed");
            d.changes.changed("providers", d.plugin_sources(&manifest.id));
            Ok::<_, crate::error::Error>(format!("{PREFIX}{}", manifest.id))
        })
        .await
    }

    pub async fn list_plugins(self: &Arc<Self>) -> Result<String> {
        let serial = self.changes.serial();
        let plugins = self
            .run(move |d| {
                let listed = Installed::all(&root()).into_iter().map(|(id, opened)| match opened {
                    Ok(installed) => info(&installed.manifest, d.plugin_sources(&id)),
                    Err(e) => broken(id.clone(), d.plugin_sources(&id), e.to_string()),
                });
                Ok::<_, crate::error::Error>(listed.collect())
            })
            .await?;
        reply(PluginsReply { model: MODEL, serial, plugins })
    }

    pub async fn remove_plugin(self: &Arc<Self>, id: String) -> Result<()> {
        self.run(move |d| {
            let sources = d.plugin_sources(&id);
            if !sources.is_empty() {
                return Err(invalid(format!(
                    "the provider plugin {id} is the provider of {}: remove them first",
                    sources.join(", ")
                )));
            }
            if !reiwand_plugin::remove(&root(), &id).map_err(super::refused)? {
                return Err(not_found(format!("no provider plugin {id:?} is installed")));
            }
            tracing::info!(plugin = id, "provider plugin removed");
            d.changes.changed("providers", Vec::new());
            Ok(())
        })
        .await
    }
}

fn broken(id: String, sources: Vec<String>, error: String) -> PluginInfo {
    let kind = format!("{PREFIX}{id}");
    let (version, title, license, hosts) = (String::new(), String::new(), String::new(), Vec::new());
    PluginInfo { id, kind, version, title, license, hosts, secret: None, sources, error: Some(error) }
}
