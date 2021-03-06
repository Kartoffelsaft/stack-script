use regex::Regex;

#[cfg(test)]
mod tokenizer_tests;

pub fn tokenize(string: &str) -> Vec<&str> {
    let whitespace = Regex::new(r"[ \t\r\n\v\f]+")
        .expect("could not parse whitespace regex");
    whitespace.split(string)
        .filter(|s| *s != "")
        .collect()
}
