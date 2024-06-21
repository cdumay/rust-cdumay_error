# cdumay_error ![License: BSD-3](https://img.shields.io/badge/license-BSD--3-blue) [![cdumay_error on crates.io](https://img.shields.io/crates/v/cdumay_error)](https://crates.io/crates/cdumay_error) [![cdumay_error on docs.rs](https://docs.rs/cdumay_error/badge.svg)](https://docs.rs/cdumay_error) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/rust-cdumay_error)

[![Build Status](https://travis-ci.org/cdumay/rust-cdumay_error.svg?branch=master)][__link0]
[![Latest version](https://img.shields.io/crates/v/cdumay_error.svg)][__link1]
[![Documentation](https://docs.rs/cdumay_error/badge.svg)][__link2]
![License](https://img.shields.io/crates/l/cdumay_error.svg)

cdumay_error is a basic library used to standardize errors and serialize them using [serde][__link3].

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
    use cdumay_error::{ErrorBuilder, ErrorRepr, GenericErrors};
    use std::collections::BTreeMap;
    use serde_json::Value;

    let err = ErrorBuilder::from(GenericErrors::GENERIC_ERROR)
        .message("This is a useless generic error.".into())
        .extra({
            let mut extra = BTreeMap::new();
            extra.insert("context".into(), Value::String("Example".to_string()));
            extra
        })
        .build();
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

### Project Links

* Issues: https://github.com/cdumay/rust-cdumay_error/issues
* Documentation: https://docs.rs/cdumay_error


 [__link0]: https://travis-ci.org/cdumay/rust-cdumay_error
 [__link1]: https://crates.io/crates/cdumay_error
 [__link2]: https://docs.rs/cdumay_error
 [__link3]: https://docs.serde.rs/serde/
