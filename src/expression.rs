#[derive(PartialEq, Debug)]
pub enum Expr {
    Arithmetic(BinaryExpression),
    Compare(BinaryExpression),
    Equality(BinaryExpression),
    Grouping(Box<Expr>),
    Literal(LiteralOperator),
    Unary(UnaryOperator, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unary(x, r) => write!(f, "({x}{r})"),
            Self::Arithmetic(expr) => write!(f, "({expr})"),
            Self::Compare(expr) => write!(f, "({expr})"),
            Self::Equality(expr) => write!(f, "({expr})"),
            Self::Grouping(e) => write!(f, "({e})"),
            Self::Literal(x) => write!(f, "{x}"),
        }
    }
}

#[derive(Debug, PartialEq)]
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
}


#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum LiteralOperator {
    False,
    Nil,
    Number(f64),
    String(String),
    True,
}

impl std::fmt::Display for LiteralOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{x}"),
            Self::String(x) => write!(f, "{x}"),
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
            Self::Nil => write!(f, "nil"),
        }
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

#[derive(Debug)]
pub enum Error {
    ExpectExpression,
    ExpectRightParen,
    UnexpecedCharacter(crate::token::TokenKind),
}
