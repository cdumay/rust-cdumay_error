use std::collections::BTreeMap;

pub trait ErrorInfo {
    fn code(&self) -> u16;
    fn extra(&self) -> Option<BTreeMap<String, serde_json::Value>>;
    fn message(&self) -> String;
    fn msgid(&self) -> String;
}