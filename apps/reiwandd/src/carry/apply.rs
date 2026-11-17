use super::choices::Choices;
use super::local::local_of;
use super::report::Report;
use super::stage::open;
use crate::daemon::Daemon;
use crate::error::{Error, Result, invalid};
use crate::jobs::Ctl;
use reiwand_carry::{Change, Provider, plan};
use reiwand_core::dbus::error::EXISTS;
use reiwand_core::model::{JobKind, JobState};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use zeroize::Zeroizing;

fn text(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Bool(_) | Value::Number(_) => Some(value.to_string()),
        _ => None,
    }
}

fn options_of(provider: &Provider) -> HashMap<String, String> {
    let given = provider.config.as_object().into_iter().flatten();
    given.filter_map(|(k, v)| Some((k.clone(), text(v)?))).collect()
}

impl Daemon {
    pub fn import_carry(self: &Arc<Self>, path: &str, password: &str, choices: &str) -> Result<u64> {
        let choices = Choices::parse(choices).map_err(invalid)?;
        open(path, password)?;
        let (path, password) = (path.to_owned(), password.to_owned());
        let target = path.clone();
        Ok(self.start_job(JobKind::CarryImport, &target, move |d, ctl| {
            d.apply_import(&path, &password, &choices, ctl)
        }))
    }

    fn settle(&self, ids: &[u64], section: &'static str, names: &[String], report: &mut Report) {
        self.jobs.wait(ids);
        for (id, name) in ids.iter().zip(names) {
            match self.jobs.get(*id) {
                Ok(job) if job.state == JobState::Done => report.add(section),
                Ok(job) => {
                    let reason = job.error.map_or_else(|| "it did not finish".to_owned(), |e| e.message);
                    report.pend(section, name, &reason);
                }
                Err(e) => report.fail(section, name, &e),
            }
        }
    }

    fn apply_import(
        self: &Arc<Self>,
        path: &str,
        password: &str,
        choices: &Choices,
        ctl: &Ctl,
    ) -> Result<Value> {
        let mut opened = open(path, password)?;
        let local = local_of(&self.carry_state()?.content, self.carry_keys_held());
        let changes = plan(&opened.content, &local, &choices.pick()).changes;
        let (mut report, total) = (Report::default(), 6);
        ctl.progress(0, total);
        let (mut ids, mut names) = (Vec::new(), Vec::new());
        for change in &changes {
            if let Change::AddRepository(repo) = change {
                match self.add_repo(&repo.address, "") {
                    Ok(id) => {
                        ids.push(id);
                        names.push(repo.address.clone());
                    }
                    Err(Error::Named(EXISTS, _)) => (),
                    Err(e) => report.fail("repositories", &repo.address, &e),
                }
            }
        }
        self.settle(&ids, "repositories", &names, &mut report);
        ctl.check()?;
        ctl.progress(1, total);
        for change in &changes {
            if let Change::AddProvider(provider) = change {
                let options = options_of(provider);
                let add = self.add_provider(
                    provider.name.clone(),
                    provider.kind.clone(),
                    options,
                    Zeroizing::default(),
                );
                match self.jobs.block_on(add) {
                    Ok(_) => report.add("providers"),
                    Err(e) => report.fail("providers", &provider.name, &e),
                }
            }
            if let Change::AddSecrets(secrets) = change {
                for (id, key) in &secrets.provider_keys {
                    match self.jobs.block_on(self.set_secret(id.clone(), Zeroizing::new(key.clone()))) {
                        Ok(()) => report.add("secrets"),
                        Err(e) => report.fail("secrets", id, &e),
                    }
                }
                if !secrets.sessions.is_empty() {
                    report.note("account sessions are not imported yet");
                }
            }
        }
        ctl.check()?;
        ctl.progress(2, total);
        let (mut ids, mut names) = (Vec::new(), Vec::new());
        for change in &changes {
            if let Change::Install(i) = change {
                let spec = format!("{}/{}", i.repo_id, i.id);
                match self.install(&spec, false, false) {
                    Ok(id) => {
                        ids.push(id);
                        names.push(i.id.clone());
                    }
                    Err(e) => report.fail("installed", &i.id, &e),
                }
            }
        }
        self.settle(&ids, "installed", &names, &mut report);
        ctl.check()?;
        ctl.progress(3, total);
        for change in &changes {
            if let Change::AddOwn(own) = change {
                match self.carry_own(&mut opened, own) {
                    Ok(_) => report.add("own"),
                    Err(e) => report.fail("own", &own.name, &e),
                }
            }
        }
        ctl.check()?;
        ctl.progress(4, total);
        self.carry_library(&changes, &mut report)?;
        for change in &changes {
            if let Change::SetSetting { key, value } = change {
                match text(value).map(|t| self.set_option(key, &t)) {
                    Some(Ok(())) => report.add("settings"),
                    Some(Err(e)) => report.pend("settings", key, &e.to_string()),
                    None => report.pend("settings", key, "the value is not plain"),
                }
            }
        }
        self.library_changed(Vec::new());
        self.changes.changed("playlists", Vec::new());
        if changes.iter().any(|c| matches!(c, Change::AddOwn(_))) {
            self.analyze_job();
        }
        ctl.progress(total, total);
        tracing::info!("configuration imported");
        Ok(report.value())
    }
}
