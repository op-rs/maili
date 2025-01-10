## `maili`

<a href="https://github.com/op-rs/maili/actions/workflows/ci.yml"><img src="https://github.com/op-rs/maili/actions/workflows/ci.yml/badge.svg?label=ci" alt="CI"></a>
<a href="https://crates.io/crates/maili"><img src="https://img.shields.io/crates/v/maili.svg" alt="op-alloy crate"></a>
<a href="https://github.com/op-rs/maili/blob/main/LICENSE-APACHE"><img src="https://img.shields.io/badge/License-APACHE-d1d1f6.svg?label=license&labelColor=2a2f35" alt="License"></a>
<a href="https://github.com/op-rs/maili/blob/main/LICENSE-MIT"><img src="https://img.shields.io/badge/License-MIT-d1d1f6.svg?label=license&labelColor=2a2f35" alt="License"></a>
<a href="https://op-rs.github.io/maili"><img src="https://img.shields.io/badge/Book-854a15?logo=mdBook&labelColor=2a2f35" alt="Book"></a>


Built on [Alloy][alloy], `maili` connects applications to the OP Stack.


### Usage

To use `maili`, add the crate as a dependency to a `Cargo.toml`.

```toml
maili = "0.1"
```

### Development Status

`maili` is currently in active development, and is not yet ready for use in production.


### Supported Rust Versions (MSRV)

The current MSRV (minimum supported rust version) is 1.81.

Maili may use the latest stable release, to benefit from the latest features.

The MSRV is not increased automatically, and will be updated
only as part of a patch (pre-1.0) or minor (post-1.0) release.


### Contributing

Maili is built by open source contributors like you, thank you for improving the project!

A [contributing guide][contributing] is available that sets guidelines for contributing.

Pull requests will not be merged unless CI passes, so please ensure that your contribution follows the
linting rules and passes clippy.


### `no_std`

Maili is intended to be `no_std` compatible, initially for use in [kona][kona].

The following crates support `no_std`.
Notice, provider crates do not support `no_std` compatibility.

- [`maili-protocol`][maili-protocol]
- [`maili-registry`][maili-registry]
- [`maili-rpc-types-engine`][maili-rpc-types-engine]

If you would like to add no_std support to a crate,
please make sure to update [scripts/check_no_std.sh][check-no-std].


### Credits

Maili is inspired by the work of several teams and projects, most notably [the Alloy project][alloy].

This would not be possible without the hard work from open source contributors. Thank you.


### License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.


<!-- Hyperlinks -->

[check-no-std]: https://github.com/op-rs/maili/blob/main/scripts/check_no_std.sh

[kona]: https://github.com/anton-rs/kona
[alloy]: https://github.com/alloy-rs/alloy
[contributing]: https://op-rs.github.io/maili

[maili-protocol]: https://crates.io/crates/maili-protocol
[maili-registry]: https://crates.io/crates/maili-registry
[maili-rpc-types-engine]: https://crates.io/crates/maili-rpc-types-engine
