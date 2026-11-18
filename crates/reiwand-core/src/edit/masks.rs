use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Mask {
    Linear { from: [f32; 2], to: [f32; 2], invert: bool },
    Radial { center: [f32; 2], radius: [f32; 2], feather: f32, invert: bool },
    Luminance { range: [f32; 2], soft: f32, invert: bool },
    Depth { range: [f32; 2], soft: f32, invert: bool },
    Brush { strokes: Vec<Stroke>, invert: bool },
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stroke {
    pub radius: f32,
    pub hardness: f32,
    pub erase: bool,
    pub points: Vec<[f32; 2]>,
}
