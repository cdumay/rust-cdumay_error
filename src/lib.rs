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
pub struct ErrorType(pub u16, pub  &'static str, pub  &'static str);

impl ErrorType {
    pub fn id(&self) -> &str { self.1 }
    pub fn code(&self) -> &u16 { &self.0 }
    pub fn message(&self) -> &str { self.2 }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorRepr {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<std::collections::HashMap<String, serde_value::Value>>,
    message: String,
    msgid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<String>,
}

pub trait ErrorProperties {
    fn new(etype: ErrorType) -> Self;
    fn code(&self) -> &u16;
    fn msgid(&self) -> &String;
    fn extra(&self) -> &Option<std::collections::HashMap<String, serde_value::Value>>;
    fn message(&self) -> &String;
    fn stack(&self) -> &Option<String>;
    fn repr(&self) -> ErrorRepr;
}

impl ErrorProperties for ErrorRepr {
    fn new(etype: ErrorType) -> ErrorRepr {
        ErrorRepr {
            code: *etype.code(),
            message: etype.message().to_string(),
            msgid: etype.id().to_string(),
            extra: None,
            stack: None,
        }
    }
    fn code(&self) -> &u16 { &self.code }
    fn msgid(&self) -> &String { &self.msgid }
    fn extra(&self) -> &Option<std::collections::HashMap<String, serde_value::Value>> { &self.extra }
    fn message(&self) -> &String { &self.message }
    fn stack(&self) -> &Option<String> { &self.stack }
    fn repr(&self) -> ErrorRepr { self.clone() }
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
    fn default() -> ErrorType { generic::GenericErrors::GENERIC_ERROR }
    fn from_msgid(msgid: &str) -> ErrorType;
}

pub struct ErrorReprBuilder {
    code: u16,
    extra: Option<std::collections::HashMap<String, serde_value::Value>>,
    message: String,
    msgid: String,
    stack: Option<String>,
}

impl ErrorReprBuilder {
    pub fn new(etype: ErrorType) -> ErrorReprBuilder {
        ErrorReprBuilder {
            code: *etype.code(),
            message: etype.message().to_string(),
            msgid: etype.id().to_string(),
            extra: None,
            stack: None,
        }
    }
    pub fn extra(mut self, extra: std::collections::HashMap<String, serde_value::Value>) -> ErrorReprBuilder {
        self.extra = Some(extra);
        self
    }
    pub fn message(mut self, message: String) -> ErrorReprBuilder {
        self.message = message;
        self
    }
    pub fn stack(mut self, stack: String) -> ErrorReprBuilder {
        self.stack = Some(stack);
        self
    }
    pub fn build(self) -> ErrorRepr {
        ErrorRepr {
            code: self.code,
            message: self.message,
            msgid: self.msgid,
            extra: self.extra,
            stack: self.stack,
        }
    }
}