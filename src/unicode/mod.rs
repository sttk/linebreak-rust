// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use icu::properties::maps;
use icu::properties::maps::CodePointMapDataBorrowed;
use icu::properties::EastAsianWidth;
use icu::properties::GeneralCategory;

const GENERAL_CATEGORY: CodePointMapDataBorrowed<'static, GeneralCategory> =
    maps::general_category();
pub const EAST_ASIAN_WIDTH: CodePointMapDataBorrowed<'static, EastAsianWidth> =
    maps::east_asian_width();

/// Checks whether the specified codepoint is one of the printable characters
/// that includes letters, marks, numbers, punctuations, symbols from Unicode
/// categories L, M, N, P, S, and the ASCII space character.
///
/// ```rust
///    use linebreak::is_print;
///
///    assert_eq!(is_print(' '), true);
///    assert_eq!(is_print('\n'), false);
///    assert_eq!(is_print('a'), true);
/// ```
pub fn is_print(ch: char) -> bool {
    if ch == ' ' {
        // 0x20,SP,SPACE
        return true;
    }
    match GENERAL_CATEGORY.get(ch) {
        GeneralCategory::LowercaseLetter => true,      // Ll
        GeneralCategory::ModifierLetter => true,       // Lm
        GeneralCategory::OtherLetter => true,          // Lo
        GeneralCategory::TitlecaseLetter => true,      // Lt
        GeneralCategory::UppercaseLetter => true,      // Lu
        GeneralCategory::SpacingMark => true,          // Mc
        GeneralCategory::EnclosingMark => true,        // Me
        GeneralCategory::NonspacingMark => true,       // Mn
        GeneralCategory::DecimalNumber => true,        // Nd
        GeneralCategory::LetterNumber => true,         // Nl
        GeneralCategory::OtherNumber => true,          // No
        GeneralCategory::ConnectorPunctuation => true, // Pc
        GeneralCategory::DashPunctuation => true,      // Pd
        GeneralCategory::ClosePunctuation => true,     // Pe
        GeneralCategory::FinalPunctuation => true,     // Pf
        GeneralCategory::InitialPunctuation => true,   // Pi
        GeneralCategory::OtherPunctuation => true,     // Po
        GeneralCategory::OpenPunctuation => true,      // Ps
        GeneralCategory::CurrencySymbol => true,       // Sc
        GeneralCategory::ModifierSymbol => true,       // Sk
        GeneralCategory::MathSymbol => true,           // Sm
        GeneralCategory::OtherSymbol => true,          // So
        _ => false,
    }
}

/// Returns the display width of the specified character.
/// A display width is determined by the Unicode Standard Annex #11 (UAX11)
/// East-Asian-Width.
///
/// ```rust
///     use linebreak::char_width;
///
///     assert_eq!(char_width('\n'), 0);
///     assert_eq!(char_width(' '), 1);
///     assert_eq!(char_width('a'), 1);
///     assert_eq!(char_width('ａ'), 2);
/// ```
pub fn char_width(ch: char) -> usize {
    if !is_print(ch) {
        return 0;
    }
    match EAST_ASIAN_WIDTH.get(ch) {
        EastAsianWidth::Halfwidth => 1,
        EastAsianWidth::Narrow => 1,
        EastAsianWidth::Neutral => 1,
        EastAsianWidth::Fullwidth => 2,
        EastAsianWidth::Wide => 2,
        _ => 2, // EastAsianWidth::Ambiguous => 2,
    }
}

/// Returns the display width of the specified text.
/// This function calculates the width of the text taking into account the
/// letter width determined by the Unicode Standard Annex #11 (UAX11)
/// East-Asian-Width.
///
/// ```rust
///     use linebreak::text_width;
///
///    assert_eq!(text_width("Hello, world!"), 13);
///    assert_eq!(text_width("こんにちわ、世界！"), 18);
/// ```
pub fn text_width(text: &str) -> usize {
    let mut w: usize = 0;
    for ch in text.chars() {
        w += char_width(ch);
    }
    return w;
}

#[cfg(test)]
mod test_of_unicode {
    use super::*;

    #[test]
    fn test_is_print() {
        for ch in '\0'..char::MAX {
            check_is_print(ch);
        }
    }

    fn check_is_print(ch: char) {
        let b = is_print(ch);
        match GENERAL_CATEGORY.get(ch) {
            GeneralCategory::Control => assert_eq!(b, false), // Cc
            GeneralCategory::Format => assert_eq!(b, false),  // Cf
            //GeneralCategory::Surrogate => assert_eq!(b, false), // Cs, impossible
            GeneralCategory::PrivateUse => assert_eq!(b, false), // Co
            GeneralCategory::Unassigned => assert_eq!(b, false), // Cn
            GeneralCategory::LineSeparator => assert_eq!(b, false), // Zl
            GeneralCategory::ParagraphSeparator => assert_eq!(b, false), // Zp
            GeneralCategory::SpaceSeparator => {
                // Zs
                if ch == ' ' {
                    // 0x20
                    assert_eq!(b, true);
                } else {
                    assert_eq!(b, false);
                }
            }
            _ => assert_eq!(b, true),
        }
    }

    #[test]
    fn test_char_width() {
        let ch = 'क';
        assert_eq!(EAST_ASIAN_WIDTH.get(ch), EastAsianWidth::Neutral);
        assert_eq!(char_width(ch), 1);

        let ch = 'α';
        assert_eq!(EAST_ASIAN_WIDTH.get(ch), EastAsianWidth::Ambiguous);
        assert_eq!(char_width(ch), 2);

        let ch = 'ｱ';
        assert_eq!(EAST_ASIAN_WIDTH.get(ch), EastAsianWidth::Halfwidth);
        assert_eq!(char_width(ch), 1);

        let ch = 'Ａ';
        assert_eq!(EAST_ASIAN_WIDTH.get(ch), EastAsianWidth::Fullwidth);
        assert_eq!(char_width(ch), 2);

        let ch = 'A';
        assert_eq!(EAST_ASIAN_WIDTH.get(ch), EastAsianWidth::Narrow);
        assert_eq!(char_width(ch), 1);

        let ch = 'ア';
        assert_eq!(EAST_ASIAN_WIDTH.get(ch), EastAsianWidth::Wide);
        assert_eq!(char_width(ch), 2);

        let ch = '\t';
        assert_eq!(EAST_ASIAN_WIDTH.get(ch), EastAsianWidth::Neutral);
        assert_eq!(char_width(ch), 0);
    }

    #[test]
    fn test_text_width() {
        assert_eq!(text_width("abc"), 3);
        assert_eq!(text_width("あいう"), 6);
        assert_eq!(text_width(""), 0);
    }
}
