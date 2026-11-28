use crate::error::{Error, Result, failed, invalid};
use reiwand_core::dbus::error::CONFLICT;
use reiwand_core::model::{MODEL, PluginInfo, PluginOffer, PluginSecret, ProviderKind};
use reiwand_plugin::{Bundle, Installed, Manifest};
use std::path::{Path, PathBuf};

mod install;
#[cfg(test)]
mod tests;

pub use reiwand_plugin::build;

pub const PREFIX: &str = "plugin:";

pub fn root() -> PathBuf {
    crate::store::data_home().join("plugins")
}

pub fn kind(root: &Path, id: &str) -> Result<ProviderKind> {
    let manifest = Installed::manifest(root, id).map_err(|e| invalid(format!("{e}; see ListPlugins")))?;
    Ok(crate::schema::plugin(&manifest))
}

pub fn kinds(root: &Path) -> Vec<ProviderKind> {
    let names = std::fs::read_dir(root).into_iter().flatten().flatten();
    let mut names: Vec<String> = names.map(|e| e.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    let manifests =
        names.iter().filter(|n| !n.starts_with('.')).filter_map(|n| Installed::manifest(root, n).ok());
    manifests.map(|m| crate::schema::plugin(&m)).collect()
}

pub fn info(manifest: &Manifest, sources: Vec<String>) -> PluginInfo {
    let secret = manifest.secret.as_ref().map(|s| PluginSecret {
        label: s.label.clone(),
        header: s.header.clone(),
        hosts: s.hosts.clone(),
        required: s.required,
    });
    PluginInfo {
        id: manifest.id.clone(),
        kind: format!("{PREFIX}{}", manifest.id),
        version: manifest.version.clone(),
        title: manifest.title.clone(),
        license: manifest.license.clone(),
        hosts: manifest.hosts.clone(),
        secret,
        sources,
        error: None,
    }
}

pub fn refused(e: reiwand_plugin::Error) -> Error {
    match e {
        reiwand_plugin::Error::Io(message) | reiwand_plugin::Error::Host(message) => failed(message),
        e => invalid(e.to_string()),
    }
}

fn read(path: &Path) -> Result<Bundle> {
    if !path.is_absolute() {
        return Err(invalid("give the absolute path of the plugin folder or zip"));
    }
    let bundle = Bundle::read(path).map_err(refused)?;
    bundle.strip().map_err(refused)?;
    Ok(bundle)
}

pub fn offer(root: &Path, path: &Path, sources: Vec<String>) -> Result<PluginOffer> {
    let bundle = read(path)?;
    let manifest = &bundle.manifest;
    let before = Installed::manifest(root, &manifest.id).ok();
    let new = |h: &&String| before.as_ref().is_none_or(|b| !b.hosts.contains(h));
    Ok(PluginOffer {
        model: MODEL,
        plugin: info(manifest, sources),
        added_hosts: manifest.hosts.iter().filter(new).cloned().collect(),
        secret_changed: before.as_ref().is_some_and(|b| b.secret != manifest.secret),
        step: reiwand_plugin::step(before.as_ref().map(|b| b.version.as_str()), &manifest.version).into(),
        installed: before.map(|b| b.version),
        token: bundle.token(),
    })
}

pub fn install(root: &Path, path: &Path, shown: &str) -> Result<Installed> {
    if shown.is_empty() {
        return Err(invalid(
            "a plugin is installed only after the user agreed to what InspectPlugin shows: pass its token",
        ));
    }
    let bundle = read(path)?;
    if bundle.token() != shown {
        return Err(Error::Named(
            CONFLICT,
            "the plugin is not the one that was shown: its files changed since InspectPlugin; show it again and ask"
                .into(),
        ));
    }
    reiwand_plugin::install(&bundle, root).map_err(refused)
}
