use reiwand_core::Config;
use std::path::Path;

pub(crate) fn load(
    path: &Path,
    store: &crate::store::Store,
    starting: bool,
) -> Result<Config, reiwand_core::ConfigError> {
    let mut config = Config::load(path).inspect(|c| reiwand_core::net::set_private(c.network.private))?;
    let previews = starting && crate::drafts::forget_previews(&mut config);
    if (store.drop_dangling(&mut config) || previews)
        && let Err(e) = config.save(path)
    {
        tracing::warn!("cannot write the tidied configuration, it is used as it is: {e}");
    }
    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    #[test]
    fn a_configuration_that_cannot_be_written_back_is_still_used_at_start() {
        let dir = tempfile::tempdir().unwrap();
        let (settings, data) = (dir.path().join("settings"), dir.path().join("data"));
        std::fs::create_dir_all(&settings).unwrap();
        let path = settings.join("config.toml");
        let text = "[outputs.\"preview:1\"]\nimage = \"/a.png\"\n[outputs.DP-1]\nimage = \"/b.png\"\n";
        std::fs::write(&path, text).unwrap();
        let store = crate::store::Store::at(data, dir.path().join("cache"));
        std::fs::set_permissions(&settings, std::fs::Permissions::from_mode(0o500)).unwrap();
        let writable = std::fs::write(settings.join("probe"), b"").is_ok();
        let loaded = super::load(&path, &store, true);
        std::fs::set_permissions(&settings, std::fs::Permissions::from_mode(0o700)).unwrap();
        if writable {
            return eprintln!("this user writes anywhere: the read-only folder is not tried");
        }
        let config = loaded.expect("a full or read-only disk turned a sound configuration into the defaults");
        assert_eq!(config.outputs.keys().collect::<Vec<_>>(), ["DP-1"]);
        assert_eq!(config.outputs["DP-1"].image.as_deref(), Some(Path::new("/b.png")));
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            text,
            "the file changed while it could not be written"
        );
    }
}
