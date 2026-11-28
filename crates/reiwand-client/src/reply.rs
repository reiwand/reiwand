use anyhow::{Context, ensure};
use reiwand_core::dbus::BUS_NAME;
use reiwand_core::model::MODEL;
use serde::de::DeserializeOwned;
use zbus::DBusError;

const FDO: &str = "org.freedesktop.DBus.Error.";
const MATCH: &str = "install matching versions of Reiwand";

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    pub name: String,
    pub message: String,
}

pub fn parse<T: DeserializeOwned>(json: &str) -> anyhow::Result<T> {
    let reply: serde_json::Value = serde_json::from_str(json).context("reiwandd sent invalid JSON")?;
    check(reply["model"].as_u64().unwrap_or(0))?;
    serde_json::from_value(reply).context("reiwandd sent a reply this client cannot read")
}

pub(crate) fn check(model: u64) -> anyhow::Result<()> {
    ensure!(
        model == u64::from(MODEL),
        "reiwandd uses data model {model} but this client uses {MODEL}; {MATCH}"
    );
    Ok(())
}

pub fn explain(error: zbus::Error) -> anyhow::Error {
    let (name, message) = match error {
        zbus::Error::FDO(e) => match *e {
            zbus::fdo::Error::ZBus(e) => return explain(e),
            e => (e.name().to_string(), e.description().unwrap_or_default().to_owned()),
        },
        zbus::Error::MethodError(name, message, _) => (name.to_string(), message.unwrap_or_default()),
        error => return error.into(),
    };
    let message = match name.strip_prefix(FDO) {
        Some("ServiceUnknown" | "NameHasNoOwner") => {
            format!("reiwandd is not running ({BUS_NAME} is not on the session bus)")
        }
        Some("NoReply" | "Disconnected") => {
            format!("reiwandd did not answer ({message}); it may be busy, stuck or have stopped")
        }
        _ if older(&name) => format!("reiwandd is older than this client; {MATCH}"),
        _ => reiwand_core::dbus::error::reason(&name, &message),
    };
    Error { name, message }.into()
}

pub fn error_name(error: &anyhow::Error) -> Option<&str> {
    error.downcast_ref::<Error>().map(|e| e.name.as_str()).filter(|name| !name.is_empty())
}

pub(crate) fn older(name: &str) -> bool {
    matches!(name.strip_prefix(FDO), Some("UnknownMethod" | "UnknownInterface" | "UnknownProperty"))
}
