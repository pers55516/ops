use std::{error, fmt};

/// Errors that can happen when running an ops server.
#[derive(Debug)]
pub enum Error {
    /// Hyper error
    Hyper(hyper::Error),
    /// JSON error
    Json(serde_json::Error),
    /// UTF-8 Decoding error
    Utf8(std::string::FromUtf8Error),
    /// HTTP error
    Http(hyper::http::Error),
    /// Prometheus error
    Prometheus(prometheus::Error),
    /// Address parsing error
    ParseAddress(std::net::AddrParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Hyper(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
            Error::Utf8(ref err) => err.fmt(f),
            Error::Http(ref err) => err.fmt(f),
            Error::Prometheus(ref err) => err.fmt(f),
            Error::ParseAddress(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Hyper(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            Error::Http(ref err) => Some(err),
            Error::Prometheus(ref err) => Some(err),
            Error::ParseAddress(ref err) => Some(err),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Self::Hyper(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Utf8(err)
    }
}

impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Self {
        Self::Http(err)
    }
}

impl From<prometheus::Error> for Error {
    fn from(err: prometheus::Error) -> Self {
        Self::Prometheus(err)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Self::ParseAddress(err)
    }
}
