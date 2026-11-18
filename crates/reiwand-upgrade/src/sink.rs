use anyhow::Context;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Sink {
    check: bool,
    plain: bool,
    copies: PathBuf,
    kept: usize,
    changed: Vec<PathBuf>,
    unsigned: Vec<String>,
}

impl Sink {
    pub fn copying(copies: &Path) -> Self {
        Sink {
            check: false,
            plain: false,
            copies: copies.to_path_buf(),
            kept: 0,
            changed: Vec::new(),
            unsigned: Vec::new(),
        }
    }

    pub fn plain() -> Self {
        Sink {
            check: false,
            plain: true,
            copies: PathBuf::new(),
            kept: 0,
            changed: Vec::new(),
            unsigned: Vec::new(),
        }
    }

    pub fn checking() -> Self {
        Sink {
            check: true,
            plain: false,
            copies: PathBuf::new(),
            kept: 0,
            changed: Vec::new(),
            unsigned: Vec::new(),
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
        self.kept += 1;
        let name = path.file_name().context("no file name")?.to_string_lossy().into_owned();
        let mut index =
            fs::OpenOptions::new().create(true).append(true).open(self.copies.join("index.txt"))?;
        writeln!(index, "{}\t{}", self.kept, path.display())?;
        Ok(self.copies.join(format!("{}-{name}", self.kept)))
    }

    pub fn keep(&mut self, path: &Path) -> anyhow::Result<()> {
        if path.is_file() && !self.plain {
            let slot = self.slot(path)?;
            fs::copy(path, slot).with_context(|| format!("cannot keep a copy of {}", path.display()))?;
        }
        Ok(())
    }

    pub fn move_away(&mut self, path: &Path) -> anyhow::Result<()> {
        if path.is_file() {
            let slot = self.slot(path)?;
            fs::rename(path, slot)?;
        }
        Ok(())
    }

    pub fn save(&mut self, path: &Path, converted: &[u8]) -> anyhow::Result<()> {
        self.note(path);
        if self.check {
            return Ok(());
        }
        self.keep(path)?;
        let name = path.file_name().context("no file name")?.to_string_lossy().into_owned();
        let staged = path.with_file_name(format!("{name}.upgrading"));
        fs::write(&staged, converted)?;
        Ok(fs::rename(staged, path)?)
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
