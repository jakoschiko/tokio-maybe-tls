# tokio-maybe-tls

Convenience wrapper for streams that allows to choose between plain and TLS streams
at runtime.

The wrapper consists of two enums, `MaybeTlsStream` and `TlsStream`, with variants
for all possible cases. Currently supported:

- plain
- TLS with [`tokio-rustls`]
- TLS with [`tokio-native-tls`]

All TLS implementations are optional features and disabled by default.

[`tokio-rustls`]: https://crates.io/crates/tokio-rustls
[`tokio-native-tls`]: https://crates.io/crates/tokio-native-tls

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
