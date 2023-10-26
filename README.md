# lite-log [![](https://img.shields.io/github/tag/mark0725/rust-lite-log.svg)](https://github.com/mark0725/rust-lite-log/tags) [![](https://img.shields.io/travis/mark0725/rust-lite-log.svg)](https://travis-ci.org/mark0725/rust-lite-log) [![](https://img.shields.io/github/issues/mark0725/rust-lite-log.svg)](https://github.com/mark0725/rust-lite-log/issues)
Forked from [rust-simple_logger](https://github.com/borntyping/rust-simple_logger), and changed lib time to chrono.
A logger that prints all messages with a readable output format.

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