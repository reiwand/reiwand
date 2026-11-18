use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_FRAMES: u32 = 256;
pub const COLLECTIONS: [&str; 3] = ["ttc", "otc", "woff2"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    pub path: String,
    pub linear: bool,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub atlas: Option<[u32; 2]>,
    pub filter: Filter,
    pub wrap: Wrapping,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub mipmaps: Option<bool>,
    #[schemars(extend("x-range" = [0, 255]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub face: Option<u32>,
    #[schemars(extend("x-unit" = "px"))]
    pub rects: Vec<[u32; 4]>,
    #[schemars(extend("x-range" = [1, 256]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub frames: Option<u32>,
    #[schemars(extend("x-range" = [0.1, 120]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub fps: Option<f32>,
    #[schemars(extend("x-unit" = "s"))]
    pub durations: Vec<f32>,
    pub animated: bool,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required, extend("x-null" = true))]
    pub media: Option<crate::Cover>,
    pub shapes: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    #[default]
    Linear,
    Nearest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Wrapping {
    #[default]
    Clamp,
    Repeat,
    Mirror,
}

impl Asset {
    pub fn extension(&self) -> String {
        self.path.rsplit_once('.').map_or(String::new(), |(_, e)| e.to_ascii_lowercase())
    }

    pub fn moves(&self) -> bool {
        let timed = self.frames.is_some() || self.fps.is_some() || !self.durations.is_empty();
        timed || !self.rects.is_empty() || self.animated
    }

    pub fn sampled(&self) -> bool {
        self.filter != Filter::default() || self.wrap != Wrapping::default() || self.mipmaps.is_some()
    }

    pub fn collection(&self) -> bool {
        self.face.is_some() || COLLECTIONS.contains(&self.extension().as_str())
    }

    pub fn cells(&self) -> u32 {
        match (self.rects.len(), self.atlas) {
            (0, Some([rows, columns])) => rows.saturating_mul(columns),
            (0, None) => 1,
            (n, _) => n as u32,
        }
    }
}
