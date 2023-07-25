
mod binary;
mod literal;
mod unary;

pub use self::literal::LiteralExpression;
pub use self::unary::UnaryExpression;
pub use self::binary::BinaryExpression;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Arithmetic(binary::BinaryExpression),
    Compare(binary::BinaryExpression),
    Equality(binary::BinaryExpression),
    Grouping(Box<Expr>),
    Literal(literal::LiteralExpression),
    Unary(unary::UnaryExpression),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unary(expr) => write!(f, "({expr})"),
            Self::Arithmetic(expr) => write!(f, "({expr})"),
            Self::Compare(expr) => write!(f, "({expr})"),
            Self::Equality(expr) => write!(f, "({expr})"),
            Self::Grouping(e) => write!(f, "({e})"),
            Self::Literal(x) => write!(f, "{x}"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ExpectExpression,
    ExpectRightParen,
    UnexpecedCharacter(crate::token::TokenKind),
}
