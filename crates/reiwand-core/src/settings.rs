use crate::quality::QualityChoice;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub use crate::services::{Media, Models, Repos};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Playback {
    pub volume: f64,
    pub muted: bool,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    #[serde(with = "fps")]
    pub max_fps: u32,
    pub software_parallax: bool,
    pub allow_flashing: bool,
    pub scene_sound: bool,
    pub scene_capture: bool,
    pub duck: bool,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    pub quality: QualityChoice,
    pub auto_pace: crate::AutoPace,
    pub hdr: crate::HdrChoice,
    pub lowest_below_percent: u8,
}

impl Default for Playback {
    fn default() -> Self {
        let (volume, muted, max_fps) = (0.5, true, fps::DISPLAY);
        let (software_parallax, allow_flashing, scene_sound, scene_capture) = (false, false, true, true);
        let (duck, quality) = (false, QualityChoice::Auto);
        Self {
            volume,
            muted,
            max_fps,
            software_parallax,
            allow_flashing,
            scene_sound,
            scene_capture,
            duck,
            quality,
            auto_pace: crate::AutoPace::Detail,
            hdr: crate::HdrChoice::Auto,
            lowest_below_percent: 0,
        }
    }
}

pub mod fps {
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub const DISPLAY: u32 = 0;
    pub const WORD: &str = "display";

    pub fn lower(a: u32, b: u32) -> u32 {
        match (a, b) {
            (DISPLAY, other) | (other, DISPLAY) => other,
            _ => a.min(b),
        }
    }

    pub fn serialize<S: Serializer>(fps: &u32, s: S) -> Result<S::Ok, S::Error> {
        match *fps {
            DISPLAY => s.serialize_str(WORD),
            fps => s.serialize_u32(fps),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Fps {
            Number(u32),
            Text(String),
        }
        match Fps::deserialize(d)? {
            Fps::Number(fps) => Ok(fps),
            Fps::Text(text) if text == WORD => Ok(DISPLAY),
            Fps::Text(text) => {
                text.parse().map_err(|_| D::Error::custom(format!("expected 1 to 240 or {WORD}")))
            }
        }
    }
}

impl Playback {
    pub const MAX_FPS: u32 = 240;

    pub fn sanitized(&self) -> Playback {
        Playback {
            volume: if self.volume.is_finite() { self.volume.clamp(0.0, 1.0) } else { 0.0 },
            max_fps: self.max_fps.min(Self::MAX_FPS),
            lowest_below_percent: self.lowest_below_percent.min(100),
            ..self.clone()
        }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransitionKind {
    #[default]
    Own,
    Fade,
    None,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transition {
    pub kind: TransitionKind,
    pub duration_ms: u32,
    pub less_motion: crate::transition::LessMotion,
}

impl Default for Transition {
    fn default() -> Self {
        Self { kind: TransitionKind::Own, duration_ms: 600, less_motion: Default::default() }
    }
}

impl Transition {
    pub const MAX_MS: u32 = 10_000;

    pub fn duration(&self) -> Duration {
        match self.kind {
            TransitionKind::Own | TransitionKind::Fade => {
                Duration::from_millis(u64::from(self.duration_ms.min(Self::MAX_MS)))
            }
            TransitionKind::None => Duration::ZERO,
        }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pause {
    pub when_covered: bool,
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    pub on_battery: crate::BatteryPause,
    pub when_locked: bool,
}

impl Default for Pause {
    fn default() -> Self {
        Self { when_covered: true, on_battery: crate::BatteryPause::Always, when_locked: true }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Library {
    pub watch: bool,
    pub thumb_cache_mb: u32,
    pub hide_mature: bool,
    pub show_mature: bool,
}

impl Default for Library {
    fn default() -> Self {
        Self { watch: true, thumb_cache_mb: 512, hide_mature: false, show_mature: false }
    }
}

impl Library {
    pub const MIN_MB: u32 = 64;
    pub const MAX_MB: u32 = 8192;
}
