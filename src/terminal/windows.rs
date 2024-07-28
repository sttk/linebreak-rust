// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use super::Size;
use std::io;

use windows::Win32::System::Console::{
    GetConsoleScreenBufferInfo, GetStdHandle, CONSOLE_CHARACTER_ATTRIBUTES,
    CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT, STD_OUTPUT_HANDLE,
};

pub fn term_cols() -> Result<usize, io::Error> {
    let mut bi = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: CONSOLE_CHARACTER_ATTRIBUTES(0),
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 },
    };

    let h = match unsafe { GetStdHandle(STD_OUTPUT_HANDLE) } {
        Ok(h) => h,
        Err(e) => return Err(io::Error::from_raw_os_error(e.code().0)),
    };

    match unsafe { GetConsoleScreenBufferInfo(h, &mut bi) } {
        Ok(_) => Ok((bi.srWindow.Right - bi.srWindow.Left + 1) as usize),
        Err(e) => Err(io::Error::from_raw_os_error(e.code().0)),
    }
}

pub fn term_size() -> Result<Size, io::Error> {
    let mut bi = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: CONSOLE_CHARACTER_ATTRIBUTES(0),
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 },
    };

    let h = match unsafe { GetStdHandle(STD_OUTPUT_HANDLE) } {
        Ok(h) => h,
        Err(e) => return Err(io::Error::from_raw_os_error(e.code().0)),
    };

    match unsafe { GetConsoleScreenBufferInfo(h, &mut bi) } {
        Ok(_) => Ok(Size {
            col: (bi.srWindow.Right - bi.srWindow.Left + 1) as usize,
            row: (bi.srWindow.Bottom - bi.srWindow.Top + 1) as usize,
        }),
        Err(e) => Err(io::Error::from_raw_os_error(e.code().0)),
    }
}
