use anyhow::{Context, bail};
use rusqlite::Connection;

const REPO_NAME: &str = "Your wallpaper server";

fn chosen(key: &str, allowed: &[&str], default: &str) -> anyhow::Result<String> {
    let name = format!("REIWAND_{key}");
    match std::env::var(&name) {
        Err(_) => Ok(default.to_owned()),
        Ok(value) => {
            let value = value.trim().to_owned();
            if !allowed.contains(&value.as_str()) {
                bail!("{name} is \"{value}\"; expected one of {}", allowed.join(", "));
            }
            Ok(value)
        }
    }
}

pub fn complete(conn: &Connection) -> anyhow::Result<()> {
    let done: Option<String> =
        conn.query_row("SELECT value FROM settings WHERE key = 'setup'", [], |r| r.get(0)).ok();
    if done.as_deref() != Some("done") {
        return Ok(());
    }
    let network =
        if chosen("TLS", &["off", "self-signed"], "off")? == "self-signed" { "home" } else { "own" };
    let admin: i64 = conn
        .query_row("SELECT id FROM users WHERE role = 'admin' ORDER BY id LIMIT 1", [], |r| r.get(0))
        .context("a finished server has no administrator")?;
    let pairs = [
        ("registration", chosen("REGISTRATION", &["closed", "invite", "open"], "invite")?),
        ("auto_publish", chosen("AUTO_PUBLISH", &["never", "trusted", "all"], "trusted")?),
        ("network", network.to_owned()),
        ("cert_path", String::new()),
        ("key_path", String::new()),
        ("repo_name", REPO_NAME.to_owned()),
        ("admin", admin.to_string()),
    ];
    for (key, value) in pairs {
        conn.execute("INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)", [key, &value])?;
    }
    Ok(())
}
