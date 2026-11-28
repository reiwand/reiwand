use crate::manifest::{Checked, RESERVED, fault, text};
use reiwand_core::explicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Setting {
    pub key: String,
    pub label: String,
    pub kind: Kind,
    #[schemars(extend("type" = ["string", "integer", "boolean"]))]
    pub prefill: Value,
    #[serde(deserialize_with = "explicit::null")]
    #[schemars(required)]
    pub min: Option<i64>,
    #[serde(deserialize_with = "explicit::null")]
    #[schemars(required)]
    pub max: Option<i64>,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Text,
    Int,
    Bool,
    Choice,
}

impl Setting {
    pub(crate) fn check(&self, repeated: bool) -> Checked {
        let word = self.key.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        let starts = self.key.bytes().next().is_some_and(|b| b.is_ascii_lowercase());
        if !word || !starts || self.key.len() > 32 || repeated || RESERVED.contains(&self.key.as_str()) {
            return fault("key", "a unique lowercase word that Reiwand does not use itself");
        }
        if !text(&self.label, 64) {
            return fault("label", "a short text");
        }
        let bounded = self.min.is_some() || self.max.is_some();
        match self.kind {
            Kind::Int => {}
            _ if bounded => return fault("min", "null: only an int has bounds"),
            _ => {}
        }
        if self.kind != Kind::Choice && !self.values.is_empty() {
            return fault("values", "[]: only a choice has values");
        }
        self.value(&self.prefill).map_err(|why| ("prefill".into(), why))
    }

    pub fn value(&self, value: &Value) -> Result<(), String> {
        let ok = match (self.kind, value) {
            (Kind::Text, Value::String(t)) => t.chars().count() <= 256 && !t.chars().any(char::is_control),
            (Kind::Bool, Value::Bool(_)) => true,
            (Kind::Int, Value::Number(n)) => n
                .as_i64()
                .is_some_and(|n| self.min.is_none_or(|min| n >= min) && self.max.is_none_or(|max| n <= max)),
            (Kind::Choice, Value::String(t)) => self.values.contains(t),
            _ => false,
        };
        let hint = match self.kind {
            Kind::Text => "a text of at most 256 characters".into(),
            Kind::Bool => "true or false".into(),
            Kind::Int => format!("a whole number from {:?} to {:?}", self.min, self.max),
            Kind::Choice => format!("one of {}", self.values.join(", ")),
        };
        if ok { Ok(()) } else { Err(format!("expected {hint}")) }
    }
}
