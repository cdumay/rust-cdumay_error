use std::collections::BTreeMap;

use serde_value::Value;

use crate::ErrorInfo;

#[derive(Clone, Copy, Debug)]
pub struct ErrorType(pub u16, pub  &'static str, pub  &'static str);

impl ErrorInfo for ErrorType {
    fn code(&self) -> u16 { self.0 }
    fn extra(&self) -> Option<BTreeMap<String, Value>> { None }
    fn message(&self) -> String { self.2.to_string() }
    fn msgid(&self) -> String { self.1.to_string() }
}


pub trait Registry {
    fn default() -> ErrorType { GenericErrors::GENERIC_ERROR }
    fn from_msgid(msgid: &str) -> ErrorType;
}


pub struct GenericErrors;

impl GenericErrors {
    pub const GENERIC_ERROR: ErrorType = ErrorType(500, "Err-15452", "Generic Error");
    pub const UNKNOWN_ERROR: ErrorType = ErrorType(500, "Err-32583", "Unknown Error");
    pub const INVALID_CONFIGURATION: ErrorType = ErrorType(500, "Err-15160", "Invalid Configuration");
    pub const IO_ERROR: ErrorType = ErrorType(500, "Err-11553", "IO Error");
    pub const VALIDATION_ERROR: ErrorType = ErrorType(400, "Err-05612", "Validation Error");
    pub const SERIALIZATION_ERROR: ErrorType = ErrorType(500, "Err-31807", "Serialization Error");
    pub const DESERIALIZATION_ERROR: ErrorType = ErrorType(500, "Err-01394", "Deserialization Error");
    pub const NONE: ErrorType = ErrorType(404, "Err-85941", "None value found");
}

impl Registry for GenericErrors {
    fn from_msgid(msgid: &str) -> ErrorType {
        match msgid {
            "Err-01394" => Self::DESERIALIZATION_ERROR,
            "Err-05612" => Self::VALIDATION_ERROR,
            "Err-11553" => Self::IO_ERROR,
            "Err-15160" => Self::INVALID_CONFIGURATION,
            "Err-15452" => Self::GENERIC_ERROR,
            "Err-31807" => Self::SERIALIZATION_ERROR,
            "Err-32583" => Self::UNKNOWN_ERROR,
            "Err-85941" => Self::NONE,
            _ => Self::default()
        }
    }
}

impl From<std::option::NoneError> for ErrorType {
    fn from(_: std::option::NoneError) -> ErrorType { GenericErrors::NONE }
}
