use crate::daemon::Daemon;
use crate::error::{Error, Result, invalid};
use crate::import::OLDER;
use crate::jobs::now;
use reiwand_format::{Limits, installed, open};
use semver::Version;
use std::fs;
use std::path::Path;

impl Daemon {
    pub(crate) fn local_query(&self, query: &str) -> String {
        let packages = self.store.data.join("packages");
        let kept = format!("source:none -folder:\"{}\"", packages.display());
        query.replace("local:yes", &kept)
    }

    pub(crate) fn import_package(&self, path: &Path, older: bool) -> Result<String> {
        let package = open(path, &Limits::default())
            .and_then(|mut p| p.look_closer().map(|()| p))
            .map_err(|e| invalid(format!("the package is damaged or was changed: {e}")))?;
        let (id, version) = (package.manifest.id.clone(), package.manifest.version.clone());
        let wanted = Version::parse(&version).map_err(|e| invalid(format!("{id} has a bad version: {e}")))?;
        let kept: Vec<_> = installed(&self.store.data)?.into_iter().filter(|i| i.id == id).collect();
        if !kept.is_empty() {
            return Err(invalid(format!("{id} is already installed from a repository; uninstall it first")));
        }
        let found = self.store.read(|lib| lib.loose_packages(&id))?;
        let newest = found.iter().filter_map(|f| Version::parse(&f.2).ok()).max();
        match newest {
            Some(n) if n == wanted => {
                return Ok(found.iter().find(|f| f.2 == version).map_or_else(String::new, |f| f.0.clone()));
            }
            Some(n) if n > wanted && !older => {
                return Err(Error::Named(OLDER, format!("{id} {version} is older than the {n} you have")));
            }
            _ => {}
        }
        let copied = self.copy_in(path)?;
        let Some((uid, old, _)) = found.into_iter().max_by_key(|f| Version::parse(&f.2).ok()) else {
            let added = self.store.write(|lib| lib.import_file(&copied, now()));
            return added.inspect_err(|_| drop(fs::remove_file(&copied)));
        };
        let before = self.store.read(|lib| lib.item(&uid))?;
        let relinked = self.store.write(|lib| lib.relink_item(&uid, &copied, now()));
        relinked.inspect_err(|_| drop(fs::remove_file(&copied)))?;
        self.forget_files(&[before]);
        if old.starts_with(self.store.files()) {
            fs::remove_file(&old).ok();
        }
        Ok(uid)
    }
}
