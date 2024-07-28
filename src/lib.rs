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
//! linebreak = "0.3.0"
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
mod terminal;
mod unicode;

pub use line_iter::LineIter;
pub use terminal::Size;
pub use unicode::{char_width, is_print, text_width};

/// Returns the column number of the current terminal.
///
/// If failing to retrieve the column number, this function returns the
/// tentative value `80`.
/// This is because this crate would be used on character output terminals,
/// and errors occure only in special circumstances such as during CI
/// execution.
/// In such circumstances, it is assumed that returning a tentative value would
/// be beneficial than returning an error.
pub fn term_cols() -> usize {
    match terminal::term_cols() {
        Ok(cols) => cols,
        Err(_) => 80,
    }
}

/// Returns the size of the current terminal.
///
/// If failing to retrieve the column number, this function returns the
/// tentative size `{ col: 80, row: 24 }`.
/// This is because this crate would be used on character output terminals,
/// and errors occure only in special circumstances such as during CI
/// execution.
/// In such circumstances, it is assumed that returning a tentative value would
/// be beneficial than returning an error.
pub fn term_size() -> Size {
    match terminal::term_size() {
        Ok(size) => size,
        Err(_) => Size { col: 80, row: 24 },
    }
}
