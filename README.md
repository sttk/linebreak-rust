# [linebreak-rust][repo-url] [![crate.io][crateio-img]][crateio-url] [![doc.rs][docrs-img]][docrs-url] [![CI Status][ci-img]][ci-url] [![MIT License][mit-img]][mit-url]

A library for breaking a given text into lines within a specified width. This library also supports per-line indentation.

## Install

In `Cargo.toml`, write this crate as a dependency.

```toml
[dependencies]
linebreak = "0.1.0"
```

## Usage

The usage example of `LineIter` struct in this crate is as follows:

```rust
use linebreak::LineIter;

fn main() {
    let text = "Welcome to The Rust Programming Language, an introductory \
      book about Rust. The Rust programming language helps you write faster, \
      more reliable software. High-level ergonomics and low-level control are \
      often at odds in programming language design; Rust challenges that \
      conflict. Through balancing powerful technical capacity and a great \
      developer experience, Rust gives you the option to control low-level \
      details (such as memory usage) without all the hassle traditionally \
      associated with such control.";

    let mut iter = LineIter::new(&text, 80);
    iter.set_indent("_______");

    println!("....:....1....:....2....:....3....:....4....:....5....:....6\
              ....:....7....:....8");
    while let Some(line) = iter.next() {
        println!("{}", line);
    }
}
```

The output of the above code is as follows:

```
....:....1....:....2....:....3....:....4....:....5....:....6....:....7....:....8
_______Welcome to The Rust Programming Language, an introductory book about
_______Rust. The Rust programming language helps you write faster, more reliable
_______software. High-level ergonomics and low-level control are often at odds
_______in programming language design; Rust challenges that conflict. Through
_______balancing powerful technical capacity and a great developer experience,
_______Rust gives you the option to control low-level details (such as memory
_______usage) without all the hassle traditionally associated with such control.
```

## Supporting Rust versions

This crate supports Rust 1.67.1 or later.

```sh
% cargo msrv
Fetching index
Determining the Minimum Supported Rust Version (MSRV) for toolchain x86_64-apple-darwin
Using check command cargo check

Check for toolchain '1.66.1-x86_64-apple-darwin' failed with:
┌───────────────────────────────────────────────────────────────────────────────────────┐
│ error: package `icu_locid v1.4.0` cannot be built because it requires rustc 1.67 or   │
│ newer, while the currently active rustc version is 1.66.1                             │
│ Either upgrade to rustc 1.67 or newer, or use                                         │
│ cargo update -p icu_locid@1.4.0 --precise ver                                         │
│ where `ver` is the latest version of `icu_locid` supporting rustc 1.66.1              │
└───────────────────────────────────────────────────────────────────────────────────────┘
Check for toolchain '1.71.1-x86_64-apple-darwin' succeeded
Check for toolchain '1.68.2-x86_64-apple-darwin' succeeded
Check for toolchain '1.67.1-x86_64-apple-darwin' succeeded
   Finished The MSRV is: 1.67.1   ██████████████████████████████████████████████ 00:02:18
```

## License

Copyright (C) 2024 Takayuki Sato

This program is free software under MIT License.<br>
See the file LICENSE in this distribution for more details.


[repo-url]: https://github.com/sttk/linebreak-rust
[crateio-img]: https://img.shields.io/badge/crate.io-ver.0.1.0-fc8d62?logo=rust
[crateio-url]: https://crates.io/crates/linebreak
[docrs-img]: https://img.shields.io/badge/doc.rs-linebreak-66c2a5?logo=docs.rs
[docrs-url]: https://docs.rs/linebreak
[ci-img]: https://github.com/sttk/linebreak-rust/actions/workflows/rust.yml/badge.svg?branch=main
[ci-url]: https://github.com/sttk/linebreak-rust/actions
[mit-img]: https://img.shields.io/badge/license-MIT-green.svg
[mit-url]: https://opensource.org/licenses/MIT
