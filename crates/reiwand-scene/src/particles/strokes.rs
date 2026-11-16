use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Trail {
    pub length: u32,
    pub width: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-range" = [0, 7]))]
    pub subdivide: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv_scale: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv_scroll: Option<f32>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub textured: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Stretch {
    #[serde(default)]
    pub time: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Rope {
    #[schemars(extend("x-range" = [0, 7]))]
    pub subdivide: u32,
    #[schemars(extend("x-unit" = "px"))]
    pub width: f32,
    pub uv_scale: f32,
    pub uv_scroll: f32,
    pub smooth: bool,
}

impl Stretch {
    pub(super) fn is_off(&self) -> bool {
        *self == Stretch::default()
    }
}

impl Default for Rope {
    fn default() -> Self {
        Rope { subdivide: 2, width: 4.0, uv_scale: 1.0, uv_scroll: 0.0, smooth: true }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Orient {
    Screen,
    Velocity,
    Fixed,
}
