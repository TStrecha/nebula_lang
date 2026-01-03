use nebula_ast::item::Item;
use nebula_lexer::token::Token;
use nebula_tst::TypedSyntaxTree;
use crate::ast::parser::AstParser;
use crate::tst::parser::{TypedAstParser};

mod ast;
mod tst;

pub fn build_ast(tokens: Vec<Token>) -> Vec<Item> {
    let mut ast = vec![];
    let mut parser = AstParser::new(tokens);

    while let Some(item) = parser.next_item() {
        ast.push(item);
    }

    ast
}

pub fn build_tst(ast: Vec<Item>) -> TypedSyntaxTree {
    TypedAstParser::new(ast).build_typed_tree()
}
