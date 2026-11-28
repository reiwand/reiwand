use crate::config::Config;
use crate::db::{Db, now};
use crate::repo::{Repo, valid_name};
use anyhow::{Context, ensure};
use rusqlite::{Connection, params};
use std::fs::DirBuilder;
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, PoisonError, RwLock};

pub struct Repos {
    db: Db,
    data: PathBuf,
    list: RwLock<Vec<Arc<Repo>>>,
}

fn read(conn: &Connection, data: &Path) -> rusqlite::Result<Vec<Repo>> {
    let mut statement = conn.prepare("SELECT name, folder, online_key FROM repos ORDER BY n")?;
    let found = statement.query_map([], |r| {
        Ok(Repo::at(data, &r.get::<_, String>(0)?, &r.get::<_, String>(1)?, &r.get::<_, String>(2)?))
    })?;
    found.collect()
}

impl Repos {
    pub fn new(db: Db, data: PathBuf) -> Repos {
        let repos = Repos { db, data, list: RwLock::default() };
        repos.reload();
        repos
    }

    pub fn reload(&self) {
        let found = self.db.read(|c| read(c, &self.data)).unwrap_or_default();
        let mut list = self.list.write().unwrap_or_else(PoisonError::into_inner);
        let kept = std::mem::take(&mut *list);
        *list = found
            .into_iter()
            .map(|r| kept.iter().find(|k| k.name == r.name).cloned().unwrap_or_else(|| Arc::new(r)))
            .collect();
    }

    pub fn all(&self) -> Vec<Arc<Repo>> {
        if self.list.read().unwrap_or_else(PoisonError::into_inner).is_empty() {
            self.reload();
        }
        self.list.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub fn ready(&self) -> Vec<Arc<Repo>> {
        self.all().into_iter().filter(|r| r.ready()).collect()
    }

    pub fn get(&self, name: &str) -> Option<Arc<Repo>> {
        let find = || self.all().into_iter().find(|r| r.name == name);
        find()
            .or_else(|| {
                self.reload();
                find()
            })
            .or_else(|| {
                (name == "main" && self.all().is_empty()).then(|| Arc::new(Repo::named(&self.data, "main")))
            })
    }

    pub fn first(&self) -> Arc<Repo> {
        let all = self.all();
        let ready = all.iter().find(|r| r.ready()).or(all.first());
        ready.cloned().unwrap_or_else(|| Arc::new(Repo::named(&self.data, "main")))
    }
}

fn open(cfg: &Config) -> anyhow::Result<Db> {
    DirBuilder::new().recursive(true).mode(0o700).create(cfg.data.join("db"))?;
    Db::open(&cfg.data.join("db/reiwand.sqlite"))
}

pub fn load(cfg: &Config) -> anyhow::Result<Vec<Repo>> {
    Ok(open(cfg)?.read(|c| read(c, &cfg.data))?)
}

pub fn prepare(cfg: &Config, name: &str) -> anyhow::Result<Repo> {
    ensure!(valid_name(name), "repository names are 1 to 32 lowercase letters, digits or hyphens");
    if let Some(found) = load(cfg)?.into_iter().find(|r| r.name == name) {
        return Ok(found);
    }
    Ok(Repo::named(&cfg.data, name))
}

pub fn record(cfg: &Config, repo: &Repo) -> anyhow::Result<()> {
    let id = reiwand_format::status(&repo.dir).map(|s| s.repo_id).context("the repository has no root")?;
    let (folder, key) = (repo.folder(), repo.key_name());
    let sql = "INSERT INTO repos (name, folder, online_key, repo_id, dirty, signed_at, created_at, private, publishing) \
               VALUES (?1, ?2, ?3, ?4, 1, 0, ?5, 0, 'open') \
               ON CONFLICT (name) DO UPDATE SET repo_id = ?4, dirty = 1";
    open(cfg)?.read(|c| c.execute(sql, params![repo.name, folder, key, id, now()]))?;
    Ok(())
}
