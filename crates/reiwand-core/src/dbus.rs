pub const BUS_NAME: &str = "io.github.reiwand.Daemon";
pub const OBJECT_PATH: &str = "/io/github/reiwand/Daemon";
pub const INTERFACE: &str = "io.github.reiwand.Daemon1";
pub const LIBRARY: &str = "io.github.reiwand.Library1";
pub const PLAYLISTS: &str = "io.github.reiwand.Playlists1";
pub const PROFILES: &str = "io.github.reiwand.Profiles1";
pub const PROVIDERS: &str = "io.github.reiwand.Providers1";
pub const REPOS: &str = "io.github.reiwand.Repos1";
pub const INPUTS: &str = "io.github.reiwand.Inputs1";
pub const SYNC: &str = "io.github.reiwand.Sync1";
pub const APP_ID: &str = "io.github.reiwand";
pub const APP_PATH: &str = "/io/github/reiwand";
pub const DEFAULT_OUTPUT: &str = "default";
pub const MAX_STREAM_EDGE: u32 = 16384;

pub fn launcher(sandbox_id: Option<String>) -> String {
    sandbox_id.filter(|id| !id.is_empty()).unwrap_or_else(|| APP_ID.into())
}

pub fn uri_scheme(text: &str) -> Option<&str> {
    let (scheme, _) = text.split_once(':')?;
    let mut chars = scheme.chars();
    let first = chars.next().is_some_and(|c| c.is_ascii_alphabetic());
    (first && chars.all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c))).then_some(scheme)
}

pub mod error {
    pub const NOT_FOUND: &str = "io.github.reiwand.Error.NotFound";
    pub const EXISTS: &str = "io.github.reiwand.Error.Exists";
    pub const NEEDS_SECRET: &str = "io.github.reiwand.Error.NeedsSecret";
    pub const KEYRING_UNAVAILABLE: &str = "io.github.reiwand.Error.KeyringUnavailable";
    pub const SIGNED_OUT: &str = "io.github.reiwand.Error.SignedOut";
    pub const STALE: &str = "io.github.reiwand.Error.Stale";
    pub const CONFLICT: &str = "io.github.reiwand.Error.Conflict";
    pub const NEWER: &str = "io.github.reiwand.Error.Newer";
    pub const UNCONFIRMED: &str = "io.github.reiwand.Error.Unconfirmed";
    pub const FLASHING: &str = "io.github.reiwand.Error.FlashingDisabled";
    pub const OFFLINE: &str = "io.github.reiwand.Error.Offline";
    pub const UNTRUSTED: &str = "io.github.reiwand.Error.Untrusted";

    const SILENT: &str = "failed without giving a reason";

    pub fn reason(name: &str, message: &str) -> String {
        let short = name.rsplit('.').next().unwrap_or_default();
        match (message.trim(), short) {
            ("", "") => SILENT.into(),
            ("", short) => format!("{SILENT} ({short})"),
            (message, _) => message.into(),
        }
    }

    pub fn silent(message: &str) -> Option<&str> {
        match message.trim().strip_prefix(SILENT) {
            Some("") => Some(""),
            Some(rest) => rest.strip_prefix(" (")?.strip_suffix(')'),
            None => message.trim().is_empty().then_some(""),
        }
    }
}
