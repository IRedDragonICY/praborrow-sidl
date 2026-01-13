use crate::error::SidlError;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Struct,
    Service,
    Fn,
    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,
    Colon,
    SemiColon,
    Arrow,
    Comma,
    Ident(String),
    Eof,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, SidlError> {
        self.skip_whitespace();

        match self.chars.next() {
            Some('{') => Ok(Token::BraceOpen),
            Some('}') => Ok(Token::BraceClose),
            Some('(') => Ok(Token::ParenOpen),
            Some(')') => Ok(Token::ParenClose),
            Some(':') => Ok(Token::Colon),
            Some(';') => Ok(Token::SemiColon),
            Some(',') => Ok(Token::Comma),
            Some('-') => {
                if let Some('>') = self.chars.peek() {
                    self.chars.next();
                    Ok(Token::Arrow)
                } else {
                    Err(SidlError::UnexpectedChar('-'))
                }
            }
            Some(c) if c.is_alphabetic() || c == '_' => Ok(self.lex_ident(c)),
            None => Ok(Token::Eof),
            Some(c) => Err(SidlError::UnexpectedChar(c)),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn lex_ident(&mut self, start: char) -> Token {
        let mut ident = String::new();
        ident.push(start);

        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "struct" => Token::Struct,
            "service" => Token::Service,
            "fn" => Token::Fn,
            _ => Token::Ident(ident),
        }
    }
}
