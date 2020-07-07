use stack_script::token_type::*;
use rand::prelude::*;
use either::Either;

#[test]
fn test_numbers() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let rand_num = rng.gen::<i32>();
        assert_eq!(
            RawToken::from_str(format!("{}", rand_num).as_str()),
            RawToken::Number(Either::Left(rand_num as isize))
        )
    }
}

// TODO: make this expect a none or error
#[test]
fn type_failure() {
    assert_eq!(
        RawToken::from_str("[fake_type]"),
        RawToken::UserDefinedToken("[fake_type]")
    )
}

#[test]
fn type_success() {
    assert_eq!(
        RawToken::from_str("[i32]"),
        RawToken::LangType(PrimitiveType::Int32)
    )
}

#[test]
fn type_cast() {
    assert_eq!(
        RawToken::from_str("(f32)"),
        RawToken::LangTypeCast(PrimitiveType::Float32)
    )
}

#[test]
fn keyword() {
    assert_eq!(
        RawToken::from_str("stdout"),
        RawToken::Keyword(StandardKeyword::Stdout)
    )
}

#[test]
fn from_vec_str() {
    assert_eq!(
        RawToken::from_vec_str(vec!["{", "test", "}"]),
        vec![
            RawToken::StartBlock,
            RawToken::UserDefinedToken("test"),
            RawToken::EndBlock
        ]
    )
}
