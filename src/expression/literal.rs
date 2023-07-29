#[derive(Debug, PartialEq, Clone)]
pub struct LiteralExpression {
    value: LiteralOperator,
}

impl LiteralExpression {
    pub fn new(operator: LiteralOperator) -> Self { Self { value: operator } }

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

    pub fn value(&self) -> &LiteralOperator {
        &self.value
    }
}

impl std::fmt::Display for LiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum LiteralOperator {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl LiteralOperator {
    pub fn as_number(&self) -> Option<&f64> {
        if let Self::Number(v) = self {
            Some(v)
        } else {
            None
        }
    }
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

