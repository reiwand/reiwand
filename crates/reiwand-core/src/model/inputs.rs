use std::collections::{BTreeMap, BTreeSet};

model! {
    #[derive(Copy, Eq, Hash, PartialOrd, Ord)]
    pub enum Input { Audio, Pointer, Clicks, Network, Rgb, Media, MediaNetwork, AudioNetwork }

    #[derive(Copy, Eq, Hash)]
    pub enum InputsGate { On, Still, Covered, Unseen }

    pub struct SceneInputs {
        pub scene: String,
        pub audio: bool,
        pub pointer: bool,
        pub clicks: bool,
        pub network: bool,
        pub rgb: bool,
        pub media: bool,
        pub title: String,
        pub key: String,
        pub granted: BTreeSet<Input>,
        pub without: BTreeSet<Input>,
        pub undecided: BTreeSet<Input>,
    }

    pub struct InputsReply {
        pub model: u32,
        pub serial: u64,
        pub capturing: bool,
        pub tracking: bool,
        pub grants: BTreeMap<String, BTreeSet<Input>>,
        pub shown: BTreeMap<String, SceneInputs>,
        pub parallax: BTreeSet<String>,
        pub declined: BTreeMap<String, BTreeSet<Input>>,
        pub asking: BTreeMap<String, BTreeSet<Input>>,
        pub quiet: BTreeMap<String, BTreeSet<Input>>,
        pub clicks: BTreeSet<String>,
        pub clicking: BTreeSet<String>,
        pub unreached: BTreeSet<String>,
        pub media: MediaStatus,
        pub gates: BTreeMap<String, InputsGate>,
    }

    #[derive(Default)]
    pub struct MediaStatus {
        pub listening: bool,
        pub players: u32,
        pub playing: bool,
        pub track: bool,
        pub cover: bool,
        pub remote_covers: bool,
        pub using: BTreeSet<String>,
    }
}

impl SceneInputs {
    pub fn wanted(&self) -> BTreeSet<Input> {
        let flags = [
            (self.audio, Input::Audio),
            (self.pointer, Input::Pointer),
            (self.clicks, Input::Clicks),
            (self.network, Input::Network),
            (self.rgb, Input::Rgb),
            (self.media, Input::Media),
            (self.audio && self.network, Input::AudioNetwork),
            (self.media && self.network, Input::MediaNetwork),
        ];
        flags.into_iter().filter(|(on, _)| *on).map(|(_, input)| input).collect()
    }
}

impl Input {
    pub const ALL: [Input; 8] = [
        Input::Audio,
        Input::Pointer,
        Input::Clicks,
        Input::Network,
        Input::Rgb,
        Input::Media,
        Input::MediaNetwork,
        Input::AudioNetwork,
    ];

    pub fn parse(name: &str) -> Option<Input> {
        Input::ALL.into_iter().find(|i| i.as_str() == name)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Input::Audio => "audio",
            Input::Pointer => "pointer",
            Input::Clicks => "clicks",
            Input::Network => "network",
            Input::Rgb => "rgb",
            Input::Media => "media",
            Input::MediaNetwork => "media_network",
            Input::AudioNetwork => "audio_network",
        }
    }
}
