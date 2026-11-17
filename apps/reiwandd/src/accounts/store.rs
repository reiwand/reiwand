use super::Book;
use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::{Error, Result, failed};
use crate::store::private_dir;
use reiwand_core::model::{ServerAccount, ServerAccountRepo};
use std::io::Write;
use std::path::PathBuf;

pub use reiwand_account::{File, Link, Rec, clean};

pub fn model(rec: &Rec) -> ServerAccount {
    let repos =
        rec.links.iter().map(|l| ServerAccountRepo { url: l.url.clone(), repo_id: l.repo_id.clone() });
    ServerAccount {
        id: rec.id.clone(),
        server: rec.origin.clone(),
        user: rec.user.clone(),
        label: rec.label.clone(),
        pending: rec.pending,
        repos: repos.collect(),
    }
}

impl Daemon {
    fn accounts_path(&self) -> PathBuf {
        self.store.data.join("accounts.json")
    }

    pub(super) fn accounts_file(&self) -> File {
        let Ok(bytes) = std::fs::read(self.accounts_path()) else { return File::default() };
        serde_json::from_slice(&bytes).unwrap_or_else(|e| {
            tracing::warn!("the accounts cannot be read; none is used until one is signed in again: {e}");
            File::default()
        })
    }

    pub fn account_list(&self) -> Vec<Rec> {
        self.accounts_file().accounts
    }

    pub(super) fn edit_accounts<T>(&self, change: impl FnOnce(&mut File) -> Result<T>) -> Result<T> {
        let _gate = lock(&self.accounts.gate);
        let mut file = self.accounts_file();
        let done = change(&mut file)?;
        let path = self.accounts_path();
        let dir = path.parent().unwrap_or(std::path::Path::new("."));
        private_dir(dir)?;
        let mut temporary = tempfile::Builder::new().prefix(".tmp-").tempfile_in(dir)?;
        temporary.write_all(&serde_json::to_vec(&file).map_err(|e| failed(e.to_string()))?)?;
        temporary.persist(&path).map_err(|e| Error::from(e.error))?;
        self.changes.changed("accounts", Vec::new());
        Ok(done)
    }
}

impl Book {
    pub(super) fn cached(&self, id: &str) -> Option<zeroize::Zeroizing<String>> {
        lock(&self.tokens).get(id).cloned()
    }

    pub(super) fn keep(&self, id: &str, token: Option<zeroize::Zeroizing<String>>) {
        let mut tokens = lock(&self.tokens);
        match token {
            Some(token) => tokens.insert(id.to_owned(), token),
            None => tokens.remove(id),
        };
    }
}
