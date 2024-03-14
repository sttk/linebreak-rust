use linebreak::{term_cols, term_size};
use std::env;

#[test]
fn it_should_get_terminal_column_number() {
    let cols = term_cols();
    if env::var("CI").is_err() {
        assert!(cols > 0);
    } else {
        assert_eq!(cols, 80);
    }
}

#[test]
fn it_should_get_terminal_size() {
    let size = term_size();
    if env::var("CI").is_err() {
        assert!(size.col > 0);
        assert!(size.row > 0);
    } else {
        assert_eq!(size.col, 80);
        assert_eq!(size.row, 24);
    }
}
