# maili

<a href="https://crates.io/crates/v/maili"><img src="https://img.shields.io/crates/v/maili?label=maili" alt="Maili Crate"></a>
<a href="https://github.com/op-rs/maili/actions/workflows/ci.yml"><img src="https://github.com/op-rs/maili/actions/workflows/ci.yml/badge.svg?label=ci" alt="CI"></a>
<a href="https://github.com/op-rs/maili/blob/main/LICENSE-APACHE"><img src="https://img.shields.io/badge/License-APACHE-d1d1f6.svg?label=license&labelColor=2a2f35" alt="License"></a>
<a href="https://github.com/op-rs/maili/blob/main/LICENSE-MIT"><img src="https://img.shields.io/badge/License-MIT-d1d1f6.svg?label=license&labelColor=2a2f35" alt="License"></a>
<a href="https://op-rs.github.io/maili"><img src="https://img.shields.io/badge/Book-854a15?logo=mdBook&labelColor=2a2f35" alt="Book"></a>

Types and interfaces unique to the OP Stack.

> [!IMPORTANT]
>
> Ethereum types modified for the OP Stack live in [op-alloy](https://github.com/alloy-rs/op-alloy).


## Usage

The following crates are provided by `maili`.

- ![maili-rpc](https://img.shields.io/crates/v/maili-rpc?label=maili-rpc)
- ![maili-serde](https://img.shields.io/crates/v/maili-serde?label=maili-serde)
- ![maili-genesis](https://img.shields.io/crates/v/maili-genesis?label=maili-genesis)
- ![maili-interop](https://img.shields.io/crates/v/maili-interop?label=maili-interop)
- ![maili-protocol](https://img.shields.io/crates/v/maili-protocol?label=maili-protocol)
- ![maili-registry](https://img.shields.io/crates/v/maili-registry?label=maili-registry)


## Development Status

`maili` is currently in active development, and is not yet ready for use in production.


## Supported Rust Versions (MSRV)

The current MSRV (minimum supported rust version) is 1.81.

The MSRV is not increased automatically, and will be updated
only as part of a patch (pre-1.0) or minor (post-1.0) release.


## Contributing

`maili` is built by open source contributors like you, thank you for improving the project!

A [contributing guide][contributing] is available that sets guidelines for contributing.

Pull requests will not be merged unless CI passes, so please ensure that your contribution
follows the linting rules and passes clippy.


## `no_std`

`maili` is intended to be `no_std` compatible, most notably for use in [kona][kona].

All `maili` crates currently support `no_std` and are listed below.

- [`maili-rpc`][maili-rpc]
- [`maili-serde`][maili-serde]
- [`maili-genesis`][maili-genesis]
- [`maili-interop`][maili-interop]
- [`maili-protocol`][maili-protocol]
- [`maili-registry`][maili-registry] (note: requires `serde`)

If you would like to add `no_std` support to a crate,
please make sure to update [scripts/check_no_std.sh][check-no-std].


## Credits

`maili` contains OP Stack specific specs, whereas [op-alloy][op-alloy]
providers OP modifications to Ethereum types.

`maili` is only possible with the hard work from open source contributors. Thank you.


## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.


<!-- Hyperlinks -->

[check-no-std]: ./scripts/check_no_std.sh

[kona]: https://github.com/op-rs/kona
[op-alloy]: https://github.com/alloy-rs/op-alloy
[contributing]: https://op-rs.github.io/maili

[maili-protocol]: https://crates.io/crates/maili-protocol
[maili-serde]: https://crates.io/crates/maili-serde
[maili-interop]: https://crates.io/crates/maili-interop
[maili-registry]: https://crates.io/crates/maili-registry
[maili-genesis]: https://crates.io/crates/maili-genesis
[maili-rpc]: https://crates.io/crates/maili-rpc
