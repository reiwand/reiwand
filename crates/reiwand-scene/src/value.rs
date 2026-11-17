use crate::signal::{Reduce, Smooth};
use crate::timeline::{Key, Loop};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod emitters;
mod kinds;
mod listen;

pub use emitters::{EMITTER_TEXELS, EffectEmitter, MAX_EFFECT_EMITTERS, emitter_key, emitter_texels};
pub use kinds::{AudioChannel, AudioKind, AudioUnit, Axis, Band, Level, Param, Reference, Vec2, Vec3};
pub use listen::{Listen, MAX_LISTENS, MAX_SCRIPT_LISTENS};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(rename = "Value_for_{T}")]
pub enum Value<T> {
    Fixed(T),
    Bound(Box<Binding<T>>),
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, bound(deserialize = "T: Deserialize<'de>"))]
#[schemars(rename = "Binding_for_{T}")]
pub struct Binding<T> {
    #[schemars(extend("x-ref" = "property"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub prop: Option<String>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub map: Option<BTreeMap<String, T>>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub keys: Option<Vec<Key<T>>>,
    #[schemars(extend("x-ref" = "timeline"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub timeline: Option<String>,
    #[serde(rename = "loop")]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub repeat: Option<Loop>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub audio: Option<Band>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub channel: Option<AudioChannel>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub kind: Option<AudioKind>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub unit: Option<AudioUnit>,
    #[schemars(extend("x-unit" = "s", "x-range" = [0.05, 60]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub window: Option<f32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub reference: Option<Reference>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub hold_silence: Option<bool>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub cursor: Option<Axis>,
    #[schemars(extend("x-format" = true))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub clock: Option<String>,
    #[schemars(extend("x-format" = true))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub date: Option<String>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub script: Option<String>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub scale: Option<f32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub offset: Option<f32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub clamp: Option<[f32; 2]>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub smooth: Option<Smooth>,
    pub invert: bool,
    #[schemars(extend("x-ref" = "layer"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub layer: Option<String>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub base: Option<T>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub of: Option<u32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub reduce: Option<Reduce>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub bounds: Option<[f32; 2]>,
    #[schemars(extend("x-range" = [0.1, 10]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub exponent: Option<f32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub x: Option<Track>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub y: Option<Track>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub r: Option<Track>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub g: Option<Track>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub b: Option<Track>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub a: Option<Track>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub media: Option<crate::Media>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub placeholder: Option<crate::props::Text>,
    #[serde(rename = "fn")]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub call: Option<String>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub params: Option<BTreeMap<String, Param>>,
}

pub type Track = Box<Value<f32>>;

impl<T> Value<T> {
    pub fn binding(&self) -> Option<&Binding<T>> {
        match self {
            Value::Fixed(_) => None,
            Value::Bound(b) => Some(b),
        }
    }
}

impl<T> Binding<T> {
    pub fn sources(&self) -> usize {
        let present = [self.prop.is_some(), self.keys.is_some(), self.audio.is_some(), self.cursor.is_some()];
        let more = [self.clock.is_some(), self.script.is_some(), self.layer.is_some(), self.tracked()];
        let last = [self.media.is_some(), self.date.is_some()];
        present.into_iter().chain(more).chain(last).filter(|p| *p).count()
    }

    pub fn tracks(&self) -> [(&'static str, Option<&Value<f32>>); 6] {
        let [x, y, r, g, b, a] = [&self.x, &self.y, &self.r, &self.g, &self.b, &self.a].map(|t| t.as_deref());
        [("x", x), ("y", y), ("r", r), ("g", g), ("b", b), ("a", a)]
    }

    pub fn tracked(&self) -> bool {
        self.tracks().iter().any(|(_, t)| t.is_some())
    }

    pub fn modify(&self, x: f32) -> f32 {
        let x = self.bounds.map_or(x, |[a, b]| ((x - a) / (b - a).max(f32::EPSILON)).clamp(0.0, 1.0));
        let x = self.exponent.map_or(x, |e| x.abs().powf(e).copysign(x));
        let x = if self.invert { 1.0 - x } else { x };
        let x = x * self.scale.unwrap_or(1.0) + self.offset.unwrap_or(0.0);
        self.clamp.map_or(x, |[lo, hi]| x.clamp(lo, hi.max(lo)))
    }
}
