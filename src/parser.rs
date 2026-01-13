use crate::error::{Location, SidlError};
use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct MethodDef {
    pub name: String,
    pub arg_name: String,
    pub arg_type: String,
    pub ret_type: String,
}

#[derive(Debug)]
pub struct ServiceDef {
    pub name: String,
    pub methods: Vec<MethodDef>,
}

#[derive(Debug)]
pub enum Def {
    Struct(StructDef),
    Service(ServiceDef),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    current_loc: Location,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self, SidlError> {
        let (current_token, current_loc) = lexer.next_token()?;
        Ok(Self {
            lexer,
            current_token,
            current_loc,
        })
    }

    fn advance(&mut self) -> Result<(), SidlError> {
        let (token, loc) = self.lexer.next_token()?;
        self.current_token = token;
        self.current_loc = loc;
        Ok(())
    }

    fn expect(&mut self, expected: Token) -> Result<(), SidlError> {
        if self.current_token == expected {
            self.advance()?;
            Ok(())
        } else {
            Err(SidlError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: format!("{:?}", self.current_token),
                loc: self.current_loc,
            })
        }
    }

    fn parse_ident(&mut self) -> Result<String, SidlError> {
        if let Token::Ident(name) = &self.current_token {
            let name = name.clone();
            self.advance()?;
            Ok(name)
        } else {
            Err(SidlError::UnexpectedToken {
                expected: "Identifier".to_string(),
                found: format!("{:?}", self.current_token),
                loc: self.current_loc,
            })
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Def>, SidlError> {
        let mut defs = Vec::new();
        loop {
            match self.current_token {
                Token::Struct => defs.push(Def::Struct(self.parse_struct()?)),
                Token::Service => defs.push(Def::Service(self.parse_service()?)),
                Token::Eof => break,
                _ => {
                    return Err(SidlError::UnexpectedTopLevelToken(
                        format!("{:?}", self.current_token),
                        self.current_loc,
                    ));
                }
            }
        }
        Ok(defs)
    }

    fn parse_struct(&mut self) -> Result<StructDef, SidlError> {
        self.advance()?; // consume 'struct'
        let name = self.parse_ident()?;

        self.expect(Token::BraceOpen)?;

        let mut fields = Vec::new();
        while self.current_token != Token::BraceClose {
            let field_name = self.parse_ident()?;
            self.expect(Token::Colon)?;
            let field_type = self.parse_ident()?;
            self.expect(Token::Comma)?;
            fields.push((field_name, field_type));
        }

        self.expect(Token::BraceClose)?;
        Ok(StructDef { name, fields })
    }

    fn parse_service(&mut self) -> Result<ServiceDef, SidlError> {
        self.advance()?; // consume 'service'
        let name = self.parse_ident()?;

        self.expect(Token::BraceOpen)?;

        let mut methods = Vec::new();
        while self.current_token != Token::BraceClose {
            self.expect(Token::Fn)?;
            let method_name = self.parse_ident()?;

            self.expect(Token::ParenOpen)?;
            let arg_name = self.parse_ident()?;
            self.expect(Token::Colon)?;
            let arg_type = self.parse_ident()?;
            self.expect(Token::ParenClose)?;

            self.expect(Token::Arrow)?;
            let ret_type = self.parse_ident()?;
            self.expect(Token::SemiColon)?;

            methods.push(MethodDef {
                name: method_name,
                arg_name,
                arg_type,
                ret_type,
            });
        }

        self.expect(Token::BraceClose)?;
        Ok(ServiceDef { name, methods })
    }
}
