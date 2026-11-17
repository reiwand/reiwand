use super::Text;
use super::slider::Slider;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum Control {
    Color {
        #[schemars(extend("x-color" = true))]
        default: String,
        alpha: bool,
    },
    Slider(Slider),
    Bool {
        default: bool,
    },
    Combo {
        options: Vec<Choice>,
        #[schemars(extend("x-ref" = "option"))]
        default: Json,
    },
    Text {
        default: String,
        #[schemars(extend("x-range" = [1, 1024]))]
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        max_len: Option<u32>,
        multiline: bool,
        secret: bool,
    },
    File {
        #[schemars(extend("items" = {"type": "string", "enum": ["image", "video"]}))]
        accept: Vec<String>,
        #[schemars(extend("x-ref" = "media"))]
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        default_asset: Option<String>,
    },
    Folder {
        #[schemars(extend("items" = {"type": "string", "enum": ["image", "video"]}))]
        accept: Vec<String>,
        pick: Pick,
        #[schemars(extend("x-unit" = "s", "x-range" = [10, 86400]))]
        every: u32,
        mode: Mode,
        #[schemars(extend("x-range" = [1, 1000]))]
        max_files: u32,
        #[schemars(extend("x-range" = [0, 2]))]
        recursive: u8,
    },
    Shortcut {},
    Button {},
    Group {
        collapsed: bool,
    },
    Label {},
    Divider {},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[default]
    One,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Pick {
    #[default]
    Random,
    Sequence,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Choice {
    Labeled {
        #[schemars(extend("x-scalar" = true))]
        value: Json,
        label: Text,
        #[schemars(extend("x-ref" = "icon"))]
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required)]
        icon: Option<String>,
    },
    Plain(Json),
}
