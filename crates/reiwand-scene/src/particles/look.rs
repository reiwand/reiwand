use super::{Orient, Rope, Stack, Stretch, Trail};
use crate::{Color, Value};
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
    #[schemars(extend("x-ref" = "image"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub texture: Option<Value<String>>,
    #[schemars(extend("x-range" = [0.05, 20]))]
    pub aspect: f32,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub frames: Option<Frames>,
    pub fps: f32,
    pub mix: bool,
    #[schemars(extend("x-curve" = true))]
    pub size_over_life: Vec<[f32; 2]>,
    pub color_over_life: Vec<(f32, Color)>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub stretch: Option<Stretch>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub trail: Option<Trail>,
    #[schemars(extend("x-range" = [0, 16]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub brightness: Option<Value<f32>>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub cutout: Option<[f32; 3]>,
    #[schemars(extend("x-range" = [1, 64]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub cycles: Option<u32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub order: Option<Stack>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub rope: Option<Rope>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub orient: Option<Orient>,
    #[schemars(extend("x-unit" = "°"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub angle: Option<f32>,
    #[schemars(extend("x-ref" = "image"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub normal: Option<String>,
    pub lit: bool,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub refract: Option<Refract>,
    #[schemars(extend("x-ref" = "material"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
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

impl Default for Render {
    fn default() -> Self {
        let (texture, frames, trail, size_over_life, color_over_life) = Default::default();
        let (brightness, cutout, cycles, rope, orient, angle, normal, refract, shader) = Default::default();
        Render {
            texture,
            aspect: 1.0,
            frames,
            fps: 12.0,
            mix: false,
            size_over_life,
            color_over_life,
            stretch: None,
            trail,
            brightness,
            cutout,
            cycles,
            order: None,
            rope,
            orient,
            angle,
            normal,
            lit: false,
            refract,
            shader,
        }
    }
}
