use super::ids::{reopen, scene_id};
use crate::daemon::Daemon;
use crate::error::{Result, invalid, not_found};
use crate::inputs::{Hub, grants_path};
use reiwand_core::edit::{self, Action, Edit, History};
use reiwand_core::model::Input;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub(super) fn following(d: &Daemon, parallax: &BTreeSet<String>) -> BTreeMap<String, BTreeSet<Input>> {
    let st = d.state();
    let file = |output: &String| Some(st.outputs.get(output)?.uploaded.as_ref()?.2.display().to_string());
    let real = parallax.iter().filter(|output| !output.starts_with("preview:"));
    real.filter_map(file).map(|file| (file, BTreeSet::from([Input::Pointer]))).collect()
}

pub(super) fn revoke(d: &Daemon, hub: &Hub, scene: &str, input: Input) -> Result<()> {
    if !scene.starts_with('/') {
        let scene = scene_id(scene)?;
        hub.revoke(&grants_path(d), scene, input)?;
        return Ok(reopen(d, input)?);
    }
    let parallax = crate::compositor::lock(&hub.control).parallax.clone();
    if input != Input::Pointer || !following(d, &parallax).contains_key(scene) {
        return Err(not_found(format!(
            "{scene} is not a picture shown now whose parallax follows the pointer"
        )));
    }
    let file = Path::new(scene);
    let (before, after) = still(file, &d.store.data.join("edits"))?;
    let mut history = History::load(file);
    history.record(&before, &after);
    history.save(file).map_err(invalid)?;
    tracing::info!("a picture stopped following the pointer");
    Ok(())
}

pub(super) fn still(file: &Path, edits: &Path) -> Result<(Edit, Edit)> {
    let found = edit::load(file, edits).map_err(|e| invalid(e.to_string()))?;
    let Some((_, before)) = found else { return Err(not_found("the picture has no edit")) };
    let mut after = before.clone();
    let (strength, focus, steps, smoothing, overscan) = (None, None, None, None, None);
    let (fill, layers) = (None, None);
    let off = Action::Parallax {
        source: None,
        strength,
        focus,
        steps,
        smoothing,
        overscan,
        fill,
        layers,
        off: true,
    };
    after.act(off).map_err(|e| invalid(e.message().to_string()))?;
    edit::commit(&after, file, edits).map_err(|e| invalid(e.to_string()))?;
    Ok((before, after))
}

#[cfg(test)]
mod tests {
    use super::still;
    use reiwand_core::edit::{self, Action, Edit, Source};

    #[test]
    fn stopping_a_picture_turns_its_parallax_off_and_keeps_its_settings() {
        let dir = tempfile::tempdir().unwrap();
        let (file, edits) = (dir.path().join("a.png"), dir.path().join("edits"));
        std::fs::write(&file, b"picture").unwrap();
        let mut edit = Edit { source: Source::read(&file, [4, 2]).unwrap(), ..Edit::default() };
        let (strength, focus, steps, smoothing, overscan) = (Some(0.02), None, None, None, None);
        let (source, fill, layers) = (Some("pointer".to_owned()), None, None);
        edit.act(Action::Parallax {
            source,
            strength,
            focus,
            steps,
            smoothing,
            overscan,
            fill,
            layers,
            off: false,
        })
        .unwrap();
        edit::commit(&edit, &file, &edits).unwrap();
        still(&file, &edits).unwrap();
        let (_, after) = edit::load(&file, &edits).unwrap().unwrap();
        let parallax = after.parallax.unwrap();
        assert!(!parallax.enabled, "the picture still follows the pointer");
        assert_eq!(parallax.strength, 0.02, "its settings were lost");
    }
}
