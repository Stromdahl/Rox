#![allow(dead_code, unused_variables)]

mod parser {
    // TODO: Parsing can be cone much better!!
    
    use crate::expression::{Binary, Compare, Error, Expr, Unary, Equality, Literal};
    use crate::token::{Keyword, Token, TokenKind};

    pub fn parse_expression<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        parse_equality(tokens)
    }

    pub fn parse_equality<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let left = parse_comparison(tokens)?;
        let operator = tokens.next_if(|x| {x.kind == TokenKind::EqualEqual || x.kind == TokenKind::BangEqual});
        let expr = match operator {
            Some(x) => {
                let right = parse_comparison(tokens)?;
                match x.kind {
                    TokenKind::EqualEqual => {
                        Expr::Equality(Box::new(left), Equality::Equal, Box::new(right))
                    }
                    TokenKind::BangEqual => {
                        Expr::Equality(Box::new(left), Equality::NotEqual, Box::new(right))
                    }
                    _ => return Err(Error::UnexpecedCharacter),
                }
            }
            None => left,
        };
        Ok(expr)
    }

    pub fn parse_comparison<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let left = parse_term(tokens)?;
        let operator = tokens.next_if(|x| {
            x.kind == TokenKind::Greather
                || x.kind == TokenKind::GreatherEqual
                || x.kind == TokenKind::Less
                || x.kind == TokenKind::LessEqual
        });

        let expr = match operator {
            Some(x) => {
                let right = parse_term(tokens)?;
                match x.kind {
                    TokenKind::Greather => {
                        Expr::Compare(Box::new(left), Compare::Greater, Box::new(right))
                    }
                    TokenKind::Less => {
                        Expr::Compare(Box::new(left), Compare::Less, Box::new(right))
                    }
                    TokenKind::GreatherEqual => {
                        Expr::Compare(Box::new(left), Compare::GreaterEqual, Box::new(right))
                    }
                    TokenKind::LessEqual => {
                        Expr::Compare(Box::new(left), Compare::LessEqual, Box::new(right))
                    }
                    _ => return Err(Error::UnexpecedCharacter),
                }
            }
            None => left,
        };
        Ok(expr)
    }

    pub fn parse_term<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let left = parse_factor(tokens)?;
        let operator = tokens.next_if(|x| x.kind == TokenKind::Plus || x.kind == TokenKind::Minus);

        let expr = match operator {
            Some(x) => {
                let right = parse_factor(tokens)?;
                match x.kind {
                    TokenKind::Plus => Expr::Binary(Box::new(left), Binary::Add, Box::new(right)),
                    TokenKind::Minus => Expr::Binary(Box::new(left), Binary::Sub, Box::new(right)),
                    _ => return Err(Error::UnexpecedCharacter),
                }
            }
            None => left,
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
        let expr = match token.kind {
            TokenKind::Keyword(Keyword::True) =>  Expr::Literal(Literal::True),
            TokenKind::Keyword(Keyword::False) => Expr::Literal(Literal::False),
            TokenKind::Keyword(Keyword::Nil) =>   Expr::Literal(Literal::Nil),
            TokenKind::Number => Expr::Number,
            TokenKind::String(literal) => Expr::String(literal),
            TokenKind::LeftParen => {
                let expr = parse_expression(tokens)?;
                let _ = tokens.next_if(|x|x.kind ==  TokenKind::RightParen).ok_or(Error::ExpectRightParen)?;
                Expr::Grouping(Box::new(expr))
            }
            _ => todo!(),
        };
        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::{Binary, Compare, Expr, Unary, Equality, Literal};
    use crate::lexer::Lexer;

    use super::parser::{parse_comparison, parse_factor, parse_primary, parse_term, parse_unary, parse_equality};

    #[test]
    fn parser_equality_equal() {
        let mut tokens = Lexer::from_iter("2 == 2".chars()).peekable();
        let expect = Expr::Equality(
            Box::new(Expr::Number),
            Equality::Equal,
            Box::new(Expr::Number),
        );
        assert_eq!(expect, parse_equality(&mut tokens).unwrap());
    }


    #[test]
    fn parser_compare_greater() {
        let mut tokens = Lexer::from_iter("2 > 2".chars()).peekable();
        let expect = Expr::Compare(
            Box::new(Expr::Number),
            Compare::Greater,
            Box::new(Expr::Number),
        );
        assert_eq!(expect, parse_comparison(&mut tokens).unwrap());
    }

    #[test]
    fn parser_presidence() {
        let mut tokens = Lexer::from_iter("2 + 2 * 2".chars()).peekable();
        let expect = Expr::Binary(
            Box::new(Expr::Number),
            Binary::Add,
            Box::new(Expr::Binary(
                Box::new(Expr::Number),
                Binary::Mult,
                Box::new(Expr::Number),
            )),
        );
        assert_eq!(expect, parse_term(&mut tokens).unwrap());
    }

    #[test]
    fn parser_factor_add_negative() {
        let mut tokens = Lexer::from_iter("2 + -2".chars()).peekable();
        let expect = Expr::Binary(
            Box::new(Expr::Number),
            Binary::Add,
            Box::new(Expr::Unary(Unary::Minus, Box::new(Expr::Number))),
        );
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
            Box::new(Expr::Unary(Unary::Minus, Box::new(Expr::Number))),
        );
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
        let expect = Expr::Unary(Unary::Bang, Box::new(Expr::Literal(Literal::True)));
        assert_eq!(expect, parse_unary(&mut tokens).unwrap());

        let mut tokens = Lexer::from_iter("!!false".chars()).peekable();
        let expect = Expr::Unary(
            Unary::Bang,
            Box::new(Expr::Unary(Unary::Bang, Box::new(Expr::Literal(Literal::False)))),
        );
        assert_eq!(expect, parse_unary(&mut tokens).unwrap());
    }

    #[test]
    fn parser_primary_group() {
        let mut tokens = Lexer::from_iter("( \"grouped\" )".chars()).peekable();
        let expect = Expr::Grouping(Box::new(Expr::String("grouped".to_string())));
        assert_eq!(expect, parse_unary(&mut tokens).unwrap());
    }

    #[test]
    fn parser_primary_literals() {
        let mut tokens = Lexer::from_iter("true false nil 123 \"string\"".chars()).peekable();
        assert_eq!(Expr::Literal(Literal::True), parse_primary(&mut tokens).unwrap());
        assert_eq!(Expr::Literal(Literal::False), parse_primary(&mut tokens).unwrap());
        assert_eq!(Expr::Literal(Literal::Nil), parse_primary(&mut tokens).unwrap());
        assert_eq!(Expr::Number, parse_primary(&mut tokens).unwrap());
        assert_eq!(Expr::String("string".to_string()), parse_primary(&mut tokens).unwrap());
    }
}
