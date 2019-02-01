# cdumay_errors

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

cdumay_errors is a basic library used to standardize errors and to serialize them into json using [serde_json](https://docs.serde.rs/serde/).

## Quickstart

```toml
[dependencies]
cdumay_error = "0.1"
```

```rust
extern crate cdumay_error;
extern crate serde_json;

fn main() {
    use cdumay_error::ErrorReprBuilder;
    use cdumay_error::http::HttpErrors;
    
    let err = ErrorReprBuilder::new(HttpErrors::NOT_FOUND)
        .extra({
            let mut extra = HashMap::new();
            extra.insert("url".to_string(), Value::from("https://www.example.com/cdumay"));
            extra
        })
        .message("The requested resource could not be found but may be available in the future. Subsequent requests by the client are permissible.".to_string())
        .build();
    println!("{}", to_string_pretty(&err).unwrap());
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

## Project Links

- Issues: https://github.com/cdumay/cdumay-errors-rs/issues
- Documentation: not available yet
