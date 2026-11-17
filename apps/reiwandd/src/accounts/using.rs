use super::store::{Link, Rec, clean};
use crate::daemon::Daemon;
use crate::error::{Result, invalid, not_found};
use crate::jobs::now;
use crate::repos::pins::origin;

impl Daemon {
    fn link_of(&self, repo: &str) -> Result<Link> {
        if repo.contains("://") {
            return Ok(Link { url: clean(repo), repo_id: None });
        }
        let id = self.repo_id(repo)?;
        let known = reiwand_format::repos(&self.store.data, now())?;
        let url = known.iter().find(|r| r.repo_id == id).map(|r| clean(&r.url)).unwrap_or_default();
        Ok(Link { url, repo_id: Some(id) })
    }

    fn link_origin(&self, link: &Link) -> Option<String> {
        if let Some(id) = &link.repo_id {
            let hosts = self.bases(id);
            return hosts.iter().chain([&link.url]).find_map(|base| origin(base));
        }
        origin(&link.url)
    }

    pub fn use_account(&self, repo: &str, account: &str) -> Result<()> {
        let link = self.link_of(repo)?;
        let site = self.link_origin(&link).unwrap_or_default();
        self.edit_accounts(|file| {
            let Some(index) = file.accounts.iter().position(|r| r.id == account) else {
                return Err(not_found(format!("account {account} not found")));
            };
            if file.accounts[index].origin != site {
                return Err(invalid("that account belongs to another server than the repository"));
            }
            let id = link.repo_id.clone().unwrap_or_default();
            file.accounts.iter_mut().for_each(|r| r.links.retain(|l| !same(l, &link, &id)));
            file.accounts[index].links.push(link.clone());
            Ok(())
        })?;
        self.changes.changed("repos", Vec::new());
        Ok(())
    }

    pub fn unlink_account(&self, repo: &str) -> Result<()> {
        let link = self.link_of(repo)?;
        let id = link.repo_id.clone().unwrap_or_default();
        let removed = self.edit_accounts(|file| {
            let before: usize = file.accounts.iter().map(|r| r.links.len()).sum();
            file.accounts.iter_mut().for_each(|r| r.links.retain(|l| !same(l, &link, &id)));
            Ok(before - file.accounts.iter().map(|r| r.links.len()).sum::<usize>())
        })?;
        match removed {
            0 => Err(not_found("no account is linked to that repository")),
            _ => {
                self.changes.changed("repos", Vec::new());
                Ok(())
            }
        }
    }

    pub fn sign_out_account(&self, account: &str) -> Result<()> {
        self.edit_accounts(|file| {
            let Some(rec) = file.accounts.iter_mut().find(|r| r.id == account) else {
                return Err(not_found(format!("account {account} not found")));
            };
            rec.pending = true;
            Ok(())
        })?;
        if let Err(e) = self.drop_token(account) {
            tracing::warn!("the access of a signed-out account could not be deleted: {}", e.parts().1);
        }
        self.changes.changed("repos", Vec::new());
        Ok(())
    }

    pub fn remove_account(&self, account: &str) -> Result<()> {
        let gone: Rec = self.edit_accounts(|file| {
            let index = file.accounts.iter().position(|r| r.id == account);
            index
                .map(|i| file.accounts.remove(i))
                .ok_or_else(|| not_found(format!("account {account} not found")))
        })?;
        if let Err(e) = self.drop_token(&gone.id) {
            tracing::warn!("the access of a removed account could not be deleted: {}", e.parts().1);
        }
        self.changes.changed("repos", Vec::new());
        Ok(())
    }

    pub fn list_accounts(&self) -> Result<String> {
        let serial = self.changes.serial();
        let accounts = self.account_list().iter().map(super::store::model).collect();
        crate::items::reply(reiwand_core::model::AccountsReply {
            model: reiwand_core::model::MODEL,
            serial,
            accounts,
        })
    }

    pub(crate) fn resolve_links(&self) {
        if !self.account_list().iter().flat_map(|r| &r.links).any(|l| l.repo_id.is_none()) {
            return;
        }
        let known = reiwand_format::repos(&self.store.data, now()).unwrap_or_default();
        let done = self.edit_accounts(|file| {
            for link in file.accounts.iter_mut().flat_map(|r| r.links.iter_mut()) {
                let wanted = clean(&link.url);
                let found = known
                    .iter()
                    .find(|r| clean(&r.url) == wanted || r.hosts.iter().any(|h| clean(h) == wanted));
                if let (None, Some(repo)) = (&link.repo_id, found) {
                    link.repo_id = Some(repo.repo_id.clone());
                }
            }
            Ok(())
        });
        if let Err(e) = done {
            tracing::warn!("an account could not be linked to its repository: {}", e.parts().1);
        }
    }
}

fn same(link: &Link, wanted: &Link, id: &str) -> bool {
    let by_url = !wanted.url.is_empty() && clean(&link.url) == wanted.url;
    by_url || (!id.is_empty() && link.repo_id.as_deref() == Some(id))
}
