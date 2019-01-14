#![feature(try_trait)]
extern crate serde;
extern crate serde_value;

#[macro_use]
extern crate serde_derive;


#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<std::collections::HashMap<String, serde_value::Value>>,
    message: String,
    msgid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<String>,
}

pub trait ErrorProperties {
    fn code(&self) -> &u16;
    fn code_mut(&mut self) -> &mut u16;
    fn extra(&self) -> &Option<std::collections::HashMap<String, serde_value::Value>>;
    fn extra_mut(&mut self) -> &mut Option<std::collections::HashMap<String, serde_value::Value>>;
    fn message(&self) -> &String;
    fn message_mut(&mut self) -> &mut String;
    fn msgid(&self) -> &String;
    fn msgid_mut(&mut self) -> &mut String;
    fn stack(&self) -> &Option<String>;
    fn stack_mut(&mut self) -> &mut Option<String>;
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

impl ErrorProperties for Error {
    fn code(&self) -> &u16 { &self.code }
    fn code_mut(&mut self) -> &mut u16 { &mut self.code }

    fn extra(&self) -> &Option<std::collections::HashMap<String, serde_value::Value>> { &self.extra }
    fn extra_mut(&mut self) -> &mut Option<std::collections::HashMap<String, serde_value::Value>> { &mut self.extra }

    fn message(&self) -> &String { &self.message }
    fn message_mut(&mut self) -> &mut String { &mut self.message }

    fn msgid(&self) -> &String { &self.msgid }
    fn msgid_mut(&mut self) -> &mut String { &mut self.msgid }

    fn stack(&self) -> &Option<String> { &self.stack }
    fn stack_mut(&mut self) -> &mut Option<String> { &mut self.stack }
}


impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message().as_str()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.msgid(), self.message())
    }
}


impl From<std::option::NoneError> for Error {
    fn from(_err: std::option::NoneError) -> Error {
        Error {
            msgid: "Err-08414".to_string(),
            message: "Not Found".to_string(),
            code: 404,
            extra: None,
            stack: None,
        }
    }
}