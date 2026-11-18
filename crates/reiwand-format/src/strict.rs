use crate::{Error, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub(crate) fn value<T: DeserializeOwned>(file: &str, value: &Value) -> Result<T> {
    reiwand_core::explicit::read(file, value)
        .map_err(|e| Error::Invalid { field: e.path.clone(), reason: e.said().into() })
}

pub(crate) fn read<T: DeserializeOwned>(file: &str, bytes: &[u8]) -> Result<T> {
    value(file, &serde_json::from_slice(bytes)?)
}
