use ::{ErrorType, Registry};

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
    fn from_msgid<'a>(msgid: &'a str) -> ErrorType {
        match msgid {
            "Err-01394" => Self::DESERIALIZATION_ERROR,
            "Err-05612" => Self::VALIDATION_ERROR,
            "Err-11553" => Self::IO_ERROR,
            "Err-15160" => Self::INVALID_CONFIGURATION,
            "Err-15452" => Self::GENERIC_ERROR,
            "Err-31807" => Self::SERIALIZATION_ERROR,
            "Err-32583" => Self::UNKNOWN_ERROR,
            "Err-85941" => Self::NONE,
            _ => Self::default(),
        }
    }
}
