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
        let depth = match parsed(&depth.to_string()) {
            Ok(depth) => depth,
            Err(e) => {
                self.as_mut().rust_mut().viewing = false;
                return self.as_mut().set_error(QString::from(&e));
            }
        };
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
        let depth = match parsed(&depth.to_string()) {
            Ok(depth) => depth,
            Err(e) => return self.as_mut().set_error(QString::from(&e)),
        };
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

/// The depth block of the edit, or `null` for a picture without one, which uses the defaults.
fn parsed(text: &str) -> Result<Depth, String> {
    let block: Option<Depth> =
        serde_json::from_str(text).map_err(|e| format!("the depth settings cannot be read: {e}"))?;
    Ok(block.unwrap_or_default())
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

#[cfg(test)]
mod tests {
    use super::*;
    use reiwand_depth::Map;

    #[test]
    fn saves_the_raw_and_the_coloured_map() {
        let dir = tempfile::tempdir().unwrap();
        let image = dir.path().join("pic.png");
        let places = Places { data: dir.path().join("data"), cache: dir.path().join("cache") };
        let raw = Map::from_fn(4, 4, |x, _| image::Luma([(x * 16000) as u16]));
        write(&raw, &reiwand_depth::beside(&image)).unwrap();
        let depth = Depth::default();
        let plain = dir.path().join("plain.png");
        save_map(&places, (&image, "none"), &depth, (&plain, false, false)).unwrap();
        assert_eq!(read(&plain).unwrap().get_pixel(3, 0).0, [48000]);
        let tinted = dir.path().join("tinted.png");
        save_map(&places, (&image, "none"), &depth, (&tinted, true, true)).unwrap();
        assert!(image::open(&tinted).unwrap().color().has_color());
        let missing = dir.path().join("gone.png");
        let error = save_map(&places, (&missing, "none"), &depth, (&plain, false, false)).unwrap_err();
        assert!(error.to_string().contains("no depth map"));
    }

    #[test]
    fn a_picture_without_depth_settings_uses_the_defaults_and_broken_ones_are_refused() {
        assert_eq!(parsed("null").unwrap(), Depth::default());
        let whole = serde_json::to_string(&Depth { input: 700, ..Depth::default() }).unwrap();
        assert_eq!(parsed(&whole).unwrap().input, 700);
        assert!(parsed("{}").unwrap_err().contains("cannot be read"));
        assert!(parsed("{\"input\":").is_err());
    }
}
