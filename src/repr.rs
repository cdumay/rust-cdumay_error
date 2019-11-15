use std::collections::BTreeMap;
use std::error::Error;

use serde_value::Value;

use crate::{ErrorInfo, GenericErrors};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorRepr {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<BTreeMap<String, Value>>,
    pub message: String,
    pub msgid: String,
}

impl<I: ErrorInfo> From<I> for ErrorRepr {
    fn from(info: I) -> ErrorRepr {
        ErrorRepr {
            msgid: info.msgid().clone(),
            message: info.message().clone(),
            code: info.code().clone(),
            extra: info.extra().clone(),
        }
    }
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
