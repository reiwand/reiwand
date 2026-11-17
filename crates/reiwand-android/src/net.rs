use crate::{Error, said, settings};
use reiwand_account::Api;
use reiwand_format::Fetch;
use reiwand_providers::HttpFetch;
use std::io::{self, Write};
use std::sync::Arc;

pub(crate) const PRIVATE: &str = "private_mode";
const CUT: &str = "private mode: the network is cut";

pub(crate) fn private() -> bool {
    settings::chosen(PRIVATE) == "on"
}

pub(crate) fn open() -> Result<(), Error> {
    if private() {
        Err(said("android.private_mode", "private mode: the network is cut", &[]))
    } else {
        Ok(())
    }
}

pub(crate) fn api(url: &str, pin: Option<&str>) -> Result<Api, Error> {
    open()?;
    Ok(Api::pinned(url, pin)?)
}

pub(crate) async fn peek(url: &str) -> Result<String, Error> {
    open()?;
    Ok(reiwand_account::peek(url).await?)
}

pub(crate) fn local(base: &str) -> bool {
    base.starts_with("file://") || !base.contains("://")
}

pub(crate) fn reach(base: &str) -> Result<(), Error> {
    if local(base) { Ok(()) } else { open() }
}

pub(crate) struct Gated(HttpFetch);

impl Gated {
    pub(crate) fn within(self, bases: &[String]) -> Self {
        Self(self.0.within(bases))
    }

    fn allowed(base: &str) -> io::Result<()> {
        if local(base) || !private() { Ok(()) } else { Err(io::Error::other(CUT)) }
    }
}

impl Fetch for Gated {
    fn get(&self, base: &str, path: &str, max: u64) -> io::Result<Option<Vec<u8>>> {
        Self::allowed(base)?;
        self.0.get(base, path, max)
    }

    fn copy(&self, base: &str, path: &str, max: u64, out: &mut dyn Write) -> io::Result<bool> {
        Self::allowed(base)?;
        self.0.copy(base, path, max, out)
    }
}

pub(crate) fn fetch() -> Result<Gated, Error> {
    fetch_with(None)
}

pub(crate) fn fetch_with(extra: Option<(&str, &str)>) -> Result<Gated, Error> {
    let failed =
        |e: &dyn std::fmt::Display| said("android.web_client", "web connections cannot start: %1", &[&e]);
    let mut web = HttpFetch::new(Arc::default()).map_err(|e| failed(&e))?;
    let mut pins = crate::account::pins::all();
    if let Some((origin, print)) = extra {
        pins.insert(origin.to_owned(), print.to_ascii_lowercase());
    }
    for (origin, print) in pins {
        web = web.pinned(&origin, reiwand_account::pinned_tls(&print)?).map_err(|e| failed(&e))?;
    }
    Ok(Gated(crate::account::using::with_tokens(web)?))
}

#[cfg(test)]
#[test]
fn no_connection_opens_beside_the_gate() {
    let raw = [
        "Api::pinned(",
        "HttpFetch::",
        "reqwest::Client",
        "TcpStream",
        "UdpSocket",
        "ToSocketAddrs",
        "lookup_host",
    ];
    let built = [
        "Wallhaven::new",
        "WebDav::",
        "Immich::",
        "reiwand_providers::client",
        "client()",
        " peek(",
        "(peek(",
        "reiwand_account::peek",
    ];
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut stack = vec![root.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let (path, name) = (entry.path(), entry.file_name().to_string_lossy().into_owned());
            if path.is_dir() && name != "tests" {
                stack.push(path);
                continue;
            }
            let own = path.strip_prefix(&root).unwrap().to_string_lossy().replace('\\', "/");
            if !name.ends_with(".rs") || own == "net.rs" || own == "tests.rs" {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            let code = text.split("#[cfg(test)]").next().unwrap_or_default();
            let gated = own == "providers/build.rs" && code.contains("crate::net::open()?;");
            let found = raw.iter().chain(built.iter().filter(|_| !gated)).find(|t| code.contains(**t));
            assert!(found.is_none(), "{own} opens a connection beside the gate: {found:?}");
        }
    }
}
