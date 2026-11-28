mod api;
mod book;
mod counted;
mod enroll;
mod pin;
#[cfg(test)]
mod tests_pinned;

pub use api::{AGENT, Api, MAIL, PAGE, SMALL, check_url, same_origin};
pub use book::{File, Link, Rec, clean, origin};
pub use enroll::{Poll, poll};
pub use pin::{fingerprint, peek, pinned_tls, untrusted, valid_pin};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Crypto(&'static str),
    #[error("{detail}")]
    Server { status: u16, code: String, detail: String },
    #[error("cannot reach the server: {0}")]
    Network(String),
    #[error("{0}")]
    Invalid(String),
    #[error(
        "the server's certificate is not trusted: check its fingerprint and add it with that fingerprint"
    )]
    Untrusted,
}

impl Error {
    pub fn code(&self) -> &str {
        match self {
            Error::Server { code, .. } => code,
            _ => "",
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Invalid(format!("unexpected data: {e}"))
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
