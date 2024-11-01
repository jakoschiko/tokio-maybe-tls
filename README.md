# tokio-maybe-tls

Convenience wrapper for streams that allow to switch between plain TCP and different TLS
implementations at runtime.

Currently supported:

- plain TCP
- TLS with [`tokio-rustls`]
- TLS with [`tokio-native-tls`]

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
