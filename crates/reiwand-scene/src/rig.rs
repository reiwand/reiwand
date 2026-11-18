use crate::timeline::{Event, Loop};
use crate::value::{Value, Vec2};
use reiwand_core::explicit::{block, null};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod colliders;
mod tracks;

pub use colliders::{BoneCollider, Hull, LOW_BONE_COLLIDERS, MAX_BONE_COLLIDERS};
pub use tracks::BoneKeys;

pub const MAX_CLIPS_PER_MODEL: usize = 64;
pub const MAX_ANIMATION_LAYERS: usize = 16;
pub const MAX_CHAINS: usize = 16;
pub const MAX_CHAIN: usize = 8;
pub const MAX_BODIES: usize = 64;
pub const MAX_EXPRESSIONS: usize = 64;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Rig {
    pub clips: Vec<AnimClip>,
    pub animations: Vec<AnimLayer>,
    pub ik: Vec<IkChain>,
    pub physics: Vec<BonePhysics>,
    pub colliders: Vec<BoneCollider>,
    pub rules: Vec<BoneRule>,
    #[schemars(extend("x-ref" = "shape"))]
    pub shapes: BTreeMap<String, Value<f32>>,
    pub expressions: BTreeMap<String, BTreeMap<String, f32>>,
    #[schemars(extend("x-ref" = "bone"))]
    pub hit: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AnimClip {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub animation: Option<String>,
    #[schemars(extend("x-unit" = "s"))]
    pub from: f32,
    #[schemars(extend("x-unit" = "s"))]
    #[serde(deserialize_with = "null")]
    #[schemars(required, extend("x-null" = true))]
    pub to: Option<f32>,
    #[serde(rename = "loop")]
    pub repeat: Loop,
    pub events: Vec<Event>,
    #[schemars(extend("x-ref" = "bone"))]
    pub bones: BTreeMap<String, BoneKeys>,
    pub shapes: BTreeMap<String, Vec<crate::Key<f32>>>,
    pub order: BTreeMap<String, Vec<crate::Key<f32>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AnimLayer {
    #[schemars(extend("x-ref" = "clip"))]
    pub clip: String,
    #[schemars(extend("x-range" = [0, 1]))]
    pub weight: Value<f32>,
    #[schemars(extend("x-range" = [0, 16]))]
    pub rate: Value<f32>,
    pub additive: bool,
    #[schemars(extend("x-unit" = "s", "x-range" = [0, 10]))]
    pub blend_in: f32,
    #[schemars(extend("x-unit" = "s", "x-range" = [0, 10]))]
    pub blend_out: f32,
    pub enabled: Value<bool>,
    #[schemars(extend("x-ref" = "bone"))]
    pub bones: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Reach {
    Point(Value<Vec2>),
    #[schemars(extend("pattern" = "^(cursor|bone:.+|layer:[a-z0-9_]{1,32})$"))]
    Named(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Bend {
    #[default]
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IkChain {
    #[schemars(extend("x-ref" = "bone"))]
    pub chain: Vec<String>,
    #[schemars(extend("x-ref" = "reach"))]
    pub target: Reach,
    #[schemars(extend("x-unit" = "°", "x-ref" = "bone"))]
    pub limits: BTreeMap<String, [f32; 2]>,
    #[schemars(extend("x-range" = [0, 1]))]
    pub weight: Value<f32>,
    pub bend: Bend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Body {
    #[default]
    Spring,
    Rigid,
    Rope,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BonePhysics {
    #[schemars(extend("x-ref" = "bone"))]
    pub bone: String,
    pub mode: Body,
    #[schemars(extend("x-range" = [0, 1]))]
    pub stiffness: f32,
    #[schemars(extend("x-range" = [0, 1]))]
    pub damping: f32,
    #[schemars(extend("x-range" = [0, 1]))]
    pub inertia: f32,
    #[schemars(extend("x-range" = [-10, 10]))]
    pub gravity: f32,
    #[schemars(extend("x-range" = [0, 1]))]
    pub wind: f32,
    #[schemars(extend("x-unit" = "°"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub rotation: Option<[f32; 2]>,
    #[schemars(extend("x-unit" = "px"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub translation: Option<[f32; 2]>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BoneRule {
    #[schemars(extend("x-ref" = "bone"))]
    pub bone: String,
    #[schemars(extend("x-ref" = "bone"))]
    pub parent: String,
    #[schemars(extend("x-range" = [0, 1]))]
    pub weight: Value<f32>,
}

impl Rig {
    pub fn is_empty(&self) -> bool {
        *self == Rig::default()
    }
}
