use std::collections::BTreeMap;

use serde_value::Value;

use crate::{ErrorInfo, ErrorRepr, ErrorType};

pub struct ErrorBuilder {
    pub code: u16,
    pub extra: Option<BTreeMap<String, Value>>,
    pub message: Option<String>,
    pub msgid: String,
}

impl ErrorBuilder {
    pub fn new(code: u16, msgid: String) -> ErrorBuilder {
        ErrorBuilder { code, extra: None, message: None, msgid }
    }
    pub fn extra(mut self, extra: BTreeMap<String, Value>) -> ErrorBuilder {
        self.extra = Some(extra);
        self
    }
    pub fn message(mut self, message: String) -> ErrorBuilder {
        self.message = Some(message);
        self
    }
    pub fn build(self) -> ErrorRepr {
        ErrorRepr {
            code: self.code,
            extra: self.extra,
            message: self.message.unwrap_or(String::new()),
            msgid: self.msgid,
        }
    }
}

impl From<ErrorType> for ErrorBuilder {
    fn from(etype: ErrorType) -> ErrorBuilder {
        ErrorBuilder {
            code: etype.code(),
            extra: None,
            message: Some(etype.message().to_string()),
            msgid: etype.msgid().to_string(),
        }
    }
}