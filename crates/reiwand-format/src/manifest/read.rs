use super::{Header, Manifest, PREFIX};
use crate::{Error, Result, features};
use reiwand_core::message::said;

impl Manifest {
    pub fn from_json(bytes: &[u8]) -> Result<Self> {
        let header: Header = serde_json::from_slice(bytes)?;
        let major = header.schema.strip_prefix(PREFIX).and_then(|n| n.parse::<u32>().ok());
        if let Some(major) = major.filter(|m| *m > 2) {
            return Err(Error::Newer(
                said("format.manifest_version", "manifest version %1", &[&major]).into(),
            ));
        }
        if major == Some(1) {
            let reason = said(
                "format.first_version",
                "reiwand.wpak/1 is no longer read; the package needs reiwand.wpak/2",
                &[],
            );
            return Err(crate::refuse("schema", reason));
        }
        features::supported(header.requires.engine.as_deref(), &header.requires.features)?;
        Ok(serde_json::from_slice(bytes)?)
    }
}
