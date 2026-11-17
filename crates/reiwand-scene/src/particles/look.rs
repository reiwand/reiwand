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
#[serde(default, deny_unknown_fields)]
pub struct Render {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-ref" = "image"))]
    pub texture: Option<Value<String>>,
    #[schemars(extend("x-range" = [0.05, 20]))]
    pub aspect: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<Frames>,
    pub fps: f32,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub mix: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[schemars(extend("x-curve" = true))]
    pub size_over_life: Vec<[f32; 2]>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub color_over_life: Vec<(f32, Color)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stretch: Option<Stretch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail: Option<Trail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-range" = [0, 16]))]
    pub brightness: Option<Value<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutout: Option<[f32; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-range" = [1, 64]))]
    pub cycles: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Stack>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rope: Option<Rope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<Orient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-unit" = "°"))]
    pub angle: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-ref" = "image"))]
    pub normal: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub lit: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refract: Option<Refract>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-ref" = "material"))]
    pub shader: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Refract {
    #[schemars(extend("x-ref" = "image"))]
    pub normal: String,
    #[schemars(extend("x-range" = [0, 1]))]
    pub amount: f32,
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
