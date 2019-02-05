#![feature(try_trait)]
extern crate serde;
extern crate serde_value;

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "http")]
extern crate hyper;
#[cfg(feature = "json")]
extern crate serde_json;

pub mod types;
pub mod repr;

pub trait ErrorBuilder {
    fn set_extra(self, extra: std::collections::HashMap<String, serde_value::Value>) -> Self;
    fn set_message(self, message: String) -> Self;
}

pub trait ErrorSetters {
    fn code_mut(&mut self) -> &mut u16;
    fn msgid_mut(&mut self) -> &mut String;
    fn extra_mut(&mut self) -> &mut Option<std::collections::HashMap<String, serde_value::Value>>;
    fn message_mut(&mut self) -> &mut String;
}

pub trait ErrorGetters {
    fn code(&self) -> &u16;
    fn msgid(&self) -> &String;
    fn extra(&self) -> &Option<std::collections::HashMap<String, serde_value::Value>>;
    fn message(&self) -> &String;
}