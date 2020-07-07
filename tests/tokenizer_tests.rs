use stack_script::tokenizer::*;

#[test]
fn hello_world() {
    assert_eq!(
        tokenize("Hello, World!"),
        vec!["Hello,", "World!"]
    )
}

#[test]
fn empty() {
    assert_eq!(
        tokenize(""),
        Vec::<&str>::new()
    )
}

#[test]
fn extra_whitespace_between() {
    assert_eq!(
        tokenize("text_a  \t\t  \n   \r\n      \t  text_b"),
        vec!["text_a", "text_b"]
    )
}

#[test]
fn extra_whitespace_around() {
    assert_eq!(
        tokenize("     \t  \t \r\n   \n\n\n abc  \r    \t \n"),
        vec!["abc"]
    )
}

#[test]
fn escaped_letters_are_escaped() {
    assert_eq!(
        tokenize("1 trnvf 2"),
        vec!["1", "trnvf", "2"]
    )
}
