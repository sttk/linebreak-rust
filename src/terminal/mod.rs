// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

/// `Size` is the struct for storing the size of the current terminal.
#[derive(Debug)]
pub struct Size {
    /// The column number of the terminal.
    pub col: u16,
    /// The row number of the terminal.
    pub row: u16,
}

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::{term_cols, term_size};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::{term_cols, term_size};

#[cfg(not(any(unix, windows)))]
mod unknown;
#[cfg(not(any(unix, windows)))]
pub use self::unknown::{term_cols, term_size};

#[cfg(test)]
mod test_of_term_cols {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn test_get_terminal_cols() {
        match term_cols() {
            Ok(c) => println!("term cols = {}", c),
            Err(e) => {
                println!("term cols error = {}", e.to_string());
                assert_eq!(e.raw_os_error().unwrap(), 25); // NOTTY
            }
        }
    }

    #[cfg(windows)]
    #[test]
    fn test_get_terminal_cols() {
        match term_cols() {
            Ok(c) => println!("term cols = {}", c),
            Err(e) => {
                println!("term cols error = {}", e.to_string());
                assert_eq!(e.raw_os_error().unwrap() & 0xffff, 6); // Invalid Handler
            }
        }
    }

    #[cfg(not(any(unix, windows)))]
    #[test]
    fn test_get_terminal_cols() {
        match term_cols() {
            Ok(cols) => assert!(false),
            Err(e) => assert_eq!(e.kind(), ErrorKind::Unsupported),
        }
    }
}

#[cfg(test)]
mod test_of_term_size {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn test_get_terminal_size() {
        match term_size() {
            Ok(sz) => println!("term size = {} x {}", sz.col, sz.row),
            Err(e) => {
                println!("term size error = {}", e.to_string());
                assert_eq!(e.raw_os_error().unwrap(), 25); // NOTTY
            }
        }
    }

    #[cfg(windows)]
    #[test]
    fn test_get_terminal_size() {
        match term_size() {
            Ok(sz) => println!("term size = {} x {}", sz.col, sz.row),
            Err(e) => {
                println!("term size error = {}", e.to_string());
                assert_eq!(e.raw_os_error().unwrap() & 0xffff, 6); // Invalid Handler
            }
        }
    }

    #[cfg(not(any(unix, windows)))]
    #[test]
    fn test_get_terminal_cols() {
        match term_cols() {
            Ok(cols) => assert!(false),
            Err(e) => assert_eq!(e.kind(), ErrorKind::Unsupported),
        }
    }
}
