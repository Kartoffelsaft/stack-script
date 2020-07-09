use stack_script::token_type::*;
use rand::prelude::*;
use either::Either;

#[test]
fn ints() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let rand_num = rng.gen::<i32>();
        assert_eq!(
            RawToken::from_str(format!("{}", rand_num).as_str()),
            RawToken::Number(Either::Left(rand_num as isize))
        )
    }
}

#[test]
fn floats() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let rand_num = rng.gen::<f32>() * 100f32 - 50f32;
        if let RawToken::Number(Either::Right(parsed_value))
        = RawToken::from_str(format!("{}", rand_num).as_str()) {
            assert!(
                (rand_num as f64 - parsed_value).abs() < 0.00001,
                "parsed value out of bounds of expected"
            )
        } else { panic!("float was not parsed to be one"); }
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
        RawToken::Keyword(RawStandardKeyword::Stdout)
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
