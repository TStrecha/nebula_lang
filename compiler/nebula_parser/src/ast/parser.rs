use nebula_ast::item::Item;
use nebula_lexer::{
    token::{LiteralKind, OperatorKind, Token},
};

pub struct AstParser<'t> {
    tokens: Vec<Token<'t>>,
    index: usize,
}

impl<'t> AstParser<'t> {
    pub fn new(tokens: Vec<Token<'t>>) -> Self {
        Self {
            tokens,
            index: 0,
        }
    }

    pub fn next_item(&mut self) -> Option<Item> {
        let token = self.peek();

        match token {
            Token::Keyword(_) => Some(Item::Expr(self.parse_expr())),
            _ => None,
        }
    }

    pub fn consume_ident(&mut self) -> &str {
        let token = self.consume();

        if let Token::Ident(name) = token {
            return name;
        }

        panic!("Expected identifier, found {:?}", token);
    }

    pub fn consume_value_type(&mut self) -> &str {
        self.consume_colon();
        self.consume_ident()
    }

    pub fn consume_operator(&mut self) -> &OperatorKind {
        let token = self.consume();

        if let Token::Operator(kind) = token {
            return kind;
        }

        panic!("Expected operator, found {:?}", token);
    }

    pub fn _consume_lit(&mut self) -> &LiteralKind<'_> {
        let token = self.consume();

        if let Token::Literal(kind) = token {
            return kind;
        }

        panic!("Expected literal, found {:?}", token);
    }

    pub fn consume_semicolon(&mut self) {
        let token = self.consume();

        if *token != Token::Semicolon {
            panic!("Expected semicolon, found {:?}", token);
        }
    }

    pub fn consume_colon(&mut self) {
        let token = self.consume();

        if *token != Token::Colon {
            panic!("Expected colon, found {:?}", token);
        }
    }

    pub fn peek(&self) -> &Token<'_> {
        &self.tokens[self.index.clone()]
    }

    pub fn consume(&mut self) -> &Token<'_> {
        let token = &self.tokens[self.index.clone()];

        self.index = self.index + 1;

        token
    }
}