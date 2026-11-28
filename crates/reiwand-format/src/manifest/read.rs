use super::{Manifest, PREFIX};
use crate::{Error, Result, features};
use reiwand_core::message::said;
use serde_json::Value;

fn needs(value: &Value) -> (Option<&str>, Vec<String>) {
    let requires = value.get("requires");
    let engine = requires.and_then(|r| r.get("engine")).and_then(Value::as_str);
    let listed = requires.and_then(|r| r.get("features")).and_then(Value::as_array);
    (engine, listed.into_iter().flatten().filter_map(|f| f.as_str().map(String::from)).collect())
}

impl Manifest {
    pub fn from_json(bytes: &[u8]) -> Result<Self> {
        let value: Value = serde_json::from_slice(bytes)?;
        let schema = value.get("schema").and_then(Value::as_str).ok_or_else(|| {
            crate::refuse(
                "schema",
                said(
                    "format.manifest_missing",
                    "the manifest names no schema: expected %1",
                    &[&super::SCHEMA],
                ),
            )
        })?;
        let major = schema.strip_prefix(PREFIX).and_then(|n| n.parse::<u32>().ok());
        let current = PREFIX.to_owned() + "3";
        if let Some(major) = major.filter(|m| *m > 3) {
            return Err(Error::Newer(
                said("format.manifest_version", "manifest version %1", &[&major]).into(),
            ));
        }
        if schema != current {
            let reason = said(
                "format.old_manifest",
                "%1 is no longer read: this version reads only %2, so the package has to be made again in that format and signed again",
                &[&schema, &current],
            );
            return Err(crate::refuse("schema", reason));
        }
        let (engine, listed) = needs(&value);
        features::supported(engine, &listed)?;
        crate::strict::value("manifest.json", &value)
    }
}
