use super::gather::gather;
use crate::library::now;
use crate::providers::carried;
use crate::{Error, said};
use reiwand_carry::{Content, Cost, Export, Item, Secrets};
use serde_json::{Value, json};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

fn without_own(content: &mut Content) {
    let kept = |item: &Item| item.repo_id.is_some();
    content.own.clear();
    content.lists.iter_mut().for_each(|l| l.items.retain(kept));
    content.favourites.retain(kept);
    content.ratings.retain(|r| kept(&r.item));
    content.tags.retain(|t| kept(&t.item));
}

fn refused(what: &str) -> Error {
    said("android.carry_file", "The file cannot be written: %1", &[&what])
}

pub(super) fn export(path: &str, own: bool, secrets: bool, password: &str) -> Result<Value, Error> {
    if secrets && password.is_empty() {
        return Err(said(
            "android.carry_secrets_password",
            "Passwords and keys are only exported with a password",
            &[],
        ));
    }
    let path = PathBuf::from(path);
    if !path.is_absolute() || path.file_name().is_none() || path.exists() {
        return Err(refused("the place is not free"));
    }
    let gathered = gather()?;
    let (mut content, mut files) = (gathered.content, gathered.own);
    if !own {
        without_own(&mut content);
        files.clear();
    }
    if secrets {
        let provider_keys = carried::secrets().into_iter().collect();
        content.secrets = Some(Secrets { sessions: Vec::new(), provider_keys });
    }
    let sections = content.sections();
    let (owned, encrypted) = (files.len(), !password.is_empty());
    let export = Export {
        program: "reiwand".to_owned(),
        version: reiwand_core::VERSION.to_owned(),
        created: now().to_string(),
        content,
        own: files,
        password: Some(password.to_owned()).filter(|p| !p.is_empty()),
        cost: Cost::default(),
    };
    let partial = PathBuf::from(format!("{}.partial", path.display()));
    OpenOptions::new().write(true).create_new(true).open(&partial).map_err(|e| refused(&e.to_string()))?;
    let done = reiwand_carry::write(&partial, &export)
        .map_err(|e| refused(&e.to_string()))
        .and_then(|()| fs::rename(&partial, &path).map_err(|e| refused(&e.to_string())));
    if let Err(e) = done {
        drop(fs::remove_file(Path::new(&partial)));
        return Err(e);
    }
    Ok(json!({"path": path, "sections": sections, "own": owned, "encrypted": encrypted}))
}
