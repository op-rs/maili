## `maili-serde`

<a href="https://github.com/op-rs/maili/actions/workflows/ci.yml"><img src="https://github.com/op-rs/maili/actions/workflows/ci.yml/badge.svg?label=ci" alt="CI"></a>
<a href="https://crates.io/crates/maili-serde"><img src="https://img.shields.io/crates/v/maili-serde.svg" alt="maili-serde crate"></a>
<a href="https://github.com/op-rs/maili/blob/main/LICENSE-MIT"><img src="https://img.shields.io/badge/License-MIT-d1d1f6.svg?label=license&labelColor=2a2f35" alt="MIT License"></a>
<a href="https://github.com/op-rs/maili/blob/main/LICENSE-APACHE"><img src="https://img.shields.io/badge/License-APACHE-d1d1f6.svg?label=license&labelColor=2a2f35" alt="Apache License"></a>
<a href="https://op-rs.github.io/maili"><img src="https://img.shields.io/badge/Book-854a15?logo=mdBook&labelColor=2a2f35" alt="Book"></a>

Serde related helpers for Maili.

### Graceful u128 Serialization

The primary functionality this crate exposes is a helper for
gracefully working with the `u128` type during deserialization.

By itself, `u128` toml deserialization does not work.
[This rust playground][invalid] demonstrates how toml fails to deserialize a native `u128` internal value.

With `maili-serde`, tagging the inner `u128` field with `#[serde(with = "maili_serde::quantity")]`,
allows the `u128` to be deserialized by toml properly. Below demonstrates this.

```rust
use serde::{Serialize, Deserialize};

/// My wrapper type.
#[derive(Debug, Serialize, Deserialize)]
pub struct MyStruct {
    /// The inner `u128` value.
    #[serde(with = "maili_serde::quantity")]
    pub inner: u128,
}

// Correctly deserializes a raw value.
let raw_toml = r#"inner = 120"#;
let b: MyStruct = toml::from_str(raw_toml).expect("failed to deserialize toml");
println!("{}", b.inner);

// Notice that a string value is also deserialized correctly.
let raw_toml = r#"inner = "120""#;
let b: MyStruct = toml::from_str(raw_toml).expect("failed to deserialize toml");
println!("{}", b.inner);
```

### Provenance

This code is heavily based on the [`alloy-serde`][alloy-serde] crate.


<!-- Hyperlinks -->

[invalid]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d3c674d02a90c574e3f543144621418d
[alloy-serde]: https://crates.io/crates/alloy-serde
