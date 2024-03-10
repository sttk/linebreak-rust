// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use std::cmp::min;

pub struct CharBuffer {
    ch_vec: Vec<char>,
}

impl CharBuffer {
    pub fn new(capacity: usize) -> CharBuffer {
        CharBuffer {
            ch_vec: Vec::with_capacity(capacity),
        }
    }

    pub fn add(&mut self, ch: char) -> bool {
        if self.ch_vec.len() >= self.ch_vec.capacity() {
            return false;
        }
        self.ch_vec.push(ch);
        return true;
    }

    pub fn cr(&mut self, start: usize) {
        if start >= self.ch_vec.len() {
            self.ch_vec.clear();
            return;
        }
        self.ch_vec.drain(..start);
    }

    #[cfg(test)]
    fn full(&self) -> String {
        String::from_iter(self.ch_vec.iter())
    }

    pub fn clear(&mut self) {
        self.ch_vec.clear();
    }

    pub fn len(&self) -> usize {
        self.ch_vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ch_vec.is_empty()
    }

    pub fn to_string_trimmed_end(&self) -> String {
        let n = self.ch_vec.len();
        if n > 0 {
            for i in 1..=n {
                if !self.ch_vec[n - i].is_whitespace() {
                    return String::from_iter(&self.ch_vec[0..(n - i + 1)]);
                }
            }
        }
        return String::from("");
    }

    pub fn substring_trimmed_end(&self, start: usize, end: usize) -> String {
        let end = min(end, self.ch_vec.len());
        if start < end {
            let n = end - start;
            for i in 1..=n {
                let last = start + n - i;
                if !self.ch_vec[last].is_whitespace() {
                    return String::from_iter(&self.ch_vec[start..(last + 1)]);
                }
            }
        }
        return String::from("");
    }
}

#[cfg(test)]
mod test_of_char_buffer {
    use super::*;

    #[test]
    fn test_new() {
        let buf = CharBuffer::new(0);
        assert_eq!(buf.is_empty(), true);
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.full(), "");
    }

    #[test]
    fn test_add() {
        let mut buf = CharBuffer::new(3);
        assert_eq!(buf.is_empty(), true);
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.full(), "");

        assert_eq!(buf.add('1'), true);
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 1);
        assert_eq!(buf.full(), "1");

        assert_eq!(buf.add('2'), true);
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 2);
        assert_eq!(buf.full(), "12");

        assert_eq!(buf.add('3'), true);
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 3);
        assert_eq!(buf.full(), "123");

        assert_eq!(buf.add('4'), false);
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 3);
        assert_eq!(buf.full(), "123");
    }

    #[test]
    fn test_cr() {
        let mut buf = CharBuffer::new(5);
        assert_eq!(buf.full(), "");
        assert_eq!(buf.is_empty(), true);
        assert_eq!(buf.len(), 0);

        assert_eq!(buf.add('1'), true);
        assert_eq!(buf.add('2'), true);
        assert_eq!(buf.add('3'), true);
        assert_eq!(buf.add('4'), true);
        assert_eq!(buf.add('5'), true);
        assert_eq!(buf.full(), "12345");
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 5);

        buf.cr(3);
        assert_eq!(buf.full(), "45");
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 2);

        assert_eq!(buf.add('6'), true);
        assert_eq!(buf.full(), "456");
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 3);

        buf.cr(3);
        assert_eq!(buf.full(), "");
        assert_eq!(buf.is_empty(), true);
        assert_eq!(buf.len(), 0);

        buf.cr(0);
        assert_eq!(buf.full(), "");
        assert_eq!(buf.is_empty(), true);
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_clear() {
        let mut buf = CharBuffer::new(5);
        assert_eq!(buf.full(), "");
        assert_eq!(buf.is_empty(), true);
        assert_eq!(buf.len(), 0);

        buf.clear();
        assert_eq!(buf.full(), "");
        assert_eq!(buf.is_empty(), true);
        assert_eq!(buf.len(), 0);

        buf.add('1');
        buf.add('2');
        assert_eq!(buf.full(), "12");
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 2);

        buf.clear();
        assert_eq!(buf.full(), "");
        assert_eq!(buf.is_empty(), true);
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_to_string_trimmed_end() {
        let mut buf = CharBuffer::new(5);
        assert_eq!(buf.full(), "");
        assert_eq!(buf.is_empty(), true);
        assert_eq!(buf.len(), 0);

        assert_eq!(buf.to_string_trimmed_end(), "");

        buf.add('1');
        assert_eq!(buf.full(), "1");
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 1);

        assert_eq!(buf.to_string_trimmed_end(), "1");

        buf.add('2');
        assert_eq!(buf.full(), "12");
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 2);

        assert_eq!(buf.to_string_trimmed_end(), "12");

        buf.add(' ');
        assert_eq!(buf.full(), "12 ");
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 3);

        assert_eq!(buf.to_string_trimmed_end(), "12");

        buf.add('4');
        assert_eq!(buf.full(), "12 4");
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 4);

        assert_eq!(buf.to_string_trimmed_end(), "12 4");

        buf.add(' ');
        assert_eq!(buf.full(), "12 4 ");
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 5);

        assert_eq!(buf.to_string_trimmed_end(), "12 4");
    }

    #[test]
    fn test_substring_trimmed_end() {
        let mut buf = CharBuffer::new(5);
        assert_eq!(buf.full(), "");
        assert_eq!(buf.is_empty(), true);
        assert_eq!(buf.len(), 0);

        assert_eq!(buf.substring_trimmed_end(0, 0), "");

        buf.add('1');
        buf.add('2');
        buf.add(' ');
        buf.add('4');
        buf.add(' ');
        assert_eq!(buf.full(), "12 4 ");
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.len(), 5);

        assert_eq!(buf.substring_trimmed_end(0, 5), "12 4");
        assert_eq!(buf.substring_trimmed_end(0, 4), "12 4");
        assert_eq!(buf.substring_trimmed_end(0, 3), "12");
        assert_eq!(buf.substring_trimmed_end(0, 2), "12");
        assert_eq!(buf.substring_trimmed_end(0, 1), "1");
        assert_eq!(buf.substring_trimmed_end(0, 0), "");
        assert_eq!(buf.substring_trimmed_end(1, 5), "2 4");
        assert_eq!(buf.substring_trimmed_end(1, 4), "2 4");
        assert_eq!(buf.substring_trimmed_end(1, 3), "2");
        assert_eq!(buf.substring_trimmed_end(1, 2), "2");
        assert_eq!(buf.substring_trimmed_end(1, 1), "");
        assert_eq!(buf.substring_trimmed_end(1, 0), "");
        assert_eq!(buf.substring_trimmed_end(2, 5), " 4");
        assert_eq!(buf.substring_trimmed_end(2, 4), " 4");
        assert_eq!(buf.substring_trimmed_end(2, 3), "");
        assert_eq!(buf.substring_trimmed_end(2, 2), "");
        assert_eq!(buf.substring_trimmed_end(2, 1), "");
        assert_eq!(buf.substring_trimmed_end(2, 0), "");
        assert_eq!(buf.substring_trimmed_end(3, 5), "4");
        assert_eq!(buf.substring_trimmed_end(3, 4), "4");
        assert_eq!(buf.substring_trimmed_end(3, 3), "");
        assert_eq!(buf.substring_trimmed_end(3, 2), "");
        assert_eq!(buf.substring_trimmed_end(3, 1), "");
        assert_eq!(buf.substring_trimmed_end(3, 0), "");
        assert_eq!(buf.substring_trimmed_end(4, 5), "");
        assert_eq!(buf.substring_trimmed_end(4, 4), "");
        assert_eq!(buf.substring_trimmed_end(4, 3), "");
        assert_eq!(buf.substring_trimmed_end(4, 2), "");
        assert_eq!(buf.substring_trimmed_end(4, 1), "");
        assert_eq!(buf.substring_trimmed_end(4, 0), "");
        assert_eq!(buf.substring_trimmed_end(5, 5), "");
        assert_eq!(buf.substring_trimmed_end(5, 4), "");
        assert_eq!(buf.substring_trimmed_end(5, 3), "");
        assert_eq!(buf.substring_trimmed_end(5, 2), "");
        assert_eq!(buf.substring_trimmed_end(5, 1), "");
        assert_eq!(buf.substring_trimmed_end(4, 0), "");
    }
}
