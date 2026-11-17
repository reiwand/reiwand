use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_CHILDREN: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Moment {
    Static,
    Spawn,
    Death,
    Follow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Inheritable {
    Color,
    Size,
    Alpha,
    Velocity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Child {
    #[schemars(extend("x-ref" = "system"))]
    pub system: String,
    pub on: Moment,
    #[schemars(extend("x-range" = [0, 1]))]
    pub probability: f32,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub max: Option<u32>,
    #[schemars(extend("x-unit" = "px"))]
    pub offset: [f32; 2],
    #[schemars(extend("x-unit" = "°"))]
    pub angle: f32,
    #[schemars(extend("x-range" = [0, 100]))]
    pub scale: f32,
    pub points_from: bool,
    pub inherit: Vec<Inheritable>,
    pub follow_inherit: bool,
}
