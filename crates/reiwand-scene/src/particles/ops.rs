use super::Force;
use reiwand_core::explicit::{read, untagged};
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::de::Error as _;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as Json};
use std::borrow::Cow;

pub const FULL_WINDOW: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    pub force: Force,
    pub window: [f32; 4],
}

#[derive(JsonSchema)]
#[allow(dead_code)]
struct Shown {
    #[serde(flatten)]
    force: Force,
    #[schemars(extend("x-range" = [0, 1]), required)]
    window: [f32; 4],
}

impl From<Force> for Operator {
    fn from(force: Force) -> Self {
        Operator { force, window: FULL_WINDOW }
    }
}

impl std::ops::Deref for Operator {
    type Target = Force;
    fn deref(&self) -> &Force {
        &self.force
    }
}

#[derive(Serialize)]
struct Written<'a> {
    #[serde(flatten)]
    force: &'a Force,
    window: [f32; 4],
}

impl Serialize for Operator {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        Written { force: &self.force, window: self.window }.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Operator {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let mut map = Map::deserialize(d)?;
        let window = map.remove("window").ok_or_else(|| D::Error::missing_field("window"))?;
        let window = serde_json::from_value(window).map_err(D::Error::custom)?;
        let force = untagged(Json::Object(map), &[(&[], &|v| read("", v))])
            .map_err(|e| D::Error::custom(e.to_string()))?;
        Ok(Operator { force, window })
    }
}

impl JsonSchema for Operator {
    fn schema_name() -> Cow<'static, str> {
        "Operator".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        Shown::json_schema(generator)
    }
}
