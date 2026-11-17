use crate::Vault;
use anyhow::{Context, bail};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use zbus::{ObjectPath, OwnedObjectPath, OwnedVariant, Variant};

const SCHEMA: &str = "io.github.reiwand.Server";
const KIND: &str = "server";
const COLLECTION: &str = "/org/freedesktop/secrets/aliases/default";
type Secret = (OwnedObjectPath, Vec<u8>, Vec<u8>, String);

pub struct KeyringVault {
    file: PathBuf,
    runtime: tokio::runtime::Runtime,
    bus: Option<zbus::Connection>,
}

impl KeyringVault {
    pub fn open(data: &Path) -> anyhow::Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        let bus = runtime.block_on(zbus::Connection::session()).ok();
        Ok(KeyringVault { file: data.join("secrets.json"), runtime, bus })
    }

    fn in_file(&self) -> bool {
        self.file.exists()
    }

    fn held(&self) -> anyhow::Result<BTreeMap<String, String>> {
        serde_json::from_slice(&std::fs::read(&self.file)?).context("the private secrets file is damaged")
    }

    fn call(
        &self,
        path: &str,
        interface: &str,
        method: &str,
        body: &(impl serde::Serialize + zbus::zvariant::DynamicType),
    ) -> anyhow::Result<zbus::Message> {
        let bus = self.bus.as_ref().context("no Secret Service is available")?;
        let path = ObjectPath::try_from(path)?;
        let sent = bus.call_method(Some("org.freedesktop.secrets"), path, Some(interface), method, body);
        Ok(self.runtime.block_on(sent)?)
    }

    fn search(&self, item: &str) -> anyhow::Result<Vec<OwnedObjectPath>> {
        let attributes = HashMap::from([("xdg:schema", SCHEMA), (KIND, item)]);
        let reply = self.call(
            "/org/freedesktop/secrets",
            "org.freedesktop.Secret.Service",
            "SearchItems",
            &attributes,
        )?;
        let (open, locked): (Vec<OwnedObjectPath>, Vec<OwnedObjectPath>) = reply.body().deserialize()?;
        if open.is_empty() && !locked.is_empty() {
            bail!("the keyring is locked: unlock it and run the upgrade again");
        }
        Ok(open)
    }

    fn session(&self) -> anyhow::Result<OwnedObjectPath> {
        let reply = self.call(
            "/org/freedesktop/secrets",
            "org.freedesktop.Secret.Service",
            "OpenSession",
            &("plain", Variant::from("")),
        )?;
        let (_, session): (OwnedVariant, OwnedObjectPath) = reply.body().deserialize()?;
        Ok(session)
    }

    fn close(&self, session: &OwnedObjectPath) {
        self.call(session.as_str(), "org.freedesktop.Secret.Session", "Close", &()).ok();
    }

    fn read_ring(&self, item: &str) -> anyhow::Result<Option<String>> {
        let Some(found) = self.search(item)?.into_iter().next() else { return Ok(None) };
        let session = self.session()?;
        let reply = self.call(
            "/org/freedesktop/secrets",
            "org.freedesktop.Secret.Service",
            "GetSecrets",
            &(vec![&found], &session),
        );
        self.close(&session);
        let mut secrets: HashMap<OwnedObjectPath, Secret> = reply?.body().deserialize()?;
        let (.., value, _) = secrets.remove(&found).context("the keyring returned no secret")?;
        Ok(Some(String::from_utf8(value)?))
    }

    fn write_ring(&self, item: &str, secret: &str) -> anyhow::Result<()> {
        let session = self.session()?;
        let attributes = HashMap::from([("xdg:schema", SCHEMA), (KIND, item)]);
        let properties = HashMap::from([
            ("org.freedesktop.Secret.Item.Label", Variant::from(format!("Reiwand: {item}"))),
            ("org.freedesktop.Secret.Item.Attributes", Variant::from(attributes)),
        ]);
        let value: Secret =
            (session.clone(), Vec::new(), secret.as_bytes().to_vec(), "text/plain".to_owned());
        let created = self.call(
            COLLECTION,
            "org.freedesktop.Secret.Collection",
            "CreateItem",
            &(properties, value, true),
        );
        self.close(&session);
        let (_, prompt): (OwnedObjectPath, OwnedObjectPath) = created?.body().deserialize()?;
        if prompt.as_str() != "/" {
            bail!("the keyring asks for an unlock: unlock it and run the upgrade again");
        }
        Ok(())
    }

    fn delete_ring(&self, item: &str) -> anyhow::Result<()> {
        for found in self.search(item)? {
            self.call(found.as_str(), "org.freedesktop.Secret.Item", "Delete", &())?;
        }
        Ok(())
    }
}

impl Vault for KeyringVault {
    fn take(&mut self, item: &str) -> anyhow::Result<Option<String>> {
        if self.in_file() {
            return Ok(self.held()?.get(&format!("{KIND}/{item}")).cloned());
        }
        self.read_ring(item)
    }

    fn put(&mut self, item: &str, secret: &str) -> anyhow::Result<()> {
        if self.in_file() {
            let mut held = self.held()?;
            held.insert(format!("{KIND}/{item}"), secret.to_owned());
            return Ok(std::fs::write(&self.file, serde_json::to_vec(&held)?)?);
        }
        self.write_ring(item, secret)
    }

    fn drop_item(&mut self, item: &str) -> anyhow::Result<()> {
        if self.in_file() {
            let mut held = self.held()?;
            held.remove(&format!("{KIND}/{item}"));
            return Ok(std::fs::write(&self.file, serde_json::to_vec(&held)?)?);
        }
        self.delete_ring(item)
    }
}
