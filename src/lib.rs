//! cdumay_error is a basic library used to standardize errors and serialize them using [serde](https://docs.serde.rs/serde/).
//!
//! ## Quickstart
//!
//! _Cargo.toml_:
//! ```toml
//! [dependencies]
//! cdumay_error = "0.3"
//! serde_json = "1.0"
//! ```
//!
//! _main.rs_:
//! ```rust
//! extern crate cdumay_error;
//! extern crate serde_json;
//!
//! use cdumay_error::{ErrorBuilder, GenericErrors, JsonError};
//! use std::collections::BTreeMap;
//! use serde_json::Value;
//!
//! fn main() {
//!
//!     let err = ErrorBuilder::from(GenericErrors::GENERIC_ERROR)
//!         .message("This is a useless generic error.".to_string())
//!         .extra({
//!             let mut extra = BTreeMap::new();
//!             extra.insert("context".into(), Value::String("Example".to_string()));
//!             extra
//!         })
//!         .build();
//!     println!("{}", serde_json::to_string_pretty(&JsonError::from(err)).unwrap());
//! }
//! ```
//! _Output_:
//! ```json
//! {
//!   "code": 500,
//!   "extra": {
//!     "context": "Example"
//!   },
//!   "message": "This is a useless generic error.",
//!   "msgid": "Err-15452"
//! }
//! ```


extern crate serde;
extern crate serde_json;

pub use builder::ErrorBuilder;
pub use error::Error;
pub use jsonify::JsonError;
pub use kind::{ErrorKind, GenericErrors};
pub use registry::Registry;

mod registry;
mod kind;
mod error;
mod jsonify;
mod builder;