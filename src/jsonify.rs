use std::collections::BTreeMap;

use serde::Serialize;

use crate::Error;

#[derive(Serialize, Debug, Clone)]
pub struct JsonError {
    code: u16,
    msgid: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<BTreeMap<String, cdumay_core::Value>>,
}

impl From<Error> for JsonError {
    fn from(value: Error) -> Self {
        JsonError {
            code: value.kind.1,
            msgid: value.kind.0.to_string(),
            message: value.message,
            extra: value.extra,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use cdumay_core::Value;

    use crate::{ErrorBuilder, JsonError};

    #[test]
    fn test_serializer() {
        let error = ErrorBuilder::default()
            .extra({
                let mut extra = BTreeMap::new();
                extra.insert("Hello".into(), Value::String("World".to_string()));
                extra
            })
            .build();
        let repr = JsonError::from(error);
        assert_eq!(
            serde_json::to_string(&repr).unwrap(),
            "{\"code\":500,\"msgid\":\"Err-15452\",\"message\":\"Generic Error\",\"extra\":{\"Hello\":\"World\"}}"
        );
    }
}
