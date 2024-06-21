# cdumay_error ![License: BSD-3](https://img.shields.io/badge/license-BSD--3-blue) [![cdumay_error on crates.io](https://img.shields.io/crates/v/cdumay_error)](https://crates.io/crates/cdumay_error) [![cdumay_error on docs.rs](https://docs.rs/cdumay_error/badge.svg)](https://docs.rs/cdumay_error) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/rust-cdumay_error)

cdumay_error is a basic library used to standardize errors and serialize them using [serde][__link0].

### Quickstart

*Cargo.toml*:

```toml
[dependencies]
cdumay_error = { git = "https://github.com/cdumay/rust-cdumay_error" }
serde_json = "1.0"
```

*main.rs*:

```rust
extern crate cdumay_error;
extern crate serde_json;

fn main() {
    use cdumay_error::{ErrorRepr, GenericErrors};
    use std::collections::BTreeMap;
    use serde_json::Value;

    let mut err = ErrorRepr::from(GenericErrors::GENERIC_ERROR);
    err.message = "This is a useless generic error.".to_string();
    err.extra = Some({
        let mut extra = BTreeMap::new();
        extra.insert("context".into(), Value::String("Example".to_string()));
        extra
    });
    println!("{}", serde_json::to_string_pretty(&err).unwrap());
}
```

*Output*:

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


 [__link0]: https://docs.serde.rs/serde/
