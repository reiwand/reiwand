use reiwand_client::{Error, explain};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::sync::LazyLock;

pub const INVALID: &str = "org.freedesktop.DBus.Error.InvalidArgs";
pub const FAILED: &str = "org.freedesktop.DBus.Error.Failed";
static FIXTURES: LazyLock<Value> =
    LazyLock::new(|| serde_json::from_str(include_str!("fixtures.json")).unwrap());

pub fn invalid(message: impl Into<String>) -> Error {
    Error { name: INVALID.into(), message: message.into() }
}

pub fn fail(error: zbus::Error) -> Error {
    explain(error)
        .downcast::<Error>()
        .unwrap_or_else(|e| Error { name: FAILED.into(), message: e.to_string() })
}

pub fn carries_secret(iface: &str, member: &str, args: &[Value]) -> bool {
    let at = |n: usize| args.get(n).and_then(Value::as_str).is_some_and(|s| !s.is_empty());
    let provider = iface.ends_with("Providers1") && (member == "SetSecret" || member == "Add" && at(3));
    let approval = ["ApproveDevice", "RevokeDevice"].contains(&member) && at(1);
    let carry = iface.ends_with("Carry1") && ["Export", "PreviewImport", "ImportChosen"].contains(&member);
    provider
        || carry
        || iface.ends_with("Sync1") && (["SetEscrow", "Restore"].contains(&member) && at(0) || approval)
}

pub const MAX_PAGE: u64 = 500;

pub fn fixture(iface: &str, member: &str) -> String {
    FIXTURES.get(format!("{iface}.{member}")).unwrap_or(&json!([])).to_string()
}

pub fn next<T: DeserializeOwned>(args: &mut impl Iterator<Item = Value>, name: &str) -> Result<T, Error> {
    serde_json::from_value(args.next().unwrap_or(Value::Null)).map_err(|e| invalid(format!("{name}: {e}")))
}

pub trait Out {
    fn out(self) -> Value;
}

macro_rules! out {
    ($($t:ty),*) => {$(impl Out for $t { fn out(self) -> Value { json!([self]) } })*};
}

out!(String, u64);

impl Out for () {
    fn out(self) -> Value {
        json!([])
    }
}
