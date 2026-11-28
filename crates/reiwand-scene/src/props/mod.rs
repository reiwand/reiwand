mod check;
mod cond;
mod expr;
mod features;
mod leveling;
mod slider;
mod text;
mod value;
mod values;

pub use check::{ICONS, check_presets, cycle, icon};
pub use expr::{Expr, rename};
pub use leveling::Leveling;
pub use slider::Slider;
pub use text::tag;
pub use values::{effective, exposed, stale, veiled};

use reiwand_core::explicit::block;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::collections::BTreeMap;

mod control;

pub use control::{Choice, Control, Mode, Pick};
pub(crate) use text::choose;

pub type Values = BTreeMap<String, Json>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Text {
    Plain(String),
    Localized(BTreeMap<String, String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Property {
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub label: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub help: Option<Text>,
    #[schemars(extend("x-ref" = "group"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub group: Option<String>,
    #[schemars(extend("x-condition" = true))]
    pub visible_if: Condition,
    #[schemars(extend("x-condition" = true))]
    pub enabled_if: Condition,
    #[serde(serialize_with = "whole")]
    pub order: f64,
    #[schemars(extend("x-ref" = "icon"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub icon: Option<String>,
    pub quality: BTreeMap<crate::Quality, Json>,
    #[serde(flatten)]
    pub control: Control,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Condition {
    Expr(String),
    Equal(Values),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Preset {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub label: Option<Text>,
    pub values: Values,
}

impl Default for Condition {
    fn default() -> Self {
        Condition::Equal(Values::new())
    }
}

fn whole<S: serde::Serializer>(v: &f64, s: S) -> Result<S::Ok, S::Error> {
    if v.fract() == 0.0 && v.abs() < 1e15 { s.serialize_i64(*v as i64) } else { s.serialize_f64(*v) }
}
