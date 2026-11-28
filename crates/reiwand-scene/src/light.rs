use crate::Color;
use crate::value::Value;
use reiwand_core::explicit::{block, null};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_LIGHTS: usize = 12;
pub const MAX_PROJECTIONS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Beam {
    #[default]
    Point,
    Spot,
    Tube,
    Directional,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Light {
    pub light: Beam,
    pub color: Value<Color>,
    #[schemars(extend("x-range" = [0, 100]))]
    pub intensity: Value<f32>,
    #[schemars(extend("x-unit" = "px", "x-range" = [1, 100000]))]
    pub radius: Value<f32>,
    #[schemars(extend("x-unit" = "px", "x-range" = [-100000, 100000]))]
    pub z: Value<f32>,
    #[schemars(extend("x-unit" = "°", "x-range" = [0, 180]))]
    pub cone: [f32; 2],
    #[schemars(extend("x-unit" = "px", "x-range" = [1, 100000]))]
    pub length: f32,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub volumetric: Option<Volumetric>,
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub texture: Option<Value<String>>,
    pub shadows: bool,
    #[schemars(extend("x-unit" = "px", "x-range" = [256, 4096]))]
    #[serde(deserialize_with = "null")]
    #[schemars(required, extend("x-null" = true))]
    pub shadow_resolution: Option<u32>,
    #[schemars(extend("x-range" = [0, 1]))]
    pub shadow_bias: f32,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub position3d: Option<Value<crate::Vec3>>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub aim3d: Option<Value<crate::Vec3>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Volumetric {
    #[schemars(extend("x-range" = [0, 1]))]
    pub density: f32,
    #[schemars(extend("x-range" = [4, 64]))]
    pub samples: u32,
    #[schemars(extend("x-range" = [0, 1]))]
    pub shafts: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Lighting {
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub normal: Option<String>,
    #[schemars(extend("x-range" = [0, 1]))]
    pub roughness: Value<f32>,
    #[schemars(extend("x-range" = [0, 1]))]
    pub reflection: Value<f32>,
    #[schemars(extend("x-range" = [0, 16]))]
    pub emissive: Value<f32>,
    #[schemars(extend("x-range" = [0, 1]))]
    pub strength: Value<f32>,
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub env: Option<String>,
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub roughness_map: Option<String>,
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub emissive_map: Option<String>,
}

impl Default for Volumetric {
    fn default() -> Self {
        Volumetric { density: 0.2, samples: 16, shafts: 0.0 }
    }
}

impl Default for Lighting {
    fn default() -> Self {
        let [roughness, reflection, emissive, strength] = [0.5, 0.0, 0.0, 1.0].map(Value::Fixed);
        let (normal, env, roughness_map, emissive_map) = Default::default();
        Lighting { normal, roughness, reflection, emissive, strength, env, roughness_map, emissive_map }
    }
}

impl crate::Quality {
    pub fn air(self, tier: crate::Tier) -> Option<(f32, f32)> {
        match self {
            crate::Quality::Q0 => None,
            crate::Quality::Q1 => Some((4.0, 0.5)),
            _ if tier <= crate::Tier::T1 => Some((4.0, 1.0)),
            _ => Some((2.0, 1.0)),
        }
    }

    pub fn bend(self) -> Option<f32> {
        match self {
            crate::Quality::Q3 => Some(1.0),
            crate::Quality::Q2 => Some(0.5),
            _ => None,
        }
    }
}
