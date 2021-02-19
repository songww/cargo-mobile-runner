This tool extends Cargo to allow you to run, test, bench on mobile device directly.

subcommands:

- [`cargo mobi run`](#cargo-mobi-run)
- [`cargo mobi test`](#cargo-mobi-test)
- [`cargo mobi bench`](#cargo-mobi-bench)
- [`cargo mobi criterion`](#cargo-mobi-criterion)

[![Build Status](https://github.com/songww/cargo-mobile-runner/workflows/build/badge.svg)](https://github.com/songww/cargo-mobile-runner/actions)

```sh
$ cargo install cargo-mobile-runner --
```

If you wish to use a bundled version of `openssl`:

```sh
$ cargo install cargo-edit --features vendored-openssl
```

*Compiler support: requires rustc 1.44+*

(Please check [`cargo`'s documentation](http://doc.crates.io/) to learn how `cargo install` works and how to set up your system so it finds binaries installed by `cargo`.)

Install a sub-set of the commands with `cargo install -f --no-default-features --features "<COMMANDS>"`, where `<COMMANDS>` is a space-separated list of commands; i.e. `mobi-run mobi-test mobi-bench` for the full set.

## Available Subcommands

### `cargo mobi devices`

List all devices, that connected currently.

#### Examples

TODO
```sh
```

#### Usage

```plain
```

## License

This project is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
