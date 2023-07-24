mod parse;

use crate::expression::{Error, Expr};
use crate::token::Token;

pub fn parse<I: Iterator<Item = Token>>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Expr, Error> {
    parse::parse_expression(tokens)
}

#[cfg(test)]
mod tests {
    use crate::expression::{Binary, Compare, Equality, Expr, Literal, Unary};
    use crate::lexer::Lexer;

    use super::parse;

    #[test]
    fn test_parser_expression_presidence_add_mult() {
        let mut tokens = Lexer::from_iter("2 + 2 * 2".chars()).peekable();
        let expect = Expr::Binary(
            Box::new(Expr::Number(2_f64)),
            Binary::Add,
            Box::new(Expr::Binary(
                Box::new(Expr::Number(2_f64)),
                Binary::Mult,
                Box::new(Expr::Number(2_f64)),
            )),
        );
        assert_eq!(expect, parse(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_expression_presidence_mult_add() {
        let mut tokens = Lexer::from_iter("2 * 2 + 2".chars()).peekable();
        let expect = Expr::Binary(
            Box::new(Expr::Binary(
                Box::new(Expr::Number(2_f64)),
                Binary::Mult,
                Box::new(Expr::Number(2_f64)),
            )),
            Binary::Add,
            Box::new(Expr::Number(2_f64)),
        );
        assert_eq!(expect, parse(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_expression() {
        let mut tokens = Lexer::from_iter("2 < 3 >= 1 == false".chars()).peekable();
        let left = Expr::Compare(
            Box::new(Expr::Compare(
                Box::new(Expr::Number(2_f64)),
                Compare::Less,
                Box::new(Expr::Number(3_f64)),
            )),
                Compare::GreaterEqual,
            Box::new(Expr::Number(1_f64)),
        );
        let right = Expr::Literal(Literal::False);
        let expect = Expr::Equality(Box::new(left), Equality::Equal, Box::new(right));
        let result = parse(&mut tokens).unwrap();
        assert_eq!(expect, result);
    }
}
