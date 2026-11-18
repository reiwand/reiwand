use super::gather::gather;
use super::libapply;
use super::local::local_of;
use super::report::Report;
use super::stage::open;
use crate::providers::carried;
use crate::{Error, repos, settings, store};
use reiwand_carry::{Change, Opened, OwnFile, Pick, Provider, plan};
use serde_json::{Value, json};
use std::io::{Seek, SeekFrom};

fn text(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Bool(_) | Value::Number(_) => Some(value.to_string()),
        _ => None,
    }
}

fn options_of(provider: &Provider) -> std::collections::BTreeMap<String, String> {
    let given = provider.config.as_object().into_iter().flatten();
    given.filter_map(|(k, v)| Some((k.clone(), text(v)?))).collect()
}

fn own_file(opened: &mut Opened, own: &OwnFile) -> Result<(), Error> {
    let mut staged = tempfile::tempfile_in(store::dir()?)?;
    opened.extract_own(&own.id, &mut staged).map_err(|e| Error::Unreadable(e.to_string()))?;
    staged.seek(SeekFrom::Start(0))?;
    let name = std::path::Path::new(&own.name)
        .file_name()
        .map_or_else(|| own.id.clone(), |n| n.to_string_lossy().into_owned());
    store::store(&mut staged, &name).map(drop)
}

fn providers(changes: &[Change], report: &mut Report) {
    for change in changes {
        match change {
            Change::AddProvider(p) => match carried::import(&p.name, &p.kind, &options_of(p)) {
                Ok(_) => report.add("providers"),
                Err(e) => report.fail("providers", &p.name, &e),
            },
            Change::AddSecrets(secrets) => {
                for (id, key) in &secrets.provider_keys {
                    match carried::keep_secret(id, key) {
                        Ok(()) => report.add("secrets"),
                        Err(e) => report.fail("secrets", id, &e),
                    }
                }
                if !secrets.sessions.is_empty() {
                    report.note("android.carry_sessions");
                }
            }
            _ => {}
        }
    }
}

pub(super) fn import(path: &str, password: &str, pick: &Pick) -> Result<Value, Error> {
    let mut opened = open(path, password)?;
    let local = local_of(&gather()?.content, carried::keys_held()?);
    let changes = plan(&opened.content, &local, pick).changes;
    let mut report = Report::default();
    for change in &changes {
        if let Change::AddRepository(repo) = change {
            match repos::add(&repo.address, None) {
                Ok(_) => report.add("repositories"),
                Err(e) => report.fail("repositories", &repo.address, &e),
            }
        }
    }
    providers(&changes, &mut report);
    for change in &changes {
        if let Change::Install(i) = change {
            match repos::install(&i.repo_id, &i.id) {
                Ok(_) => report.add("installed"),
                Err(e) => report.fail("installed", &i.id, &e),
            }
        }
    }
    for change in &changes {
        if let Change::AddOwn(own) = change {
            match own_file(&mut opened, own) {
                Ok(()) => report.add("own"),
                Err(e) => report.fail("own", &own.name, &e),
            }
        }
    }
    libapply::apply(&gather()?.uids, &changes, &mut report)?;
    for change in &changes {
        if let Change::SetSetting { key, value } = change {
            match text(value).map(|t| settings::setting(key, Some(&t))) {
                Some(Ok(_)) => report.add("settings"),
                Some(Err(e)) => report.fail("settings", key, &e),
                None => report.pend("settings", key, "the value is not plain"),
            }
        }
    }
    store::changed();
    Ok(json!(report.value()))
}
