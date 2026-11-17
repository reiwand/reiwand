use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_BONE_COLLIDERS: usize = 8;
pub const LOW_BONE_COLLIDERS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Hull {
    #[default]
    Capsule,
    Sphere,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BoneCollider {
    #[schemars(extend("x-ref" = "bone"))]
    pub bone: String,
    pub shape: Hull,
    #[schemars(extend("x-range" = [0, 100]))]
    pub radius: f32,
    pub offset: [f32; 3],
}
