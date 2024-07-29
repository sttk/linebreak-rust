// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use std::str::Chars;

use crate::char_buffer::CharBuffer;
use crate::linebreak::*;
use crate::unicode::char_width;

/// `LineIter` is the struct that outputs the given string line by line.
/// This struct can control the overall line width and the indentation from any
/// desired line.
pub struct LineIter<'a> {
    scanner: Chars<'a>,
    buffer: CharBuffer,
    width: [usize; 2],
    lbo_pos: usize,
    limit: usize,
    indent: &'a str,
    indent_width: usize,
    open_quot: u8,
    open_apos: u8,
    has_next: bool,
}

impl<'a> LineIter<'a> {
    /// Creates a `LineIter` instance which outputs the given string line by
    /// line.
    /// The second argument is the width of the output lines.
    ///
    /// ```rust
    ///    use linebreak::LineIter;
    ///
    ///    let mut iter = LineIter::new("...", 80);
    /// ```
    pub fn new(text: &'a str, line_width: usize) -> LineIter<'a> {
        LineIter {
            scanner: text.chars(),
            buffer: CharBuffer::new(line_width),
            width: [0; 2],
            lbo_pos: 0,
            limit: line_width,
            indent: "",
            indent_width: 0,
            open_quot: 0,
            open_apos: 0,
            has_next: true,
        }
    }

    /// Sets an indentation for the subsequent lines.
    ///
    /// ```rust
    ///     use linebreak::LineIter;
    ///
    ///     let mut iter = LineIter::new("abcdefghijklmnopqrstuvwxyz", 10);
    ///     assert_eq!(iter.next().unwrap(), "abcdefghij");
    ///     iter.set_indent("    ");
    ///     assert_eq!(iter.next().unwrap(), "    klmnop");
    ///     assert_eq!(iter.next().unwrap(), "    qrstuv");
    ///     assert_eq!(iter.next().unwrap(), "    wxyz");
    ///     assert_eq!(iter.next().is_none(), true);
    /// ```
    pub fn set_indent(&mut self, indent: &'a str) {
        self.indent = indent;
        self.indent_width = crate::text_width(indent);
    }

    /// Re-initializes with an argument string for reusing this instance.
    ///
    /// ```rust
    ///     use linebreak::LineIter;
    ///
    ///     let mut iter = LineIter::new("abcdefghijklmn", 10);
    ///     assert_eq!(iter.next().unwrap(), "abcdefghij");
    ///     assert_eq!(iter.next().unwrap(), "klmn");
    ///     assert_eq!(iter.next().is_none(), true);
    ///
    ///     iter.init("opqrstuvwxyz");
    ///     assert_eq!(iter.next().unwrap(), "opqrstuvwx");
    ///     assert_eq!(iter.next().unwrap(), "yz");
    ///     assert_eq!(iter.next().is_none(), true);
    /// ```
    pub fn init(&mut self, text: &'a str) {
        self.scanner = text.chars();
        self.buffer.clear();
        self.width[0] = 0;
        self.width[1] = 0;
        self.lbo_pos = 0;
        self.open_quot = 0;
        self.open_apos = 0;
        self.has_next = true;
    }

    /// Returns an Option of a line string.
    /// If there is a line string to be printed, this method returns a
    /// `Some(String)`, otherwise returns `None.`
    ///
    /// ```rust
    ///     use linebreak::LineIter;
    ///
    ///     let text = "The Rust programming language helps you write faster, \
    ///         more reliable software.";
    ///     let mut iter = LineIter::new(&text, 30);
    ///     assert_eq!(iter.next().unwrap(), "The Rust programming language");
    ///     assert_eq!(iter.next().unwrap(), "helps you write faster, more");
    ///     assert_eq!(iter.next().unwrap(), "reliable software.");
    ///     assert_eq!(iter.next().is_none(), true);
    /// ```
    pub fn next(&mut self) -> Option<String> {
        if !self.has_next {
            return None;
        }

        let limit = self.limit - self.indent_width;

        if self.width[0] > limit {
            let mut diff = self.width[0] - limit;
            self.width[0] = diff;
            let mut i = self.buffer.len();
            while i > 0 {
                i -= 1;
                if let Some(ch) = self.buffer.get(i) {
                    let ch_width = char_width(ch);
                    if diff <= ch_width {
                        let mut line = self.buffer.substring_trimmed_end(0, i);
                        self.buffer.cr(i);
                        if !line.is_empty() {
                            line.insert_str(0, self.indent);
                        }
                        return Some(line);
                    }
                    diff -= ch_width;
                } else {
                    break;
                }
            }
        } else if self.width[0] == limit {
            self.width[0] = 0;
            let mut line = self.buffer.to_string_trimmed_end();
            self.buffer.cr(0);
            if !line.is_empty() {
                line.insert_str(0, self.indent);
            }
            return Some(line);
        }

        let mut state = LboState {
            lbo_type: LboType::Never,
            lbo_prev: LboType::Never,
            open_quot: self.open_quot,
            open_apos: self.open_apos,
        };

        while let Some(ch) = self.scanner.next() {
            line_break_opportunity(ch, &mut state);

            if state.lbo_type == LboType::Break {
                let mut line = self.buffer.to_string_trimmed_end();
                self.buffer.clear();
                self.width[0] = 0;
                self.width[1] = 0;
                self.lbo_pos = 0;
                self.open_quot = 0;
                self.open_apos = 0;
                if !line.is_empty() {
                    line.insert_str(0, self.indent);
                }
                self.has_next = true;
                return Some(line);
            }

            if self.buffer.is_empty() && state.lbo_type == LboType::Space {
                continue;
            }

            let ch_width = char_width(ch);
            let mut lbo_pos = self.lbo_pos;

            if self.width[0] + self.width[1] + ch_width > limit {
                if state.lbo_prev == LboType::Before {
                    let mut line = self.buffer.substring_trimmed_end(0, lbo_pos);
                    self.buffer.cr(lbo_pos);

                    self.buffer.add(ch);
                    self.width[0] = self.width[1] + ch_width;
                    self.width[1] = 0;
                    self.lbo_pos = self.buffer.len();

                    self.open_quot = state.open_quot;
                    self.open_apos = state.open_apos;

                    if !line.is_empty() {
                        line.insert_str(0, self.indent);
                    }
                    self.has_next = true;
                    return Some(line);
                }

                match state.lbo_type {
                    LboType::Before | LboType::Both | LboType::Space => {
                        lbo_pos = self.buffer.len();
                    }
                    _ => (),
                }
                // break forcely when no lbo in the current line
                if lbo_pos == 0 {
                    self.width[0] += self.width[1];
                    self.width[1] = 0;
                    lbo_pos = self.buffer.len();
                }

                let mut line = self.buffer.substring_trimmed_end(0, lbo_pos);
                self.buffer.cr(lbo_pos);

                match state.lbo_type {
                    LboType::Space => {
                        self.width[0] = 0;
                        self.width[1] = 0;
                        self.lbo_pos = 0;
                    }
                    LboType::Before | LboType::Both => {
                        self.buffer.add(ch);
                        self.width[0] = ch_width;
                        self.width[1] = 0;
                        self.lbo_pos = 0;
                    }
                    LboType::After => {
                        self.buffer.add(ch);
                        self.width[0] = self.width[1] + ch_width;
                        self.width[1] = 0;
                        self.lbo_pos = self.buffer.len();
                    }
                    _ => {
                        self.buffer.add(ch);
                        self.width[0] = self.width[1] + ch_width;
                        self.width[1] = 0;
                        self.lbo_pos = 0;
                    }
                }

                self.open_quot = state.open_quot;
                self.open_apos = state.open_apos;

                if !line.is_empty() {
                    line.insert_str(0, self.indent);
                }
                self.has_next = true;
                return Some(line);
            }

            if ch_width > 0 {
                self.buffer.add(ch);
            }
            match state.lbo_type {
                LboType::Before => {
                    if state.lbo_prev != LboType::Before {
                        self.lbo_pos = self.buffer.len() - 1;
                    }
                    self.width[0] += self.width[1];
                    self.width[1] = ch_width;
                }
                LboType::Both => {
                    self.lbo_pos = self.buffer.len() - 1;
                    self.width[0] += self.width[1];
                    self.width[1] = ch_width;
                }
                LboType::After | LboType::Space => {
                    self.lbo_pos = self.buffer.len();
                    self.width[0] += self.width[1] + ch_width;
                    self.width[1] = 0;
                }
                _ => {
                    self.width[1] += ch_width;
                }
            }
        }

        let mut line = self.buffer.to_string_trimmed_end();
        self.buffer.clear();

        if !line.is_empty() {
            line.insert_str(0, self.indent);
        }
        self.has_next = false;
        return Some(line);
    }
}

#[cfg(test)]
mod test_of_line_iter {
    use super::*;

    #[test]
    fn test_empty() {
        let text = "";
        let mut iter = LineIter::new(text, 20);

        let s = iter.next().unwrap();
        assert_eq!(s, text);

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_one_char_text() {
        let text = "a";
        let mut iter = LineIter::new(text, 20);

        let s = iter.next().unwrap();
        assert_eq!(s, text);

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_less_than_line_width() {
        let text = "1234567890123456789";
        let mut iter = LineIter::new(text, 20);

        let s = iter.next().unwrap();
        assert_eq!(s, text);

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_equal_to_line_width() {
        let text = "12345678901234567890";
        let mut iter = LineIter::new(text, 20);

        let s = iter.next().unwrap();
        assert_eq!(s, text);

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_break_at_line_break_opportunity() {
        let text = "1234567890 abcdefghij";
        let mut iter = LineIter::new(text, 20);

        let s = iter.next().unwrap();
        assert_eq!(s, text[0..10]);

        let s = iter.next().unwrap();
        assert_eq!(s, text[11..21]);

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_remove_heading_space_of_each_line() {
        let text = "12345678901234567890   abcdefghij";
        let mut iter = LineIter::new(text, 20);

        let s = iter.next().unwrap();
        assert_eq!(s, text[0..20]);

        let s = iter.next().unwrap();
        assert_eq!(s, text[23..]);

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_remove_tailing_space_of_each_line() {
        let text = "12345678901234567      abcdefghij";
        let mut iter = LineIter::new(text, 20);

        let s = iter.next().unwrap();
        assert_eq!(s, text[0..17]);

        let s = iter.next().unwrap();
        assert_eq!(s, text[23..]);

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_remove_spaces_of_all_space_line() {
        let text = "       ";
        let mut iter = LineIter::new(text, 10);

        let s = iter.next().unwrap();
        assert_eq!(s, "");

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_remove_there_is_no_line_break_opportunity() {
        let text = "12345678901234567890abcdefghij";
        let mut iter = LineIter::new(text, 20);

        let s = iter.next().unwrap();
        assert_eq!(s, text[0..20]);

        let s = iter.next().unwrap();
        assert_eq!(s, text[20..]);

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_set_indent() {
        let text = "12345678901234567890abcdefghij";
        let mut iter = LineIter::new(text, 10);

        let s = iter.next().unwrap();
        assert_eq!(s, text[0..10]);

        iter.set_indent("   ");

        let s = iter.next().unwrap();
        assert_eq!(s, "   ".to_string() + &text[10..17]);

        let s = iter.next().unwrap();
        assert_eq!(s, "   ".to_string() + &text[17..24]);

        let s = iter.next().unwrap();
        assert_eq!(s, "   ".to_string() + &text[24..]);

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_break_position_after_indent_width_is_increased() {
        let line_width = 30;
        let indent = " ".repeat(7);
        let indent_width = crate::text_width(&indent);
        let text = "aaaaa ".to_string()
            + &"b".repeat(line_width - indent_width)
            + &"c".repeat(line_width - indent_width)
            + "ddd";

        let mut iter = LineIter::new(&text, line_width);
        let line = iter.next().unwrap();
        assert_eq!(line, "aaaaa");
        assert_eq!(crate::text_width(&line), 5);

        iter.set_indent(&indent);

        let line = iter.next().unwrap();
        assert_eq!(line, " ".repeat(7) + &"b".repeat(line_width - indent_width));
        assert_eq!(crate::text_width(&line), line_width);

        let line = iter.next().unwrap();
        assert_eq!(line, " ".repeat(7) + &"c".repeat(line_width - indent_width));
        assert_eq!(crate::text_width(&line), line_width);

        let line = iter.next().unwrap();
        assert_eq!(line, " ".repeat(7) + "ddd");
        assert_eq!(crate::text_width(&line), 10);

        let line = iter.next();
        assert_eq!(line, None);
    }

    #[test]
    fn test_break_position_if_indent_contains_full_width_chars() {
        let line_width = 30;
        let indent = "__ああ__".to_string(); // width is 8.
        let indent_width = crate::text_width(&indent);
        let text = "aaaaa ".to_string()
            + &"b".repeat(line_width - indent_width)
            + &"c".repeat(line_width - indent_width)
            + "ddd";

        let mut iter = LineIter::new(&text, line_width);
        let line = iter.next().unwrap();
        assert_eq!(line, "aaaaa");
        assert_eq!(crate::text_width(&line), 5);

        iter.set_indent(&indent);

        let line = iter.next().unwrap();
        assert_eq!(
            line,
            "__ああ__".to_string() + &"b".repeat(line_width - indent_width)
        );
        assert_eq!(crate::text_width(&line), line_width);

        let line = iter.next().unwrap();
        assert_eq!(
            line,
            "__ああ__".to_string() + &"c".repeat(line_width - indent_width)
        );
        assert_eq!(crate::text_width(&line), line_width);

        let line = iter.next().unwrap();
        assert_eq!(line, "__ああ__ddd");
        assert_eq!(crate::text_width(&line), 11);

        let line = iter.next();
        assert_eq!(line, None);
    }

    #[test]
    fn test_init() {
        let text = "12345678901234567890";
        let mut iter = LineIter::new(text, 12);

        let s = iter.next().unwrap();
        assert_eq!(s, text[0..12]);

        let s = iter.next().unwrap();
        assert_eq!(s, text[12..]);

        let opt = iter.next();
        assert!(opt.is_none());

        let text = "abcdefghijklmnopqrstuvwxyz";
        iter.init(text);

        let s = iter.next().unwrap();
        assert_eq!(s, text[0..12]);

        let s = iter.next().unwrap();
        assert_eq!(s, text[12..24]);

        let s = iter.next().unwrap();
        assert_eq!(s, text[24..]);

        let opt = iter.next();
        assert!(opt.is_none());
    }

    // This text is quoted from https://go.dev/doc/
    const LONG_TEXT: &'static str = r#"The Go programming language is an open source project to make programmers more productive.

Go is expressive, concise, clean, and efficient. Its concurrency mechanisms make it easy to write programs that get the most out of multicore and networked machines, while its novel type system enables flexible and modular program construction. Go compiles quickly to machine code yet has the convenience of garbage collection and the power of run-time reflection. It's a fast, statically typed, compiled language that feels like a dynamically typed, interpreted language.    "#;

    #[test]
    fn test_try_long_text() {
        let mut iter = LineIter::new(LONG_TEXT, 20);

        let s = iter.next().unwrap();
        assert_eq!(s, "The Go programming");

        let s = iter.next().unwrap();
        assert_eq!(s, "language is an open");

        let s = iter.next().unwrap();
        assert_eq!(s, "source project to");

        let s = iter.next().unwrap();
        assert_eq!(s, "make programmers");

        let s = iter.next().unwrap();
        assert_eq!(s, "more productive.");

        let s = iter.next().unwrap();
        assert_eq!(s, "");

        let s = iter.next().unwrap();
        assert_eq!(s, "Go is expressive,");

        let s = iter.next().unwrap();
        assert_eq!(s, "concise, clean, and");

        let s = iter.next().unwrap();
        assert_eq!(s, "efficient. Its");

        let s = iter.next().unwrap();
        assert_eq!(s, "concurrency");

        let s = iter.next().unwrap();
        assert_eq!(s, "mechanisms make it");

        let s = iter.next().unwrap();
        assert_eq!(s, "easy to write");

        let s = iter.next().unwrap();
        assert_eq!(s, "programs that get");

        let s = iter.next().unwrap();
        assert_eq!(s, "the most out of");

        let s = iter.next().unwrap();
        assert_eq!(s, "multicore and");

        let s = iter.next().unwrap();
        assert_eq!(s, "networked machines,");

        let s = iter.next().unwrap();
        assert_eq!(s, "while its novel type");

        let s = iter.next().unwrap();
        assert_eq!(s, "system enables");

        let s = iter.next().unwrap();
        assert_eq!(s, "flexible and modular");

        let s = iter.next().unwrap();
        assert_eq!(s, "program");

        let s = iter.next().unwrap();
        assert_eq!(s, "construction. Go");

        let s = iter.next().unwrap();
        assert_eq!(s, "compiles quickly to");

        let s = iter.next().unwrap();
        assert_eq!(s, "machine code yet has");

        let s = iter.next().unwrap();
        assert_eq!(s, "the convenience of");

        let s = iter.next().unwrap();
        assert_eq!(s, "garbage collection");

        let s = iter.next().unwrap();
        assert_eq!(s, "and the power of");

        let s = iter.next().unwrap();
        assert_eq!(s, "run-time reflection.");

        let s = iter.next().unwrap();
        assert_eq!(s, "It's a fast,");

        let s = iter.next().unwrap();
        assert_eq!(s, "statically typed,");

        let s = iter.next().unwrap();
        assert_eq!(s, "compiled language");

        let s = iter.next().unwrap();
        assert_eq!(s, "that feels like a");

        let s = iter.next().unwrap();
        assert_eq!(s, "dynamically typed,");

        let s = iter.next().unwrap();
        assert_eq!(s, "interpreted");

        let s = iter.next().unwrap();
        assert_eq!(s, "language.");

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_print_long_text() {
        let mut iter = LineIter::new(LONG_TEXT, 20);

        while let Some(s) = iter.next() {
            println!("{}", s);
        }
    }

    #[test]
    fn test_set_indent_to_long_text() {
        let mut iter = LineIter::new(LONG_TEXT, 40);

        if let Some(s) = iter.next() {
            println!("{}", s);
        }

        let indent = " ".repeat(8);
        iter.set_indent(&indent);

        while let Some(s) = iter.next() {
            println!("{}", s);
        }
    }

    #[test]
    fn test_text_contains_non_print_char() {
        let text = "abcdefg\u{0002}hijklmn";
        let mut iter = LineIter::new(text, 10);

        let s = iter.next().unwrap();
        assert_eq!(s, "abcdefghij");

        let s = iter.next().unwrap();
        assert_eq!(s, "klmn");

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_letter_width_of_east_asian_wide_letter() {
        let text = "東アジアの全角文字は２文字分の幅をとります。";
        let mut iter = LineIter::new(text, 20);

        let s = iter.next().unwrap();
        assert_eq!(s, "東アジアの全角文字は");

        let s = iter.next().unwrap();
        assert_eq!(s, "２文字分の幅をとりま");

        let s = iter.next().unwrap();
        assert_eq!(s, "す。");

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_line_break_of_east_asian_wide_letter() {
        let text =
            "東アジアの全角文字は基本的に、文字の前後どちらに行の終わりが来ても改行が行われます。";
        let mut iter = LineIter::new(text, 28);

        let s = iter.next().unwrap();
        assert_eq!(s, "東アジアの全角文字は基本的");

        let s = iter.next().unwrap();
        assert_eq!(s, "に、文字の前後どちらに行の終");

        let s = iter.next().unwrap();
        assert_eq!(s, "わりが来ても改行が行われま");

        let s = iter.next().unwrap();
        assert_eq!(s, "す。");

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_prohibitions_of_line_break_of_japanese_start() {
        let text = "句読点は、行頭に置くことは禁止である。";
        let mut iter = LineIter::new(text, 8);

        let s = iter.next().unwrap();
        assert_eq!(s, "句読点");

        let s = iter.next().unwrap();
        assert_eq!(s, "は、行頭");

        let s = iter.next().unwrap();
        assert_eq!(s, "に置くこ");

        let s = iter.next().unwrap();
        assert_eq!(s, "とは禁止");

        let s = iter.next().unwrap();
        assert_eq!(s, "である。");

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_prohibitions_of_line_break_of_japanese_end() {
        let text = "開き括弧は「行末に置く」ことは禁止である。";
        let mut iter = LineIter::new(text, 12);

        let s = iter.next().unwrap();
        assert_eq!(s, "開き括弧は");

        let s = iter.next().unwrap();
        assert_eq!(s, "「行末に置");

        let s = iter.next().unwrap();
        assert_eq!(s, "く」ことは禁");

        let s = iter.next().unwrap();
        assert_eq!(s, "止である。");

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_prohibitions_of_line_break_of_english() {
        let text = "abc def ghi(jkl mn opq rst uvw xyz)";
        let mut iter = LineIter::new(text, 11);

        let s = iter.next().unwrap();
        assert_eq!(s, "abc def ghi");

        let s = iter.next().unwrap();
        assert_eq!(s, "(jkl mn opq");

        let s = iter.next().unwrap();
        assert_eq!(s, "rst uvw");

        let s = iter.next().unwrap();
        assert_eq!(s, "xyz)");

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_prohibitions_of_line_break_of_english_quots() {
        let text = r#"abc def " ghi j " kl mno pq" rst uvw" xyz"#;
        let mut iter = LineIter::new(text, 9);

        let s = iter.next().unwrap();
        assert_eq!(s, "abc def");

        let s = iter.next().unwrap();
        assert_eq!(s, "\" ghi j \"");

        let s = iter.next().unwrap();
        assert_eq!(s, "kl mno pq");

        let s = iter.next().unwrap();
        assert_eq!(s, "\" rst");

        let s = iter.next().unwrap();
        assert_eq!(s, "uvw\" xyz");

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_prohibitions_of_line_break_of_english_mixed_quots() {
        let text = r#"abc def " ghi j ' kl mno pq' rst uvw" xyz"#;
        let mut iter = LineIter::new(text, 9);

        let s = iter.next().unwrap();
        assert_eq!(s, "abc def");

        let s = iter.next().unwrap();
        assert_eq!(s, "\" ghi j");

        let s = iter.next().unwrap();
        assert_eq!(s, "' kl mno");

        let s = iter.next().unwrap();
        assert_eq!(s, "pq' rst");

        let s = iter.next().unwrap();
        assert_eq!(s, "uvw\" xyz");

        let opt = iter.next();
        assert!(opt.is_none());

        iter.init(text);

        while let Some(s) = iter.next() {
            println!("{}", s);
        }
    }

    #[test]
    fn test_print_japanese() {
        let text = "".to_string()
            + "私はその人を常に先生と呼んでいた。だからここでもただ先生と書く"
            + "だけで本名は打ち明けない。これは世間を憚かる遠慮というよりも、"
            + "その方が私にとって自然だからである。私はその人の記憶を呼び起す"
            + "ごとに、すぐ「先生」といいたくなる。筆を執っても心持は同じ事で"
            + "ある。よそよそしい頭文字などはとても使う気にならない。\n"
            + "（夏目漱石「こころ」から引用）";

        let mut iter = LineIter::new(&text, 50);

        while let Some(s) = iter.next() {
            println!("{}", s);
        }
    }
}
