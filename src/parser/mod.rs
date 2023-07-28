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
            !matches!(x.kind, TokenKind::Semicolon
                    | TokenKind::Keyword(Keyword::Class) 
                    | TokenKind::Keyword(Keyword::Fun) 
                    | TokenKind::Keyword(Keyword::Var) 
                    | TokenKind::Keyword(Keyword::For) 
                    | TokenKind::Keyword(Keyword::If) 
                    | TokenKind::Keyword(Keyword::While) 
                    | TokenKind::Keyword(Keyword::Print) 
                    | TokenKind::Keyword(Keyword::Return) 
            )
        });
        if x.is_none() {
            let _ = tokens.next_if(|x| x.kind == TokenKind::Semicolon);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::{Expr, BinaryExpression, LiteralExpression};
    use crate::lexer::Lexer;

    use super::{parse, syncronize};

    #[test]
    fn test_parser_expression_presidence_add_mult() {
        let mut tokens = Lexer::from_iter("2 + 2 * 2".chars()).peekable();
        let expect = Expr::Arithmetic(BinaryExpression::add(
            Expr::Literal(LiteralExpression::number(2_f64)),
            Expr::Arithmetic(BinaryExpression::mult(
                Expr::Literal(LiteralExpression::number(2_f64)),
                Expr::Literal(LiteralExpression::number(2_f64)),
            )),
        ));
        assert_eq!(expect, parse(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_expression_presidence_mult_add() {
        let mut tokens = Lexer::from_iter("2 * 2 + 2".chars()).peekable();
        let expect = Expr::Arithmetic(BinaryExpression::add(
            Expr::Arithmetic(BinaryExpression::mult(
                Expr::Literal(LiteralExpression::number(2_f64)),
                Expr::Literal(LiteralExpression::number(2_f64)),
            )),
            Expr::Literal(LiteralExpression::number(2_f64))
        ));
        assert_eq!(expect, parse(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_expression() {
        let mut tokens = Lexer::from_iter("2 < 3 >= 1 == false".chars()).peekable();
        let left = Expr::Compare( BinaryExpression::greater_equal(
            Expr::Compare(BinaryExpression::less(
               Expr::Literal(LiteralExpression::number(2_f64)),
               Expr::Literal(LiteralExpression::number(3_f64)),
            )),
            Expr::Literal(LiteralExpression::number(1_f64)),
        ));
        let right = Expr::Literal(LiteralExpression::boolean(false));
        let expect = Expr::Equality(BinaryExpression::equal(left, right));
        let result = parse(&mut tokens).unwrap();
        assert_eq!(expect, result);
    }

    #[test]
    fn test_parser_parse_syncronize() {
        let mut tokens = Lexer::from_iter("x == 2; 2 == 2".chars()).peekable();
        syncronize(&mut tokens);
        let expect = Expr::Equality( BinaryExpression::equal(
            Expr::Literal(LiteralExpression::number(2_f64)),
            Expr::Literal(LiteralExpression::number(2_f64)),
        ));
        assert_eq!(expect, parse(&mut tokens).unwrap());
    }

}
