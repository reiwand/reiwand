use crate::Color;
use crate::content::{Align, white};
use crate::value::{Value, Vec2};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_FALLBACK: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Layout {
    pub wrap: Option<Wrap>,
    pub line_height: Option<f32>,
    pub padding: f32,
    pub backing: Option<(Color, [f32; 2], f32)>,
    pub spacing: f32,
    pub reach: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TextLayer {
    #[schemars(extend("x-multiline" = true))]
    pub text: crate::words::Words,
    #[schemars(extend("x-ref" = "font"))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub font: Option<String>,
    #[schemars(extend("x-ref" = "font"))]
    pub fallback: Vec<String>,
    #[schemars(extend("x-unit" = "px", "x-range" = [1, 1024]))]
    pub size: f32,
    pub color: Value<Color>,
    pub align: Align,
    pub valign: Valign,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub wrap: Option<Wrap>,
    #[schemars(extend("x-range" = [0.5, 4]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub line_height: Option<f32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub background: Option<Backing>,
    #[schemars(extend("x-unit" = "px", "x-range" = [0, 1024]))]
    pub padding: f32,
    #[schemars(extend("x-unit" = "px", "x-range" = [-100, 1000]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub letter_spacing: Option<Value<f32>>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub outline: Option<Outline>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub shadow: Option<Shadow>,
    #[schemars(extend("x-unit" = "px", "x-range" = [0, 64]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub blur: Option<Value<f32>>,
    pub render: Glyphs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Valign {
    Top,
    #[default]
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Glyphs {
    #[default]
    Auto,
    Raster,
    Sdf,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Wrap {
    #[schemars(extend("x-unit" = "px", "x-range" = [1, 16384]))]
    pub width: f32,
    #[schemars(extend("x-range" = [1, 1000]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub lines: Option<u32>,
    pub ellipsis: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Backing {
    pub color: Value<Color>,
    #[schemars(extend("x-unit" = "px"))]
    pub padding: [f32; 2],
    #[schemars(extend("x-unit" = "px"))]
    pub radius: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Outline {
    pub color: Value<Color>,
    #[schemars(extend("x-unit" = "px", "x-range" = [0, 64]))]
    pub width: Value<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Shadow {
    pub color: Value<Color>,
    #[schemars(extend("x-unit" = "px"))]
    pub offset: Value<Vec2>,
    #[schemars(extend("x-unit" = "px", "x-range" = [0, 64]))]
    pub blur: Value<f32>,
}

impl Valign {
    pub fn fraction(self) -> f32 {
        self as u8 as f32 / 2.0
    }
}

impl TextLayer {
    pub fn new(text: Value<String>) -> TextLayer {
        let (font, fallback, valign, wrap, line_height, background, padding) = Default::default();
        let (letter_spacing, outline, shadow, blur, render) = Default::default();
        let (size, color, align) = (size(), white(), Align::default());
        TextLayer {
            text: text.into(),
            font,
            fallback,
            size,
            color,
            align,
            valign,
            wrap,
            line_height,
            background,
            padding,
            letter_spacing,
            outline,
            shadow,
            blur,
            render,
        }
    }

    pub fn laid_out(&self) -> bool {
        let boxed = self.background.is_some() || self.padding != 0.0 || self.line_height.is_some();
        boxed || self.valign != Valign::default() || self.wrap.is_some()
    }

    pub fn styled(&self) -> bool {
        let drawn = self.outline.is_some() || self.shadow.is_some() || self.blur.is_some();
        drawn || self.letter_spacing.is_some() || self.render == Glyphs::Sdf
    }
}

fn size() -> f32 {
    48.0
}
