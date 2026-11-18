use crate::Color;
use crate::value::{AudioChannel, Param, Value};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Mask {
    #[schemars(extend("x-ref" = "image"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub texture: Option<Value<String>>,
    #[schemars(extend("x-ref" = "layer"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub layer: Option<String>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub shape: Option<Shape>,
    pub channel: MaskChannel,
    pub invert: bool,
    pub feather: f32,
    #[schemars(extend("x-range" = [0, 1], "default" = 1.0))]
    pub strength: Value<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MaskChannel {
    R,
    G,
    B,
    A,
    Luminance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EffectUse {
    #[schemars(extend("x-ref" = "effect"))]
    pub effect: String,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub name: Option<String>,
    pub enabled: Value<bool>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub mask: Option<Mask>,
    pub params: BTreeMap<String, Value<Param>>,
    pub optional: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Shape {
    Rect { size: [f32; 2], radius: f32 },
    Ellipse { size: [f32; 2] },
    Ring { size: [f32; 2], width: f32 },
    Polygon { sides: u32, size: f32 },
    Star { points: u32, size: f32, inner: f32 },
    Line { to: [f32; 2], width: f32 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Gradient {
    pub radial: bool,
    pub from: [f32; 2],
    pub to: [f32; 2],
    pub stops: Vec<(f32, Color)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Stroke {
    pub color: Value<Color>,
    pub width: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Spectrum {
    pub size: [f32; 2],
    pub style: SpectrumStyle,
    pub bands: u32,
    pub channel: AudioChannel,
    pub color: Value<Color>,
    pub gap: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SpectrumStyle {
    #[default]
    Bars,
    Circle,
    Wave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    Left,
    #[default]
    Center,
    Right,
}

pub(crate) fn shown() -> Value<bool> {
    Value::Fixed(true)
}

pub(crate) fn white() -> Value<Color> {
    Value::Fixed(Color([1.0; 4]))
}
