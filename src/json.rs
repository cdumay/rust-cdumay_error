use crate::{ErrorRepr, ErrorReprBuilder};
use crate::generic::GenericErrors;

impl From<serde_json::Error> for ErrorRepr {
    fn from(err: serde_json::Error) -> ErrorRepr {
        ErrorReprBuilder::new(GenericErrors::SERIALIZATION_ERROR)
            .message(format!("{}", err))
            .build()
    }
}
