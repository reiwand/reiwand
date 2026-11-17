use crate::Color;
use crate::content::{shown, white};
use crate::value::{Value, Vec2};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Glide {
    #[schemars(extend("x-range" = [0, 4]))]
    pub amount: Value<f32>,
    #[schemars(extend("x-unit" = "s", "x-range" = [0, 2]))]
    pub delay: f32,
    #[schemars(extend("x-range" = [0, 1]))]
    pub pointer: Value<f32>,
    #[schemars(extend("x-range" = [-1, 1]))]
    pub offset: Value<Vec2>,
    pub gyro: bool,
}

impl Glide {
    pub fn bound(&self) -> bool {
        let fixed = |v: &Value<f32>| matches!(v, Value::Fixed(_));
        !fixed(&self.amount) || !fixed(&self.pointer) || !matches!(self.offset, Value::Fixed(_))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Camera {
    #[schemars(extend("x-unit" = "px"))]
    pub center: crate::Ctx<Value<Vec2>, { crate::CANVAS }>,
    #[schemars(extend("x-range" = [0.1, 10]))]
    pub zoom: Value<f32>,
    #[schemars(extend("x-unit" = "°", "x-range" = [-360, 360]))]
    pub rotation: Value<f32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub shake: Option<Shake>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Shake {
    pub enabled: Value<bool>,
    #[schemars(extend("x-unit" = "px", "x-range" = [0, 1000]))]
    pub amplitude: Value<f32>,
    #[schemars(extend("x-range" = [0, 1]))]
    pub roughness: Value<f32>,
    #[schemars(extend("x-unit" = "Hz", "x-range" = [0, 60]))]
    pub speed: Value<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Bloom {
    #[schemars(extend("x-range" = [0, 4]))]
    pub threshold: Value<f32>,
    #[schemars(extend("x-range" = [0, 8]))]
    pub strength: Value<f32>,
    #[schemars(extend("x-unit" = "px", "x-range" = [0, 128]))]
    pub radius: Value<f32>,
    pub tint: Value<Color>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub hdr: Option<Glare>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Glare {
    #[schemars(extend("x-range" = [0, 16]))]
    pub threshold: f32,
    #[schemars(extend("x-range" = [0, 8]))]
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Tonemap {
    #[default]
    None,
    Aces,
    Agx,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Playback {
    #[schemars(extend("x-range" = [0, 16]))]
    pub rate: Value<f32>,
    pub playing: Value<bool>,
    #[serde(rename = "loop")]
    pub repeat: bool,
    #[schemars(extend("x-unit" = "s"))]
    pub start: f32,
    #[schemars(extend("x-unit" = "s"))]
    pub end: crate::Ctx<f32, { crate::END }>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub frame: Option<Value<f32>>,
    pub own_clock: bool,
}

impl Default for Glide {
    fn default() -> Self {
        let offset = Value::Fixed(Vec2([0.0; 2]));
        Glide { amount: Value::Fixed(1.0), delay: 0.15, pointer: Value::Fixed(1.0), offset, gyro: false }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let (zoom, rotation) = (Value::Fixed(1.0), Value::Fixed(0.0));
        Camera { center: crate::Ctx::word(), zoom, rotation, shake: None }
    }
}

impl Default for Shake {
    fn default() -> Self {
        let [amplitude, roughness, speed] = [8.0, 0.5, 1.0].map(Value::Fixed);
        Shake { enabled: shown(), amplitude, roughness, speed }
    }
}

impl Default for Bloom {
    fn default() -> Self {
        let [threshold, strength, radius] = [0.8, 1.0, 16.0].map(Value::Fixed);
        Bloom { threshold, strength, radius, tint: white(), hdr: None }
    }
}

impl Default for Playback {
    fn default() -> Self {
        Playback {
            rate: Value::Fixed(1.0),
            playing: shown(),
            repeat: true,
            start: 0.0,
            end: crate::Ctx::word(),
            frame: None,
            own_clock: false,
        }
    }
}
