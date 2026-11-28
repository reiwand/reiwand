use crate::daemon::Daemon;
use crate::error::{Result, failed};
use std::sync::{Arc, Weak};

pub struct Accounts(pub Weak<Daemon>);

impl Accounts {
    fn d(&self) -> Result<Arc<Daemon>> {
        self.0.upgrade().ok_or_else(|| failed("the daemon is stopping"))
    }
}

#[zbus::interface(name = "io.github.reiwand.Accounts1")]
impl Accounts {
    #[zbus(out_args("json"))]
    async fn list(&self) -> Result<String> {
        self.d()?.run(|d| d.list_accounts()).await
    }

    #[zbus(out_args("user_code", "verification_uri", "job"))]
    async fn sign_in(&self, repo_url: String) -> Result<(String, String, u64)> {
        self.d()?.run(move |d| d.sign_in(&repo_url)).await
    }

    #[zbus(name = "Use")]
    async fn use_account(&self, repo: String, account: String) -> Result<()> {
        self.d()?.run(move |d| d.use_account(&repo, &account)).await
    }

    async fn unlink(&self, repo: String) -> Result<()> {
        self.d()?.run(move |d| d.unlink_account(&repo)).await
    }

    async fn sign_out(&self, account: String) -> Result<()> {
        self.d()?.run(move |d| d.sign_out_account(&account)).await
    }

    async fn remove(&self, account: String) -> Result<()> {
        self.d()?.run(move |d| d.remove_account(&account)).await
    }
}
