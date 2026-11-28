use super::{Around, Between, Handle, Num, Source};
use reiwand_core::explicit::block;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum EmitShape {
    Point([f32; 2]),
    Box(Area),
    Circle {
        center: [f32; 2],
        radius: Num,
        inner: Num,
        scale: [f32; 2],
        sign: [i8; 2],
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
        #[schemars(extend("x-null" = true, "x-block" = true))]
        at: Option<Handle>,
    },
    Sphere {
        center: [f32; 2],
        depth: f32,
        radius: [f32; 3],
        inner: f32,
        sign: [i8; 3],
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
        #[schemars(extend("x-null" = true, "x-block" = true))]
        push: Option<f32>,
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
        #[schemars(extend("x-null" = true, "x-block" = true))]
        at: Option<Handle>,
    },
    Line([[f32; 2]; 2]),
    Edge,
    Mask {
        #[schemars(extend("x-ref" = "image"))]
        texture: String,
        threshold: Num,
    },
    Layer(Source),
    Around(Around),
    Between(Between),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Area {
    pub from: [f32; 2],
    pub to: [f32; 2],
    pub inner: [f32; 2],
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub at: Option<Handle>,
}
