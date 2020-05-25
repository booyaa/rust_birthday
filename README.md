# rust_birthday

[![crates.io](https://img.shields.io/crates/v/cargo-cake.svg)](https://crates.io/crates/rust_birthday)
![github actions](https://github.com/booyaa/rust_birthday/workflows/Rust%20CI/badge.svg)
![crates.io downloads](https://img.shields.io/crates/d/rust_birthday)
![github issues open](https://img.shields.io/github/issues/booyaa/rust_birthday)
![github pull requests open](https://img.shields.io/github/issues-pr/booyaa/rust_birthday)
![github license](https://img.shields.io/github/license/booyaa/rust_birthday)

A crate for determining if it's Rust's birthday (15th of May).

## usage

This crate is [crates.io](https://crates.io/crates/rust_birthday) and can be used by adding `rust_birthday` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
rust_birthday = "0.1.*"
```

and this to your crate root:

```rust
extern crate rust_birthday;
```

### example

```rust
use rust_birthday::*;
let mut rust_birthday = RustBirthday::new();
println!("{:?}", rust_birthday.is_now());
```

## changelog

### v0.1.0

- initial release

## licence / copyright

License is WTFPL v2.0.

There is no copyright.
