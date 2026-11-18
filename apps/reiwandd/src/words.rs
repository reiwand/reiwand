use reiwand_core::locale::{catalog, languages};
use std::collections::HashMap;
use std::sync::LazyLock;

mod ask;

pub use ask::{answers, flashing_disabled, request_group};

const CATALOGS: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/catalogs.rs"));
static CATALOG: LazyLock<HashMap<String, String>> = LazyLock::new(|| catalog(CATALOGS, &languages()));

fn tr(text: &str) -> String {
    CATALOG.get(text).cloned().unwrap_or_else(|| text.to_owned())
}

pub fn too_old() -> String {
    tr("This server needs a newer version of Reiwand: update it on this computer to keep using the account.")
}

pub fn signed_out() -> String {
    tr("The server no longer accepts this account: sign in again to keep using it.")
}

pub fn flashing_moves() -> String {
    tr("A package that flashes cannot keep moving when the desktop asks for reduced motion.")
}

pub fn nothing_before() -> String {
    tr("There is no earlier wallpaper to bring back.")
}

pub fn not_plasma() -> String {
    tr(
        "This only works in a Plasma session; elsewhere, removing the wallpaper already shows the desktop's own.",
    )
}

pub fn plasma_unreachable() -> String {
    tr("Plasma did not answer; change the wallpaper type in the desktop settings instead.")
}

pub fn earlier_gone() -> String {
    tr("The earlier wallpaper's file is no longer there, so it cannot be brought back.")
}

pub fn no_settings(screen: &str) -> String {
    match screen.is_empty() {
        true => tr("No screen shows a wallpaper with settings: apply a scene or a video first."),
        false => tr("%1 shows no wallpaper with settings.").replace("%1", screen),
    }
}

pub fn which_screen() -> String {
    tr("The screens show different wallpapers: name the screen whose settings you mean.")
}

pub fn volume() -> String {
    tr("Volume")
}

pub fn video() -> [String; 8] {
    let (speed, start, stop, volume) = (tr("Speed"), tr("Start"), tr("Stop at"), tr("Volume"));
    let (zoom, across, down, end) =
        (tr("Zoom"), tr("Horizontal position"), tr("Vertical position"), tr("0 plays to the end"));
    [speed, start, stop, volume, zoom, across, down, end]
}

pub fn scripts_off() -> String {
    tr("The scripts of this scene failed three times in a row and are off; the scene goes on without them.")
}

pub fn script_stopped(why: &str) -> String {
    tr("A script of this scene stopped and the scene goes on without it: %1").replace("%1", why)
}

pub fn web_missing() -> String {
    tr("Web wallpapers cannot be shown here: the web helper reiwand-web is not installed.")
}

pub fn web_engine(flatpak: bool) -> String {
    match flatpak {
        true => tr(
            "Web wallpapers cannot be shown here: Qt WebEngine 6.5 or newer cannot be loaded from this Flatpak. The Flatpak is broken or out of date: update or reinstall it, or install Reiwand from a package of your distribution.",
        ),
        false => tr(
            "Web wallpapers cannot be shown here: they need Qt WebEngine 6.5 or newer, which is not installed. Ubuntu 24.04 ships Qt 6.4, which is too old; install qt6-qtwebengine on Fedora or qt6-webengine on Arch Linux.",
        ),
    }
}

pub fn unshown(why: &str) -> String {
    tr("The wallpaper cannot be shown (%1); the background color shows instead.").replace("%1", why)
}

pub fn web_old(why: &str) -> String {
    tr("Web wallpapers cannot be shown here: they need Qt WebEngine 6.5 or newer (%1).").replace("%1", why)
}

pub fn web_unbuilt(why: &str) -> String {
    tr("Web wallpapers cannot be shown here: the web helper of this installation was built without Qt WebEngine, so the package is broken or out of date; reinstall or update Reiwand (%1).")
        .replace("%1", why)
}

pub fn web_broken(why: &str) -> String {
    tr("Web wallpapers cannot be shown here: the web helper does not start (%1).").replace("%1", why)
}

pub fn web_page_error(line: &str) -> String {
    tr("The page reported an error and may not be drawing: %1").replace("%1", line)
}

pub fn scene_script_error(line: &str) -> String {
    tr("A script of the scene reported an error: %1").replace("%1", line)
}

pub fn web_failed() -> String {
    tr("The web page stopped four times in a row and is off; the background color shows instead.")
}

pub fn web_stalled() -> String {
    tr("The web page drew nothing for 10 seconds and is off; the background color shows instead.")
}

pub fn web_full() -> String {
    tr(
        "The web page keeps much more data than it declared, so it runs without keeping anything until its data is cleared.",
    )
}

pub fn web_crashed() -> String {
    tr(
        "The web page crashed or stopped answering four times in a row and is off; the background color shows instead.",
    )
}

pub fn gpu_copies() -> String {
    tr(
        "The desktop draws with another graphics card than the one in use, so the frames of the wallpaper are copied through the processor.",
    )
}

pub fn gpu_missing(name: &str) -> String {
    tr("No graphics card matches “%1”; the automatic choice is used.").replace("%1", name)
}

pub fn gpu_unusable(name: &str) -> String {
    tr("The graphics card chosen by “%1” cannot be opened; the automatic choice is used.").replace("%1", name)
}

pub fn incomplete(missing: &str) -> String {
    tr("The package is installed but incomplete: it could not install %1, which it needs.")
        .replace("%1", missing)
}

#[cfg(test)]
mod tests {
    use reiwand_core::model::Input;

    #[test]
    fn consent_texts_speak_of_wallpapers_not_only_scenes() {
        for input in [Input::Media, Input::Audio, Input::Pointer] {
            let (title, body) = super::ask::request("Page", input);
            assert!(!title.contains("scene") && !body.contains("scene"));
        }
    }
}
