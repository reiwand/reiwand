use crate::compositor::Settings;
use reiwand_core::{Config, Playback, Transition, dbus};
use zbus::fdo;

pub(crate) fn first_load(
    path: &std::path::Path,
    store: &crate::store::Store,
) -> anyhow::Result<Result<Config, reiwand_core::ConfigError>> {
    use anyhow::Context;
    if !path.exists() {
        Config::first_start(path).context("cannot write the first configuration")?;
    }
    let loaded = crate::controls::load(path, store, true);
    if let Err(e @ reiwand_core::ConfigError::Format { .. }) = &loaded {
        anyhow::bail!("{e}");
    }
    Ok(loaded)
}

pub(crate) fn settings(config: &Config) -> Settings {
    let playback = config.playback.sanitized();
    let transition =
        std::time::Duration::from_millis(config.transition.duration_ms.min(Transition::MAX_MS).into());
    let Playback {
        max_fps,
        volume,
        muted,
        software_parallax,
        allow_flashing,
        scene_sound,
        quality,
        auto_pace,
        ..
    } = playback;
    let duck = crate::duck::gain();
    Settings {
        transition,
        max_fps,
        volume,
        muted,
        software_parallax,
        flash_guard: !allow_flashing,
        scene_sound,
        duck,
        quality,
        pace: auto_pace,
        less_motion: config.transition.less_motion,
    }
}

pub(crate) fn output_name(output: &str) -> fdo::Result<String> {
    let output = output.trim();
    if output.len() > 128 || output.chars().any(char::is_control) {
        return Err(fdo::Error::InvalidArgs("invalid output name".into()));
    }
    Ok(if output.is_empty() { dbus::DEFAULT_OUTPUT.to_owned() } else { output.to_owned() })
}
