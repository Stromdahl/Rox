use super::Expr;

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
    operator :UnaryOperator,
    right: Box<Expr>
}

impl std::fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.operator, self.right)
    }
}

impl UnaryExpression {
    fn new(operator: UnaryOperator, right: Expr) -> Self { 
        Self { operator, right: Box::new(right) } 
    }

    pub fn bang(right: Expr) -> Self {
        Self::new(UnaryOperator::Bang, right)
    }

    pub fn minus(right: Expr) -> Self {
        Self::new(UnaryOperator::Minus, right)
    }

}

#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Bang,
    Minus,
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Bang => write!(f, "!"),
        }
    }
}

