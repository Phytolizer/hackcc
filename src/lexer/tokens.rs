use super::keywords::Keyword;
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    ParenO,
    ParenC,
    BraceO,
    BraceC,
    IntLiteral(usize),
    Semicolon,
}