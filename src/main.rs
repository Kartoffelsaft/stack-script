use stack_script::*;

fn main() {
    println!(
        "{:#?}",
        token_type::RawToken::from_vec_str(
            tokenizer::tokenize("stdin\n3.14 * stdout doThing [i32] [f64] {\n\t+\n} fndef")
        )
    );
}
