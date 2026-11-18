use reiwand_account::{File, Link, Rec, origin};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub trait Vault {
    fn take(&mut self, item: &str) -> anyhow::Result<Option<String>>;
    fn put(&mut self, item: &str, secret: &str) -> anyhow::Result<()>;
    fn drop_item(&mut self, item: &str) -> anyhow::Result<()>;
}

#[derive(Debug, Default, PartialEq)]
pub struct Moved {
    pub accounts: File,
    pub pins: BTreeMap<String, String>,
    pub moved: bool,
}

pub struct NoVault;

impl Vault for NoVault {
    fn take(&mut self, _: &str) -> anyhow::Result<Option<String>> {
        Ok(None)
    }
    fn put(&mut self, _: &str, _: &str) -> anyhow::Result<()> {
        Ok(())
    }
    fn drop_item(&mut self, _: &str) -> anyhow::Result<()> {
        Ok(())
    }
}

fn write(path: &Path, bytes: Vec<u8>) -> anyhow::Result<()> {
    let staged = path.with_extension("json.upgrading");
    fs::write(&staged, bytes)?;
    Ok(fs::rename(staged, path)?)
}

pub fn apply(data: &Path, vault: &mut dyn Vault) -> anyhow::Result<()> {
    let Ok(bytes) = fs::read(data.join("sync.json")) else { return Ok(()) };
    let sync: Value = serde_json::from_slice(&bytes)?;
    let pins = fs::read(data.join("repo-pins.json")).ok().and_then(|b| serde_json::from_slice(&b).ok());
    let done = accounts_step(&sync, pins.unwrap_or_default(), vault)?;
    if done.moved && !data.join("accounts.json").exists() {
        write(&data.join("accounts.json"), done.accounts.text()?)?;
        write(&data.join("repo-pins.json"), serde_json::to_vec(&done.pins)?)?;
    }
    Ok(())
}

fn text(saved: &Value, field: &str) -> String {
    saved[field].as_str().unwrap_or_default().to_owned()
}

pub fn accounts_step(
    saved: &Value,
    pins: BTreeMap<String, String>,
    vault: &mut dyn Vault,
) -> anyhow::Result<Moved> {
    let mut moved = Moved { pins, ..Moved::default() };
    let Some(site) = origin(&text(saved, "url")) else { return Ok(moved) };
    let (user, user_id) = match saved["owner"].as_array() {
        Some(owner) => {
            (owner[0].as_str().unwrap_or_default().to_owned(), owner[1].as_i64().unwrap_or_default())
        }
        None => (String::new(), 0),
    };
    let pin = text(saved, "pin");
    if !pin.is_empty() {
        moved.pins.insert(site.clone(), pin.to_ascii_lowercase());
    }
    let id = format!("a{user_id:x}{}", site.len());
    let token = vault
        .take("server")?
        .and_then(|held| serde_json::from_str::<Value>(&held).ok())
        .and_then(|held| held["token"].as_str().map(str::to_owned))
        .filter(|token| !token.is_empty());
    if let Some(token) = &token {
        vault.put(&format!("account {id}"), token)?;
    }
    let repo_url = text(saved, "repo_url");
    let links = match repo_url.is_empty() {
        true => Vec::new(),
        false => vec![Link { url: repo_url, repo_id: saved["repo_id"].as_str().map(str::to_owned) }],
    };
    let label = text(saved, "label");
    let label = if label.is_empty() { user.clone() } else { label };
    let pending = token.is_none();
    moved.accounts.accounts.push(Rec { id, origin: site, user, user_id, label, pending, links });
    moved.moved = true;
    Ok(moved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Default)]
    struct Memory(BTreeMap<String, String>);

    impl Vault for Memory {
        fn take(&mut self, item: &str) -> anyhow::Result<Option<String>> {
            Ok(self.0.get(item).cloned())
        }
        fn put(&mut self, item: &str, secret: &str) -> anyhow::Result<()> {
            self.0.insert(item.to_owned(), secret.to_owned());
            Ok(())
        }
        fn drop_item(&mut self, item: &str) -> anyhow::Result<()> {
            self.0.remove(item);
            Ok(())
        }
    }

    fn sync() -> Value {
        json!({"url": "https://srv.example:8443/api", "pin": "AB12", "owner": ["ana", 7],
            "repo_url": "https://srv.example:8443/r/shelf", "repo_id": "rid", "label": "home", "offers": []})
    }

    #[test]
    fn the_saved_account_becomes_a_linked_account_with_its_secret_moved() {
        let mut vault =
            Memory(BTreeMap::from([("server".to_owned(), r#"{"token":"tok","key":""}"#.to_owned())]));
        let done = accounts_step(&sync(), BTreeMap::new(), &mut vault).unwrap();
        let rec = &done.accounts.accounts[0];
        assert_eq!(
            (rec.origin.as_str(), rec.user.as_str(), rec.pending),
            ("https://srv.example:8443", "ana", false)
        );
        assert_eq!(rec.links[0].repo_id.as_deref(), Some("rid"));
        assert_eq!(done.pins["https://srv.example:8443"], "ab12");
        assert_eq!(vault.0.get(&format!("account {}", rec.id)).map(String::as_str), Some("tok"));
    }

    #[test]
    fn a_missing_secret_leaves_the_account_pending() {
        let done = accounts_step(&sync(), BTreeMap::new(), &mut Memory::default()).unwrap();
        assert!(done.accounts.accounts[0].pending);
    }

    #[test]
    fn apply_writes_the_account_and_the_pin_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("sync.json"), serde_json::to_vec(&sync()).unwrap()).unwrap();
        apply(dir.path(), &mut NoVault).unwrap();
        let file = File::read(&dir.path().join("accounts.json")).unwrap();
        assert!(file.accounts[0].pending);
        let pins: BTreeMap<String, String> =
            serde_json::from_slice(&fs::read(dir.path().join("repo-pins.json")).unwrap()).unwrap();
        assert_eq!(pins["https://srv.example:8443"], "ab12");
    }

    #[test]
    fn nothing_saved_moves_nothing() {
        let done = accounts_step(&json!({}), BTreeMap::new(), &mut Memory::default()).unwrap();
        assert!(!done.moved && done.accounts.accounts.is_empty());
    }
}
