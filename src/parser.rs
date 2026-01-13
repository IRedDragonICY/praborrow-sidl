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
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) {
        if self.current_token == expected {
            self.advance();
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.current_token);
        }
    }

    fn parse_ident(&mut self) -> String {
        if let Token::Ident(name) = &self.current_token {
            let name = name.clone();
            self.advance();
            name
        } else {
            panic!("Expected identifier, found {:?}", self.current_token);
        }
    }

    pub fn parse(&mut self) -> Vec<Def> {
        let mut defs = Vec::new();
        loop {
            match self.current_token {
                Token::Struct => defs.push(Def::Struct(self.parse_struct())),
                Token::Service => defs.push(Def::Service(self.parse_service())),
                Token::Eof => break,
                _ => panic!("Unexpected token at top level: {:?}", self.current_token),
            }
        }
        defs
    }

    fn parse_struct(&mut self) -> StructDef {
        self.advance(); // consume 'struct'
        let name = self.parse_ident();

        self.expect(Token::BraceOpen);

        let mut fields = Vec::new();
        while self.current_token != Token::BraceClose {
            let field_name = self.parse_ident();
            self.expect(Token::Colon);
            let field_type = self.parse_ident();
            self.expect(Token::Comma);
            fields.push((field_name, field_type));
        }

        self.expect(Token::BraceClose);
        StructDef { name, fields }
    }

    fn parse_service(&mut self) -> ServiceDef {
        self.advance(); // consume 'service'
        let name = self.parse_ident();

        self.expect(Token::BraceOpen);

        let mut methods = Vec::new();
        while self.current_token != Token::BraceClose {
            self.expect(Token::Fn);
            let method_name = self.parse_ident();

            self.expect(Token::ParenOpen);
            let arg_name = self.parse_ident();
            self.expect(Token::Colon);
            let arg_type = self.parse_ident();
            self.expect(Token::ParenClose);

            self.expect(Token::Arrow);
            let ret_type = self.parse_ident();
            self.expect(Token::SemiColon);

            methods.push(MethodDef {
                name: method_name,
                arg_name,
                arg_type,
                ret_type,
            });
        }

        self.expect(Token::BraceClose);
        ServiceDef { name, methods }
    }
}
