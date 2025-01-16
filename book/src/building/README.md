# Building

This section offers in-depth documentation into the various `maili` crates.
Some of the primary crates and their types are listed below.

- [`maili-genesis`][maili-genesis] provides the
  [`RollupConfig`][rollup-config] and [`SystemConfig`][system-config] types.
- [`maili-consensus`][maili-consensus] provides deposit transaction types.
- [`maili-rpc`][maili-rpc] provides JSON-RPC server and client implementations
- [`maili-protocol`][maili-protocol] provides [`Frame`][frame],
  [`Channel`][channel], [`Batch`][batch] types and more.

<!-- Links -->

[frame]: https://docs.rs/maili-protocol/latest/maili_protocol/struct.Frame.html
[channel]: https://docs.rs/maili-protocol/latest/maili_protocol/struct.Channel.html
[batch]: https://docs.rs/maili-protocol/latest/maili_protocol/enum.Batch.html

[system-config]: https://docs.rs/maili-genesis/latest/maili_genesis/enum.SystemConfig.html
[rollup-config]: https://docs.rs/maili-genesis/latest/maili_genesis/enum.RollupConfig.html

[maili-rpc]: https://crates.io/crates/maili-rpc
[maili-genesis]: https://crates.io/crates/maili-genesis
[maili-protocol]: https://crates.io/crates/maili-protocol
[maili-consensus]: https://crates.io/crates/maili-consensus
