use super::Target;
use reiwand_core::explicit::block;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Ring {
    #[schemars(extend("x-unit" = "px"))]
    pub radius: f32,
    #[schemars(extend("x-unit" = "px"))]
    pub width: f32,
    pub pull: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Wobble {
    Position,
    Alpha,
    Size,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Base {
    Top,
    Floor,
}

/// How an oscillation draws its frequency, scale and phase for each particle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OscillateRandom {
    /// One random number each (what an oscillation without `random` does).
    Separate,
    /// One random number for the three, so a faster particle also swings wider and later.
    Shared,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Oscillate {
    pub what: Wobble,
    pub from: Base,
    #[schemars(extend("x-unit" = "Hz"))]
    pub freq: [f32; 2],
    pub scale: [f32; 2],
    pub phase: [f32; 2],
    pub mask: [f32; 2],
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub random: Option<OscillateRandom>,
}

impl Oscillate {
    pub fn shared(&self) -> bool {
        self.random == Some(OscillateRandom::Shared)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SlowNear {
    pub target: Target,
    #[schemars(extend("x-unit" = "px"))]
    pub inner: f32,
    #[schemars(extend("x-unit" = "px"))]
    pub outer: f32,
    #[schemars(extend("x-range" = [0, 1]))]
    pub factor: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct KeepDistance {
    pub target: Target,
    #[schemars(extend("x-unit" = "px"))]
    pub distance: f32,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct KeepBetween {
    pub from: Target,
    pub to: Target,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NoiseField {
    pub strength: Interval,
    pub phase: Interval,
    pub scale: f32,
    #[schemars(extend("x-unit" = "px/s"))]
    pub drift: f32,
    #[schemars(extend("x-range" = [0, 1]))]
    pub mask: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Boids {
    #[schemars(extend("x-unit" = "px"))]
    pub radius: f32,
    pub separation: f32,
    pub alignment: f32,
    pub cohesion: f32,
    #[schemars(extend("x-unit" = "px/s"))]
    pub max_speed: f32,
}
