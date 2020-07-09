use either::Either;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    user_defs: HashMap<&'a str, Block<'a>>,
    operations: Vec<RefinedToken<'a>>
}

#[derive(Debug, PartialEq)]
enum RefinedToken<'a> {
    LangType(PrimitiveType),
    LangTypeCast(PrimitiveType),
    Number(Either<isize, f64>),
    Keyword(RefinedStandardKeyword),
    Call(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
enum RefinedStandardKeyword {
    Stdin,
    Stdout,
    Add,
    Mul,
    Sub,
    Div,
    Copy,
}

#[derive(Debug, PartialEq)]
pub enum RawToken<'a> {
    StartBlock,
    EndBlock,
    LangType(PrimitiveType),
    LangTypeCast(PrimitiveType),
    Number(Either<isize, f64>),
    Keyword(RawStandardKeyword),
    UserDefinedToken(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
pub enum PrimitiveType {
    Float32,
    Int32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum RawStandardKeyword {
    Fndef,
    Stdin,
    Stdout,
    Add,
    Mul,
    Sub,
    Div,
    Copy,
}

impl Block<'_> {
    pub fn from_rawtoken_iter<'a>(iter: &mut std::vec::IntoIter<RawToken<'a>>) -> Block<'a> {
        let mut new_user_defs = HashMap::new();
        let mut new_operations = Vec::new();

        let mut token_stack = Vec::<(&str, usize)>::new();
        let mut block_stack = Vec::<Block>::new();

        use RawToken::*;
        while let Some(next) = iter.next() { match next {
            StartBlock => block_stack.push(Block::from_rawtoken_iter(iter)),
            EndBlock => break,
            LangType(t) => new_operations.push(RefinedToken::LangType(t)),
            LangTypeCast(t) => new_operations.push(RefinedToken::LangTypeCast(t)),
            Number(n) => new_operations.push(RefinedToken::Number(n)),
            Keyword(w) => match w {
                RawStandardKeyword::Fndef => {
                    let (new_fn_name, definition_index) =
                        token_stack.pop().expect("Function defined without name");
                    match new_user_defs.insert(
                        new_fn_name,
                        block_stack.pop().expect("Function defined without body"),
                    ) { None => (), Some(_) => eprintln!("WARN: Function defined twice. Overwriting."),}
                    new_operations.remove(definition_index);
                },
                r => new_operations.push(
                    RefinedToken::Keyword(
                        RefinedStandardKeyword::from_raw(r)
                            .expect("raw standard keyword not properly filtered")
                    )
                ),
            },
            UserDefinedToken(t) => {
                token_stack.push((t, new_operations.len()));
                new_operations.push(RefinedToken::Call(t))
            },
        }}
        Block {
            user_defs: new_user_defs,
            operations: new_operations,
        }
    }
}

impl RefinedStandardKeyword {
    pub fn from_raw(kw: RawStandardKeyword) -> Result<RefinedStandardKeyword, ()> {
        match kw {
            RawStandardKeyword::Stdin => Ok(RefinedStandardKeyword::Stdin),
            RawStandardKeyword::Stdout => Ok(RefinedStandardKeyword::Stdout),
            RawStandardKeyword::Add => Ok(RefinedStandardKeyword::Add),
            RawStandardKeyword::Mul => Ok(RefinedStandardKeyword::Mul),
            RawStandardKeyword::Sub => Ok(RefinedStandardKeyword::Sub),
            RawStandardKeyword::Div => Ok(RefinedStandardKeyword::Div),
            RawStandardKeyword::Copy => Ok(RefinedStandardKeyword::Copy),
            _ => Err(()),
        }
    }
}

impl RawToken<'_> {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(string: &str) -> RawToken {
        lazy_static! {
            static ref LANG_TYPE_RE: Regex = Regex::new(r"^\[(.*)\]$").unwrap();
            static ref LANG_TYPE_CAST_RE: Regex = Regex::new(r"^\((.*)\)$").unwrap();
            static ref NUMBER_RE: Regex = Regex::new(r"^-?[\d\.]+$").unwrap();
        }
        let match_string = string.trim();

        if match_string == "{" {
            return RawToken::StartBlock;
        }
        if match_string == "}" {
            return RawToken::EndBlock;
        }
        if let Some(capture) = LANG_TYPE_RE.captures(match_string) {
            if let Some(ptype) = PrimitiveType::from_str(capture.get(1).unwrap().as_str()) {
                return RawToken::LangType(ptype);
            } else { eprintln!("WARN:\"{}\" appears to be a type, but is not one", match_string); }
        }
        if let Some(capture) = LANG_TYPE_CAST_RE.captures(match_string) {
            if let Some(ptype) = PrimitiveType::from_str(capture.get(1).unwrap().as_str()) {
                return RawToken::LangTypeCast(ptype);
            } else { eprintln!("WARN:\"{}\" appears to be a type cast, but is not one", match_string); }
        }
        if NUMBER_RE.is_match(match_string) {
            if match_string.contains('.') {
                return RawToken::Number(Either::Right(match_string.parse::<f64>().expect("regex matched unparseable float")));
            } else {
                return RawToken::Number(Either::Left(match_string.parse::<isize>().expect("regex matched unparseable int")));
            }
        }
        if let Some(keyword) = RawStandardKeyword::from_str(match_string) {
            return RawToken::Keyword(keyword);
        }

        RawToken::UserDefinedToken(match_string)
    }

    pub fn from_vec_str<'a>(strings: Vec<&'a str>) -> Vec<RawToken<'a>> {
        strings.iter().map(|s| RawToken::from_str(*s)).collect()
    }
}


impl PrimitiveType {
    fn from_str(string: &str) -> Option<PrimitiveType> {
        match string {
            "f32" => Some(PrimitiveType::Float32),
            "i32" => Some(PrimitiveType::Int32),
            _ => None
        }
    }
}

impl RawStandardKeyword {
    fn from_str(string: &str) -> Option<RawStandardKeyword> {
        match string {
            "fndef" => Some(RawStandardKeyword::Fndef),
            "stdin" => Some(RawStandardKeyword::Stdin),
            "stdout" => Some(RawStandardKeyword::Stdout),
            "+" => Some(RawStandardKeyword::Add),
            "*" => Some(RawStandardKeyword::Mul),
            "-" => Some(RawStandardKeyword::Sub),
            "/" => Some(RawStandardKeyword::Div),
            "#" => Some(RawStandardKeyword::Copy),
            _ => None
        }
    }
}
