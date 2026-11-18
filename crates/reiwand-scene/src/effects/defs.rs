use super::tools::Tool;
use super::{Entry, ParamKind, PassBlend, Unit, Use};
use crate::value::Param;
use reiwand_core::explicit::block;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParamDef {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: ParamKind,
    pub prefill: Param,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub min: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub max: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub uses: Option<Use>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub unit: Option<Unit>,
    pub tools: Vec<Tool>,
    pub label: BTreeMap<String, String>,
    pub help: BTreeMap<String, String>,
    pub values: Vec<Entry>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub channels: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub paint: Option<Param>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub yields_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub waits_for: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub tool: Option<Tool>,
}

impl ParamDef {
    pub fn fits_kind(&self) -> Result<(), String> {
        let ranged = matches!(self.kind, ParamKind::Float | ParamKind::Int | ParamKind::Vec2);
        let id = &self.id;
        let state = |need: bool, has: bool, field: &str| match (need, has) {
            (true, false) => Err(format!("parameter {id}: a {:?} needs \"{field}\"", self.kind)),
            (false, true) => Err(format!("parameter {id}: a {:?} has no \"{field}\"", self.kind)),
            _ => Ok(()),
        };
        state(ranged, self.min.is_some(), "min")?;
        state(ranged, self.max.is_some(), "max")?;
        state(self.kind == ParamKind::Enum, !self.values.is_empty(), "values")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PassDef {
    pub entry: String,
    pub scale: f32,
    pub inputs: Vec<String>,
    pub output: Output,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub copy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub blend: Option<PassBlend>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub vertex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub grid: Option<[u32; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub swap: Option<[String; 2]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Output {
    #[default]
    Next,
    Step,
    State,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cost {
    pub taps: f32,
    pub passes: u32,
    pub scale: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub scale: f32,
    pub settle: f32,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub fade: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tune {
    pub params: BTreeMap<String, Param>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    pub scale: Option<f32>,
    pub freeze: bool,
}

impl Default for Cost {
    fn default() -> Self {
        Cost { taps: 1.0, passes: 1, scale: 1.0 }
    }
}

impl Default for State {
    fn default() -> Self {
        State { scale: 1.0, settle: 0.0, fade: None }
    }
}
