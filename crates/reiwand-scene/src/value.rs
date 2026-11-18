use crate::signal::{Reduce, Smooth};
use crate::timeline::{Key, Loop};
use reiwand_core::explicit::block;
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
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub prop: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub map: Option<BTreeMap<String, T>>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub keys: Option<Vec<Key<T>>>,
    #[schemars(extend("x-ref" = "timeline"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub timeline: Option<String>,
    #[serde(rename = "loop")]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub repeat: Option<Loop>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub audio: Option<Band>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub channel: Option<AudioChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub kind: Option<AudioKind>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub unit: Option<AudioUnit>,
    #[schemars(extend("x-unit" = "s", "x-range" = [0.05, 60]))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub window: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub reference: Option<Reference>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub hold_silence: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub cursor: Option<Axis>,
    #[schemars(extend("x-format" = true))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub clock: Option<String>,
    #[schemars(extend("x-format" = true))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub scale: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub offset: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub clamp: Option<[f32; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub smooth: Option<Smooth>,
    pub invert: bool,
    #[schemars(extend("x-ref" = "layer"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub base: Option<T>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub of: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub reduce: Option<Reduce>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub bounds: Option<[f32; 2]>,
    #[schemars(extend("x-range" = [0.1, 10]))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub exponent: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub x: Option<Track>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub y: Option<Track>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub r: Option<Track>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub g: Option<Track>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub b: Option<Track>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub a: Option<Track>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub media: Option<crate::Media>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub placeholder: Option<crate::props::Text>,
    #[serde(rename = "fn")]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub call: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
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
