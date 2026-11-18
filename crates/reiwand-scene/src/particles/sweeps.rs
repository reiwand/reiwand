use super::{Num, Target};
use reiwand_core::explicit::block;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Sweep {
    Repeat,
    Mirror,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Around {
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
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
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub arc: Option<Bow>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub taper: Option<Taper>,
}
