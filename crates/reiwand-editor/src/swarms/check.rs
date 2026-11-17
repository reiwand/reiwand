use super::*;

pub(super) fn parsed<T: serde::de::DeserializeOwned>(v: &Value, what: &str) -> Result<()> {
    match reiwand_core::explicit::read::<T>(what, v) {
        Err(e) if e.fault != reiwand_core::explicit::Fault::Missing => {
            Err(anyhow!(said("editor.particle_value", "%1 does not fit: %2", &[&what, &e])))
        }
        _ => Ok(()),
    }
}

fn issues(dir: &Path, doc: &Value) -> Result<Vec<(String, String)>> {
    let mut scene: reiwand_scene::Scene = serde_json::from_value(doc["scene"].clone())?;
    let unread = crate::cache::read_files(&mut scene, dir);
    let props = crate::meta::read(doc, Some(dir)).map(|m| m.properties).unwrap_or_default();
    let all = unread.into_iter().chain(scene.validate(&props));
    Ok(all.map(|i| (i.to_string(), i.message().to_string())).collect())
}

pub(crate) fn checked(dir: &Path, doc: &Value, ops: Vec<Op>) -> Result<Vec<Op>> {
    let mut probe = doc.clone();
    let mut ops = ops;
    apply(&mut probe, &ops)?;
    let filled = crate::fill::ops(&probe);
    apply(&mut probe, &filled)?;
    ops.extend(filled);
    let before = issues(dir, doc).unwrap_or_default();
    let fresh = issues(dir, &probe)?.into_iter().find(|i| !before.contains(i));
    if let Some((_, message)) = fresh {
        bail!(message);
    }
    Ok(ops)
}
