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

/// Returns true if the given character `c` is a delimiter, such as a space or semicolon.
fn is_delimiter(c: char) -> bool {
    c == ' ' || c == ';'
}

fn read_until_delimiter(lexer: &mut Lexer) -> String {
    let mut buffer = String::new();
    while let Some(c) = lexer.chars.peek() {
        if is_delimiter(*c) {
            break;
        }

        buffer.push(lexer.chars.next().unwrap())
    }

    buffer
}

fn try_read_keyword(lexer: &mut Lexer) -> Option<LexToken> {


    todo!();
}

/// Eats whitespace.
/// The next character in the buffer is guaranteed to either be `None` or a non-whitespace character.
fn eat_whitespace(lexer: &mut Lexer) {
    while let Some(c) = lexer.chars.peek() {
        if c.is_whitespace() {
            lexer.chars.next();
        } else {
            break;
        }
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

    #[test]
    fn it_eats_whitespace() {
        let input = "   a   b";

        let mut lexer = Lexer::new(input.into());

        eat_whitespace(&mut lexer);
        assert_eq!(lexer.chars.next(), Some('a'));

        eat_whitespace(&mut lexer);
        assert_eq!(lexer.chars.next(), Some('b'));
    }

    #[test]
    fn it_reads_until_delimiter() {
        let input = "asdf ghjk;qwerty";

        let mut lexer = Lexer::new(input.into());

        let actual = read_until_delimiter(&mut lexer);
        assert_eq!("asdf", actual);
        lexer.chars.next();

        let actual = read_until_delimiter(&mut lexer);
        assert_eq!("ghjk", actual);
        lexer.chars.next();

        let actual = read_until_delimiter(&mut lexer);
        assert_eq!("qwerty", actual);
        lexer.chars.next();

        let actual = read_until_delimiter(&mut lexer);
        assert_eq!("", actual);
    }
}