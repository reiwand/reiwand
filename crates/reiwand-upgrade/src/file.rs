use crate::sink::Sink;
use std::path::Path;

pub fn convert(path: &Path, password: Option<&str>, sink: &mut Sink) -> anyhow::Result<usize> {
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let ext = path.extension().map(|e| e.to_string_lossy().into_owned()).unwrap_or_default();
    let done = if path.is_dir() {
        return crate::scenes::walk_in(path, sink);
    } else if ext == "wpak" {
        crate::packages::wpak(path, sink)?
    } else if ext == "reiwand" {
        crate::carry::file(path, password, true)? != crate::Outcome::Nothing && {
            sink.note(path);
            if !sink.is_check() {
                sink.keep(path)?;
                crate::carry::file(path, password, false)?;
            }
            true
        }
    } else if name.ends_with(".reiwand.json") {
        crate::edits::file(path, sink)?
    } else if name == "scene.json" {
        crate::scenes::scene(path, sink)?
    } else if name == "manifest.json" {
        crate::scenes::manifest(path, sink)?
    } else {
        crate::scenes::effect(path, sink)?
    };
    Ok(usize::from(done))
}
