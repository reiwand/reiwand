use reiwand_core::explicit::null;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Trail {
    pub length: u32,
    pub width: f32,
    #[serde(deserialize_with = "null")]
    #[schemars(required, extend("x-null" = true))]
    pub segments: Option<u32>,
    #[schemars(extend("x-range" = [0, 7]))]
    pub subdivide: u32,
    pub uv_scale: f32,
    pub uv_scroll: f32,
    pub textured: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StretchMode {
    Add,
    Relative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StretchDepth {
    Flat,
    Projected,
    Motion,
    Segment,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Stretch {
    pub mode: StretchMode,
    pub depth: StretchDepth,
    pub time: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Rope {
    #[schemars(extend("x-range" = [0, 7]))]
    pub subdivide: u32,
    #[schemars(extend("x-unit" = "px"))]
    pub width: f32,
    pub uv_scale: f32,
    pub uv_scroll: f32,
    pub smooth: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Orient {
    Screen,
    Velocity,
    Fixed,
}
