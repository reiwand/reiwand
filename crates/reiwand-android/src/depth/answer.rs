use super::{Request, STOP, change, edited, found, loaded, places};
use crate::power::Power;
use crate::said;
use crate::{Error, framing, library, store};
use reiwand_core::edit::{self, Edit, Parallax};
use reiwand_depth::Device;
use serde_json::{Value, json};
use std::fs::File;
use std::os::fd::BorrowedFd;
use std::path::Path;
use std::sync::atomic::Ordering::Relaxed;

fn tune(edit: &mut Edit, on: Option<bool>, strength: Option<f32>, focus: Option<f32>) -> Result<(), Error> {
    tune_depth(edit, (on, strength, focus), (None, None))
}

fn tune_depth(
    edit: &mut Edit,
    (on, strength, focus): (Option<bool>, Option<f32>, Option<f32>),
    (fill, layers): (Option<reiwand_core::Fill>, Option<u8>),
) -> Result<(), Error> {
    let p = edit.parallax.get_or_insert_with(Parallax::default);
    (p.fill, p.layers) = (fill.or(p.fill), layers.or(p.layers));
    p.enabled = on.unwrap_or(p.enabled);
    (p.strength, p.focus) = (strength.unwrap_or(p.strength), focus.unwrap_or(p.focus));
    if on == Some(true) && p.strength <= 0.0 {
        p.strength = Parallax::default().strength;
    }
    Ok(())
}

fn moving(image: &Path) -> Result<Value, Error> {
    change(image, |edit| tune(edit, Some(true), None, None)).map(|()| Value::Null)
}

pub(super) fn answer(request: Request) -> Result<Value, Error> {
    if let Request::Cancel = request {
        return Ok(json!(!STOP.swap(true, Relaxed)));
    }
    let places = places()?;
    let image = |uid: &str| library::with(|lib| Ok(crate::pack::media(&lib.item(uid)?)?.0));
    Ok(match request {
        Request::Model => super::model::shown(&places, super::model::assets()),
        Request::Install => super::model::install(&places, super::model::assets())?,
        Request::Forget => json!(super::model::forget(&places)),
        Request::Lama => super::model::shown(&places, super::model::lama()),
        Request::InstallLama => super::model::install(&places, super::model::lama())?,
        Request::ForgetLama => json!(super::model::forget_lama(&places)),
        Request::Layers { uid, library, device } => {
            let device: Device = device.as_deref().unwrap_or("cpu").parse()?;
            let image = image(&uid)?;
            STOP.store(false, Relaxed);
            reiwand_depth::background();
            super::layers::build(&image, &edited(&image)?, library.as_deref(), device)?;
            store::changed();
            Value::Null
        }
        Request::Make { uid, library, device } => {
            let device: Device = device.as_deref().unwrap_or("cpu").parse()?;
            super::model::ready(&places)?;
            let image = image(&uid)?;
            let edit = edited(&image)?;
            STOP.store(false, Relaxed);
            reiwand_depth::background();
            let (depth, sha) = (edit.depth.clone().unwrap_or_default(), &edit.source.sha256);
            let runtime = (library.as_deref().filter(|l| l.is_file()), device);
            let made =
                reiwand_depth::generate_until(&places, (&image, sha), &depth, runtime, &mut |_| {}, &STOP);
            if made.as_ref().is_err_and(|e| e.is::<reiwand_depth::Stopped>()) {
                Err(said("android.depth_cancelled", "the depth map was cancelled", &[]))?;
            }
            made?;
            moving(&image)?;
            super::layers::best(&image, library.as_deref(), device)
        }
        Request::Cancel => Value::Null,
        Request::Look { uid, colour } => {
            let image = image(&uid)?;
            json!(super::maps::look(&image, &loaded(&image)?, colour)?)
        }
        Request::Unmap { uid } => {
            let image = image(&uid)?;
            let edit = edited(&image)?;
            places.unlayer(&edit.source.sha256);
            let removed = places.unmap(&image, &edit.source.sha256, &edit.depth.unwrap_or_default());
            store::changed();
            json!(removed)
        }
        Request::Levels { uid, near, far, gamma, smooth, dilate } => {
            let levels = edit::Action::Depth { near, far, gamma, smooth, dilate, input: None };
            change(&image(&uid)?, |edit| edit.act(levels).map_err(|e| Error::Said(e.message().clone())))?;
            Value::Null
        }
        Request::Import { uid, fd } => {
            let image = image(&uid)?;
            let mut source = File::from(unsafe { BorrowedFd::borrow_raw(fd) }.try_clone_to_owned()?);
            let mut copy = tempfile::NamedTempFile::new_in(store::dir()?)?;
            std::io::copy(&mut source, &mut copy)?;
            places.import(copy.path(), &image, &edited(&image)?.source.sha256)?;
            moving(&image)?;
            super::layers::best(&image, None, Device::Cpu)
        }
        Request::Item { uid } => {
            let image = image(&uid)?;
            let edit = loaded(&image)?;
            let p = edit.parallax.clone().unwrap_or_default();
            let levels = edit.depth.clone().unwrap_or_default();
            let still = super::layers::still(&image, &edited(&image)?).map(|s| s.word());
            let size = super::places().map(|p| reiwand_depth::model_size(&p)).unwrap_or_default();
            json!({ "parallax": edit.moving().is_some(), "still": still, "model_size": size, "map": found(&image, &edit).is_some(),
                "near": levels.near, "far": levels.far, "gamma": levels.gamma, "smooth": levels.smooth, "dilate": levels.dilate,
                "strength": p.strength, "focus": p.focus, "layers": super::layers::path(&image, &edit).is_some(),
                "fill": super::layers::shown(&p), "fit": edit.fit, "base": edit.geometry.crop,
                "crop": framing::draft(&uid).or_else(|| framing::own(&edit)), "paused": Power::now().stills(),
                "changed": super::maps::changed(&image, &edit) })
        }
        Request::Parallax { uid, on, strength, focus, fill, layers } => {
            let layers = layers.map(|n| {
                reiwand_core::LAYERS
                    .contains(&n)
                    .then_some(n)
                    .ok_or_else(|| said("android.depth_layers", "Use 4 to 6 depth layers", &[]))
            });
            let layers = layers.transpose()?;
            change(&image(&uid)?, |edit| tune_depth(edit, (on, strength, focus), (fill, layers)))?;
            Value::Null
        }
        Request::Fit { uids, fit } => framing::fit(&uids, fit)?,
        Request::Frame { uid, pan, zoom, screen, done } => framing::frame(uid, pan, zoom, screen, done)?,
        Request::Unframe { uid } => framing::unframe(&image(&uid)?, &uid)?,
    })
}
