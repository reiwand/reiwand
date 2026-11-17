use reiwand_upgrade::Vault;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Memory {
    pub items: BTreeMap<String, String>,
    pub unreachable: bool,
}

impl Vault for Memory {
    fn take(&mut self, item: &str) -> anyhow::Result<Option<String>> {
        anyhow::ensure!(!self.unreachable, "the keyring is locked");
        Ok(self.items.get(item).cloned())
    }
    fn put(&mut self, item: &str, secret: &str) -> anyhow::Result<()> {
        self.items.insert(item.to_owned(), secret.to_owned());
        Ok(())
    }
    fn drop_item(&mut self, item: &str) -> anyhow::Result<()> {
        self.items.remove(item);
        Ok(())
    }
}
