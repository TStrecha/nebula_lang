#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Item {
    Expr(Expr),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Expr {
    VarDecl { name: String, ty: String, value: Box<Expr> },
    Lit(Literal),
    Ident(String),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal {
    Number(u128),
    Decimal(f64),
    StringLit(String),
}