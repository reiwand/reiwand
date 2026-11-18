use super::{
    Credit, GpuAdapter, HdrState, LayerShellState, OutputPlaylist, PreviewState, TransitionState, Uid,
};
use crate::{Filter, Fit, Screen};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

model! {
    pub struct State {
        pub model: u32,
        pub version: String,
        pub adapter: String,
        pub tier: String,
        pub tier_measured: bool,
        pub config: PathBuf,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub config_error: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub library_error: Option<String>,
        pub serial: u64,
        pub paused: bool,
        pub on_battery: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub battery_percent: Option<u8>,
        pub locked: bool,
        pub volume: f64,
        pub muted: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub profile: Option<Uid>,
        pub jobs: u32,
        pub options: BTreeMap<String, Value>,
        pub outputs: BTreeMap<String, OutputState>,
        pub previews: BTreeMap<String, PreviewState>,
        pub calibrating: bool,
        pub compute: bool,
        pub adapters: Vec<GpuAdapter>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub gpu_warning: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub gpu_note: Option<String>,
        pub gpu_restart: bool,
        pub qualities: BTreeMap<String, String>,
        pub hdrs: BTreeMap<String, String>,
        pub transitions: BTreeMap<String, String>,
        pub layer_shell: LayerShellState,
        pub plasma: bool,
    }

    pub struct OutputState {
        pub width: u32,
        pub height: u32,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub image: Option<PathBuf>,
        pub fit: Fit,
        pub background: String,
        pub kind: Shown,
        pub playing: bool,
        pub active: bool,
        pub covered: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub still_reason: Option<StillReason>,
        pub clients: u32,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub error: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub item: Option<Uid>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub preset: Option<Uid>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub playlist: Option<OutputPlaylist>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub credit: Option<Credit>,
        pub edited: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub screen: Option<Screen>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub mm: Option<[f64; 4]>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub follows: Option<String>,
        pub parallax: bool,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub still: Option<crate::travel::Still>,
        pub frames: u64,
        pub page_frames: u64,
        pub refresh: u32,
        pub fps: u32,
        pub buffers: String,
        pub sound: String,
        pub shown_fit: Fit,
        pub flashes: String,
        pub left_out: Vec<String>,
        pub page_errors: Vec<String>,
        pub blank: bool,
        pub previous: bool,
        pub released: bool,
        pub particles: String,
        pub quality: String,
        pub quality_reason: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub quality_choice: Option<String>,
        pub quality_cost: f64,
        pub quality_budget: f64,
        pub quality_auto: String,
        #[serde(flatten)]
        pub quality_timing: QualityTiming,
        pub decode: String,
        pub drafted: bool,
        pub sandbox: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub helper: Option<u32>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub accent: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub tones: Option<super::Tones>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub filter: Option<Filter>,
        pub hdr: HdrState,
        pub transition: TransitionState,
    }

    #[derive(Copy, Eq, Hash)]
    pub enum StillReason { Covered, Battery, Locked, Paused }

    #[derive(Copy, Eq, Hash)]
    pub enum Shown { Color, Image, Video, Scene, Web, #[serde(other)] Unknown }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QualityTiming {
    #[serde(deserialize_with = "crate::explicit::null")]
    pub quality_frame_ms: Option<f64>,
    #[serde(deserialize_with = "crate::explicit::null")]
    pub quality_period_ms: Option<f64>,
    pub quality_step: String,
    pub quality_pace: u32,
}
