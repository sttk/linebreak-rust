// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use super::Size;
use std::io;

pub fn term_cols() -> Result<usize, io::Error> {
    Error::new(ErrorKind::Unsupported)
}

pub fn term_size() -> Result<usize, io::Error> {
    Error::new(ErrorKind::Unsupported)
}
