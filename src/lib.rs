// Copyright 2019-present, OVH SAS
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![serde-value-flatten on crates.io](https://img.shields.io/crates/v/serde-value-flatten)](https://crates.io/crates/serde-value-flatten)
//! [![serde-value-flatten on docs.rs](https://docs.rs/serde-value-flatten/badge.svg)](https://docs.rs/serde-value-flatten)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/serde-value-flatten)
//!
//! Based on `serde-value`, `serde-value-flatten` provides a function to flatten any struct which
//! implement `serde::Serialize`.
//!
//! # Quickstart
//!
//! You can start using it by first adding it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde-value = "0.7"
//! serde-value-flatten = "0.2"
//! ```
//!
//! Then, create a structure which implement the `serde::Serialize` trait and use it with any
//! serde lib.
//!
//! # Example
//!
//! ```rust
//! use serde::Serialize;
//!
//! #[derive(Serialize, Clone, Debug)]
//! struct SubFoo {
//!     a: String,
//!     b: u64,
//! }
//!
//! #[derive(Serialize, Clone, Debug)]
//! struct Foo {
//!     a: String,
//!     b: f64,
//!     c: Vec<i8>,
//!     d: SubFoo,
//! }
//!
//! fn main() {
//!     let foo = Foo { a: "test".into(), b: 0.5, c: vec![5, 9], d: SubFoo { a: "subtest".into(), b: 695217 } };
//!     let ser = serde_value_flatten::to_flatten_maptree("_", Some("_"), &foo).unwrap();
//!
//!     println!("{}", serde_json::to_string_pretty(&ser).unwrap());
//! }
//! ```
//! **Output**:
//! ```json
//!  {
//!   "_a": "test",
//!   "_b": 0.5,
//!   "_c_0": 5,
//!   "_c_1": 9,
//!   "_d_a": "subtest",
//!   "_d_b": 695217
//! }
//! ```
//!
//! # Feature ovh-ldp
//!
//! The feature `ovh-ldp` allow to suffix fields names to suits to the [LDP naming conventions](https://docs.ovh.com/fr/logs-data-platform/field-naming-conventions/).
//!
//! In your `Cargo.toml`, set:
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde-value = "0.7"
//! serde-value-flatten = { version = "0.2", features = ["ovh-ldp"] }
//! ```
//!
//! Re-run the previous example, and now the output will be :
//!
//! ```json
//! {
//!   "_a": "test",
//!   "_b_float": 0.5,
//!   "_c_0_long": 5,
//!   "_c_1_long": 9,
//!   "_d_a": "subtest",
//!   "_d_b_double": 695217
//! }
//! ```
#![doc(
    html_logo_url = "https://eu.api.ovh.com/images/com-square-bichro.png",
    html_favicon_url = "https://www.ovh.com/favicon.ico"
)]
#![deny(warnings, missing_docs)]
extern crate serde;
extern crate serde_value;

use std::collections::BTreeMap;

mod ser;

#[inline]
/// Function to flatten any structure which implement the `serde::Serialize`.
///
/// Keys or attributes names will be concatenated as path (e.g: `{ a: {b: 5}} -> { a_b: 5 }`).
///
/// # Configuration
///
/// * **key_separator**: Separator to use at each level change.
/// * **prefix**: Prefix to use on the first level before the attribute / key / index name
///
/// ## Example
///
/// ```rust
/// use serde::Serialize;
///
/// #[derive(Serialize, Clone, Debug)]
/// struct SubFoo {
///     a: String,
///     b: u64,
/// }
///
/// #[derive(Serialize, Clone, Debug)]
/// struct Foo {
///     a: String,
///     b: f64,
///     c: Vec<i8>,
///     d: SubFoo,
/// }
///
/// fn main() {
///     let foo = Foo { a: "test".into(), b: 0.5, c: vec![5, 9], d: SubFoo { a: "subtest".into(), b: 695217 } };
///     let ser = serde_value_flatten::to_flatten_maptree("|", None, &foo).unwrap();
///
///     println!("{}", serde_json::to_string_pretty(&ser).unwrap());
/// }
/// ```
/// **Output**:
/// ```json
/// {
///   "a": "test",
///   "b": 0.5,
///   "c|0": 5,
///   "c|1": 9,
///   "d|a": "subtest",
///   "d|b": 695217
/// }
/// ```
pub fn to_flatten_maptree<T: ?Sized>(
    key_separator: &str,
    prefix: Option<&str>,
    src: &T,
) -> Result<BTreeMap<String, serde_value::Value>, serde_value::SerializerError>
where
    T: serde::Serialize,
{
    Ok(
        ser::FlatSerializer::new(key_separator.into(), prefix.unwrap_or("").into()).disassemble(
            "",
            "",
            &serde_value::to_value(src)?,
        ),
    )
}
