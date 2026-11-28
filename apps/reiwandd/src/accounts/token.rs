use super::block_on;
use crate::daemon::Daemon;
use crate::error::Result;
use crate::keyring::Stored;
use zeroize::Zeroizing;

fn ring(id: &str) -> String {
    format!("account {id}")
}

impl Daemon {
    pub(super) fn account_token(&self, id: &str) -> Option<Zeroizing<String>> {
        if let Some(token) = self.accounts.cached(id) {
            return Some(token);
        }
        let ring_id = ring(id);
        let read = block_on(async { self.server_keyring()?.read(&ring_id, false).await });
        match read {
            Ok(Stored::Secret(token)) => {
                self.accounts.keep(id, Some(token.clone()));
                Some(token)
            }
            Ok(Stored::Locked) => {
                tracing::debug!("the keyring is locked; the account is not used until it is unlocked");
                None
            }
            Ok(Stored::Missing) => None,
            Err(e) => {
                tracing::debug!("the access of an account cannot be read: {}", e.parts().1);
                None
            }
        }
    }

    pub(super) fn save_token(&self, id: &str, token: &str) -> Result<()> {
        let ring_id = ring(id);
        block_on(async { self.server_keyring()?.store(&ring_id, token).await })?;
        self.accounts.keep(id, Some(Zeroizing::new(token.to_owned())));
        Ok(())
    }

    pub(super) fn drop_token(&self, id: &str) -> Result<()> {
        self.accounts.keep(id, None);
        let ring_id = ring(id);
        block_on(async { self.server_keyring()?.delete(&ring_id).await })
    }
}
