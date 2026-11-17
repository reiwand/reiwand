use super::seed::{Carry, Hsv, OffsetNoise, Palette, Remap, Turbulent};
use super::{EmitShape, Periodic, Spread, Tilt};
use crate::Color;
use crate::value::Value;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_STOPS: usize = 8;
pub const MAX_BURSTS: usize = 8;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Emitter {
    pub shape: EmitShape,
    pub rate: Value<f32>,
    pub bursts: Vec<Burst>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub follow: Option<Follow>,
    pub inherit: f32,
    #[schemars(extend("x-unit" = "s"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub duration: Option<f32>,
    #[schemars(extend("x-unit" = "s"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub delay: Option<f32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub periodic: Option<Periodic>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub max_per_step: Option<u32>,
    pub audio_gate: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Follow {
    Cursor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cue {
    Beat,
    Click,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Burst {
    pub count: u32,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub every: Option<f32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub on: Option<Cue>,
    #[schemars(extend("x-unit" = "s"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub at: Option<f32>,
}

impl Burst {
    pub fn immediate(&self) -> bool {
        self.on.is_none() && self.every.is_none() && self.at.is_none_or(|a| a <= 0.0)
    }

    pub fn timed(&self) -> bool {
        self.on.is_none() && !self.immediate()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Init {
    pub life: Spread<f32>,
    pub speed: Spread<f32>,
    pub angle: Spread<f32>,
    pub size: Spread<f32>,
    pub rotation: Spread<f32>,
    pub spin: Spread<f32>,
    pub frame: Spread<f32>,
    pub color: Spread<Color>,
    #[schemars(extend("x-range" = [0, 1]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub alpha: Option<Spread<f32>>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub hsv: Option<Hsv>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub palette: Option<Palette>,
    pub gradient: Vec<(f32, Color)>,
    #[schemars(extend("x-unit" = "px/s"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub velocity: Option<Spread<[f32; 2]>>,
    #[schemars(extend("x-unit" = "px/s"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub radial: Option<Spread<f32>>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub inherit: Option<Carry>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub turbulent: Option<Turbulent>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub offset_noise: Option<OffsetNoise>,
    #[schemars(extend("x-unit" = "px"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub depth: Option<Spread<f32>>,
    #[schemars(extend("x-unit" = "px/s"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub depth_speed: Option<Spread<f32>>,
    pub tilt: Tilt,
    pub remap: Vec<Remap>,
}

impl Default for Init {
    fn default() -> Self {
        let white = Color([1.0; 4]);
        let [rotation, spin, frame] = [[0.0; 2]; 3].map(Spread::from);
        let [life, speed, angle, size] =
            [[1.0, 2.0], [10.0, 40.0], [0.0, 360.0], [2.0, 6.0]].map(Spread::from);
        let (alpha, hsv, palette, velocity, radial, inherit, turbulent, offset_noise) = Default::default();
        let (depth, depth_speed, tilt, remap) = Default::default();
        Init {
            life,
            speed,
            angle,
            size,
            rotation,
            spin,
            frame,
            color: Spread::from([white, white]),
            alpha,
            hsv,
            palette,
            gradient: Vec::new(),
            velocity,
            radial,
            inherit,
            turbulent,
            offset_noise,
            depth,
            depth_speed,
            tilt,
            remap,
        }
    }
}
