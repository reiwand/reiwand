use super::error::{Fault, ReadError, join};
use serde::de::{Deserialize, DeserializeOwned, Deserializer, Error};
use serde_json::Value;

pub fn null<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(d)
}

fn missing_field(message: &str) -> Option<(&str, &str)> {
    let rest = message.strip_prefix("missing field `")?;
    let (name, tail) = rest.split_once('`')?;
    Some((name, tail.strip_prefix(" at ").unwrap_or("")))
}

pub fn read<T: DeserializeOwned>(file: &str, value: &Value) -> Result<T, ReadError> {
    serde_path_to_error::deserialize(value).map_err(|e| {
        let path = e.path().to_string();
        let text = e.into_inner().to_string();
        match missing_field(&text) {
            Some((name, inner)) => {
                let path = join(&join(&path, inner), name);
                ReadError { file: file.to_owned(), path, fault: Fault::Missing }
            }
            None => {
                ReadError { file: file.to_owned(), path: path_or_root(path), fault: Fault::Invalid(text) }
            }
        }
    })
}

fn path_or_root(path: String) -> String {
    if path == "." { String::new() } else { path }
}

pub fn read_str<T: DeserializeOwned>(file: &str, text: &str) -> Result<T, ReadError> {
    let value = serde_json::from_str::<Value>(text).map_err(|e| ReadError {
        file: file.to_owned(),
        path: String::new(),
        fault: Fault::Invalid(e.to_string()),
    })?;
    read(file, &value)
}

pub type Try<'a, R> = &'a dyn Fn(&Value) -> Result<R, ReadError>;

pub fn either<R>(value: &Value, tries: &[(&[&str], Try<'_, R>)]) -> Result<R, ReadError> {
    let mut first: Option<ReadError> = None;
    let mut hinted: Option<ReadError> = None;
    for (hints, attempt) in tries {
        match attempt(value) {
            Ok(found) => return Ok(found),
            Err(e) => {
                let applies = value.as_object().is_some_and(|m| hints.iter().any(|h| m.contains_key(*h)));
                if applies && hinted.is_none() {
                    hinted = Some(e.clone());
                }
                first.get_or_insert(e);
            }
        }
    }
    Err(hinted.or(first).unwrap_or(ReadError {
        file: String::new(),
        path: String::new(),
        fault: Fault::Invalid("no variant to try".into()),
    }))
}

pub fn untagged<'de, D: Deserializer<'de>, R>(d: D, tries: &[(&[&str], Try<'_, R>)]) -> Result<R, D::Error> {
    let value = Value::deserialize(d)?;
    either(&value, tries).map_err(|e| match e.fault {
        Fault::Missing => match e.path.rsplit_once('.') {
            Some((parent, name)) => D::Error::custom(format!("missing field `{name}` at {parent}")),
            None => D::Error::custom(format!("missing field `{}`", e.path)),
        },
        Fault::Invalid(why) => D::Error::custom(why),
    })
}
