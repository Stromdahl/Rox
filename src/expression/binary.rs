use super::Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    left: Box<Expr>,
    operator :BinaryOperator,
    right: Box<Expr>
}

impl std::fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl BinaryExpression {
    pub fn new(left: Expr, operator: BinaryOperator, right: Expr) -> Self { 
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right)
        } 
    }

    pub fn add(left: Expr, right: Expr) -> Self {
        Self::new(left, BinaryOperator::Add, right)
    }

    pub fn mult(left: Expr, right: Expr) -> Self {
        Self::new(left, BinaryOperator::Mult, right)
    }

    pub fn div(left: Expr, right: Expr) -> Self {
        Self::new(left, BinaryOperator::Div, right)
    }

    pub fn sub(left: Expr, right: Expr) -> Self {
        Self::new(left, BinaryOperator::Sub, right)
    }

    pub fn greater_equal(left: Expr, right: Expr) -> Self {
        Self::new(left, BinaryOperator::GreaterEqual, right)
    }

    pub fn greater(left: Expr, right: Expr) -> Self {
        Self::new(left, BinaryOperator::Greater, right)
    }

    pub fn equal(left: Expr, right: Expr) -> Self {
        Self::new(left, BinaryOperator::Equal, right)
    }

    pub fn less_equal(left: Expr, right: Expr) -> Self {
        Self::new(left, BinaryOperator::LessEqual, right)
    }

    pub fn less(left: Expr, right: Expr) -> Self {
        Self::new(left, BinaryOperator::Less, right)
    }

    pub fn not_equal(left: Expr, right: Expr) -> Self {
        Self::new(left, BinaryOperator::NotEqual, right)
    }

    pub fn left(&self) -> &Expr {
        self.left.as_ref()
    }

    pub fn operator(&self) -> BinaryOperator {
        self.operator
    }

    pub fn right(&self) -> &Expr {
        self.right.as_ref()
    }
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BinaryOperator {
    Mult,
    Div,
    Add,
    Sub,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mult => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Greater => write!(f, ">"),
            Self::Less => write!(f, "<"),
            Self::GreaterEqual => write!(f, ">="),
            Self::LessEqual => write!(f, "<="),
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
        }
    }
}

