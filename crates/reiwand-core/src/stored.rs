use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::path::Path;

pub const FORMAT: u32 = 3;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Stored<T> {
    format: u32,
    content: T,
}

#[derive(Debug)]
pub enum StoredError {
    Format { path: String, found: String },
    Invalid { path: String, why: String },
}

impl std::fmt::Display for StoredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoredError::Format { path, found } => write!(
                f,
                "{path} has format {found} and this version needs {FORMAT}: run `reiwand-upgrade client` once to convert it; nothing was overwritten"
            ),
            StoredError::Invalid { path, why } => write!(f, "{path} is not valid: {why}"),
        }
    }
}

impl std::error::Error for StoredError {}

pub fn text<T: Serialize>(content: &T) -> serde_json::Result<Vec<u8>> {
    serde_json::to_vec_pretty(&Stored { format: FORMAT, content })
}

pub fn parse<T: DeserializeOwned>(path: &str, bytes: &[u8]) -> Result<T, StoredError> {
    let invalid = |why: String| StoredError::Invalid { path: path.to_owned(), why };
    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|e| invalid(e.to_string()))?;
    let found = value.get("format").and_then(serde_json::Value::as_u64);
    if found != Some(u64::from(FORMAT)) {
        let found = found.map_or_else(|| "none".to_owned(), |n| n.to_string());
        return Err(StoredError::Format { path: path.to_owned(), found });
    }
    serde_json::from_value::<Stored<T>>(value).map(|s| s.content).map_err(|e| invalid(e.to_string()))
}

pub fn read<T: DeserializeOwned>(path: &Path) -> Result<Option<T>, StoredError> {
    match std::fs::read(path) {
        Ok(bytes) => parse(&path.display().to_string(), &bytes).map(Some),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(StoredError::Invalid { path: path.display().to_string(), why: e.to_string() }),
    }
}
