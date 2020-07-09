use stack_script::*;

fn main() {
    println!(
        "{:#?}",
        token_type::Block::from_rawtoken_iter(
            &mut token_type::RawToken::from_vec_str(
                tokenizer::tokenize("stdin\n3.14 * doThing stdout doThing [i32] [f32] {\n\t+\n} fndef")
            ).into_iter()
        )
    );
}
