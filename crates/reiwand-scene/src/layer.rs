use crate::content::{EffectUse, Mask, shown};
use crate::figure::Attach;
use crate::light::Lighting;
use crate::value::Value;
use crate::{Blend, LayerSpace};
use reiwand_core::explicit::block;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod kind;
mod transform;

pub use kind::{Kind, Solid};
pub use transform::{Anchor, Inherit, Parallax, Transform};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Layer {
    pub id: String,
    pub transform: Transform,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub anchor: Option<Anchor>,
    #[schemars(extend("x-range" = [0, 1]))]
    pub opacity: Value<f32>,
    pub visible: Value<bool>,
    pub blend: Blend,
    pub blend_space: LayerSpace,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub mask: Option<Mask>,
    pub effects: Vec<EffectUse>,
    #[schemars(extend("x-range" = [-1, 1], "default" = 0))]
    pub parallax: Parallax,
    #[schemars(extend("default" = false))]
    pub solid: Solid,
    pub cache: bool,
    #[schemars(extend("x-range" = [0, 16]))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub brightness: Option<Value<f32>>,
    #[schemars(extend("x-ref" = "layer"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub inherit: Option<Vec<Inherit>>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub lighting: Option<Lighting>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub attach: Option<Attach>,
    pub depth_test: bool,
    /// Room around the layer for its effects, in layer pixels: `[left, top, right, bottom]`.
    #[schemars(extend("x-unit" = "px", "x-range" = [0, 4096]))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub margin: Option<[f32; 4]>,
    #[serde(flatten)]
    pub kind: Kind,
}

impl Layer {
    pub fn new(id: impl Into<String>, kind: Kind) -> Layer {
        let (transform, anchor, blend, mask, effects, parallax) = Default::default();
        let (brightness, parent, inherit, lighting, attach) = Default::default();
        let (opacity, visible) = (one(), shown());
        Layer {
            id: id.into(),
            transform,
            anchor,
            opacity,
            visible,
            blend,
            blend_space: LayerSpace::Canvas,
            mask,
            effects,
            parallax,
            solid: Solid::default(),
            cache: false,
            brightness,
            parent,
            inherit,
            lighting,
            attach,
            depth_test: false,
            margin: None,
            kind,
        }
    }

    pub fn children(&self) -> &[Layer] {
        match &self.kind {
            Kind::Group { layers, .. } => layers,
            _ => &[],
        }
    }

    pub fn drawn(&self) -> bool {
        !matches!(self.kind, Kind::Sound(_) | Kind::Light(_))
    }

    pub fn flat(&self) -> bool {
        self.drawn()
    }
}

fn one() -> Value<f32> {
    Value::Fixed(1.0)
}
