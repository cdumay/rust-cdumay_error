# cdumay_error

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

cdumay_error is a basic library used to standardize errors and to serialize them into json using [serde_json](https://docs.serde.rs/serde/).

## Quickstart

```toml
[dependencies]
cdumay_error = { git = "https://github.com/cdumay/cdumay-errors-rs" , features = ["http"] }
serde_json = "1.0"
serde-value = "0.5"
```

```rust
extern crate cdumay_error;
extern crate serde_json;
extern crate serde_value;

fn main() {
    use cdumay_error::ErrorReprBuilder;
    use cdumay_error::http::HttpErrors;
    use std::collections::HashMap;
    use serde_value::Value;


    let err = ErrorReprBuilder::new(HttpErrors::NOT_FOUND)
        .extra({
            let mut extra = HashMap::new();
            extra.insert("url".to_string(), Value::String("https://www.example.com/cdumay".to_string()));
            extra
        })
        .message("The requested resource could not be found but may be available in the future. Subsequent requests by the client are permissible.".to_string())
        .build();
    println!("{}", serde_json::to_string_pretty(&err).unwrap());
}
```

```json
{
  "code": 404,
  "extra": {
    "url": "https://www.example.com/cdumay"
  },
  "message": "The requested resource could not be found but may be available in the future. Subsequent requests by the client are permissible.",
  "msgid": "Err-18430"
}
```

## Features

- **http**: Defines cdumay_error::http::HttpErrors and implement the From trait for `hyper::Response<hyper::Body>` and `hyper::StatusCode`.
- **json**: Implement the From trait for `serde_json::Error`.

## Project Links

- Issues: https://github.com/cdumay/cdumay-errors-rs/issues
- Documentation: not available yet
