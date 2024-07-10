use std::{ str::Chars, iter::{ Iterator, Peekable } };

#[derive(Debug, PartialEq, Eq)]
pub enum LexToken {
    // Keywords
    Let,

    // Identifiers and Literals
    Ident(String),
    Integer(i64),

    // Operators
    Eq,

    // Delimiters
    Semicolon,
}

pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable()
        }
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
    fn it_works() {
        let input = "let foo = 123;";

        let mut lexer = Lexer::new(input);

        assert_eq!(
            Some(LexToken::Let),
            lexer.next()
        );

        assert_eq!(
            Some(LexToken::Ident("foo".into())),
            lexer.next()
        );

        assert_eq!(
            Some(LexToken::Eq),
            lexer.next()
        );

        assert_eq!(
            Some(LexToken::Integer(123)),
            lexer.next()
        );

        assert_eq!(
            Some(LexToken::Semicolon),
            lexer.next()
        );
        
        assert_eq!(
            None,
            lexer.next()
        )
    }
}