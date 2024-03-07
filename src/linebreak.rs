// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::unicode::EAST_ASIAN_WIDTH;
use icu::properties::EastAsianWidth;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LboType {
    Never,
    Before,
    After,
    Both,
    Break,
    Space,
}

pub struct LboState {
    pub lbo_type: LboType,
    pub lbo_prev: LboType,
    pub open_apos: u8, // 0:not, 1:opened, 2:openend inside "..."
    pub open_quot: u8, // 0:not, 1:opened, 2:openend inside "..."
}

pub fn line_break_opportunity(ch: char, state: &mut LboState) {
    state.lbo_prev = state.lbo_type;

    match ch {
        '"' => {
            if state.open_quot == 0 {
                // open
                state.open_quot = state.open_apos + 1;
                state.lbo_type = LboType::Before;
            } else {
                // close
                if state.open_quot < state.open_apos {
                    state.open_apos = 0;
                }
                state.open_quot = 0;
                state.lbo_type = LboType::After;
            }
            return;
        }
        '\'' => {
            if state.open_apos == 0 {
                // open
                state.open_apos = state.open_quot + 1;
                state.lbo_type = LboType::Before;
            } else {
                // close
                if state.open_apos < state.open_quot {
                    state.open_quot = 0;
                }
                state.open_apos = 0;
                state.lbo_type = LboType::After;
            }
            return;
        }
        _ => {
            if contains(LBO_BREAKS, ch) {
                state.lbo_type = LboType::Break;
                return;
            }
            if contains(LBO_BEFORES, ch) {
                state.lbo_type = LboType::Before;
                return;
            }
            if contains(LBO_AFTERS, ch) {
                state.lbo_type = LboType::After;
                return;
            }
            if ch.is_whitespace() {
                state.lbo_type = LboType::Space;
                return;
            }
            match EAST_ASIAN_WIDTH.get(ch) {
                EastAsianWidth::Wide | EastAsianWidth::Fullwidth => {
                    state.lbo_type = LboType::Both;
                    return;
                }
                _ => (),
            }
            state.lbo_type = LboType::Never;
        }
    }
}

fn contains(candidates: &[char], ch: char) -> bool {
    for c in candidates {
        if *c == ch {
            return true;
        }
    }
    return false;
}

const LBO_BREAKS: &'static [char] = &[
    '\u{000A}', // LF
    '\u{000D}', // CR
];

const LBO_BEFORES: &'static [char] = &[
    '\u{0028}', // (
    '\u{005B}', // [
    '\u{007B}', // {
    '\u{00AB}', // «
    '\u{3008}', // 〈
    '\u{300A}', // 《
    '\u{300C}', // 「
    '\u{300E}', // 『
    '\u{3010}', // 【
    '\u{3014}', // 〔
    '\u{3016}', // 〖
    '\u{3018}', // 〘
    '\u{301D}', // 〝
    '\u{FF5F}', // ｟
];

const LBO_AFTERS: &'static [char] = &[
    '\u{0021}', // !
    '\u{0029}', // )
    '\u{002C}', // ,
    '\u{002E}', // .
    '\u{002F}', // /
    '\u{003A}', // :
    '\u{003B}', // ;
    '\u{003F}', // ?
    '\u{30A0}', // ゠
    '\u{30A1}', // ァ
    '\u{30A3}', // ィ
    '\u{30A5}', // ゥ
    '\u{30A7}', // ェ
    '\u{30A9}', // ォ
    '\u{30C3}', // ッ
    '\u{30E3}', // ャ
    '\u{30E5}', // ュ
    '\u{30E7}', // ョ
    '\u{30EE}', // ヮ
    '\u{30F5}', // ヵ
    '\u{30F6}', // ヶ
    '\u{3041}', // ぁ
    '\u{3043}', // ぃ
    '\u{3045}', // ぅ
    '\u{3047}', // ぇ
    '\u{3049}', // ぉ
    '\u{3063}', // っ
    '\u{3083}', // ゃ
    '\u{3085}', // ゅ
    '\u{3087}', // ょ
    '\u{308E}', // ゎ
    '\u{3095}', // ゕ
    '\u{3096}', // ゖ
    '\u{30FC}', // ー
    '\u{3001}', // 、
    '\u{3002}', // 。
    '\u{3005}', // 々
    '\u{3008}', // 〈
    '\u{3009}', // 〉
    '\u{300A}', // 《
    '\u{300B}', // 》
    '\u{300C}', // 「
    '\u{300D}', // 」
    '\u{300E}', // 』
    '\u{300F}', // 】
    '\u{3015}', // 〕
    '\u{3017}', // 〗
    '\u{3019}', // 〙
    '\u{301F}', // 〟
    '\u{FF09}', // )
    '\u{FF5D}', // ｝
];

#[cfg(test)]
mod test_of_linebreak {
    use super::*;

    #[test]
    fn test_contains_in_lbo_breaks() {
        assert_eq!(contains(LBO_BREAKS, '\r'), true);
        assert_eq!(contains(LBO_BREAKS, '\n'), true);
        assert_eq!(contains(LBO_BREAKS, '\t'), false);
        assert_eq!(contains(LBO_BREAKS, 'a'), false);
        assert_eq!(contains(LBO_BREAKS, '1'), false);
    }

    #[test]
    fn test_contains_in_lbo_befores() {
        assert_eq!(contains(LBO_BEFORES, '('), true);
        assert_eq!(contains(LBO_BEFORES, ')'), false);
        assert_eq!(contains(LBO_BEFORES, '['), true);
        assert_eq!(contains(LBO_BEFORES, ']'), false);
        assert_eq!(contains(LBO_BEFORES, '「'), true);
        assert_eq!(contains(LBO_BEFORES, '」'), false);
        assert_eq!(contains(LBO_BEFORES, 'a'), false);
        assert_eq!(contains(LBO_BEFORES, '1'), false);
    }

    #[test]
    fn test_contains_in_lbo_afters() {
        assert_eq!(contains(LBO_AFTERS, '!'), true);
        assert_eq!(contains(LBO_AFTERS, ')'), true);
        assert_eq!(contains(LBO_AFTERS, ','), true);
        assert_eq!(contains(LBO_AFTERS, '.'), true);
        assert_eq!(contains(LBO_AFTERS, '?'), true);
        assert_eq!(contains(LBO_AFTERS, 'ァ'), true);
        assert_eq!(contains(LBO_AFTERS, '、'), true);
        assert_eq!(contains(LBO_AFTERS, '。'), true);
        assert_eq!(contains(LBO_AFTERS, 'a'), false);
        assert_eq!(contains(LBO_AFTERS, '1'), false);
        assert_eq!(contains(LBO_AFTERS, 'ア'), false);
    }

    #[test]
    fn test_line_break_opportunity_ch_is_opening_quot() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 0,
            open_quot: 0,
        };

        line_break_opportunity('"', &mut state);

        assert_eq!(state.lbo_type, LboType::Before);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 1);
    }

    #[test]
    fn test_line_break_opportunity_ch_is_opening_apos() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 0,
            open_quot: 0,
        };

        line_break_opportunity('\'', &mut state);

        assert_eq!(state.lbo_type, LboType::Before);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 1);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_ch_is_closing_quot() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 0,
            open_quot: 1,
        };

        line_break_opportunity('"', &mut state);

        assert_eq!(state.lbo_type, LboType::After);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_ch_is_closing_apos() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 1,
            open_quot: 0,
        };

        line_break_opportunity('\'', &mut state);

        assert_eq!(state.lbo_type, LboType::After);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_ch_is_opening_quot_after_opening_apos() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 1,
            open_quot: 0,
        };

        line_break_opportunity('"', &mut state);

        assert_eq!(state.lbo_type, LboType::Before);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 1);
        assert_eq!(state.open_quot, 2);
    }

    #[test]
    fn test_line_break_opportunity_ch_is_opening_apos_after_opening_quot() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 0,
            open_quot: 1,
        };

        line_break_opportunity('\'', &mut state);

        assert_eq!(state.lbo_type, LboType::Before);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 2);
        assert_eq!(state.open_quot, 1);
    }

    #[test]
    fn test_line_break_opportunity_ch_is_closing_quot_inside_apos_pair() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 1,
            open_quot: 2,
        };

        line_break_opportunity('"', &mut state);

        assert_eq!(state.lbo_type, LboType::After);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 1);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_ch_is_closing_apos_inside_quot_pair() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 2,
            open_quot: 1,
        };

        line_break_opportunity('\'', &mut state);

        assert_eq!(state.lbo_type, LboType::After);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 1);
    }

    #[test]
    fn test_line_break_opportunity_ch_is_closing_quot_before_closing_apos() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 2,
            open_quot: 1,
        };

        line_break_opportunity('"', &mut state);

        assert_eq!(state.lbo_type, LboType::After);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_ch_is_closing_apos_before_closing_quot() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 1,
            open_quot: 2,
        };

        line_break_opportunity('\'', &mut state);

        assert_eq!(state.lbo_type, LboType::After);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_lbo_type_of_ch_is_lbo_break() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 0,
            open_quot: 0,
        };

        line_break_opportunity('\n', &mut state);

        assert_eq!(state.lbo_type, LboType::Break);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_lbo_type_of_ch_is_lbo_before() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 0,
            open_quot: 0,
        };

        line_break_opportunity('(', &mut state);

        assert_eq!(state.lbo_type, LboType::Before);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_lbo_type_of_ch_is_lbo_after() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 0,
            open_quot: 0,
        };

        line_break_opportunity('?', &mut state);

        assert_eq!(state.lbo_type, LboType::After);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_lbo_type_of_ch_is_lbo_space() {
        let mut state = LboState {
            lbo_type: LboType::Both,
            lbo_prev: LboType::Never,
            open_apos: 0,
            open_quot: 0,
        };

        line_break_opportunity(' ', &mut state);

        assert_eq!(state.lbo_type, LboType::Space);
        assert_eq!(state.lbo_prev, LboType::Both);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_lbo_type_of_ch_is_lbo_both() {
        let mut state = LboState {
            lbo_type: LboType::After,
            lbo_prev: LboType::Never,
            open_apos: 0,
            open_quot: 0,
        };

        line_break_opportunity('あ', &mut state);

        assert_eq!(state.lbo_type, LboType::Both);
        assert_eq!(state.lbo_prev, LboType::After);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_line_break_opportunity_lbo_type_of_ch_is_lbo_never() {
        let mut state = LboState {
            lbo_type: LboType::After,
            lbo_prev: LboType::Before,
            open_apos: 0,
            open_quot: 0,
        };

        line_break_opportunity('a', &mut state);

        assert_eq!(state.lbo_type, LboType::Never);
        assert_eq!(state.lbo_prev, LboType::After);
        assert_eq!(state.open_apos, 0);
        assert_eq!(state.open_quot, 0);
    }

    #[test]
    fn test_for_coverage() {
        let t = LboType::Never;
        assert_eq!(format!("{:?}", t.clone()), "Never")
    }
}
