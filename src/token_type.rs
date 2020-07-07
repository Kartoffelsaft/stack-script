use either::Either;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq)]
pub enum RawToken<'a> {
    StartBlock,
    EndBlock,
    LangType(PrimitiveType),
    LangTypeCast(PrimitiveType),
    Number(Either<isize, f64>),
    Keyword(StandardKeyword),
    UserDefinedToken(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
pub enum PrimitiveType {
    Float32,
    Int32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum StandardKeyword {
    Fndef,
    Stdin,
    Stdout,
    Add,
    Mul,
    Sub,
    Div,
    Copy,
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
        if let Some(keyword) = StandardKeyword::from_str(match_string) {
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

impl StandardKeyword {
    fn from_str(string: &str) -> Option<StandardKeyword> {
        match string {
            "fndef" => Some(StandardKeyword::Fndef),
            "stdin" => Some(StandardKeyword::Stdin),
            "stdout" => Some(StandardKeyword::Stdout),
            "+" => Some(StandardKeyword::Add),
            "*" => Some(StandardKeyword::Mul),
            "-" => Some(StandardKeyword::Sub),
            "/" => Some(StandardKeyword::Div),
            "#" => Some(StandardKeyword::Copy),
            _ => None
        }
    }
}
