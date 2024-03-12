// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use super::Size;
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::io;

pub fn term_cols() -> Result<u16, io::Error> {
    let mut ws = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) };
    match r {
        0 => Ok(ws.ws_col as u16),
        _ => Err(io::Error::last_os_error()),
    }
}

pub fn term_size() -> Result<Size, io::Error> {
    let mut ws = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) };
    match r {
        0 => Ok(Size {
            col: ws.ws_col as u16,
            row: ws.ws_row as u16,
        }),
        _ => Err(io::Error::last_os_error()),
    }
}
