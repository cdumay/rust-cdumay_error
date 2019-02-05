use crate::{ErrorGetters, ErrorBuilder};
use crate::types::ErrorType;
use crate::types::generic::GenericErrors;
use serde_value::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorRepr {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<HashMap<String, Value>>,
    message: String,
    msgid: String,
}


impl From<ErrorType> for ErrorRepr {
    fn from(etype: ErrorType) -> ErrorRepr {
        ErrorRepr {
            msgid: etype.id().to_string(),
            message: String::new(),
            code: *etype.code(),
            extra: None,
        }
    }
}


impl Default for ErrorRepr {
    fn default() -> ErrorRepr {
        ErrorRepr::from(GenericErrors::GENERIC_ERROR)
    }
}

impl ErrorGetters for ErrorRepr {
    fn code(&self) -> &u16 { &self.code }
    fn msgid(&self) -> &String { &self.msgid }
    fn extra(&self) -> &Option<HashMap<String, Value>> { &self.extra }
    fn message(&self) -> &String { &self.message }
}

impl ErrorBuilder for ErrorRepr {
    fn set_extra(mut self, extra: HashMap<String, Value>) -> ErrorRepr {
        self.extra = Some(extra);
        self
    }
    fn set_message(mut self, message: String) -> ErrorRepr {
        self.message = message;
        self
    }
}

impl std::error::Error for ErrorRepr {
    fn description(&self) -> &str {
        self.message().as_str()
    }
}

impl std::fmt::Display for ErrorRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.msgid(), self.message())
    }
}

impl From<serde_value::DeserializerError> for ErrorRepr {
    fn from(err: serde_value::DeserializerError) -> ErrorRepr {
        ErrorRepr::from(GenericErrors::DESERIALIZATION_ERROR)
            .set_message(format!("{}", err))
    }
}

impl From<serde_value::SerializerError> for ErrorRepr {
    fn from(err: serde_value::SerializerError) -> ErrorRepr {
        ErrorRepr::from(GenericErrors::SERIALIZATION_ERROR)
            .set_message(format!("{}", err))
    }
}

impl From<std::io::Error> for ErrorRepr {
    fn from(err: std::io::Error) -> ErrorRepr {
        ErrorRepr::from(GenericErrors::IO_ERROR)
            .set_message(format!("{}", err))
    }
}

impl From<std::option::NoneError> for ErrorRepr {
    fn from(_err: std::option::NoneError) -> ErrorRepr {
        ErrorRepr::from(GenericErrors::NONE)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for ErrorRepr {
    fn from(err: serde_json::Error) -> ErrorRepr {
        ErrorRepr::from(GenericErrors::SERIALIZATION_ERROR)
            .set_message(format!("{}", err))
    }
}

#[cfg(feature = "http")]
impl From<ErrorRepr> for hyper::Response<hyper::Body> {
    fn from(err: ErrorRepr) -> hyper::Response<hyper::Body> {
        let mut response = hyper::Response::new(hyper::Body::empty());
        *response.status_mut() = hyper::StatusCode::from_u16(*err.code())
            .unwrap_or(hyper::StatusCode::INTERNAL_SERVER_ERROR);

        if let Ok(jdata) = serde_json::to_string(&err) {
            response.headers_mut().insert(hyper::header::CONTENT_TYPE, "application/json".parse().unwrap());
            *response.body_mut() = hyper::Body::from(jdata);
        }
        response
    }
}