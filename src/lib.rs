//! cdumay_error is a basic library used to standardize errors and serialize them using [serde](https://docs.serde.rs/serde/).
//!
//! ## Quickstart
//!
//! _Cargo.toml_:
//! ```toml
//! [dependencies]
//! cdumay_error = { git = "https://github.com/cdumay/rust-cdumay_error" }
//! serde_json = "1.0"
//! ```
//!
//! _main.rs_:
//! ```rust
//! extern crate cdumay_error;
//! extern crate serde_json;
//!
//! fn main() {
//!     use cdumay_error::{ErrorRepr, GenericErrors};
//!     use std::collections::BTreeMap;
//!     use serde_json::Value;
//!
//!     let mut err = ErrorRepr::from(GenericErrors::GENERIC_ERROR);
//!     err.message = "This is a useless generic error.".to_string();
//!     err.extra = Some({
//!         let mut extra = BTreeMap::new();
//!         extra.insert("context".into(), Value::String("Example".to_string()));
//!         extra
//!     });
//!     println!("{}", serde_json::to_string_pretty(&err).unwrap());
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

pub use common::GenericErrors;
pub use info::ErrorInfo;
pub use registry::Registry;
pub use types::ErrorType;
pub use repr::ErrorRepr;

mod repr;
mod common;
mod registry;
mod info;
mod types;

