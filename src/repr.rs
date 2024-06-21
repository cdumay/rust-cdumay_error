use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;

use serde_json::Value;

use ::{ErrorInfo, GenericErrors,ErrorType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorRepr {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<BTreeMap<String, Value>>,
    pub message: String,
    pub msgid: String,
}

impl Default for ErrorRepr {
    fn default() -> ErrorRepr {
        ErrorRepr::from(GenericErrors::GENERIC_ERROR)
    }
}

impl Error for ErrorRepr {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl std::fmt::Display for ErrorRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.msgid, self.message)
    }
}

impl From<ErrorType> for ErrorRepr {
    fn from(etype: ErrorType) -> ErrorRepr {
        ErrorRepr {
            code: etype.code(),
            extra: None,
            message: etype.message().to_string(),
            msgid: etype.msgid().to_string(),
        }
    }
}
