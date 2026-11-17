use super::{Num, Target};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Sweep {
    #[default]
    Repeat,
    Mirror,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Around {
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub at: Option<Target>,
    #[schemars(extend("x-range" = [1, 4096]))]
    pub count: u32,
    #[schemars(extend("x-unit" = "°"))]
    pub arc: Num,
    #[schemars(extend("x-unit" = "px"))]
    pub radius: Num,
    pub axis: [f32; 3],
    pub speed: Num,
    pub mode: Sweep,
    pub restart: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Bow {
    pub amount: f32,
    pub side: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Taper {
    #[schemars(extend("x-range" = [0, 1]))]
    pub position: f32,
    pub speed: f32,
    #[schemars(extend("x-range" = [0, 1]))]
    pub size: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Between {
    pub from: Target,
    pub to: Target,
    #[schemars(extend("x-range" = [1, 4096]))]
    pub count: u32,
    #[schemars(extend("x-range" = [0, 1]))]
    pub span: Num,
    pub mode: Sweep,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub arc: Option<Bow>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub taper: Option<Taper>,
}

impl Default for Taper {
    fn default() -> Self {
        Taper { position: 0.5, speed: 0.0, size: 0.5 }
    }
}
