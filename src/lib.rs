#[deny(warnings)]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "cdumay-result")]
extern crate cdumay_result;

#[derive(Serialize, Deserialize)]
pub struct Error {
    code: u16,
    extra: serde_json::Map<String, serde_json::Value>,
    message: String,
    msgid: Option<String>,
}

impl Error {
    pub fn new(message: String, code: Option<u16>, msgid: Option<String>, extra: Option<serde_json::Map<String, serde_json::Value>>) -> Error {
        Error {
            code: code.unwrap_or(1),
            extra: extra.unwrap_or(serde_json::Map::new()),
            message: message,
            msgid: msgid,
        }
    }
    pub fn code(&self) -> u16 { self.code }
    pub fn extra(&self) -> serde_json::Map<String, serde_json::Value> { self.extra.clone() }
    pub fn message(&self) -> String { self.message.clone() }
    pub fn msgid(&self) -> Option<String> { self.msgid.clone() }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error<code={}, message={}", self.code(), self.message())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error {}: {}", self.code(), self.message())
    }
}

#[cfg(feature = "cdumay-result")]
use cdumay_result::BaseResult;

#[cfg(feature = "cdumay-result")]
impl From<cdumay_result::ExecutionResult> for Error {
    fn from(result: cdumay_result::ExecutionResult) -> Error {
        Error::new(result.stderr(), Some(result.retcode()), None, Some(result.retval()))
    }
}