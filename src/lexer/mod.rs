pub mod tokens;
pub mod keywords;

use tokens::Token;
use regex::Regex;

mod re {
    use lazy_static::lazy_static;
    use regex::RegexSet;
    const BRACE_O: usize = 0;
    const BRACE_C: usize = 1;
    const PAREN_O: usize = 2;
    const PAREN_C: usize = 3;
    const SEMICOLON: usize = 4;
    const KW_INT: usize = 5;
    const KW_RETURN: usize = 6;
    const IDENTIFIER: usize = 7;
    const INT_LITERAL: usize = 8;
    lazy_static! {
        pub static ref TOKEN: RegexSet = RegexSet::new(&[
            r"{",
            r"}",
            r"(",
            r")",
            r";",
            r"\bint\b",
            r"\breturn\b",
            r"\b[a-zA-Z]\w*\b",
            r"\d+",
        ]).unwrap();
    }
}

pub fn lex(source: String) -> Vec<Token> {
    let tokens: Vec<Token> = vec![];
    for token in re::TOKEN.matches(&source) {

    }
    tokens
}
