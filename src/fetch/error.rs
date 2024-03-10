use reqwest::header::{InvalidHeaderName, InvalidHeaderValue, ToStrError};
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

impl From<InvalidHeaderName> for Error {
    fn from(e: InvalidHeaderName) -> Self {
        eprintln!("{:?}", e);

        Error::ReqwestInvalidHeaderNameError
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Self {
        eprintln!("{:?}", e);

        Error::ReqwestInvalidHeaderValueError
    }
}

impl From<ToStrError> for Error {
    fn from(e: ToStrError) -> Self {
        eprintln!("{:?}", e);

        Error::ReqwestInvalidHeaderValueError
    }
}
