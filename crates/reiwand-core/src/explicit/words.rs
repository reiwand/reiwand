use serde::de::{Deserialize, Deserializer, Error};
use serde::{Serialize, Serializer};
use serde_json::Value;

fn read<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D, word: &str) -> Result<Option<T>, D::Error> {
    let value = Value::deserialize(d)?;
    if value.as_str().is_some_and(|text| text.eq_ignore_ascii_case(word)) {
        return Ok(None);
    }
    T::deserialize(value).map(Some).map_err(D::Error::custom)
}

fn write<S: Serializer, T: Serialize>(value: &Option<T>, s: S, word: &str) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => value.serialize(s),
        None => s.serialize_str(word),
    }
}

macro_rules! word {
    ($name:ident, $word:literal) => {
        pub mod $name {
            use serde::{Deserialize, Deserializer, Serialize, Serializer};

            pub const WORD: &str = $word;

            pub fn serialize<S: Serializer, T: Serialize>(
                value: &Option<T>,
                s: S,
            ) -> Result<S::Ok, S::Error> {
                super::write(value, s, WORD)
            }

            pub fn deserialize<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
                d: D,
            ) -> Result<Option<T>, D::Error> {
                super::read(d, WORD)
            }
        }
    };
}

word!(general, "general");
word!(empty, "");
word!(none, "none");
word!(auto, "auto");
