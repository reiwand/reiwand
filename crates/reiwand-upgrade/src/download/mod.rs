mod blobs;
mod content;
mod pull;

use crate::Vault;
use anyhow::{Context, bail};
use reiwand_account::Api;
use reiwand_carry::{Cost, Export, OwnFile};
use reiwand_sync::{Secrets, Session};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

pub const FILE: &str = "from-server.reiwand";
pub const DONE: &str = "sync-downloaded";
pub const PENDING: &str = "sync-download-pending";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Report {
    pub lists: usize,
    pub favourites: usize,
    pub repositories: usize,
    pub files: usize,
    pub file: Option<PathBuf>,
}

fn field(saved: &Value, name: &str) -> String {
    saved[name].as_str().unwrap_or_default().to_owned()
}

fn clear(dir: &Path) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        fs::remove_file(entry.path())?;
    }
    let _ = fs::remove_dir(dir);
    Ok(())
}

pub fn run(data: &Path, vault: &mut dyn Vault) -> anyhow::Result<Option<Report>> {
    let Ok(bytes) = fs::read(data.join("sync.json")) else { return Ok(None) };
    let saved: Value = serde_json::from_slice(&bytes)?;
    let Some(stored) = vault.take("server")? else { return Ok(None) };
    let Ok(secrets) = serde_json::from_str::<Secrets>(&stored) else { return Ok(None) };
    let Ok(key) = secrets.sync_key() else { return Ok(None) };
    let pin = Some(field(&saved, "pin")).filter(|p| !p.is_empty());
    let api = Api::pinned(&field(&saved, "url"), pin.as_deref())?.with_token(&secrets.token);
    let user = saved["owner"][1].as_i64().unwrap_or_default();
    let epoch = secrets.epoch.or(saved["epoch"].as_i64()).unwrap_or_default();
    let session = Session::new(api.clone(), user, epoch, saved["encrypted"].as_bool().unwrap_or(true), key);
    let ids = session.key.ids();
    let installed: Vec<content::Installed> = reiwand_format::installed(data)
        .unwrap_or_default()
        .into_iter()
        .map(|i| content::Installed { repo_id: i.repo_id, id: i.id, version: i.version })
        .collect();
    let stage = data.join("sync-download");
    fs::create_dir_all(&stage)?;
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    let made = runtime.block_on(async {
        let views = pull::all(&session).await?;
        let who = content::who(&ids, &installed, &views);
        let mut content = content::build(&views, &who, &installed);
        let mut seen = BTreeSet::new();
        let mut own = Vec::new();
        for view in views.values().filter_map(|v| v.get("blob")) {
            let hash = view["hash"].as_str().unwrap_or_default().to_owned();
            if seen.insert(hash.clone()) {
                let (name, path) = blobs::fetch(&api, &ids, view, &stage).await?;
                own.push((OwnFile { id: hash, name }, path));
            }
        }
        let report = Report {
            lists: content.lists.len(),
            favourites: content.favourites.len(),
            repositories: content.repositories.len(),
            files: own.len(),
            file: None,
        };
        content.own = own.iter().map(|(file, _)| file.clone()).collect();
        let created =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let export = Export {
            program: "reiwand".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            created: created.to_string(),
            content,
            own,
            password: None,
            cost: Cost::default(),
        };
        let target = data.join(FILE);
        let staged = data.join(format!("{FILE}.part"));
        reiwand_carry::write(&staged, &export).context("the download could not be written")?;
        fs::rename(&staged, &target)?;
        anyhow::Ok(Report { file: Some(target), ..report })
    });
    clear(&stage)?;
    match made {
        Ok(report) => Ok(Some(report)),
        Err(e) => bail!("{e:#}"),
    }
}
