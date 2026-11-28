use reiwand_core::explicit::{Try, block, null, read, untagged};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Sample {
    pub name: String,
    pub size: [f32; 2],
    #[serde(deserialize_with = "null")]
    #[schemars(required)]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub badge: Option<Badge>,
    pub layers: Vec<Layer>,
    #[serde(flatten)]
    pub paint: Paint,
    pub shape: Shape,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Badge {
    pub label: String,
    pub size: f32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Layer {
    pub id: String,
    pub opacity: f32,
    pub effects: Vec<Effect>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Effect {
    pub kind: String,
    pub params: Params,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Params {
    pub reach: f32,
    #[serde(deserialize_with = "null")]
    #[schemars(required)]
    pub mask: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Paint {
    pub tint: [f32; 3],
    #[serde(deserialize_with = "null")]
    #[schemars(required)]
    pub tone: Option<f32>,
}

#[derive(Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Circle {
    pub radius: f32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Square {
    pub side: f32,
    pub round: Round,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Round {
    pub corner: f32,
}

impl<'de> Deserialize<'de> for Shape {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let circle: Try<Shape> = &|v| read::<Circle>("", v).map(Shape::Circle);
        let square: Try<Shape> = &|v| read::<Square>("", v).map(Shape::Square);
        untagged(d, &[(&["radius"], circle), (&["side", "round"], square)])
    }
}

pub const FULL: &str = include_str!("full.json");
