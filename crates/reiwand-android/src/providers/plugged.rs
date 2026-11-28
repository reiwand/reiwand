use super::kinds::{KEPT, Options, WHEN, describe};
use super::options::checked;
use super::saved;
use crate::{Error, said, store};
use reiwand_plugin::{Bundle, Installed, Isolation, Manifest, Reach, Setting};
use reiwand_providers::Provider;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

pub(super) const PREFIX: &str = "plugin:";
const REFRESH: &str = "1d";

fn root() -> Result<PathBuf, Error> {
    Ok(store::dir()?.join("plugins"))
}

fn unreadable(e: reiwand_plugin::Error) -> Error {
    said("android.plugin_unreadable", "this is not a provider plugin Reiwand can install: %1", &[&e])
}

pub(super) fn manifest(id: &str) -> Result<Manifest, Error> {
    let missing = |_| said("android.plugin_missing", "the provider plugin %1 is not installed", &[&id]);
    Installed::manifest(&root()?, id).map_err(missing)
}

fn sources(id: &str) -> Result<Vec<String>, Error> {
    let kind = format!("{PREFIX}{id}");
    Ok(saved::load()?.into_iter().filter(|e| e.kind == kind).map(|e| e.id).collect())
}

fn prefill(setting: &Setting) -> String {
    match &setting.prefill {
        Value::String(text) => text.clone(),
        value => value.to_string(),
    }
}

fn field(setting: &Setting) -> Value {
    let kind = match setting.kind {
        reiwand_plugin::Kind::Text => "text",
        reiwand_plugin::Kind::Int => "int",
        reiwand_plugin::Kind::Bool => "bool",
        reiwand_plugin::Kind::Choice => "pick",
    };
    json!({"key": setting.key, "label": setting.label, "type": kind, "default": prefill(setting), "required": false,
        "values": setting.values, "min": setting.min.unwrap_or(0), "max": setting.max.unwrap_or(0)})
}

fn secret(manifest: &Manifest) -> Value {
    let secret = manifest.secret.as_ref();
    secret.map_or(Value::Null, |s| json!({"name": s.label, "required": s.required, "hosts": s.hosts}))
}

/// The kinds of source the installed plugins add, described like the phone's own kinds, with the
/// plugin's title and the label of each setting.
pub(super) fn kinds() -> Vec<Value> {
    let Ok(root) = root() else { return Vec::new() };
    let found = Installed::all(&root).into_iter().filter_map(|(_, opened)| opened.ok());
    let described = found.map(|installed| {
        let m = installed.manifest;
        let own = m.settings.iter().map(field);
        let options: Vec<Value> = own.chain([describe(&WHEN, REFRESH), describe(&KEPT, KEPT.2)]).collect();
        json!({"kind": format!("{PREFIX}{}", m.id), "title": m.title, "hosts": m.hosts, "options": options, "secret": secret(&m)})
    });
    described.collect()
}

pub(super) fn secret_of(manifest: &Manifest) -> Option<(String, bool)> {
    manifest.secret.as_ref().map(|s| (s.label.clone(), s.required))
}

pub(super) fn effective(manifest: &Manifest, given: &Options) -> Result<Options, Error> {
    let own = |key: &str| manifest.settings.iter().any(|s| s.key == key);
    if let Some(key) = given.keys().find(|key| !own(key) && **key != WHEN.0 && **key != KEPT.0) {
        return Err(said("android.provider_option_unknown", "%1 has no option %2", &[&manifest.title, key]));
    }
    let mut options = Options::new();
    for setting in &manifest.settings {
        let text = given.get(&setting.key).map_or_else(|| prefill(setting), |v| v.trim().to_owned());
        if setting.value(&reiwand_plugin::parse(setting, &text)).is_err() {
            return Err(said("android.provider_option", "the value of %1 is not valid", &[&setting.label]));
        }
        options.insert(setting.key.clone(), text);
    }
    for (field, fallback) in [(&WHEN, REFRESH), (&KEPT, KEPT.2)] {
        let value = given.get(field.0).map_or(Ok(fallback.to_owned()), |v| checked(field, v))?;
        options.insert(field.0.to_owned(), value);
    }
    Ok(options)
}

pub(super) fn build(
    manifest: &Manifest,
    o: &Options,
    secret: Option<&str>,
) -> Result<Box<dyn Provider>, Error> {
    crate::net::open()?;
    let own = o.iter().filter(|(k, _)| manifest.settings.iter().any(|s| &&s.key == k));
    let own: Options = own.map(|(k, v)| (k.clone(), v.clone())).collect();
    let secret = secret.filter(|s| !s.is_empty());
    let built =
        reiwand_plugin::build(&root()?, &manifest.id, &own, secret, Reach::default(), Isolation::standard());
    built.map_err(|e| {
        said("android.plugin_broken", "the provider plugin %1 does not work: %2", &[&manifest.title, &e])
    })
}

fn info(manifest: &Manifest, sources: Vec<String>, error: Option<String>) -> Value {
    json!({"id": manifest.id, "kind": format!("{PREFIX}{}", manifest.id), "version": manifest.version, "title": manifest.title,
        "license": manifest.license, "hosts": manifest.hosts, "secret": secret(manifest), "sources": sources, "error": error})
}

pub(super) fn listed() -> Result<Value, Error> {
    let mut plugins = Vec::new();
    for (id, opened) in Installed::all(&root()?) {
        let used = sources(&id)?;
        plugins.push(match opened {
            Ok(installed) => info(&installed.manifest, used, None),
            Err(e) => {
                json!({"id": id, "kind": format!("{PREFIX}{id}"), "version": "", "title": id, "license": "",
                "hosts": [], "secret": null, "sources": used, "error": e.to_string()})
            }
        });
    }
    Ok(json!(plugins))
}

fn read(path: &str) -> Result<Bundle, Error> {
    let bundle = Bundle::read(Path::new(path)).map_err(unreadable)?;
    bundle.strip().map_err(unreadable)?;
    Ok(bundle)
}

/// What the consent shows before a plugin is installed: the plugin, what changes against the
/// installed version, and the token that installing must give back.
pub(super) fn inspect(path: &str) -> Result<Value, Error> {
    let bundle = read(path)?;
    let m = &bundle.manifest;
    let before = Installed::manifest(&root()?, &m.id).ok();
    let new = |h: &&String| before.as_ref().is_none_or(|b| !b.hosts.contains(h));
    let added: Vec<&String> = m.hosts.iter().filter(new).collect();
    let changed = before.as_ref().is_some_and(|b| b.secret != m.secret);
    Ok(json!({"plugin": info(m, sources(&m.id)?, None), "added_hosts": added, "secret_changed": changed,
        "step": reiwand_plugin::step(before.as_ref().map(|b| b.version.as_str()), &m.version),
        "installed": before.map(|b| b.version), "token": bundle.token()}))
}

pub(super) fn install(path: &str, token: &str) -> Result<Value, Error> {
    let bundle = read(path)?;
    if token.is_empty() || bundle.token() != token {
        return Err(said(
            "android.plugin_changed",
            "the plugin is not the one that was shown: look at it again",
            &[],
        ));
    }
    let installed = reiwand_plugin::install(&bundle, &root()?).map_err(unreadable)?;
    log::info!("provider plugin {} {} installed", installed.manifest.id, installed.manifest.version);
    Ok(json!(format!("{PREFIX}{}", installed.manifest.id)))
}

pub(super) fn remove(id: &str) -> Result<Value, Error> {
    let _held = saved::held();
    let used = sources(id)?;
    if !used.is_empty() {
        return Err(said(
            "android.plugin_used",
            "%1 is the plugin of %2: remove those sources first",
            &[&id, &used.join(", ")],
        ));
    }
    match reiwand_plugin::remove(&root()?, id) {
        Ok(true) => Ok(Value::Null),
        Ok(false) => Err(said("android.plugin_missing", "the provider plugin %1 is not installed", &[&id])),
        Err(e) => Err(unreadable(e)),
    }
}
