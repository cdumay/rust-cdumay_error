use std::collections::BTreeMap;

use crate::ErrorKind;

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub extra: Option<BTreeMap<String, serde_json::Value>>,
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind.0, self.message)
    }
}
