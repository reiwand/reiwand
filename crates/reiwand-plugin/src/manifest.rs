pub use crate::setting::{Kind, Setting};
use reiwand_core::explicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const SCHEMA: &str = "reiwand.provider/1";
pub const FILE: &str = "reiwand-provider.json";
pub(crate) const RESERVED: [&str; 9] =
    ["kind", "refresh", "cache_mb", "keep", "allow_plain_http", "password", "api_key", "token", "secret"];
const FORBIDDEN: [&str; 9] = [
    "host",
    "content-length",
    "transfer-encoding",
    "connection",
    "cookie",
    "upgrade",
    "te",
    "trailer",
    "proxy-authorization",
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: String,
    pub id: String,
    pub version: String,
    pub title: String,
    pub license: String,
    pub hosts: Vec<String>,
    #[serde(deserialize_with = "explicit::null")]
    #[schemars(required)]
    pub secret: Option<Secret>,
    pub settings: Vec<Setting>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Secret {
    pub label: String,
    pub header: String,
    pub prefix: String,
    pub hosts: Vec<String>,
    pub required: bool,
}

pub(crate) type Checked = Result<(), (String, String)>;

pub(crate) fn fault(path: impl Into<String>, why: impl Into<String>) -> Checked {
    Err((path.into(), why.into()))
}

pub fn host_name(host: &str) -> bool {
    let label = |l: &str| {
        (1..=63).contains(&l.len())
            && l.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            && !l.starts_with('-')
            && !l.ends_with('-')
    };
    let numeric = host.rsplit('.').next().is_some_and(|last| last.bytes().all(|b| b.is_ascii_digit()));
    host.len() <= 253 && host.split('.').all(label) && !numeric
}

pub(crate) fn text(value: &str, max: usize) -> bool {
    !value.trim().is_empty() && value.chars().count() <= max && !value.chars().any(char::is_control)
}

pub fn plugin_id(id: &str) -> bool {
    let mut chars = id.chars();
    let first = chars.next().is_some_and(|c| c.is_ascii_lowercase());
    first
        && id.len() <= 32
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

impl Manifest {
    pub fn check(&self) -> Checked {
        if self.schema != SCHEMA {
            return fault("schema", format!("this Reiwand reads only {SCHEMA}, not {:?}", self.schema));
        }
        if !plugin_id(&self.id) {
            return fault("id", "a lowercase letter, then at most 31 lowercase letters, digits, - or _");
        }
        semver::Version::parse(&self.version)
            .map_err(|e| ("version".into(), format!("not a version: {e}")))?;
        if !text(&self.title, 80) {
            return fault("title", "a short text of at most 80 characters");
        }
        if !text(&self.license, 64) {
            return fault("license", "the licence of the plugin, such as GPL-3.0-or-later");
        }
        if !(1..=16).contains(&self.hosts.len()) {
            return fault("hosts", "from 1 to 16 host names");
        }
        for (i, host) in self.hosts.iter().enumerate() {
            if !host_name(host) || self.hosts[..i].contains(host) {
                return fault(
                    format!("hosts[{i}]"),
                    "an exact host name, once, in lowercase, with no scheme, port or address",
                );
            }
        }
        if let Some(secret) = &self.secret {
            self.check_secret(secret)?;
        }
        if self.settings.len() > 32 {
            return fault("settings", "at most 32 settings");
        }
        for (i, setting) in self.settings.iter().enumerate() {
            let known = self.settings[..i].iter().any(|s| s.key == setting.key);
            setting.check(known).map_err(|(path, why)| (join(&format!("settings[{i}]"), &path), why))?;
        }
        Ok(())
    }

    fn check_secret(&self, secret: &Secret) -> Checked {
        if !text(&secret.label, 64) {
            return fault("secret.label", "a short text that names the secret");
        }
        let token = secret.header.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-');
        if secret.header.is_empty()
            || !token
            || FORBIDDEN.contains(&secret.header.to_ascii_lowercase().as_str())
        {
            return fault("secret.header", "the name of the HTTP header that carries the secret");
        }
        if secret.prefix.chars().any(char::is_control) || secret.prefix.len() > 32 {
            return fault("secret.prefix", "at most 32 characters, such as \"Bearer \" or \"\"");
        }
        if secret.hosts.is_empty() {
            return fault("secret.hosts", "the hosts that receive the secret");
        }
        match secret.hosts.iter().position(|h| !self.hosts.contains(h)) {
            Some(i) => fault(format!("secret.hosts[{i}]"), "a host that is also in hosts"),
            None => Ok(()),
        }
    }

    pub fn header(&self, name: &str) -> bool {
        let lower = name.to_ascii_lowercase();
        self.secret.as_ref().is_some_and(|s| s.header.eq_ignore_ascii_case(name))
            || FORBIDDEN.contains(&lower.as_str())
    }
}

fn join(parent: &str, path: &str) -> String {
    if path.is_empty() { parent.into() } else { format!("{parent}.{path}") }
}
