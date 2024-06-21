use std::collections::BTreeMap;

use serde_json::Value;
use ::ErrorInfo;

#[derive(Clone, Copy, Debug)]
pub struct ErrorType(pub u16, pub &'static str, pub &'static str);

impl ErrorInfo for ErrorType {
    fn code(&self) -> u16 {
        self.0
    }
    fn extra(&self) -> Option<BTreeMap<String, Value>> {
        None
    }
    fn message(&self) -> String {
        self.2.to_string()
    }
    fn msgid(&self) -> String {
        self.1.to_string()
    }
}
