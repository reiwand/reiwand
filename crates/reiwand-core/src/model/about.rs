model! {
    pub struct About {
        pub schema: String,
        pub title: String,
        pub description: String,
        pub changes: String,
        pub authors: Vec<String>,
        pub license: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub content: Option<String>,
        pub descriptors: Vec<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub photosensitive: Option<bool>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub origin: Option<String>,
        pub derived_from: Vec<Work>,
        pub credits: Vec<PackageCredit>,
        pub links: Vec<Link>,
        pub inputs: Vec<String>,
        pub features: Vec<Feature>,
        pub languages: Vec<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub provider: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub source_url: Option<String>,
        pub platforms: Vec<String>,
        pub packages: Vec<String>,
        pub refused: Vec<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub animated: Option<String>,
        pub spread: bool,
    }

    pub struct Work {
        pub author: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub title: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub url: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub license: Option<String>,
    }

    pub struct PackageCredit {
        pub paths: Vec<String>,
        pub license: String,
        pub attribution: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub url: Option<String>,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub source: Option<String>,
    }

    pub struct Link { pub kind: String, pub url: String }

    pub struct Feature { pub name: String, pub why: String }
}
