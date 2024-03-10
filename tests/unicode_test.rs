use linebreak::{char_width, is_print, text_width};

#[test]
fn it_should_get_char_width() {
    assert_eq!(char_width('\n'), 0);
    assert_eq!(char_width('\r'), 0);

    assert_eq!(char_width(' '), 1);
    assert_eq!(char_width('a'), 1);
    assert_eq!(char_width('A'), 1);
    assert_eq!(char_width('1'), 1);
    assert_eq!(char_width('#'), 1);
    assert_eq!(char_width('ｱ'), 1);

    assert_eq!(char_width('Ａ'), 2);
    assert_eq!(char_width('あ'), 2);
    assert_eq!(char_width('ア'), 2);
    assert_eq!(char_width('ァ'), 2);
    assert_eq!(char_width('＃'), 2);
}

#[test]
fn it_should_get_text_width() {
    assert_eq!(text_width("Hello, world!"), 13);
    assert_eq!(text_width("こんにちわ、世界！"), 18);
}

#[test]
fn it_should_check_if_char_is_print() {
    assert_eq!(is_print('\r'), false);
    assert_eq!(is_print('\n'), false);

    assert_eq!(is_print(' '), true);
    assert_eq!(is_print('a'), true);
    assert_eq!(is_print('A'), true);
    assert_eq!(is_print('1'), true);
    assert_eq!(is_print('#'), true);
    assert_eq!(is_print('ｱ'), true);

    assert_eq!(is_print('Ａ'), true);
    assert_eq!(is_print('あ'), true);
    assert_eq!(is_print('ア'), true);
    assert_eq!(is_print('ァ'), true);
    assert_eq!(is_print('＃'), true);
}
