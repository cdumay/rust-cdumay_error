pub mod generic;
#[cfg(feature = "http")]
pub mod http;

#[derive(Clone, Copy)]
pub struct ErrorType(pub u16, pub  &'static str, pub  &'static str);

impl ErrorType {
    pub fn id(&self) -> &str { self.1 }
    pub fn code(&self) -> &u16 { &self.0 }
    pub fn message(&self) -> &str { self.2 }
}

pub trait Registry {
    fn default() -> ErrorType { generic::GenericErrors::GENERIC_ERROR }
    fn from_msgid(msgid: &str) -> ErrorType;
}