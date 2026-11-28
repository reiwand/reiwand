use crate::store::{optional, write};
use crate::{Installed, Result, Trusted};
use std::path::Path;

pub(crate) fn records(data: &Path) -> Result<Vec<Installed>> {
    optional(&data.join("installed.json"))
}

pub(crate) fn save(data: &Path, list: &[Installed]) -> Result<()> {
    write(&data.join("installed.json"), &reiwand_core::stored::text(&list)?, true)
}

pub(crate) fn keep_anchors(data: &Path, anchors: &[Trusted]) -> Result<()> {
    write(&data.join("anchors.json"), &reiwand_core::stored::text(&anchors)?, true)
}
