use super::{AudioKind, AudioUnit, Band, Binding, Reference};
use crate::Issue;
use crate::input::{AudioFrame, BANDS, Usual, read};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_LISTENS: usize = 2;
pub const MAX_SCRIPT_LISTENS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Listen {
    pub kind: AudioKind,
    pub unit: AudioUnit,
    #[schemars(extend("x-unit" = "s", "x-range" = [0.05, 60]))]
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub window: Option<f32>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub reference: Option<Reference>,
    #[serde(deserialize_with = "reiwand_core::explicit::null")]
    #[schemars(required)]
    pub hold_silence: Option<bool>,
}

impl Listen {
    pub fn binding(&self) -> Binding<()> {
        Binding {
            kind: Some(self.kind),
            unit: Some(self.unit),
            window: self.window,
            reference: self.reference,
            hold_silence: self.hold_silence,
            ..Binding::default()
        }
    }

    pub fn surging(&self) -> bool {
        self.kind == AudioKind::Surge
    }

    pub fn bands(&self, audio: &AudioFrame, usual: Option<&Usual>) -> [f32; BANDS] {
        let b = self.binding();
        std::array::from_fn(|i| read(audio, usual, &b, Band::Index(i as u32)))
    }

    pub fn problems(&self, path: &str) -> Vec<Issue> {
        let extras = [("window", self.window.is_some()), ("reference", self.reference.is_some())];
        let extras = extras.into_iter().chain([("hold_silence", self.hold_silence.is_some())]);
        let mut found = Vec::new();
        for (name, given) in extras {
            match (self.surging(), given) {
                (true, false) => found.push(Issue::Required(format!("{path}/{name}"))),
                (false, true) => found.push(Issue::Type(format!("{path}/{name}"))),
                _ => {}
            }
        }
        if let Some(w) = self.window.filter(|w| !(0.05..=60.0).contains(w)) {
            found.push(Issue::Range { path: format!("{path}/window"), value: w, min: 0.05, max: 60.0 });
        }
        found
    }
}
