//!
//! Crate errors
//!

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    InputStringError(&'static str),
    #[error("No value for parameter '{0}'")]
    NoValueParameter(String),
    #[error("Cannot retrieve value for parameter '{0}'")]
    ParameterValueError(String),
    #[error("Missing projection name")]
    MissingProjectionError,
    #[error("Unrecognized datum")]
    InvalidDatum,
    #[error("Unrecognized ellipsoid")]
    InvalidEllipsoid,
    #[error("{0}")]
    InvalidParameterValue(&'static str),
    #[error("Latitude out of range")]
    LatitudeOutOfRange,
    #[error("NAD grid not available")]
    NoNADGridAvailable,
    #[error("Invalid 'towgs84' string")]
    InvalidToWGS84String,
    #[error("Invalid axis")]
    InvalidAxis,
}

pub type Result<T> = std::result::Result<T, Error>;
