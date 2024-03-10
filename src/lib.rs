// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

//! This crate is a library for breaking a given text into lines within a
//! specified width.
//! This crate also supports per-line indentation.
//!
//! ## Install
//!
//! In `Cargo.toml`, write this crate as a dependency.
//!
//! ```toml
//! [dependencies]
//! linebreak = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! The usage example of `LineIter` struct in this crate is as follows:
//!
//! ```rust
//! use linebreak::LineIter;
//!
//! fn main() {
//!     let text = "Welcome to The Rust Programming Language, an introductory \
//!       book about Rust. The Rust programming language helps you write faster, \
//!       more reliable software. High-level ergonomics and low-level control are \
//!       often at odds in programming language design; Rust challenges that \
//!       conflict. Through balancing powerful technical capacity and a great \
//!       developer experience, Rust gives you the option to control low-level \
//!       details (such as memory usage) without all the hassle traditionally \
//!       associated with such control.";
//!
//!     let mut iter = LineIter::new(&text, 80);
//!     iter.set_indent("_______");
//!
//!     println!("....:....1....:....2....:....3....:....4....:....5....:....6\
//!               ....:....7....:....8");
//!     while let Some(line) = iter.next() {
//!         println!("{}", line);
//!     }
//! }
//! ```
//!
//! The output of the above code is as follows:
//!
//! ```shell
//! ....:....1....:....2....:....3....:....4....:....5....:....6....:....7....:....8
//! _______Welcome to The Rust Programming Language, an introductory book about
//! _______Rust. The Rust programming language helps you write faster, more reliable
//! _______software. High-level ergonomics and low-level control are often at odds
//! _______in programming language design; Rust challenges that conflict. Through
//! _______balancing powerful technical capacity and a great developer experience,
//! _______Rust gives you the option to control low-level details (such as memory
//! _______usage) without all the hassle traditionally associated with such control.
//! ```

mod char_buffer;
mod line_iter;
mod linebreak;
mod unicode;

pub use line_iter::LineIter;
pub use unicode::{char_width, is_print, text_width};
