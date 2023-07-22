
#[derive(PartialEq, Debug)]
pub enum Expr {
    Unary(Unary, Box<Expr>),
    Number(f64),
    String(String),
    Binary(Box<Expr>, Binary, Box<Expr>),
    Compare(Box<Expr>, Compare, Box<Expr>),
    Equality(Box<Expr>, Equality, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{x}"),
            Self::String(x) => write!(f, "{x}"),         
            Self::Unary(x, r) => write!(f, "({x}{r})"),
            Self::Binary(l, o, r) => write!(f, "({l} {o} {r})"),
            Self::Compare(l, o, r) => write!(f, "({l} {o} {r})"),
            Self::Equality(l, o, r) => write!(f, "({l} {o} {r})"),
            Self::Grouping(e) => write!(f, "({e})"),
            Self::Literal(x) => write!(f, "{x}"),         
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    Nil,
    True,
    False,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Unary {
    Bang,
    Minus,
}

impl std::fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Bang => write!(f, "!"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Binary {
    Mult,
    Div,
    Add,
    Sub,
}


impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add=> write!(f, "+"),
            Self::Sub=> write!(f, "-"),
            Self::Mult=> write!(f, "*"),
            Self::Div=> write!(f, "/"),
        }
    }
}
#[derive(PartialEq, Debug)]
pub enum Compare {
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

impl std::fmt::Display for Compare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
    Self::Greater => write!(f, ">"),
    Self::Less => write!(f, "<"),
    Self::GreaterEqual => write!(f, ">="),
    Self::LessEqual => write!(f, "<="),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Equality {
    Equal,
    NotEqual,
}

impl std::fmt::Display for Equality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ExpectExpression,
    ExpectRightParen,
    UnexpecedCharacter(crate::token::TokenKind),
}
