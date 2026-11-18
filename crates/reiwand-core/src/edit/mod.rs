mod action;
mod check;
mod curve;
mod depth;
mod fields;
mod file;
mod geometry;
mod history;
mod masks;
mod parse;
mod perform;
mod regions;
mod set;
mod tool;
mod touch;
pub mod travel;
mod whole;

use crate::Fit;
pub use action::{ACTIONS, Action, Error};
pub use curve::spline;
pub use depth::{Depth, MAX_STRENGTH, MODEL, Motion, Parallax, STEPS};
pub use file::{MAX_BYTES, adopt, commit, dir, load, missing, remove, save};
pub use geometry::{Matrix, mul};
pub use history::History;
pub use masks::{Mask, Stroke};
pub use parse::numbers;
pub use regions::{Join, MOST_JOINS, MOST_MOVES, MOST_SPLITS, MOST_TOGETHER, SPLIT_RADIUS, Shift};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub use touch::{
    FALLOFF, GAP, MOST_CUTS, MOST_POINTS, MOST_STROKE, MOST_STROKES, RADIUS, STRENGTH, Touch, points,
};

pub const VERSION: u32 = 3;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edit {
    pub version: u32,
    pub source: Source,
    pub geometry: Geometry,
    pub ops: Vec<Op>,
    pub masks: BTreeMap<String, Mask>,
    pub outputs: BTreeMap<String, OutputCrop>,
    #[cfg_attr(feature = "schema", schemars(required))]
    #[serde(deserialize_with = "crate::explicit::null")]
    pub span: Option<Span>,
    #[cfg_attr(feature = "schema", schemars(required))]
    #[serde(deserialize_with = "crate::explicit::null")]
    pub depth: Option<Depth>,
    #[cfg_attr(feature = "schema", schemars(required))]
    #[serde(deserialize_with = "crate::explicit::null")]
    pub parallax: Option<Parallax>,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::general")]
    pub fit: Option<Fit>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub name: String,
    pub bytes: u64,
    pub sha256: String,
    pub pixels: [u32; 2],
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Geometry {
    #[cfg_attr(feature = "schema", schemars(required))]
    #[serde(deserialize_with = "crate::explicit::null")]
    pub crop: Option<[f32; 4]>,
    pub rotate: u16,
    pub straighten: f32,
    pub flip: [bool; 2],
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Op {
    pub id: String,
    pub enabled: bool,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "crate::explicit::empty")]
    pub mask: Option<String>,
    #[cfg_attr(feature = "schema", schemars(required))]
    #[serde(deserialize_with = "crate::explicit::null")]
    pub opacity: Option<f32>,
    #[serde(flatten)]
    pub kind: Kind,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Kind {
    Exposure { ev: f32 },
    WhiteBalance { temperature: f32, tint: f32 },
    Contrast { amount: f32 },
    Saturation { saturation: f32, vibrance: f32 },
    Curves { rgb: Vec<[f32; 2]>, r: Vec<[f32; 2]>, g: Vec<[f32; 2]>, b: Vec<[f32; 2]> },
    Blur { radius: f32 },
    Overlay { color: String, blend: Blend },
    Vignette { amount: f32, midpoint: f32, roundness: f32, feather: f32 },
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Blend {
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    Color,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OutputCrop {
    pub connector: String,
    pub crop: [f32; 4],
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub center: [f32; 2],
    pub zoom: f32,
}

impl Default for Span {
    fn default() -> Self {
        Self { center: [0.5, 0.5], zoom: 1.0 }
    }
}

impl Default for Edit {
    fn default() -> Self {
        let (source, geometry, ops) = (Source::default(), Geometry::default(), Vec::new());
        let (masks, outputs, span, depth, parallax) = (BTreeMap::new(), BTreeMap::new(), None, None, None);
        Self { version: VERSION, source, geometry, ops, masks, outputs, span, depth, parallax, fit: None }
    }
}
