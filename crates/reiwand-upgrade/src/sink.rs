use anyhow::Context;
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Sink {
    check: bool,
    plain: bool,
    copies: PathBuf,
    kept: usize,
    kept_paths: BTreeSet<PathBuf>,
    skipped: Vec<(PathBuf, String)>,
    changed: Vec<PathBuf>,
    unsigned: Vec<String>,
}

impl Sink {
    fn make(check: bool, plain: bool, copies: PathBuf) -> Self {
        Sink {
            check,
            plain,
            copies,
            kept: 0,
            kept_paths: BTreeSet::new(),
            changed: Vec::new(),
            unsigned: Vec::new(),
            skipped: Vec::new(),
        }
    }

    pub fn copying(copies: &Path) -> Self {
        Self::make(false, false, copies.to_path_buf())
    }

    pub fn plain() -> Self {
        Self::make(false, true, PathBuf::new())
    }

    pub fn checking() -> Self {
        Self::make(true, false, PathBuf::new())
    }

    pub fn skipped(&self) -> &[(PathBuf, String)] {
        &self.skipped
    }

    pub fn skip(&mut self, path: &Path, reason: String) {
        self.skipped.push((path.to_path_buf(), reason));
    }

    pub fn attempt<T>(&mut self, path: &Path, run: impl FnOnce(&mut Sink) -> anyhow::Result<T>) -> Option<T> {
        match run(self) {
            Ok(done) => Some(done),
            Err(e) => {
                self.skip(path, format!("{e:#}"));
                None
            }
        }
    }

    pub fn is_check(&self) -> bool {
        self.check
    }

    pub fn changed(&self) -> &[PathBuf] {
        &self.changed
    }

    pub fn unsigned(&self) -> &[String] {
        &self.unsigned
    }

    pub fn lost_signature(&mut self, package: String) {
        self.unsigned.push(package);
    }

    pub fn copies(&self) -> &Path {
        &self.copies
    }

    pub fn note(&mut self, path: &Path) {
        self.changed.push(path.to_path_buf());
    }

    fn slot(&mut self, path: &Path) -> anyhow::Result<PathBuf> {
        anyhow::ensure!(!self.plain, "this run keeps no copies");
        fs::create_dir_all(&self.copies)?;
        if self.kept == 0 {
            // the settings and the data of one run keep their copies in one folder: number after the ones there
            self.kept = fs::read_to_string(self.copies.join("index.txt")).map_or(0, |t| t.lines().count());
        }
        self.kept += 1;
        let name = path.file_name().context("no file name")?.to_string_lossy().into_owned();
        let mut index =
            fs::OpenOptions::new().create(true).append(true).open(self.copies.join("index.txt"))?;
        writeln!(index, "{}\t{}", self.kept, path.display())?;
        Ok(self.copies.join(format!("{}-{name}", self.kept)))
    }

    pub fn keep(&mut self, path: &Path) -> anyhow::Result<()> {
        if path.is_file() && !self.plain && self.kept_paths.insert(path.to_path_buf()) {
            let slot = self.slot(path)?;
            fs::copy(path, slot).with_context(|| format!("cannot keep a copy of {}", path.display()))?;
        }
        Ok(())
    }

    pub fn save(&mut self, path: &Path, converted: &[u8]) -> anyhow::Result<()> {
        self.note(path);
        if self.check {
            return Ok(());
        }
        self.keep(path)?;
        crate::atomic::write(path, converted)
    }

    pub fn text(
        &mut self,
        path: &Path,
        convert: impl FnOnce(&str) -> anyhow::Result<String>,
    ) -> anyhow::Result<bool> {
        let Ok(original) = fs::read_to_string(path) else { return Ok(false) };
        let converted = convert(&original).with_context(|| format!("cannot convert {}", path.display()))?;
        if converted == original {
            return Ok(false);
        }
        self.save(path, converted.as_bytes())?;
        Ok(true)
    }
}
