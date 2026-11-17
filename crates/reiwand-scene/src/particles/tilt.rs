use super::Init;
use crate::validate::Check;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Tilt {
    #[schemars(extend("x-unit" = "deg"))]
    pub angle: [[f32; 2]; 2],
    #[schemars(extend("x-unit" = "deg/s"))]
    pub spin: [[f32; 2]; 2],
}

impl Tilt {
    pub fn turned(&self) -> bool {
        self.angle.iter().chain(&self.spin).flatten().any(|v| *v != 0.0)
    }
}

impl Check<'_> {
    pub(super) fn tilted(&mut self, p: &str, i: &Init) {
        let Some(t) = &i.tilt else { return };
        self.uses(t.turned(), "particles2");
        for (name, ends, max) in [("angle", t.angle, 3600.0), ("spin", t.spin, 36000.0)] {
            for k in 0..2 {
                self.range(&format!("{p}/tilt/{name}/0/{k}"), ends[0][k], -max, max);
                self.range(&format!("{p}/tilt/{name}/1/{k}"), ends[1][k], ends[0][k], max);
            }
        }
    }
}
