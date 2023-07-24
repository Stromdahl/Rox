mod parse;

use crate::expression::{Error, Expr};
use crate::token::{Token, TokenKind, Keyword};

pub fn parse<I: Iterator<Item = Token>>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Expr, Error> {
    parse::parse_expression(tokens)
}

#[allow(dead_code)]
pub fn syncronize<I: Iterator<Item = Token>>( tokens: &mut std::iter::Peekable<I>) {
    // Advance
    while tokens.peek().is_some(){
        let x = tokens.next_if(|x| {
            match x.kind {
                TokenKind::Semicolon
                    | TokenKind::Keyword(Keyword::Class) 
                    | TokenKind::Keyword(Keyword::Fun) 
                    | TokenKind::Keyword(Keyword::Var) 
                    | TokenKind::Keyword(Keyword::For) 
                    | TokenKind::Keyword(Keyword::If) 
                    | TokenKind::Keyword(Keyword::While) 
                    | TokenKind::Keyword(Keyword::Print) 
                    | TokenKind::Keyword(Keyword::Return) 
                    => false,
                _ => true,
            }
        });
        if x.is_none() {
            let _ = tokens.next_if(|x| x.kind == TokenKind::Semicolon);
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::expression::{BinaryOperator, Expr, LiteralOperator, BinaryExpression};
    use crate::lexer::Lexer;

    use super::{parse, syncronize};

    #[test]
    fn test_parser_expression_presidence_add_mult() {
        let mut tokens = Lexer::from_iter("2 + 2 * 2".chars()).peekable();
        let expect = Expr::Arithmetic(BinaryExpression::add(
            Expr::Literal(LiteralOperator::Number(2_f64)),
            Expr::Arithmetic(BinaryExpression::mult(
                Expr::Literal(LiteralOperator::Number(2_f64)),
                Expr::Literal(LiteralOperator::Number(2_f64)),
            )),
        ));
        assert_eq!(expect, parse(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_expression_presidence_mult_add() {
        let mut tokens = Lexer::from_iter("2 * 2 + 2".chars()).peekable();
        let expect = Expr::Arithmetic(BinaryExpression::add(
            Expr::Arithmetic(BinaryExpression::mult(
                Expr::Literal(LiteralOperator::Number(2_f64)),
                Expr::Literal(LiteralOperator::Number(2_f64)),
            )),
            Expr::Literal(LiteralOperator::Number(2_f64))
        ));
        assert_eq!(expect, parse(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_expression() {
        let mut tokens = Lexer::from_iter("2 < 3 >= 1 == false".chars()).peekable();
        let left = Expr::Compare(
            Box::new(Expr::Compare(
                Box::new(Expr::Literal(LiteralOperator::Number(2_f64))),
                BinaryOperator::Less,
                Box::new(Expr::Literal(LiteralOperator::Number(3_f64))),
            )),
                BinaryOperator::GreaterEqual,
            Box::new(Expr::Literal(LiteralOperator::Number(1_f64))),
        );
        let right = Expr::Literal(LiteralOperator::False);
        let expect = Expr::Equality(Box::new(left), BinaryOperator::Equal, Box::new(right));
        let result = parse(&mut tokens).unwrap();
        assert_eq!(expect, result);
    }

    #[test]
    fn test_parser_parse_syncronize() {
        let mut tokens = Lexer::from_iter("x == 2; 2 == 2".chars()).peekable();
        syncronize(&mut tokens);
        let expect = Expr::Equality(
            Box::new(Expr::Literal(LiteralOperator::Number(2_f64))),
            BinaryOperator::Equal,
            Box::new(Expr::Literal(LiteralOperator::Number(2_f64))),
        );
        assert_eq!(expect, parse(&mut tokens).unwrap());
    }

}
