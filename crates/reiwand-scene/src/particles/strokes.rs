use reiwand_core::explicit::{block, null};
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

/// Which point of a stretched sprite sits on its particle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StretchAnchor {
    /// The front stays on the particle and the stroke trails behind (what a stretch without
    /// `anchor` does).
    Tip,
    /// The middle of the stroke sits on the particle, half ahead of it and half behind.
    Centre,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Stretch {
    pub mode: StretchMode,
    pub depth: StretchDepth,
    pub time: f32,
    pub min: f32,
    pub max: f32,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub anchor: Option<StretchAnchor>,
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
