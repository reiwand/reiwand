use crate::depth::DepthRust;
use crate::depth::qobject::DepthMaps;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use reiwand_core::edit::Depth;
use reiwand_depth::{Places, adjust, coloured, lift_map, read, write};
use std::path::PathBuf;
use std::pin::Pin;

impl DepthMaps {
    pub fn view(mut self: Pin<&mut Self>, depth: &QString, colour: bool) {
        if std::mem::replace(&mut self.as_mut().rust_mut().viewing, true) {
            self.as_mut().rust_mut().again = Some((depth.to_string(), colour));
            return;
        }
        let depth: Depth = serde_json::from_str(&depth.to_string()).unwrap_or_default();
        let (qt, image, sha) = (self.qt_thread(), self.file.clone(), self.sha.clone());
        std::thread::spawn(move || {
            let shown = crate::depthwork::look(&Places::default(), (&image, &sha), &depth, colour);
            qt.queue(move |mut maps| {
                let old = crate::app::path(&maps.look.to_string());
                maps.as_mut().set_look(QString::from(&shown.map(|p| bus_url(&p)).unwrap_or_default()));
                old.map(std::fs::remove_file);
                maps.as_mut().rust_mut().viewing = false;
                if let Some((depth, colour)) = maps.as_mut().rust_mut().again.take() {
                    maps.view(&QString::from(&depth), colour);
                }
            })
            .ok();
        });
    }

    pub fn export_map(
        mut self: Pin<&mut Self>,
        file: &QString,
        depth: &QString,
        adjusted: bool,
        colour: bool,
    ) {
        let to = crate::app::path(&file.to_string()).map(PathBuf::from).unwrap_or_default();
        let depth: Depth = serde_json::from_str(&depth.to_string()).unwrap_or_default();
        let (qt, image, sha) = (self.qt_thread(), self.file.clone(), self.sha.clone());
        self.as_mut().set_error(QString::default());
        self.as_mut().set_note(QString::default());
        std::thread::spawn(move || {
            let made = save_map(&Places::default(), (&image, &sha), &depth, (&to, adjusted, colour));
            qt.queue(move |mut maps| match made {
                Ok(()) => maps.as_mut().set_note(QString::from(&*to.to_string_lossy())),
                Err(e) => maps.as_mut().set_error(QString::from(&format!("{e:#}"))),
            })
            .ok();
        });
    }
}

fn bus_url(path: &std::path::Path) -> String {
    crate::bus::url(&path.to_string_lossy())
}

impl Drop for DepthRust {
    fn drop(&mut self) {
        crate::app::path(&self.look.to_string()).map(std::fs::remove_file);
    }
}

fn save_map(
    places: &Places,
    (image, sha): (&std::path::Path, &str),
    depth: &Depth,
    (to, adjusted, colour): (&std::path::Path, bool, bool),
) -> anyhow::Result<()> {
    let from =
        places.find(image, sha, depth).ok_or_else(|| anyhow::anyhow!("the image has no depth map yet"))?;
    let mut map = read(&from)?;
    if adjusted {
        map = adjust(&map, depth);
        lift_map(&mut map, &depth.strokes);
    }
    match colour {
        true => coloured(&map, true).save_with_format(to, image::ImageFormat::Png)?,
        false => write(&map, to)?,
    }
    Ok(())
}
