use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as Json;
use std::borrow::Cow;

pub const CANVAS: u8 = 0;
pub const SOURCE: u8 = 1;
pub const END: u8 = 2;
pub const ANCHOR: u8 = 3;

const WORDS: [&str; 4] = ["canvas", "source", "end", "anchor"];

pub fn word(which: u8) -> &'static str {
    WORDS[which as usize]
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ctx<T, const W: u8>(pub Option<T>);

impl<T, const W: u8> Ctx<T, W> {
    pub fn word() -> Self {
        Ctx(None)
    }

    pub fn is(value: T) -> Self {
        Ctx(Some(value))
    }

    pub fn or(self, context: T) -> T {
        match self.0 {
            Some(own) => own,
            None => context,
        }
    }

    pub fn or_else(self, context: impl FnOnce() -> T) -> T {
        match self.0 {
            Some(own) => own,
            None => context(),
        }
    }

    pub fn as_ref(&self) -> Option<&T> {
        self.0.as_ref()
    }

    pub fn iter(&self) -> std::option::Iter<'_, T> {
        self.0.iter()
    }

    pub fn follows(&self) -> bool {
        self.0.is_none()
    }
}

impl<T, const W: u8> Default for Ctx<T, W> {
    fn default() -> Self {
        Ctx(None)
    }
}

impl<T: Serialize, const W: u8> Serialize for Ctx<T, W> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match &self.0 {
            Some(own) => own.serialize(s),
            None => s.serialize_str(word(W)),
        }
    }
}

impl<'de, T: serde::de::DeserializeOwned, const W: u8> Deserialize<'de> for Ctx<T, W> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Json::deserialize(d)?;
        if value.as_str() == Some(word(W)) {
            return Ok(Ctx(None));
        }
        let own = serde_json::from_value::<T>(value)
            .map_err(|e| D::Error::custom(format!("{e}, or \"{}\"", word(W))))?;
        Ok(Ctx(Some(own)))
    }
}

impl<T: JsonSchema, const W: u8> JsonSchema for Ctx<T, W> {
    fn schema_name() -> Cow<'static, str> {
        format!("{}_or_{}", T::schema_name(), word(W)).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let own = generator.subschema_for::<T>();
        json_schema!({"anyOf": [own, {"type": "string", "enum": [word(W)]}], "x-follows": word(W)})
    }
}
