use crate::{ErrorType, ErrorRepr};

pub struct GenericErrors;

impl GenericErrors {
    pub const GENERIC_ERROR: ErrorType = ErrorType(500, "Err-15452", "Generic Error");
    pub const UNKNOWN_ERROR: ErrorType = ErrorType(500, "Err-32583", "Unknown Error");
    pub const INVALID_CONFIGURATION: ErrorType = ErrorType(500, "Err-15160", "Invalid Configuration");
    pub const IO_ERROR: ErrorType = ErrorType(500, "Err-11553", "IO Error");
    pub const VALIDATION_ERROR: ErrorType = ErrorType(400, "Err-05612", "Validation Error");
    pub const SERIALIZATION_ERROR: ErrorType = ErrorType(500, "Err-31807", "Serialization Error");
    pub const DESERIALIZATION_ERROR: ErrorType = ErrorType(500, "Err-01394", "Deserialization Error");
}

impl From<serde_value::DeserializerError> for ErrorRepr {
    fn from(err: serde_value::DeserializerError) -> ErrorRepr {
        let mut res = ErrorRepr::new(GenericErrors::DESERIALIZATION_ERROR);
        *res.message_mut() = format!("{}", err);
        res
    }
}

impl From<serde_value::SerializerError> for ErrorRepr {
    fn from(err: serde_value::SerializerError) -> ErrorRepr {
        let mut res = ErrorRepr::new(GenericErrors::SERIALIZATION_ERROR);
        *res.message_mut() = format!("{}", err);
        res
    }
}
