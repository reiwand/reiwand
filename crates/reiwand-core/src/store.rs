use crate::config::Config;
use crate::keep::keep;
use std::io::Write;
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("cannot read {path}: {source}")]
    Read { path: PathBuf, source: std::io::Error },
    #[error("cannot write {path}: {source}")]
    Write { path: PathBuf, source: std::io::Error },
    #[error("invalid configuration at line {line}, column {column}: {message}")]
    Parse { line: usize, column: usize, message: String },
    #[error("cannot serialize configuration: {0}")]
    Serialize(#[from] toml::ser::Error),
    #[error("{0}")]
    Fields(#[from] crate::explicit::ReadError),
    #[error("{path} is in format {found}, this version reads format {wanted}: {fix}")]
    Format { path: String, found: String, wanted: u32, fix: String },
    #[error("no configuration directory could be determined")]
    NoConfigDir,
}

pub fn stopped_effects() -> Option<PathBuf> {
    Some(dirs::cache_dir()?.join("reiwand").join("stopped-effects"))
}

pub fn gpu_tier() -> Option<PathBuf> {
    Some(dirs::state_dir()?.join("reiwand").join("gpu-tier.json"))
}

impl Config {
    pub fn default_path() -> Result<PathBuf, ConfigError> {
        Ok(dirs::config_dir().ok_or(ConfigError::NoConfigDir)?.join("reiwand").join("config.toml"))
    }

    pub fn load(path: &Path) -> Result<Config, ConfigError> {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Config::default()),
            Err(source) => return Err(ConfigError::Read { path: path.into(), source }),
        };
        let mut config = Self::parse(&text).map_err(|e| e.at(path))?;
        for key in config.normalize() {
            tracing::warn!(key, "invalid value replaced by the nearest valid one");
        }
        Ok(config)
    }

    pub fn parse(text: &str) -> Result<Config, ConfigError> {
        let value = toml::from_str::<toml::Value>(text).map_err(|error| ConfigError::parse(text, &error))?;
        ConfigError::check_format(&value, "config.toml")?;
        let json = serde_json::to_value(&value).map_err(|e| ConfigError::Parse {
            line: 1,
            column: 1,
            message: e.to_string(),
        })?;
        Ok(crate::explicit::read("config.toml", &json)?)
    }

    pub fn first_start(path: &Path) -> Result<Config, ConfigError> {
        let config = Config::default();
        config.save(path)?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<(), ConfigError> {
        let fresh = toml::to_string_pretty(self)?;
        let read = |text: &str| toml::from_str::<Config>(text).ok();
        let kept = std::fs::read_to_string(path).ok().and_then(|old| keep(&old, &fresh));
        let kept = kept.filter(|text| read(text).is_some_and(|c| Some(c) == read(&fresh)));
        write_private(path, kept.as_deref().unwrap_or(&fresh).as_bytes())
    }

    pub fn backup_broken(path: &Path) -> Result<PathBuf, ConfigError> {
        let data = std::fs::read(path).map_err(|source| ConfigError::Read { path: path.into(), source })?;
        let backup = path.with_added_extension("broken");
        write_private(&backup, &data).map(|()| backup)
    }
}

impl ConfigError {
    fn at(self, path: &Path) -> ConfigError {
        match self {
            ConfigError::Format { found, wanted, fix, .. } => {
                ConfigError::Format { path: path.display().to_string(), found, wanted, fix }
            }
            other => other,
        }
    }

    fn check_format(value: &toml::Value, path: &str) -> Result<(), ConfigError> {
        let found = value.get("format").and_then(toml::Value::as_integer);
        let wanted = Config::FORMAT;
        let fix = match found {
            Some(n) if n == i64::from(wanted) => return Ok(()),
            Some(n) if n > i64::from(wanted) => "update Reiwand",
            _ => "run `reiwand-upgrade client` once to convert it; nothing was overwritten",
        };
        let found = found.map_or_else(|| "none".to_owned(), |n| n.to_string());
        Err(ConfigError::Format { path: path.to_owned(), found, wanted, fix: fix.to_owned() })
    }

    fn parse(text: &str, error: &toml::de::Error) -> ConfigError {
        let before = text.get(..error.span().map_or(0, |span| span.start)).unwrap_or_default();
        let line = before.split('\n').count();
        let column = before.rsplit('\n').next().unwrap_or_default().chars().count() + 1;
        ConfigError::Parse { line, column, message: error.message().trim().replace('\n', "; ") }
    }
}

pub(crate) fn write_private(path: &Path, data: &[u8]) -> Result<(), ConfigError> {
    let target = std::fs::canonicalize(path)
        .or_else(|_| std::fs::read_link(path).map(|link| path.with_file_name(link)))
        .unwrap_or_else(|_| path.to_owned());
    let write_err = |source| ConfigError::Write { path: target.clone(), source };
    let dir = target.parent().filter(|d| !d.as_os_str().is_empty()).unwrap_or(Path::new("."));
    std::fs::DirBuilder::new().recursive(true).mode(0o700).create(dir).map_err(write_err)?;
    let mut file = tempfile::Builder::new().prefix(".reiwand-").tempfile_in(dir).map_err(write_err)?;
    file.write_all(data).map_err(write_err)?;
    file.persist(&target).map_err(|e| write_err(e.error))?;
    Ok(())
}
