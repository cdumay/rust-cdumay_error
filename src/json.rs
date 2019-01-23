use crate::ErrorRepr;
use crate::generic::GenericErrors;

impl From<serde_json::Error> for ErrorRepr {
    fn from(err: serde_json::Error) -> ErrorRepr {
        let mut res = ErrorRepr::new(GenericErrors::SERIALIZATION_ERROR);
        *res.message_mut() = format!("{}", err);
        res
    }
}
