use std::collections::BTreeMap;

use crate::ErrorKind;

#[derive(Debug)]
pub struct Registry {
    pub msgids: BTreeMap<String, ErrorKind>,
    pub codes: BTreeMap<u16, ErrorKind>,
}

impl Registry {
    pub fn register(mut self, errors: Vec<ErrorKind>) -> Self {
        for error in errors {
            self.codes.insert(error.1, error.clone());
            self.msgids.insert(error.0.to_string(), error);
        }
        self
    }
}

impl Default for Registry {
    fn default() -> Self {
        Registry {
            msgids: Default::default(),
            codes: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{GenericErrors, Registry};

    #[test]
    fn test_register() {
        let reg = Registry::default()
            .register(vec![GenericErrors::GENERIC_ERROR, GenericErrors::IO_ERROR]);
        assert_eq!(reg.codes.len(), 1);
        assert_eq!(reg.msgids.len(), 2);
        assert_eq!(reg.msgids.contains_key("Err-15452"), true);
    }
}