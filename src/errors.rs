//!
//! Crate errors
//!

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    InputStringError(&'static str),
    #[error("No value for parameter '{0}'")]
    NoValueParameter(String),
    #[error("Cannot retrieve value for parameter '{name}': {reason}")]
    ParameterValueError { name: String, reason: String },
    #[error("Missing projection name")]
    MissingProjectionError,
    #[error("Unrecognized datum")]
    InvalidDatumError,
    #[error("Unrecognized ellipsoid")]
    InvalidEllipsoidError,
    #[error("{0}")]
    InvalidParameterValue(&'static str),
    #[error("Latitude out of range")]
    LatitudeOutOfRange,
    #[error("NAD grid not available")]
    NoNADGridAvailable,
}

pub type Result<T> = std::result::Result<T, Error>;
