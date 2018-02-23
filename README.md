# cdumay_errors

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

cdumay_errors is a basic library used to standardize errors and to serialize them into json using [serde_json](https://docs.serde.rs/serde/).

## Quickstart

```toml
[dependencies]
cdumay_errors = "0.1"
```

```rust
extern crate cdumay_errors;
extern crate serde_json;

use cdumay_errors::Error;
use serde_json::{Map, Value, to_string_pretty};

let mut context: Map<String, Value> = Map::new();
context.insert("url".to_string(), Value::from("https://example.dumay"));

let err = Error::new("Not Found".to_string(), Some(404), Some("NotFound".to_string()), Some(context));
println!("{}", to_string_pretty(&err).unwrap());
```

```json
{
  "code": 404,
  "extra": {
    "url": "https://example.dumay"
  },
  "message": "Not Found",
  "msgid": "NotFound"
}
```

## Features

- [cdumay-result](https://github.com/cdumay/cdumay-result-rs): A library to serialize and deserialize result using JSON.

## Project Links

- Issues: https://github.com/cdumay/cdumay-errors-rs/issues
- Documentation: 
