#[derive(Debug, Clone, PartialEq)]
pub struct ErrorKind(pub &'static str, pub u16, pub &'static str);

pub struct GenericErrors;

impl GenericErrors {
    pub const GENERIC_ERROR: ErrorKind = ErrorKind("Err-15452", 500, "Generic Error");
    pub const UNKNOWN_ERROR: ErrorKind = ErrorKind("Err-32583", 500, "Unknown Error");
    pub const INVALID_CONFIGURATION: ErrorKind = ErrorKind("Err-15160", 500, "Invalid Configuration");
    pub const IO_ERROR: ErrorKind = ErrorKind("Err-11553", 500, "IO Error");
    pub const VALIDATION_ERROR: ErrorKind = ErrorKind("Err-05612", 400, "Validation Error");
    pub const SERIALIZATION_ERROR: ErrorKind = ErrorKind("Err-31807", 500, "Serialization Error");
    pub const DESERIALIZATION_ERROR: ErrorKind = ErrorKind("Err-01394", 500, "Deserialization Error");
    pub const NONE: ErrorKind = ErrorKind("Err-85941", 404, "None value found");
}

