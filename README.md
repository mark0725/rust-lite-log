# lite-log

A logger that prints all messages with a readable output format.

[![Crates.io][crates-badge]][crates-url]
[![MIT License][license-badge]][license-link]
[![Github Tags][tags-badge]][tags-link]
[![Github Issues][issues-badge]][issues-link]

[crates-url]: https://crates.io/crates/lite-log
[crates-badge]: https://img.shields.io/crates/v/lite-log.svg
[license-badge]: https://img.shields.io/github/license/mark0725/rust-lite-log
[license-link]: https://github.com/mark0725/rust-lite-log/blob/master/LICENSE
[tags-badge]: https://img.shields.io/github/tag/mark0725/rust-lite-log.svg
[tags-link]: https://github.com/mark0725/rust-lite-log/tags
[issues-badge]: https://img.shields.io/github/issues/mark0725/rust-lite-log.svg
[issues-link]: https://github.com/mark0725/rust-lite-log/issues

Forked from [rust-simple_logger](https://github.com/borntyping/rust-simple_logger), and changed lib time to chrono.

The output format is based on the format used by [Supervisord](https://github.com/Supervisor/supervisor), with timestamps default [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) format. The format used for timestamps can be customised.

* [Source on GitHub](https://github.com/mark0725/rust-lite-log)
* [Packages on Crates.io](https://crates.io/crates/lite-log)
* [Documentation on Docs.rs](https://docs.rs/lite-log)


## Usage

```rust
use lite_log::LiteLogger;

fn main() {
    LiteLogger::new().init().unwrap();

    log::warn!("This is an example message.");
}
```

### run example

```
cargo run --example init
```

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/mark0725/lite-log/blob/master/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in lite-log by you, shall be licensed as MIT, without any additional
terms or conditions.