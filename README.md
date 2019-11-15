# cdumay_error

[![Build Status](https://travis-ci.org/cdumay/rust-cdumay_errors.svg?branch=master)](https://travis-ci.org/cdumay/rust-cdumay_errors)
[![Latest version](https://img.shields.io/crates/v/cdumay_errors.svg)](https://crates.io/crates/cdumay_errors)
[![Documentation](https://docs.rs/cdumay_errors/badge.svg)](https://docs.rs/cdumay_errors)
![License](https://img.shields.io/crates/l/cdumay_errors.svg)

cdumay_error is a basic library used to standardize errors and serialize them using [serde](https://docs.serde.rs/serde/).

## Quickstart

_Cargo.toml_:
```toml
[dependencies]
cdumay_error = { git = "https://github.com/cdumay/rust-cdumay_error" }
serde_json = "1.0"
serde-value = "0.6"
```

_main.rs_:
```rust
extern crate cdumay_error;
extern crate serde_json;
extern crate serde_value;

fn main() {
    use cdumay_error::{ErrorBuilder, ErrorRepr, GenericErrors};
    use std::collections::HashMap;
    use serde_value::Value;

    let err = ErrorRepr::from(GenericErrors::GENERIC_ERROR)
        .set_message("This is a useless generic error.".into())
        .set_extra({
            let mut extra = HashMap::new();
            extra.insert("context".into(), Value::String("Example".to_string()));
            extra
        });
    println!("{}", serde_json::to_string_pretty(&err).unwrap());
}
```
_Output_:
```json
{
  "code": 500,
  "extra": {
    "context": "Example"
  },
  "message": "This is a useless generic error.",
  "msgid": "Err-15452"
}
```
## Project Links

- Issues: https://github.com/cdumay/rust-cdumay_errors/issues
- Documentation: https://docs.rs/cdumay_errors

License: MIT
