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
maili = { version = "0.5", features = ["full"] }
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
- `k256`
- `serde`

Full enables the most commonly used crates.
- `full`

The `k256` feature flag enables the `k256` feature on the `maili-consensus` crate.
- `k256`

Arbitrary enables arbitrary features on crates, deriving the `Arbitrary` trait on types.
- `arbitrary`

Serde derives serde's Serialize and Deserialize traits on types.
- `serde`

Additionally, individual crates can be enabled using their shorthand names.
For example, the `protocol` feature flag provides the `maili-protocol` re-export
so `maili-protocol` types can be used from `maili` through `maili::protocol::InsertTypeHere`.

## Crates

- [`maili-protocol`][maili-protocol] (supports `no_std`)
- [`maili-provider`][maili-provider]
- [`maili-common`][maili-common] (supports `no_std`)
- [`maili-rpc`][maili-rpc] (supports `no_std`)

## `no_std`

As noted above, the following crates are `no_std` compatible.

- [`maili-protocol`][maili-protocol]
- [`maili-common`][maili-common]
- [`maili-rpc`][maili-rpc] 

To add `no_std` support to a crate, ensure the [check_no_std][check-no-std]
script is updated to include this crate once `no_std` compatible.


{{#include ./links.md}}
