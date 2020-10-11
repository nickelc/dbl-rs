use std::fmt;

use reqwest::StatusCode;
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    Ratelimit { retry_after: u32 },
    Reqwest(reqwest::Error),
    Url(ParseError),
}

impl Error {
    pub fn is_ratelimit(&self) -> bool {
        std::matches!(self, Error::Ratelimit { .. })
    }

    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Error::Ratelimit { .. } => Some(StatusCode::TOO_MANY_REQUESTS),
            Error::Reqwest(e) => e.status(),
            Error::Url(_) => None,
        }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Ratelimit { retry_after } => {
                write!(f, "Ratelimit reached, retry after: {}", retry_after)
            }
            Error::Reqwest(e) => e.fmt(f),
            Error::Url(e) => e.fmt(f),
        }
    }
}

pub fn ratelimit(retry_after: u32) -> Error {
    Error::Ratelimit { retry_after }
}

pub fn from(e: reqwest::Error) -> Error {
    Error::Reqwest(e)
}
