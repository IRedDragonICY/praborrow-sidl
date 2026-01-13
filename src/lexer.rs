use crate::error::{Location, SidlError};
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
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            line: 1,
            col: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), SidlError> {
        loop {
            match self.peek_char() {
                Some(c) if c.is_whitespace() => {
                    self.advance();
                }
                Some('/') => {
                    self.advance(); // consume first '/'
                    match self.peek_char() {
                        Some('/') => {
                            self.advance(); // consume second '/'
                            while let Some(&c) = self.peek_char() {
                                if c == '\n' {
                                    break;
                                }
                                self.advance();
                            }
                        }
                        _ => {
                            return Err(SidlError::UnexpectedChar {
                                char: '/',
                                loc: Location {
                                    line: self.line,
                                    col: self.col - 1,
                                },
                            });
                        }
                    }
                }
                _ => break,
            }
        }
        Ok(())
    }

    pub fn next_token(&mut self) -> Result<(Token, Location), SidlError> {
        self.skip_whitespace_and_comments()?;

        let start_loc = Location {
            line: self.line,
            col: self.col,
        };

        match self.advance() {
            Some('{') => Ok((Token::BraceOpen, start_loc)),
            Some('}') => Ok((Token::BraceClose, start_loc)),
            Some('(') => Ok((Token::ParenOpen, start_loc)),
            Some(')') => Ok((Token::ParenClose, start_loc)),
            Some(':') => Ok((Token::Colon, start_loc)),
            Some(';') => Ok((Token::SemiColon, start_loc)),
            Some(',') => Ok((Token::Comma, start_loc)),
            Some('-') => {
                if let Some('>') = self.peek_char() {
                    self.advance();
                    Ok((Token::Arrow, start_loc))
                } else {
                    Err(SidlError::UnexpectedChar {
                        char: '-',
                        loc: start_loc,
                    })
                }
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                let token = self.lex_ident(c);
                Ok((token, start_loc))
            }
            None => Ok((Token::Eof, start_loc)),
            Some(c) => Err(SidlError::UnexpectedChar {
                char: c,
                loc: start_loc,
            }),
        }
    }

    fn lex_ident(&mut self, start: char) -> Token {
        let mut ident = String::new();
        ident.push(start);

        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_basic() {
        let input = "struct Foo { x: i32 }";
        let mut lexer = Lexer::new(input);

        assert!(matches!(lexer.next_token().unwrap(), (Token::Struct, _)));
        assert!(matches!(lexer.next_token().unwrap(), (Token::Ident(s), _) if s == "Foo"));
        assert!(matches!(lexer.next_token().unwrap(), (Token::BraceOpen, _)));
        assert!(matches!(lexer.next_token().unwrap(), (Token::Ident(s), _) if s == "x"));
        assert!(matches!(lexer.next_token().unwrap(), (Token::Colon, _)));
        assert!(matches!(lexer.next_token().unwrap(), (Token::Ident(s), _) if s == "i32"));
        assert!(matches!(
            lexer.next_token().unwrap(),
            (Token::BraceClose, _)
        ));
        assert!(matches!(lexer.next_token().unwrap(), (Token::Eof, _)));
    }

    #[test]
    fn test_comments() {
        let input = "
        // This is a comment
        struct // another comment
        Foo // comment at end of line
        {
        }
        ";
        let mut lexer = Lexer::new(input);
        assert!(matches!(lexer.next_token().unwrap(), (Token::Struct, _)));
        assert!(matches!(lexer.next_token().unwrap(), (Token::Ident(s), _) if s == "Foo"));
        assert!(matches!(lexer.next_token().unwrap(), (Token::BraceOpen, _)));
        assert!(matches!(
            lexer.next_token().unwrap(),
            (Token::BraceClose, _)
        ));
        assert!(matches!(lexer.next_token().unwrap(), (Token::Eof, _)));
    }

    #[test]
    fn test_location() {
        let input = "struct\nFoo";
        let mut lexer = Lexer::new(input);

        let (_, loc) = lexer.next_token().unwrap();
        assert_eq!(loc.line, 1);
        assert_eq!(loc.col, 1);

        let (_, loc) = lexer.next_token().unwrap();
        assert_eq!(loc.line, 2);
        assert_eq!(loc.col, 1);
    }

    #[test]
    fn test_error_location() {
        let input = "struct %";
        let mut lexer = Lexer::new(input);
        lexer.next_token().unwrap(); // struct

        match lexer.next_token() {
            Err(SidlError::UnexpectedChar { char, loc }) => {
                assert_eq!(char, '%');
                assert_eq!(loc.line, 1);
                assert_eq!(loc.col, 8); // 'struct' is 6 chars + space = 7. Next is 8?
                // 'struct' consumes 6. Lexer is at col 7 (space).
                // skip_whitespace consumes space (col 7). Lexer at col 8 matching '%'.
                // Yes.
            }
            _ => panic!("Expected UnexpectedChar error"),
        }
    }
}
