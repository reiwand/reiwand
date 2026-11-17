use super::Target;
use crate::Color;
use crate::value::{AudioKind, AudioUnit, Band, Binding, Reference};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_PALETTE: usize = 10;
pub const MAX_REMAPS: usize = 8;
pub const REMAP_INPUTS: [&str; 6] = ["daytime", "runtime", "life", "age", "speed", "random"];

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Hsv {
    #[schemars(extend("x-unit" = "°"))]
    pub hue: [f32; 2],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-range" = [1, 360]))]
    pub steps: Option<u32>,
    #[serde(default = "full")]
    pub sat: [f32; 2],
    #[serde(default = "full")]
    pub val: [f32; 2],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Palette {
    pub colors: Vec<Color>,
    #[serde(default)]
    pub noise: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Carry {
    pub from: Target,
    #[serde(default = "full")]
    pub speed: [f32; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Turbulent {
    #[schemars(extend("x-unit" = "°"))]
    pub forward: f32,
    #[schemars(extend("x-range" = [0, 1]))]
    pub scale: f32,
    pub noise_speed: f32,
    pub offset: f32,
    pub speed: [f32; 2],
    pub phase: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Band>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<AudioKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<AudioUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-unit" = "s", "x-range" = [0.05, 60]))]
    pub window: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<Reference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hold_silence: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct OffsetNoise {
    #[schemars(extend("x-unit" = "px"))]
    pub distance: f32,
    pub scale: f32,
    pub sign: [i8; 2],
    #[schemars(extend("x-range" = [1, 6]))]
    pub octaves: u32,
    pub noise_scale: f32,
    pub noise_speed: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RemapOutput {
    Size,
    Alpha,
    Color,
    Speed,
    Life,
    Rotation,
    Spin,
    Frame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RemapOp {
    #[default]
    Set,
    Mul,
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Wave {
    #[default]
    None,
    Sine,
    Noise,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Remap {
    #[schemars(extend("pattern" = "^(daytime|runtime|life|age|speed|random|distance:p[0-7]|audio:.+|cursor:[xy])$"))]
    pub input: String,
    #[serde(rename = "in")]
    pub from: [f32; 2],
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub clamp_in: bool,
    pub output: RemapOutput,
    #[serde(rename = "out")]
    pub to: [f32; 2],
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub clamp_out: bool,
    #[serde(default)]
    pub op: RemapOp,
    #[serde(default)]
    pub wave: Wave,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<AudioKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<AudioUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-unit" = "s", "x-range" = [0.05, 60]))]
    pub window: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<Reference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hold_silence: Option<bool>,
}

fn full() -> [f32; 2] {
    [1.0; 2]
}

impl Default for Turbulent {
    fn default() -> Self {
        Turbulent {
            forward: 0.0,
            scale: 1.0,
            noise_speed: 1.0,
            offset: 0.0,
            speed: [10.0, 40.0],
            phase: 0.0,
            audio: None,
            kind: None,
            unit: None,
            window: None,
            reference: None,
            hold_silence: None,
        }
    }
}

impl Default for OffsetNoise {
    fn default() -> Self {
        OffsetNoise {
            distance: 10.0,
            scale: 1.0,
            sign: [0; 2],
            octaves: 1,
            noise_scale: 1.0,
            noise_speed: 0.0,
        }
    }
}

fn listening(
    audio: Option<Band>,
    (kind, unit): (Option<AudioKind>, Option<AudioUnit>),
    (window, reference, hold_silence): (Option<f32>, Option<Reference>, Option<bool>),
) -> Binding<()> {
    Binding { audio, kind, unit, window, reference, hold_silence, ..Binding::default() }
}

impl Turbulent {
    pub fn binding(&self) -> Binding<()> {
        let kinds = (self.kind, self.unit);
        listening(self.audio, kinds, (self.window, self.reference, self.hold_silence))
    }
}

impl Remap {
    pub fn binding(&self, audio: Option<Band>) -> Binding<()> {
        listening(audio, (self.kind, self.unit), (self.window, self.reference, self.hold_silence))
    }
}
