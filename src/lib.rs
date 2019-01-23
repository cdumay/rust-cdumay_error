#![feature(try_trait)]
extern crate serde;
extern crate serde_value;

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "http")]
extern crate hyper;
#[cfg(feature = "json")]
extern crate serde_json;

pub mod generic;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "json")]
pub mod json;

#[derive(Clone, Copy)]
pub struct ErrorType(u16, &'static str, &'static str);

impl ErrorType {
    fn id(&self) -> &str { self.1 }
    fn code(&self) -> &u16 { &self.0 }
    fn message(&self) -> &str { self.2 }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorRepr {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<std::collections::HashMap<String, serde_value::Value>>,
    message: String,
    msgid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<String>,
}

impl ErrorRepr {
    pub fn new(etype: ErrorType) -> ErrorRepr {
        ErrorRepr {
            code: *etype.code(),
            message: etype.message().to_string(),
            msgid: etype.id().to_string(),
            extra: None,
            stack: None,
        }
    }
    pub fn code(&self) -> &u16 { &self.code }
    pub fn msgid(&self) -> &String { &self.msgid }

    pub fn extra(&self) -> &Option<std::collections::HashMap<String, serde_value::Value>> { &self.extra }
    pub fn extra_mut(&mut self) -> &mut Option<std::collections::HashMap<String, serde_value::Value>> { &mut self.extra }

    pub fn message(&self) -> &String { &self.message }
    pub fn message_mut(&mut self) -> &mut String { &mut self.message }

    pub fn stack(&self) -> &Option<String> { &self.stack }
    pub fn stack_mut(&mut self) -> &mut Option<String> { &mut self.stack }
}

impl Default for ErrorRepr {
    fn default() -> ErrorRepr {
        ErrorRepr {
            msgid: "Err-00000".to_string(),
            message: String::new(),
            code: 500,
            extra: None,
            stack: None,
        }
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

pub trait Registry {
    fn from_msgid(msgid: &str) -> ErrorType;
}
