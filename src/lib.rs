#![feature(try_trait)]
extern crate serde;
extern crate serde_value;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

pub struct ErrorBuilder {
    code: u16,
    extra: Option<HashMap<String, serde_value::Value>>,
    message: Option<String>,
    msgid: String,
    stack: Option<String>,
}

impl ErrorBuilder {
    pub fn new(msgid: &str) -> ErrorBuilder {
        ErrorBuilder {
            code: 500,
            extra: None,
            message: None,
            msgid: msgid.to_string(),
            stack: None,
        }
    }
    pub fn code(mut self, code: u16) -> ErrorBuilder {
        self.code = code;
        self
    }
    pub fn extra(mut self, extra: HashMap<String, serde_value::Value>) -> ErrorBuilder {
        self.extra = Some(extra);
        self
    }
    pub fn message(mut self, message: String) -> ErrorBuilder {
        self.message = Some(message);
        self
    }
    pub fn stack(mut self, stack: String) -> ErrorBuilder {
        self.stack = Some(stack);
        self
    }
    pub fn build(self) -> Error {
        Error {
            code: self.code,
            extra: self.extra,
            message: self.message.unwrap_or("".to_string()),
            msgid: self.msgid,
            stack: self.stack,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<HashMap<String, serde_value::Value>>,
    message: String,
    msgid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<String>,
}

impl Error {
    pub fn code(&self) -> &u16 {
        &self.code
    }
    pub fn code_mut(&mut self) -> &mut u16 { &mut self.code }

    pub fn extra(&self) -> &Option<HashMap<String, serde_value::Value>> {
        &self.extra
    }
    pub fn extra_mut(&mut self) -> &mut Option<HashMap<String, serde_value::Value>> {
        &mut self.extra
    }

    pub fn message(&self) -> &String {
        &self.message
    }
    pub fn message_mut(&mut self) -> &mut String { &mut self.message }

    pub fn msgid(&self) -> &String { &self.msgid }
    pub fn msgid_mut(&mut self) -> &mut String { &mut self.msgid }

    pub fn stack(&self) -> &Option<String> {
        &self.stack
    }
    pub fn stack_mut(&mut self) -> &mut Option<String> {
        &mut self.stack
    }
}

impl Default for Error {
    fn default() -> Error {
        Error {
            msgid: "Err-00000".to_string(),
            message: String::new(),
            code: 500,
            extra: None,
            stack: None,
        }
    }
}


impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.msgid, self.message)
    }
}


impl From<std::option::NoneError> for Error {
    fn from(err: std::option::NoneError) -> Error {
        Error {
            msgid: "Err-08414".to_string(),
            message: "Not Found".to_string(),
            code: 404,
            extra: None,
            stack: None,
        }
    }
}