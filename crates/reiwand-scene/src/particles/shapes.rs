use super::{Around, Between, Handle, Num, Source};
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
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required, extend("x-null" = true))]
        scale: Option<[f32; 2]>,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required, extend("x-null" = true))]
        sign: Option<[i8; 2]>,
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required, extend("x-null" = true))]
        at: Option<Handle>,
    },
    Sphere {
        center: [f32; 2],
        depth: f32,
        radius: [f32; 3],
        inner: f32,
        sign: [i8; 3],
        #[serde(deserialize_with = "reiwand_core::explicit::null")]
        #[schemars(required, extend("x-null" = true))]
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
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub inner: Option<[f32; 2]>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub at: Option<Handle>,
}
