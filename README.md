# serde-value-flatten

[![Build Status](https://travis-ci.org/cdumay/rust-serde-value-flatten.svg?branch=master)](https://travis-ci.org/cdumay/rust-serde-value-flatten) 
[![Latest version](https://img.shields.io/crates/v/serde-value-flatten.svg)](https://crates.io/crates/serde-value-flatten)
[![Documentation](https://docs.rs/serde-value-flatten/badge.svg)](https://docs.rs/serde-value-flatten) 
![License](https://img.shields.io/crates/l/serde-value-flatten.svg)

Based on `serde-value`, `serde-value-flatten` provides a function to flatten any struct which
implement `serde::Serialize`.

## Quickstart

You can start using it by first adding it to your `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
serde_derive = "1.0"
serde_value_flatten = "0.1"
```

Then, create a structure which implement the `serde::Serialize` trait and use it with any
serde lib.

## Example

```rust
#[derive(Serialize, Clone, Debug)]
struct SubFoo {
    a: String,
    b: u64,
}

#[derive(Serialize, Clone, Debug)]
struct Foo {
    a: String,
    b: f64,
    c: Vec<i8>,
    d: SubFoo,
}

fn main() {
    let foo = Foo { a: "test".into(), b: 0.5, c: vec![5, 9], d: SubFoo { a: "subtest".into(), b: 695217 } };
    let ser = serde_value_flatten::to_flatten_maptree("_", Some("_"), &foo).unwrap();

    println!("{}", serde_json::to_string_pretty(&ser).unwrap());
}
```
**Output**:
```json
 {
  "_a": "test",
  "_b": 0.5,
  "_c_0": 5,
  "_c_1": 9,
  "_d_a": "subtest",
  "_d_b": 695217
}
```

## Feature ovh-ldp

The feature `ovh-ldp` allow to suffix fields names to suits to the [LDP naming conventions](https://docs.ovh.com/fr/logs-data-platform/field-naming-conventions/).

In your `Cargo.toml`, set:

```toml
[dependencies]
serde = "1.0"
serde_derive = "1.0"
serde_value_flatten = { version = "0.1", features = ["ovh-ldp"] }
```

Re-run the previous example, and now the output will be :

```json
{
  "_a": "test",
  "_b_float": 0.5,
  "_c_0_long": 5,
  "_c_1_long": 9,
  "_d_a": "subtest",
  "_d_b_double": 695217
}
```

License: BSD-3-Clause
