use reiwand_core::explicit::block;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MaterialMaps {
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub base: Option<String>,
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub orm: Option<String>,
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub normal: Option<String>,
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub occlusion: Option<String>,
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub emissive: Option<String>,
    #[schemars(extend("x-ref" = "image"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "block")]
    #[schemars(extend("x-null" = true, "x-block" = true))]
    pub comb: Option<String>,
}

pub const LINEAR_SLOTS: [bool; 5] = [false, true, true, true, false];

impl MaterialMaps {
    pub fn is_empty(&self) -> bool {
        self.slots().iter().all(Option::is_none) && self.comb.is_none()
    }

    pub fn slots(&self) -> [Option<&str>; 5] {
        [&self.base, &self.orm, &self.normal, &self.occlusion, &self.emissive].map(|s| s.as_deref())
    }

    pub fn slot_mut(&mut self, slot: usize) -> Option<&mut Option<String>> {
        Some(match slot {
            0 => &mut self.base,
            1 => &mut self.orm,
            2 => &mut self.normal,
            3 => &mut self.occlusion,
            4 => &mut self.emissive,
            _ => return None,
        })
    }
}

pub(crate) fn dressed<'a>(materials: impl Iterator<Item = (&'a String, &'a MaterialMaps)>) -> u64 {
    let mut h = std::hash::DefaultHasher::new();
    let mut any = false;
    for (name, maps) in materials.filter(|(_, m)| !m.is_empty()) {
        any = true;
        name.hash(&mut h);
        maps.hash(&mut h);
    }
    if any { h.finish() | 1 } else { 0 }
}
