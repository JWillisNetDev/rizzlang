use phf::phf_map;
use std::{
    iter::{Iterator, Peekable},
    str::Chars,
};

static KEYWORDS: phf::Map<&'static str, LexToken> = phf_map! {
    "let" => LexToken::Let,
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

    // Error
    Error(String),
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input: input.chars().peekable(), }
    }

    fn read_to_delimiter(&mut self) -> String {
        let mut buffer = String::new();

        while let Some(c) = self.input.peek() {
            if is_delimiter(*c) {
                break;
            }

            buffer.push(self.input.next().unwrap());
        }

        buffer
    }

    fn read_identifier_or_keyword(&mut self) -> Result<LexToken, String> {
        let identifier = self.read_to_delimiter();
        if identifier.is_empty() {
            return Err("Attempted to read, but found nothing.".into());
        }

        if let Some(token) = try_get_keyword(identifier.as_str()) {
            Ok(token)
        } else {
            Ok(LexToken::Identifier(identifier))
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.peek() {
            Some(c) if c.is_alphabetic() => match self.read_identifier_or_keyword() {
                Ok(identifier) => Some(identifier),
                Err(err) => Some(LexToken::Error(err))
            }
            _ => None,
        }
    }
}

fn try_get_keyword(keyword: &str) -> Option<LexToken> {
    Some(KEYWORDS.get(keyword)?.clone())
}

fn is_delimiter(c: char) -> bool {
    c == ' ' || c == ';'
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
