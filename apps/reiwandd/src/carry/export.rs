use crate::daemon::Daemon;
use crate::error::{Error, Result, failed, invalid};
use crate::jobs::{Ctl, now};
use reiwand_carry::{Content, Cost, Export};
use reiwand_core::dbus::error::EXISTS;
use reiwand_core::model::JobKind;
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs::{self, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Options {
    pub own: bool,
    pub secrets: bool,
    pub password: String,
}

impl Options {
    pub fn parse(json: &str) -> Result<Options> {
        if json.trim().is_empty() {
            return Ok(Options::default());
        }
        serde_json::from_str(json).map_err(|e| invalid(format!("the options are not valid: {e}")))
    }
}

pub fn without_own(content: &mut Content) {
    let kept = |item: &reiwand_carry::Item| item.repo_id.is_some();
    content.own.clear();
    content.lists.iter_mut().for_each(|l| l.items.retain(kept));
    content.favourites.retain(kept);
    content.ratings.retain(|r| kept(&r.item));
    content.tags.retain(|t| kept(&t.item));
}

impl Daemon {
    pub fn export_carry(self: &Arc<Self>, path: &str, options: Options) -> Result<u64> {
        if options.secrets && options.password.is_empty() {
            return Err(invalid("passwords and keys are only exported with a password"));
        }
        let path = PathBuf::from(path);
        if !path.is_absolute() || path.file_name().is_none() {
            return Err(invalid(format!("{} is not an absolute file path", path.display())));
        }
        if path.exists() {
            return Err(Error::Named(EXISTS, format!("{} already exists", path.display())));
        }
        if !path.parent().is_some_and(Path::is_dir) {
            return Err(invalid("the folder of the file does not exist"));
        }
        let target = path.to_string_lossy().into_owned();
        Ok(self.start_job(JobKind::CarryExport, &target, move |d, ctl| d.write_export(&path, &options, ctl)))
    }

    fn write_export(&self, path: &Path, options: &Options, ctl: &Ctl) -> Result<Value> {
        ctl.progress(0, 3);
        let gathered = self.carry_state()?;
        let (mut content, mut own) = (gathered.content, gathered.own);
        if !options.own {
            without_own(&mut content);
            own.clear();
        }
        if options.secrets {
            content.secrets = Some(self.carry_secrets()?);
        }
        ctl.check()?;
        ctl.progress(1, 3);
        let password = Some(options.password.clone()).filter(|p| !p.is_empty());
        let sections = content.sections();
        let owned = own.len();
        let encrypted = password.is_some();
        let (program, version) = ("reiwand".to_owned(), reiwand_core::VERSION.to_owned());
        let export = Export {
            program,
            version,
            created: now().to_string(),
            content,
            own,
            password,
            cost: Cost::default(),
        };
        let partial = PathBuf::from(format!("{}.partial", path.display()));
        let made = OpenOptions::new().write(true).create_new(true).mode(0o600).open(&partial);
        made.map_err(|e| failed(format!("cannot create the file: {e}")))?;
        let done = reiwand_carry::write(&partial, &export)
            .map_err(|e| failed(e.to_string()))
            .and_then(|()| fs::rename(&partial, path).map_err(|e| failed(e.to_string())));
        if let Err(e) = done {
            drop(fs::remove_file(&partial));
            return Err(e);
        }
        ctl.progress(3, 3);
        tracing::info!(sections = sections.len(), "configuration exported");
        Ok(json!({"path": path, "sections": sections, "own": owned, "encrypted": encrypted}))
    }
}
