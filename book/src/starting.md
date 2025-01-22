# Installation

[maili][maili] consists of a number of crates that provide a range of functionality
essential for interfacing with any OP Stack chain.

The most succinct way to work with `maili` is to add the [`maili`][maili-crate] crate
with the `full` feature flag from the command-line using Cargo.

```txt
cargo add maili --features full
```

Alternatively, you can add the following to your `Cargo.toml` file.

```txt
maili = { version = "0.1", features = ["full"] }
```

For more fine-grained control over the features you wish to include, you can add the individual
crates to your `Cargo.toml` file, or use the `maili` crate with the features you need.

After `maili` is added as a dependency, crates re-exported by `maili` are now available.

```rust
use maili::{
   protocol::BlockInfo,
   provider::ext::engine::OpEngineApi,
};
```

## Features

The [`maili`][maili-crate] defines many [feature flags][maili-ff] including the following.

Default
- `std`
- `serde`

Full enables the most commonly used crates.
- `full`

Arbitrary enables arbitrary features on crates, deriving the `Arbitrary` trait on types.
- `arbitrary`

Serde derives serde's Serialize and Deserialize traits on types.
- `serde`

Additionally, individual crates can be enabled using their shorthand names.
For example, the `protocol` feature flag provides the `maili-protocol` re-export
so `maili-protocol` types can be used from `maili` through `maili::protocol::InsertTypeHere`.

## Crates

The following crates support `no_std`.

- ![maili](https://img.shields.io/crates/v/maili?label=maili)
- ![maili-rpc](https://img.shields.io/crates/v/maili-rpc?label=maili-rpc)
- ![maili-genesis](https://img.shields.io/crates/v/maili-genesis?label=maili-genesis)
- ![maili-protocol](https://img.shields.io/crates/v/maili-protocol?label=maili-protocol)
- ![maili-registry](https://img.shields.io/crates/v/maili-registry?label=maili-registry)

To add `no_std` support to a crate, ensure the [check_no_std][check-no-std]
script is updated to include this crate once `no_std` compatible.


{{#include ./links.md}}
