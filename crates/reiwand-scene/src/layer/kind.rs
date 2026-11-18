use super::Layer;
use crate::content::{Gradient, Shape, Spectrum, Stroke};
use crate::figure::{MeshLayer, ModelLayer, Space};
use crate::light::Light;
use crate::look::Playback;
use crate::particles::{Instance, Render};
use crate::typeset::TextLayer;
use crate::value::Value;
use crate::{Color, Sound};
use reiwand_core::explicit::block;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Solid {
    Flag(bool),
    Hit {
        #[schemars(extend("x-range" = [0, 1]))]
        alpha: f32,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Kind {
    Group {
        layers: Vec<Layer>,
        clip: bool,
        color: Value<Color>,
        space: Space,
    },
    Image {
        #[schemars(extend("x-ref" = "image"))]
        image: Value<String>,
        size: crate::Ctx<[f32; 2], { crate::SOURCE }>,
        color: Value<Color>,
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
        #[schemars(extend("x-null" = true, "x-block" = true))]
        playback: Option<Playback>,
    },
    Video {
        #[schemars(extend("x-ref" = "video"))]
        video: Value<String>,
        size: crate::Ctx<[f32; 2], { crate::SOURCE }>,
        color: Value<Color>,
        playback: Playback,
    },
    Text(Box<TextLayer>),
    Shape {
        shape: Shape,
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
        #[schemars(extend("x-null" = true, "x-block" = true))]
        fill: Option<Value<Color>>,
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
        #[schemars(extend("x-null" = true, "x-block" = true))]
        gradient: Option<Gradient>,
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
        #[schemars(extend("x-null" = true, "x-block" = true))]
        stroke: Option<Stroke>,
    },
    Particles {
        #[schemars(extend("x-ref" = "system"))]
        system: String,
        color: Value<Color>,
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
        #[schemars(extend("x-null" = true, "x-block" = true))]
        instance: Option<Box<Instance>>,
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
        #[schemars(extend("x-null" = true, "x-block" = true))]
        render: Option<Box<Render>>,
    },
    Spectrum(Spectrum),
    Sound(Sound),
    Solid {
        color: Value<Color>,
        size: crate::Ctx<[f32; 2], { crate::CANVAS }>,
    },
    Light(Box<Light>),
    Adjustment {
        #[schemars(extend("x-unit" = "px"))]
        size: crate::Ctx<[f32; 2], { crate::CANVAS }>,
        redraw: bool,
        rgb: bool,
    },
    Mesh(Box<MeshLayer>),
    Model(Box<ModelLayer>),
}

impl Default for Solid {
    fn default() -> Self {
        Solid::Flag(false)
    }
}

impl Solid {
    pub fn is_off(&self) -> bool {
        *self == Solid::Flag(false)
    }
}
