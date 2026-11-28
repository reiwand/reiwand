use crate::Error;
use crate::bundle::Bundle;
use crate::manifest::Manifest;
use std::collections::BTreeMap;
use std::fs::DirBuilder;
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};

const SOURCE: &str = "source";
const STRIPPED: &str = "js";

#[derive(Debug, Clone)]
pub struct Installed {
    pub manifest: Manifest,
    pub modules: BTreeMap<String, String>,
    pub dir: PathBuf,
}

fn io(path: &Path) -> impl Fn(std::io::Error) -> Error + '_ {
    move |e| Error::Io(format!("{}: {e}", path.display()))
}

fn private(dir: &Path) -> Result<(), Error> {
    DirBuilder::new().recursive(true).mode(0o700).create(dir).map_err(io(dir))
}

fn written(root: &Path, name: &str, bytes: &[u8]) -> Result<(), Error> {
    let path = root.join(name);
    private(path.parent().unwrap_or(root))?;
    std::fs::write(&path, bytes).map_err(io(&path))
}

pub fn install(bundle: &Bundle, root: &Path) -> Result<Installed, Error> {
    let modules = bundle.strip()?;
    let id = &bundle.manifest.id;
    let (fresh, old, dir) = (root.join(format!(".{id}.new")), root.join(format!(".{id}.old")), root.join(id));
    private(root)?;
    for stale in [&fresh, &old] {
        if stale.exists() {
            std::fs::remove_dir_all(stale).map_err(io(stale))?;
        }
    }
    for (name, bytes) in &bundle.files {
        written(&fresh.join(SOURCE), name, bytes)?;
    }
    for (name, code) in &modules {
        written(&fresh.join(STRIPPED), &js(name), code.as_bytes())?;
    }
    if dir.exists() {
        std::fs::rename(&dir, &old).map_err(io(&dir))?;
    }
    std::fs::rename(&fresh, &dir).map_err(io(&dir))?;
    if old.exists() {
        std::fs::remove_dir_all(&old).map_err(io(&old))?;
    }
    Ok(Installed { manifest: bundle.manifest.clone(), modules, dir })
}

fn js(name: &str) -> String {
    format!("{}.js", name.strip_suffix(".ts").unwrap_or(name))
}

/// How an offered version stands against the installed one: `new` (none installed), `newer`,
/// `same` or `older`, so a consent can warn before going back.
pub fn step(installed: Option<&str>, offered: &str) -> &'static str {
    let parsed = |v: &str| semver::Version::parse(v).ok();
    match installed.map(|v| (parsed(v), parsed(offered))) {
        None => "new",
        Some((Some(old), Some(new))) if new == old => "same",
        Some((Some(old), Some(new))) if new < old => "older",
        Some(_) => "newer",
    }
}

impl Installed {
    /// Reads an installed plugin and strips its source again: the JavaScript it runs must be what
    /// the reviewed TypeScript gives, so a `js/` file changed on disk refuses the plugin.
    pub fn open(dir: &Path) -> Result<Installed, Error> {
        let bundle = Bundle::read(&dir.join(SOURCE))?;
        let modules = bundle.strip()?;
        for (name, code) in &modules {
            let path = dir.join(STRIPPED).join(js(name));
            if std::fs::read_to_string(&path).map_err(io(&path))? != *code {
                let why = "does not match its TypeScript source: install the plugin again";
                return Err(Error::Format(format!("{}: {why}", path.display())));
            }
        }
        Ok(Installed { manifest: bundle.manifest, modules, dir: dir.to_owned() })
    }

    pub fn manifest(root: &Path, id: &str) -> Result<Manifest, Error> {
        let path = root.join(id).join(SOURCE).join(crate::manifest::FILE);
        match crate::manifest::plugin_id(id) && path.is_file() {
            true => crate::bundle::manifest(&std::fs::read_to_string(&path).map_err(io(&path))?),
            false => Err(Error::Format(format!("no provider plugin {id:?} is installed"))),
        }
    }

    pub fn find(root: &Path, id: &str) -> Result<Installed, Error> {
        let dir = root.join(id);
        match crate::manifest::plugin_id(id) && dir.is_dir() {
            true => Installed::open(&dir),
            false => Err(Error::Format(format!("no provider plugin {id:?} is installed"))),
        }
    }

    pub fn all(root: &Path) -> Vec<(String, Result<Installed, Error>)> {
        let Ok(entries) = std::fs::read_dir(root) else { return Vec::new() };
        let mut names: Vec<String> =
            entries.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        names.retain(|n| !n.starts_with('.'));
        names.sort();
        names.into_iter().map(|n| (n.clone(), Installed::open(&root.join(n)))).collect()
    }

    pub fn source(&self) -> PathBuf {
        self.dir.join(SOURCE)
    }
}

pub fn remove(root: &Path, id: &str) -> Result<bool, Error> {
    let dir = root.join(id);
    if !crate::manifest::plugin_id(id) || !dir.is_dir() {
        return Ok(false);
    }
    std::fs::remove_dir_all(&dir).map_err(io(&dir))?;
    Ok(true)
}
