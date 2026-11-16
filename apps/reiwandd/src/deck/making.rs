use crate::daemon::Daemon;
use crate::error::failed;
use image::{GrayImage, RgbImage};
use reiwand_core::Fill;
use reiwand_core::edit::Depth;
use reiwand_core::model::JobKind;
use reiwand_depth::{Device, Inpainter, Places, Settings};
use serde_json::json;
use std::path::{Path, PathBuf};

const THREADS: usize = 4;

fn build(
    places: &Places,
    (file, sha): (&Path, &str),
    (depth, set): (&Depth, &Settings),
    ctl: &crate::jobs::Ctl,
) -> anyhow::Result<PathBuf> {
    let stop = ctl.flag();
    let threads = reiwand_depth::threads().min(THREADS);
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                reiwand_depth::background();
                let mut loaded = (set.fill == Fill::Lama)
                    .then(|| Inpainter::open(places, None, threads, Device::Auto))
                    .transpose()?;
                let mut paint = |image: &RgbImage, mask: &GrayImage| match loaded.as_mut() {
                    Some(l) => l.run(image, mask),
                    None => anyhow::bail!("LaMa is not loaded"),
                };
                let inpaint: Option<&mut reiwand_depth::Inpaint<'_>> =
                    (set.fill == Fill::Lama).then_some(&mut paint);
                places.build_layers(
                    (file, sha),
                    (depth, set),
                    inpaint,
                    &mut |done| ctl.progress(done, 100),
                    &stop,
                )
            })
            .join()
            .unwrap_or_else(|_| anyhow::bail!("the layer builder stopped"))
    })
}

impl Daemon {
    pub(super) fn build_deck(&self, file: &Path, sha: &str, (depth, set): (Depth, Settings), path: PathBuf) {
        if self.state().decks.failed.contains(&path) || reiwand_depth::has_failed(&path) {
            return;
        }
        let (file, sha) = (file.to_owned(), sha.to_owned());
        self.state().decks.building.insert(path.clone(), file.clone());
        let Some(daemon) = self.state().decks.me.upgrade() else { return };
        let target = file.to_string_lossy().into_owned();
        daemon.start_job(JobKind::Layers, &target, move |d, ctl| {
            let places = Places { data: d.store.data.clone(), cache: d.store.cache.clone() };
            let built = build(&places, (&file, &sha), (&depth, &set), ctl);
            if let Err(e) = &built {
                if ctl.check().is_err() {
                    return Err(crate::error::Error::Cancelled);
                }
                tracing::warn!("cannot build the depth layers of a picture");
                tracing::debug!("{e:#}");
                d.state().decks.failed.insert(path);
            }
            let made = built.map_err(|e| failed(format!("{e:#}")))?;
            d.configure_all();
            Ok(json!({"file": made, "fill": set.fill.as_str(), "layers": set.count}))
        });
    }
}
