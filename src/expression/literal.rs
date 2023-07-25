#[derive(Debug, PartialEq)]
pub struct LiteralExpression {
    operator: LiteralOperator,
}

impl LiteralExpression {
    pub fn new(operator: LiteralOperator) -> Self { Self { operator } }

    pub fn boolean(value: bool) -> Self {
        Self::new(LiteralOperator::Boolean(value))
    }

    pub fn nil() -> Self {
        Self::new(LiteralOperator::Nil)
    }

    pub fn number(value: f64) -> Self {
        Self::new(LiteralOperator::Number(value))
    }

    pub fn string(value: String) -> Self {
        Self::new(LiteralOperator::String(value))
    }
}

impl std::fmt::Display for LiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.operator)
    }
}

#[derive(PartialEq, Debug)]
pub enum LiteralOperator {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl std::fmt::Display for LiteralOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(x) => write!(f, "{x}"),
            Self::Number(x) => write!(f, "{x}"),
            Self::String(x) => write!(f, "{x}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

