# maili

<a href="https://github.com/op-rs/maili/actions/workflows/ci.yml"><img src="https://github.com/op-rs/maili/actions/workflows/ci.yml/badge.svg?label=ci" alt="CI"></a>
<a href="https://github.com/op-rs/maili/blob/main/LICENSE-APACHE"><img src="https://img.shields.io/badge/License-APACHE-d1d1f6.svg?label=license&labelColor=2a2f35" alt="License"></a>
<a href="https://github.com/op-rs/maili/blob/main/LICENSE-MIT"><img src="https://img.shields.io/badge/License-MIT-d1d1f6.svg?label=license&labelColor=2a2f35" alt="License"></a>
<a href="https://op-rs.github.io/maili"><img src="https://img.shields.io/badge/Book-854a15?logo=mdBook&labelColor=2a2f35" alt="Book"></a>

OP Stack unique types and interfaces.


## Usage

The following crates are provided by `maili`.

- ![maili](https://img.shields.io/crates/v/maili?label=maili)
- ![maili-rpc](https://img.shields.io/crates/v/maili-rpc?label=maili-rpc)
- ![maili-common](https://img.shields.io/crates/v/maili-common?label=maili-common)
- ![maili-protocol](https://img.shields.io/crates/v/maili-protocol?label=maili-protocol)
- ![maili-registry](https://img.shields.io/crates/v/maili-registry?label=maili-registry)
- ![maili-provider](https://img.shields.io/crates/v/maili-provider?label=maili-provider)
- ![maili-rpc](https://img.shields.io/crates/v/maili-rpc?label=maili-rpc)


## Development Status

`maili` is currently in active development, and is not yet ready for use in production.


## Supported Rust Versions (MSRV)

The current MSRV (minimum supported rust version) is 1.81.

Unlike Alloy, maili may use the latest stable release,
to benefit from the latest features.

The MSRV is not increased automatically, and will be updated
only as part of a patch (pre-1.0) or minor (post-1.0) release.


## Contributing

maili is built by open source contributors like you, thank you for improving the project!

A [contributing guide][contributing] is available that sets guidelines for contributing.

Pull requests will not be merged unless CI passes, so please ensure that your contribution follows the
linting rules and passes clippy.


## `no_std`

maili is intended to be `no_std` compatible, initially for use in [kona][kona].

The following crates support `no_std`.
Notice, provider crates do not support `no_std` compatibility.

- [`maili-protocol`][maili-protocol]
- [`maili-provider`][maili-provider]
- [`maili-registry`][maili-registry] (note: requires `serde`)
- [`maili-common`][maili-common]
- [`maili-rpc`][maili-rpc]

If you would like to add no_std support to a crate,
please make sure to update [scripts/check_no_std.sh][check-no-std].


## Credits

maili implements the OP-unique spec, [op-alloy][op-alloy] is the glue that sticks OP to Ethereum.

This would not be possible without the hard work from open source contributors. Thank you.


## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.


<!-- Hyperlinks -->

[check-no-std]: ./scripts/check_no_std.sh

[kona]: https://github.com/anton-rs/kona
[op-alloy]: https://github.com/alloy-rs/op-alloy
[contributing]: https://op-rs.github.io/maili

[maili-protocol]: https://crates.io/crates/maili-protocol
[maili-provider]: https://crates.io/crates/maili-provider
[maili-registry]: https://crates.io/crates/maili-registry
[maili-common]: https://crates.io/crates/maili-common
