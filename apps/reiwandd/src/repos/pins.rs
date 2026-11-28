use crate::daemon::Daemon;
use crate::error::{Result, failed};
use crate::jobs::Ctl;
use reiwand_providers::HttpFetch;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;

pub fn fetcher(d: &Daemon, ctl: &Ctl, bases: &[String]) -> Result<HttpFetch> {
    fetcher_trusting(d, ctl, bases, None)
}

pub(super) fn fetcher_trusting(
    d: &Daemon,
    ctl: &Ctl,
    bases: &[String],
    extra: Option<(&str, &str)>,
) -> Result<HttpFetch> {
    let fetch = HttpFetch::new(ctl.flag()).map_err(|e| failed(e.to_string()))?.within(bases);
    pinned(d, d.with_repo_pins(fetch, extra)?)
}

pub fn pinned(d: &Daemon, fetch: HttpFetch) -> Result<HttpFetch> {
    d.with_account_tokens(fetch)
}

pub fn origin(url: &str) -> Option<String> {
    let url = reqwest::Url::parse(url).ok()?;
    matches!(url.scheme(), "http" | "https").then(|| url.origin().ascii_serialization())
}

impl Daemon {
    fn pins_path(&self) -> PathBuf {
        self.store.data.join("repo-pins.json")
    }

    pub(crate) fn repo_pins(&self) -> BTreeMap<String, String> {
        reiwand_core::stored::read(&self.pins_path()).ok().flatten().unwrap_or_default()
    }

    pub(crate) fn pin_origin(&self, origin: &str, pin: &str) -> Result<()> {
        let mut pins = self.repo_pins();
        pins.insert(origin.to_owned(), pin.to_ascii_lowercase());
        let path = self.pins_path();
        let dir = path.parent().unwrap_or(std::path::Path::new("."));
        let mut file = tempfile::Builder::new().prefix(".tmp-").tempfile_in(dir)?;
        file.write_all(&reiwand_core::stored::text(&pins).map_err(|e| failed(e.to_string()))?)?;
        file.persist(&path).map_err(|e| crate::error::Error::from(e.error))?;
        Ok(())
    }

    pub(super) fn with_repo_pins(
        &self,
        mut fetch: HttpFetch,
        extra: Option<(&str, &str)>,
    ) -> Result<HttpFetch> {
        let mut pins = self.repo_pins();
        if let Some((origin, pin)) = extra {
            pins.insert(origin.to_owned(), pin.to_ascii_lowercase());
        }
        for (origin, pin) in pins {
            let tls = reiwand_account::pinned_tls(&pin).map_err(|e| failed(e.to_string()))?;
            fetch = fetch.pinned(&origin, tls).map_err(|e| failed(e.to_string()))?;
        }
        Ok(fetch)
    }
}
