use std::{
    iter::{Iterator, Peekable},
    str::Chars,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexToken {
    // Keywords
    Let,

    // Identifiers and Literals
    Identifier(String),
    Integer(String),

    // Operators
    Equals,
    Plus,

    // Delimiters
    Semicolon,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input: input.chars().peekable(), }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexToken;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_lexes() {
        let input = "let variable = 1 + 2;";

        let mut lexer = Lexer::new(input.into());

        assert_eq!(Some(LexToken::Let), lexer.next());

        assert_eq!(Some(LexToken::Identifier("variable".into())), lexer.next());

        assert_eq!(Some(LexToken::Equals), lexer.next());

        assert_eq!(Some(LexToken::Integer("1".into())), lexer.next());

        assert_eq!(Some(LexToken::Plus), lexer.next());

        assert_eq!(Some(LexToken::Integer("2".into())), lexer.next());

        assert_eq!(Some(LexToken::Semicolon), lexer.next());

        assert_eq!(None, lexer.next());
    }
}
