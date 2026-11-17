use super::store_key;
use crate::config::Config;
use crate::db::now;
use crate::repos;
use anyhow::{Context, ensure};
use reiwand_format::{Keypair, RootStatus, Settings, Signer, fingerprint, keygen, settings};
use serde_json::Value;
use std::fs::{self, DirBuilder};
use std::io::ErrorKind;
use std::os::unix::fs::DirBuilderExt;
use std::path::Path;

pub fn valid_name(name: &str) -> bool {
    let ok = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit();
    (1..=32).contains(&name.len()) && name.bytes().all(|b| ok(b) || b == b'-') && ok(name.as_bytes()[0])
}

pub fn init(cfg: &Config, title: &str, require_signature: bool, open: bool) -> anyhow::Result<Keypair> {
    init_repo(cfg, "main", title, require_signature, open)
}

pub fn init_repo(
    cfg: &Config,
    name: &str,
    title: &str,
    require_signature: bool,
    open: bool,
) -> anyhow::Result<Keypair> {
    let repo = repos::prepare(cfg, name)?;
    ensure!(!repo.dir.join("root/1.json").exists(), "the repository {name} already exists");
    let (root, online) = (keygen(None)?, keygen(None)?);
    let (root_keys, online_keys) = (vec![root.public.clone()], vec![online.public.clone()]);
    let settings = Settings {
        name: title.into(),
        root_keys,
        online_keys,
        require_author_signature: require_signature,
        online_authors: open,
        ..Settings::default()
    };
    reiwand_format::init(&repo.dir, &settings, now() / 1000)?;
    reiwand_format::sign_root(&repo.dir, &Signer::new(&root.secret, None)?)?;
    store_key(&repo.key, &online.secret)?;
    crate::seal::keep_root(&cfg.data, name, &root.secret)?;
    repos::record(cfg, &repo)?;
    Ok(root)
}

pub fn import(cfg: &Config, src: &Path, online: Option<&Path>) -> anyhow::Result<RootStatus> {
    import_repo(cfg, "main", src, online)
}

pub fn import_repo(
    cfg: &Config,
    name: &str,
    src: &Path,
    online: Option<&Path>,
) -> anyhow::Result<RootStatus> {
    let repo = repos::prepare(cfg, name)?;
    DirBuilder::new().recursive(true).mode(0o700).create(repo.dir.join("root"))?;
    for item in fs::read_dir(src.join("root")).context("the source has no root folder")? {
        let item = item?;
        let (dest, bytes) = (repo.dir.join("root").join(item.file_name()), fs::read(item.path())?);
        match fs::read(&dest) {
            Ok(stored) => ensure!(stored == bytes, "{} differs from the stored copy", dest.display()),
            Err(e) if e.kind() == ErrorKind::NotFound => fs::write(&dest, bytes)?,
            Err(e) => return Err(e.into()),
        }
    }
    if !repo.dir.join("catalog.json").exists() {
        fs::copy(src.join("catalog.json"), repo.dir.join("catalog.json"))
            .context("the source has no catalog.json")?;
    }
    let status = reiwand_format::status(&repo.dir)?;
    ensure!(status.complete, "root {} is not signed by enough root keys", status.version);
    if let Some(path) = online {
        let secret = fs::read_to_string(path)?;
        let signer = Signer::new(&secret, None).context("expected an unencrypted minisign secret key")?;
        let keys = settings(&repo.dir)?.online_keys;
        ensure!(
            keys.iter().any(|k| fingerprint(k).is_ok_and(|f| f == signer.fingerprint)),
            "the key is not the online key of the root"
        );
        store_key(&repo.key, &secret)?;
    }
    repos::record(cfg, &repo)?;
    Ok(status)
}

pub fn status(cfg: &Config) -> anyhow::Result<Value> {
    status_repo(cfg, "main")
}

pub fn status_repo(cfg: &Config, name: &str) -> anyhow::Result<Value> {
    let repo = repos::prepare(cfg, name)?;
    repo.info(cfg).with_context(|| format!("no repository {name} yet: run reiwand-server repo init"))
}

pub fn statuses(cfg: &Config) -> anyhow::Result<Value> {
    let found = repos::load(cfg)?.iter().filter_map(|r| r.info(cfg)).collect::<Vec<_>>();
    Ok(Value::Array(found))
}
