# [linebreak for Rust][repo-url] [![crates.io][cratesio-img]][cratesio-url] [![doc.rs][docrs-img]][docrs-url] [![CI Status][ci-img]][ci-url] [![MIT License][mit-img]][mit-url]

A library for breaking a given text into lines within a specified width. This library also supports per-line indentation.

## Install

In `Cargo.toml`, write this crate as a dependency.

```toml
[dependencies]
linebreak = "0.3.1"
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

This crate supports Rust 1.81.0 or later.

```sh
% ./build.sh msrv
  [Meta]   cargo-msrv 0.18.4

Compatibility Check #1: Rust 1.73.0
  [FAIL]   Is incompatible

Compatibility Check #2: Rust 1.81.0
  [OK]     Is compatible

Compatibility Check #3: Rust 1.77.2
  [FAIL]   Is incompatible

Compatibility Check #4: Rust 1.79.0
  [FAIL]   Is incompatible

Compatibility Check #5: Rust 1.80.1
  [FAIL]   Is incompatible

Result:
   Considered (min … max):   Rust 1.56.1 … Rust 1.89.0
   Search method:            bisect
   MSRV:                     1.81.0
   Target:                   x86_64-apple-darwin
```

## License

Copyright (C) 2024-2025 Takayuki Sato

This program is free software under MIT License.<br>
See the file LICENSE in this distribution for more details.


[repo-url]: https://github.com/sttk/linebreak-rust
[cratesio-img]: https://img.shields.io/badge/crates.io-ver.0.3.1-fc8d62?logo=rust
[cratesio-url]: https://crates.io/crates/linebreak
[docrs-img]: https://img.shields.io/badge/doc.rs-linebreak-66c2a5?logo=docs.rs
[docrs-url]: https://docs.rs/linebreak
[ci-img]: https://github.com/sttk/linebreak-rust/actions/workflows/rust.yml/badge.svg?branch=main
[ci-url]: https://github.com/sttk/linebreak-rust/actions?query=branch%3Amain
[mit-img]: https://img.shields.io/badge/license-MIT-green.svg
[mit-url]: https://opensource.org/licenses/MIT
