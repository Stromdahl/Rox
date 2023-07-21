#![allow(dead_code, unused_variables)]

mod parser {
    use crate::expression::{Error, Expr, Unary, Binary};
    use crate::token::{Keyword, Token, TokenKind};

    pub fn parse_compare<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        todo!();
    //     let left = parse_factor(tokens);
    //     let operator = tokens.next_if(|x| x.kind == TokenKind::Plus || x.kind == TokenKind::Minus);

    //     let expr = match operator {
    //         Some(x) => {
    //             let right = parse_unary(tokens)?;
    //             match x.kind {
    //                 TokenKind::Plus => Expr::Binary(Box::new(left?), Binary::Add, Box::new(right)),
    //                 TokenKind::Minus => Expr::Binary(Box::new(left?), Binary::Sub, Box::new(right)),
    //                 _ => return Err(Error::UnexpecedCharacter),
    //             }
    //         }
    //         None => left?,
    //     };
    //     Ok(expr)
    }


    pub fn parse_term<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let left = parse_factor(tokens);
        let operator = tokens.next_if(|x| x.kind == TokenKind::Plus || x.kind == TokenKind::Minus);

        let expr = match operator {
            Some(x) => {
                let right = parse_factor(tokens)?;
                match x.kind {
                    TokenKind::Plus => Expr::Binary(Box::new(left?), Binary::Add, Box::new(right)),
                    TokenKind::Minus => Expr::Binary(Box::new(left?), Binary::Sub, Box::new(right)),
                    _ => return Err(Error::UnexpecedCharacter),
                }
            }
            None => left?,
        };
        Ok(expr)
    }

    pub fn parse_factor<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let left = parse_unary(tokens);
        let operator = tokens.next_if(|x| x.kind == TokenKind::Slash || x.kind == TokenKind::Star);

        let expr = match operator {
            Some(x) => {
                let right = parse_unary(tokens)?;
                match x.kind {
                    TokenKind::Star => Expr::Binary(Box::new(left?), Binary::Mult, Box::new(right)),
                    TokenKind::Slash => Expr::Binary(Box::new(left?), Binary::Div, Box::new(right)),
                    _ => return Err(Error::UnexpecedCharacter),
                }
            }
            None => left?,
        };
        Ok(expr)
    }

    pub fn parse_unary<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let operator = tokens.next_if(|x| x.kind == TokenKind::Minus || x.kind == TokenKind::Bang);
        match operator {
            Some(x) => {
                let right = parse_unary(tokens)?;
                match x.kind {
                    TokenKind::Minus => Ok(Expr::Unary(Unary::Minus, Box::new(right))),
                    TokenKind::Bang => Ok(Expr::Unary(Unary::Bang, Box::new(right))),
                    _ => todo!("add primary"),
                }
            }
            None => parse_primary(tokens),
        }
    }

    pub fn parse_primary<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let token = tokens.next().ok_or(Error::ExpectExpression)?;
        let e = match token.kind {
            TokenKind::Keyword(Keyword::True) => Expr::True,
            TokenKind::Keyword(Keyword::False) => Expr::False,
            TokenKind::Keyword(Keyword::Nil) => Expr::Nil,
            TokenKind::Number => Expr::Number,
            TokenKind::String => Expr::String,
            TokenKind::LeftParen => todo!("Handle grouping"),
            _ => todo!(),
        };
        Ok(e)
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::{Expr, Unary, Binary, Compare};
    use crate::lexer::Lexer;

    use super::parser::{parse_primary, parse_unary, parse_factor, parse_term, parse_compare};

    #[test]
    fn parser_compare_greater() {
        let mut tokens = Lexer::from_iter("2 > 2".chars()).peekable();
        let expect = Expr::Compare(Box::new(Expr::Number), Compare::Greater, Box::new(Expr::Number));
        assert_eq!(expect, parse_compare(&mut tokens).unwrap());
    }

    #[test]
    fn parser_presidence() {
        let mut tokens = Lexer::from_iter("2 + 2 * 2".chars()).peekable();
        let expect = Expr::Binary(
            Box::new(Expr::Number),
            Binary::Add,
            Box::new(
                  Expr::Binary(Box::new(Expr::Number), Binary::Mult, Box::new(Expr::Number))
            )
        );
        assert_eq!(expect, parse_term(&mut tokens).unwrap());
    }

    #[test]
    fn parser_factor_add_negative() {
        let mut tokens = Lexer::from_iter("2 + -2".chars()).peekable();
        let expect = Expr::Binary(
            Box::new(Expr::Number),
            Binary::Add,
            Box::new(Expr::Unary(Unary::Minus, Box::new(Expr::Number))));
        assert_eq!(expect, parse_term(&mut tokens).unwrap());
    }

    #[test]
    fn parser_term_add() {
        let mut tokens = Lexer::from_iter("2 + 2".chars()).peekable();
        let expect = Expr::Binary(Box::new(Expr::Number), Binary::Add, Box::new(Expr::Number));
        assert_eq!(expect, parse_term(&mut tokens).unwrap());
    }

    #[test]
    fn parser_term_sub() {
        let mut tokens = Lexer::from_iter("2 - 2".chars()).peekable();
        let expect = Expr::Binary(Box::new(Expr::Number), Binary::Sub, Box::new(Expr::Number));
        assert_eq!(expect, parse_term(&mut tokens).unwrap());
    }

    #[test]
    fn parser_factor_multiply_negative() {
        let mut tokens = Lexer::from_iter("2 * -2".chars()).peekable();
        let expect = Expr::Binary(
            Box::new(Expr::Number),
            Binary::Mult,
            Box::new(Expr::Unary(Unary::Minus, Box::new(Expr::Number))));
        assert_eq!(expect, parse_factor(&mut tokens).unwrap());
    }

    #[test]
    fn parser_factor_multiply() {
        let mut tokens = Lexer::from_iter("2 * 2".chars()).peekable();
        let expect = Expr::Binary(Box::new(Expr::Number), Binary::Mult, Box::new(Expr::Number));
        assert_eq!(expect, parse_factor(&mut tokens).unwrap());
    }

    #[test]
    fn parser_factor_divide() {
        let mut tokens = Lexer::from_iter("2 / 2".chars()).peekable();
        let expect = Expr::Binary(Box::new(Expr::Number), Binary::Div, Box::new(Expr::Number));
        assert_eq!(expect, parse_factor(&mut tokens).unwrap());
    }

    #[test]
    fn parser_unary() {
        let mut tokens = Lexer::from_iter("-132".chars()).peekable();
        let expect = Expr::Unary(Unary::Minus, Box::new(Expr::Number));
        assert_eq!(expect, parse_unary(&mut tokens).unwrap());

        let mut tokens = Lexer::from_iter("!true".chars()).peekable();
        let expect = Expr::Unary(Unary::Bang, Box::new(Expr::True));
        assert_eq!(expect, parse_unary(&mut tokens).unwrap());

        let mut tokens = Lexer::from_iter("!!true".chars()).peekable();
        let expect = Expr::Unary(
            Unary::Bang,
            Box::new(Expr::Unary(Unary::Bang, Box::new(Expr::True))),
        );
        assert_eq!(expect, parse_unary(&mut tokens).unwrap());
    }

    #[test]
    fn parser_primary_literals() {
        let mut tokens = Lexer::from_iter("true false nil 123 \"string\"".chars()).peekable();
        assert_eq!(Expr::True, parse_primary(&mut tokens).unwrap());
        assert_eq!(Expr::False, parse_primary(&mut tokens).unwrap());
        assert_eq!(Expr::Nil, parse_primary(&mut tokens).unwrap());
        assert_eq!(Expr::Number, parse_primary(&mut tokens).unwrap());
        assert_eq!(Expr::String, parse_primary(&mut tokens).unwrap());
        // Todo: add test for grouping
    }
}
