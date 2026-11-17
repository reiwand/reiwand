use crate::look::{Bloom, Camera, Glide, Tonemap};
use crate::value::{Value, Vec2};
use crate::{Color, schema};
use reiwand_core::Fit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BlendSpace {
    #[default]
    Linear,
    Srgb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LayerSpace {
    #[default]
    Canvas,
    Linear,
    Srgb,
}

impl LayerSpace {
    pub fn apart_from(self, canvas: BlendSpace) -> Option<BlendSpace> {
        match (self, canvas) {
            (LayerSpace::Linear, BlendSpace::Srgb) => Some(BlendSpace::Linear),
            (LayerSpace::Srgb, BlendSpace::Linear) => Some(BlendSpace::Srgb),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Canvas {
    pub size: [u32; 2],
    #[schemars(schema_with = "schema::fit")]
    pub fit: Fit,
    pub background: Color,
    pub blend_space: BlendSpace,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub parallax: Option<Glide>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub camera: Option<Camera>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub bloom: Option<Bloom>,
    pub hdr: bool,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub tonemap: Option<Tonemap>,
    pub clear: bool,
    #[schemars(extend("x-unit" = "px", "x-range" = [100, 100000]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub perspective: Option<f32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub ambient: Option<Color>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub sky: Option<Color>,
    #[schemars(extend("x-unit" = "px/s"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub wind: Option<Value<Vec2>>,
    #[schemars(extend("x-range" = [0, 1]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub gusts: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Clock {
    #[serde(with = "reiwand_core::settings::fps")]
    #[schemars(schema_with = "schema::fps")]
    pub max_fps: u32,
    #[schemars(extend("x-unit" = "s"))]
    pub poster_time: f32,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub seed: Option<u64>,
    #[schemars(extend("x-unit" = "s", "x-range" = [1, 3600]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub step: Option<f32>,
}

impl Default for Clock {
    fn default() -> Self {
        Clock { max_fps: 60, poster_time: 0.0, seed: Some(0), step: None }
    }
}

impl Canvas {
    pub fn bright(&self) -> bool {
        self.hdr && self.tonemap.is_some_and(|t| t != Tonemap::None)
    }

    pub fn new(size: [u32; 2]) -> Canvas {
        let (fit, background, blend_space) = (Fit::Fill, Color::default(), BlendSpace::Linear);
        let (parallax, camera, bloom, hdr, tonemap, perspective, ambient, sky, wind, gusts) =
            Default::default();
        Canvas {
            size,
            fit,
            background,
            blend_space,
            parallax,
            camera,
            bloom,
            hdr,
            tonemap,
            clear: true,
            perspective,
            ambient,
            sky,
            wind,
            gusts,
        }
    }
}
