use std::collections::BTreeMap;
use crate::{Error, ErrorKind, GenericErrors};

pub struct ErrorBuilder {
    kind: ErrorKind,
    message: Option<String>,
    extra: Option<BTreeMap<String, cdumay_core::Value>>,
}

impl ErrorBuilder {
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
    pub fn extra(mut self, extra: BTreeMap<String, cdumay_core::Value>) -> Self {
        self.extra = Some(extra);
        self
    }
    pub fn build(self) -> Error {
        let message = self.message.unwrap_or(self.kind.2.to_string());
        Error {
            kind: self.kind,
            message,
            extra: self.extra,
        }
    }
}

impl From<ErrorKind> for ErrorBuilder {
    fn from(value: ErrorKind) -> Self {
        ErrorBuilder {
            kind: value,
            message: None,
            extra: None,
        }
    }
}


impl Default for ErrorBuilder {
    fn default() -> ErrorBuilder {
        ErrorBuilder::from(GenericErrors::GENERIC_ERROR)
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use cdumay_core::Value;

    use crate::{ErrorBuilder, GenericErrors};

    #[test]
    fn test_builder() {
        let builder = ErrorBuilder::default()
            .message("Test".to_string())
            .extra({
                let mut extra = BTreeMap::new();
                extra.insert("context".into(), Value::String("Example".to_string()));
                extra
            });
        assert_eq!(builder.kind, GenericErrors::GENERIC_ERROR);
        assert_eq!(builder.message, Some("Test".to_string()));
    }
}
