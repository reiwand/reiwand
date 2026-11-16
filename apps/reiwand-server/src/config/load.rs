use super::{Clients, Config, Publish, Registration};
use anyhow::{Context, Result, ensure};
use std::collections::HashMap;
use std::fs;
use std::net::IpAddr;
use std::path::PathBuf;

const LOOPBACK: &str = "127.0.0.0/8,::1";

impl Config {
    pub fn from_env() -> Result<Config> {
        let mut vars = HashMap::new();
        for (key, value) in std::env::vars().filter(|(k, _)| k.starts_with("REIWAND_")) {
            match key.strip_suffix("_FILE") {
                Some(base) => {
                    let text = fs::read_to_string(&value).with_context(|| format!("cannot read {key}"))?;
                    vars.insert(base.to_string(), text.trim().to_string());
                }
                None => drop(vars.entry(key).or_insert(value)),
            }
        }
        Config::from_vars(|key| vars.get(key).filter(|v| !v.trim().is_empty()).cloned())
    }

    pub fn from_vars(lookup: impl Fn(&str) -> Option<String>) -> Result<Config> {
        let get = |key: &str| lookup(&format!("REIWAND_{key}"));
        let text = |key: &str, default: &str| get(key).unwrap_or_else(|| default.into());
        let num = |key: &str, default: u64| -> Result<u64> {
            get(key).map_or(Ok(default), |v| {
                v.trim().parse().with_context(|| format!("REIWAND_{key}: not a number"))
            })
        };
        let flag = |key: &str, default: bool| -> Result<bool> {
            let value = get(key).map(|v| v.trim().to_ascii_lowercase());
            pick(
                key,
                value.as_deref(),
                &[("true", true), ("1", true), ("on", true), ("false", false), ("0", false), ("off", false)],
                default,
            )
        };
        let public_url = text("PUBLIC_URL", "http://localhost:8080").trim_end_matches('/').to_string();
        let scheme = public_url.starts_with("https://") || public_url.starts_with("http://");
        ensure!(scheme, "REIWAND_PUBLIC_URL must start with https:// or http://");
        let proxies = get("TRUSTED_PROXIES").unwrap_or_else(nearby);
        let entry_days = num("ENTRY_DAYS", 7)?;
        ensure!((1..=14).contains(&entry_days), "REIWAND_ENTRY_DAYS must be 1 to 14");
        Ok(Config {
            listen: text("LISTEN", "0.0.0.0:8080")
                .parse()
                .context("REIWAND_LISTEN: expected address:port")?,
            data: PathBuf::from(text("DATA", "/data")),
            trusted_proxies: proxies
                .split(',')
                .filter(|p| !p.trim().is_empty())
                .map(cidr)
                .collect::<Result<_>>()?,
            name: text("NAME", "reiwand"),
            registration: pick(
                "REGISTRATION",
                get("REGISTRATION").as_deref(),
                &[
                    ("closed", Registration::Closed),
                    ("invite", Registration::Invite),
                    ("open", Registration::Open),
                ],
                Registration::Invite,
            )?,
            auto_publish: pick(
                "AUTO_PUBLISH",
                get("AUTO_PUBLISH").as_deref(),
                &[("never", Publish::Never), ("trusted", Publish::Trusted), ("all", Publish::All)],
                Publish::Trusted,
            )?,
            licenses: text("LICENSES", "CC0-1.0,CC-BY-4.0,CC-BY-SA-4.0")
                .split(',')
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect(),
            package_max: num("PACKAGE_MAX_MIB", 256)? << 20,
            user_quota: num("USER_QUOTA_MIB", 2048)? << 20,
            uploads_per_day: num("UPLOADS_PER_DAY", 20)?,
            sync_quota: num("SYNC_QUOTA_MIB", 64)? << 20,
            files_quota: num("FILES_QUOTA_MIB", 1024)? << 20,
            entry_days: entry_days as u32,
            resign_hours: num("RESIGN_HOURS", 24)?.clamp(1, entry_days * 12),
            publish_delay: num("PUBLISH_DELAY_SECS", 30)?,
            operator: text("LEGAL_OPERATOR", ""),
            contact: text("LEGAL_CONTACT", ""),
            jurisdiction: text("LEGAL_JURISDICTION", ""),
            dmca_agent: text("LEGAL_DMCA_AGENT", ""),
            log_clients: pick(
                "LOG_CLIENTS",
                get("LOG_CLIENTS").as_deref(),
                &[("off", Clients::Off), ("prefix", Clients::Prefix), ("full", Clients::Full)],
                Clients::Off,
            )?,
            log_days: num("LOG_CLIENTS_DAYS", 7)?.clamp(1, 30),
            notice_days: num("NOTICE_DAYS", 180)?.clamp(1, 730),
            takedown_days: num("TAKEDOWN_DAYS", 90)?.clamp(1, 3650),
            rate_limits: flag("RATE_LIMITS", true)?,
            self_signed: pick("TLS", get("TLS").as_deref(), &[("off", false), ("self-signed", true)], false)?,
            advertise: flag("ADVERTISE", false)?,
            validator: std::env::current_exe().unwrap_or_else(|_| PathBuf::from("reiwand-server")),
            public_url,
        })
    }

    pub fn secure(&self) -> bool {
        self.public_url.starts_with("https://")
    }

    pub fn cookie(&self) -> &'static str {
        if self.secure() { "__Host-reiwand" } else { "reiwand" }
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{path}", self.public_url)
    }
}

fn nearby() -> String {
    let contained = ["/.dockerenv", "/run/.containerenv"].iter().any(|p| fs::exists(p).unwrap_or(false));
    near(contained, &fs::read_to_string("/proc/net/route").unwrap_or_default())
}

fn near(contained: bool, routes: &str) -> String {
    let gateway = routes.lines().skip(1).map(str::split_whitespace).find_map(|mut f| {
        let (_, to, via) = (f.next()?, f.next()?, f.next()?);
        let via = u32::from_str_radix(via, 16).ok().filter(|v| to == "00000000" && *v != 0)?;
        Some(std::net::Ipv4Addr::from(via.to_ne_bytes()))
    });
    match gateway.filter(|_| contained) {
        Some(ip) => format!("{LOOPBACK},{ip}"),
        None => LOOPBACK.to_string(),
    }
}

fn pick<T: Copy>(key: &str, value: Option<&str>, options: &[(&str, T)], default: T) -> Result<T> {
    let Some(value) = value.map(str::trim) else { return Ok(default) };
    let names: Vec<&str> = options.iter().map(|(name, _)| *name).collect();
    let found = options.iter().find(|(name, _)| *name == value).map(|(_, v)| *v);
    found.with_context(|| format!("REIWAND_{key} must be one of {}", names.join(", ")))
}

fn cidr(text: &str) -> Result<(IpAddr, u8)> {
    let (ip, bits) = text.trim().split_once('/').map_or((text.trim(), None), |(a, b)| (a, Some(b)));
    let ip: IpAddr =
        ip.parse().with_context(|| format!("REIWAND_TRUSTED_PROXIES: invalid address {text:?}"))?;
    let max = if ip.is_ipv4() { 32 } else { 128 };
    let bits = bits
        .map_or(Ok(max), str::parse)
        .with_context(|| format!("REIWAND_TRUSTED_PROXIES: invalid prefix {text:?}"))?;
    ensure!(bits <= max, "REIWAND_TRUSTED_PROXIES: prefix too long in {text:?}");
    Ok((ip, bits))
}

#[test]
fn only_loopback_and_the_container_gateway_are_proxies_by_default() {
    let routes = "Iface\tDestination\tGateway\tFlags\n\
                  eth0\t0000A8C0\t00000000\t0001\n\
                  eth0\t00000000\t0100A8C0\t0003\n";
    assert_eq!(near(true, routes), "127.0.0.0/8,::1,192.168.0.1");
    assert_eq!(near(false, routes), "127.0.0.0/8,::1");
    assert_eq!(near(true, "Iface\tDestination\tGateway\n"), "127.0.0.0/8,::1");
}
