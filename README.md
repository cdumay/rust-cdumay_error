# cdumay_error ![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue) [![cdumay_error on crates.io](https://img.shields.io/crates/v/cdumay_error)](https://crates.io/crates/cdumay_error) [![cdumay_error on docs.rs](https://docs.rs/cdumay_error/badge.svg)](https://docs.rs/cdumay_error) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/rust-cdumay_error)

cdumay_error is a basic library used to standardize errors and serialize them using [serde][__link0].

### Quickstart

*Cargo.toml*:

```toml
[dependencies]
cdumay_core = "0.0"
cdumay_error = "0.3"
serde_json = "1.0"
```

*main.rs*:

```rust
extern crate cdumay_error;
extern crate cdumay_core;
extern crate serde_json;

use cdumay_core::Value;
use cdumay_error::{ErrorBuilder, GenericErrors, JsonError};
use std::collections::BTreeMap;

fn main() {

    let err = ErrorBuilder::from(GenericErrors::GENERIC_ERROR)
        .message("This is a useless generic error.".to_string())
        .extra({
            let mut extra = BTreeMap::new();
            extra.insert("context".into(), Value::String("Example".to_string()));
            extra
        })
        .build();
    println!("{}", serde_json::to_string_pretty(&JsonError::from(err)).unwrap());
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
