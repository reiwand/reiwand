use reqwest::header::{HeaderValue, InvalidHeaderValue};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Secret(pub String);

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Secret(..)")
    }
}

impl<T: Into<String>> From<T> for Secret {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&Secret> for HeaderValue {
    type Error = InvalidHeaderValue;

    fn try_from(secret: &Secret) -> Result<Self, Self::Error> {
        let mut value = HeaderValue::from_str(&secret.0)?;
        value.set_sensitive(true);
        Ok(value)
    }
}
