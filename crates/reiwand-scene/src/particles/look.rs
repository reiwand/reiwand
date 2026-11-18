use super::{Orient, Rope, Stack, Stretch, Trail};
use crate::{Color, Value};
use reiwand_core::explicit::{block, null};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Frames {
    Time,
    Life,
    Random,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Render {
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    #[schemars(extend("x-ref" = "image"))]
    pub texture: Option<Value<String>>,
    #[schemars(extend("x-range" = [0.05, 20]))]
    pub aspect: f32,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub frames: Option<Frames>,
    pub fps: f32,
    pub mix: bool,
    #[schemars(extend("x-curve" = true))]
    pub size_over_life: Vec<[f32; 2]>,
    pub color_over_life: Vec<(f32, Color)>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub stretch: Option<Stretch>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub trail: Option<Trail>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    #[schemars(extend("x-range" = [0, 16]))]
    pub brightness: Option<Value<f32>>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub cutout: Option<[f32; 3]>,
    #[serde(deserialize_with = "null")]
    #[schemars(required, extend("x-null" = true))]
    #[schemars(extend("x-range" = [1, 64]))]
    pub cycles: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub order: Option<Stack>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub rope: Option<Rope>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub orient: Option<Orient>,
    #[serde(deserialize_with = "null")]
    #[schemars(required, extend("x-null" = true))]
    #[schemars(extend("x-unit" = "°"))]
    pub angle: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    #[schemars(extend("x-ref" = "image"))]
    pub normal: Option<String>,
    pub lit: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub refract: Option<Refract>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    #[schemars(extend("x-ref" = "material"))]
    pub shader: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "units", rename_all = "lowercase", deny_unknown_fields)]
pub enum Refract {
    Size {
        #[schemars(extend("x-ref" = "image"))]
        normal: String,
        #[schemars(extend("x-range" = [0, 1]))]
        amount: f32,
    },
    Pixels {
        #[schemars(extend("x-ref" = "image"))]
        normal: String,
        #[schemars(extend("x-range" = [0, 256]))]
        #[schemars(extend("x-unit" = "px"))]
        shift: f32,
    },
}

impl Refract {
    pub fn normal(&self) -> &str {
        match self {
            Self::Size { normal, .. } | Self::Pixels { normal, .. } => normal,
        }
    }

    pub fn by_pixels(&self) -> bool {
        matches!(self, Self::Pixels { .. })
    }

    pub fn strength(&self) -> f32 {
        match self {
            Self::Size { amount, .. } => amount.clamp(0.0, 1.0),
            Self::Pixels { shift, .. } => shift.clamp(0.0, 256.0),
        }
    }
}

impl Render {
    pub fn fill(&self) -> f32 {
        self.aspect.min(self.aspect.recip())
    }

    pub fn sides(&self) -> [f32; 2] {
        [self.aspect.min(1.0), self.aspect.recip().min(1.0)]
    }
}
