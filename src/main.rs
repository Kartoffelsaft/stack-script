use stack_script::{*, runner::Executable};
use std::env::args;

fn main() {
    let s: Vec<String> = args().skip(1).collect();
    let b = token_type::Block::from_rawtoken_iter(
        &mut token_type::RawToken::from_vec_str(
            tokenizer::tokenize(&s[0])
        ).into_iter()
    );

    b.execute().unwrap();
    println!();
}
