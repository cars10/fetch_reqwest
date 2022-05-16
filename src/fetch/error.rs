use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Error {
    ReqwestClientBuilderError,
    NetworkError,
    ReqwestInvalidHeaderNameError,
    ReqwestInvalidHeaderValueError,
    UnknownError,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        eprintln!("{:?}", e);

        if e.is_builder() {
            Error::ReqwestClientBuilderError
        } else {
            Error::UnknownError
        }
    }
}

impl From<reqwest::header::InvalidHeaderName> for Error {
    fn from(e: reqwest::header::InvalidHeaderName) -> Self {
        eprintln!("{:?}", e);

        Error::ReqwestInvalidHeaderNameError
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        eprintln!("{:?}", e);

        Error::ReqwestInvalidHeaderValueError
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(e: reqwest::header::ToStrError) -> Self {
        eprintln!("{:?}", e);

        Error::ReqwestInvalidHeaderValueError
    }
}
