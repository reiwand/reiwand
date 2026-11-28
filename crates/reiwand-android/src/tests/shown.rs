use super::open;
use crate::{library, settings, store};

/// Stores the green picture and shows it stretched with sound on, as a first wallpaper chosen by hand.
pub(super) fn green_stretched() -> String {
    let (green, _) = store::store(&mut open("green-64x32.png"), "green.png").unwrap();
    let path = library::with(|lib| Ok(lib.item(&green)?.path)).unwrap();
    settings::update(|config| {
        config.set_image("", path, None);
        config.set_fit("", reiwand_core::Fit::Stretch);
        config.playback.muted = false;
        Ok(())
    })
    .unwrap();
    green
}
