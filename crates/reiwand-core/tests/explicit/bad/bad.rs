#[derive(Default)]
pub struct Bad {
    #[serde(default)]
    pub a: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<f32>,
}

pub fn pick(x: Option<f32>) -> f32 {
    x.unwrap_or(1.0)
}
