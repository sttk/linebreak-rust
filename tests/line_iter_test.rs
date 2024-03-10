use linebreak::LineIter;

#[test]
fn it_should_print_lines() {
    let text = "Welcome to The Rust Programming Language, an introductory \
      book about Rust. The Rust programming language helps you write faster, \
      more reliable software. High-level ergonomics and low-level control are \
      often at odds in programming language design; Rust challenges that \
      conflict. Through balancing powerful technical capacity and a great \
      developer experience, Rust gives you the option to control low-level \
      details (such as memory usage) without all the hassle traditionally \
      associated with such control.";

    let mut iter = LineIter::new(&text, 80);
    iter.set_indent("_______");

    println!(
        "....:....1....:....2....:....3....:....4....:....5....:....6\
              ....:....7....:....8"
    );
    while let Some(line) = iter.next() {
        println!("{}", line);
    }
}
