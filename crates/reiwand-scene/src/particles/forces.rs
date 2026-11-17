use super::seed::Remap;
use super::{Boids, Follow, Handle, KeepBetween, KeepDistance, Oscillate, Ring, SlowNear};
use crate::value::{Value, Vec2};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Target {
    Point([f32; 2]),
    Follow(Follow),
    Control(Handle),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Falloff {
    Constant,
    Linear,
    InverseSquare,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Force {
    Gravity(Value<Vec2>),
    Drag(Value<f32>),
    Turbulence {
        strength: Value<f32>,
        scale: f32,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        speed: Option<f32>,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        time_scale: Option<f32>,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        mask: Option<[f32; 2]>,
    },
    Attractor {
        target: Target,
        strength: Value<f32>,
        #[schemars(extend("x-unit" = "px"))]
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        radius: Option<f32>,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        falloff: Option<Falloff>,
    },
    Vortex {
        center: Target,
        strength: Value<f32>,
        #[schemars(extend("x-unit" = "px"))]
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        radius: Option<f32>,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        falloff: Option<Falloff>,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        inner: Option<f32>,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        outer: Option<f32>,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        ring: Option<Ring>,
        axis: [f32; 3],
    },
    Orbit {
        center: Target,
        strength: Value<f32>,
        #[schemars(extend("x-unit" = "px"))]
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        radius: Option<f32>,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        falloff: Option<Falloff>,
    },
    Noise(Value<f32>),
    Oscillate(Oscillate),
    Cap(Value<f32>),
    Torque(Value<f32>),
    SpinDrag(Value<f32>),
    SlowNear(SlowNear),
    KeepDistance(KeepDistance),
    KeepBetween(KeepBetween),
    Boids(Boids),
    Remap(Remap),
}
