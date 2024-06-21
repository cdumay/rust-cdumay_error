use ::{ErrorType, GenericErrors};

pub trait Registry {
    fn default() -> ErrorType {
        GenericErrors::GENERIC_ERROR
    }
    fn from_msgid<'a>(msgid: &'a str) -> ErrorType;
}