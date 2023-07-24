    // TODO: Parsing can be cone much better!!

    use crate::expression::{BinaryOperator, Error, Expr, LiteralOperator, UnaryOperator, BinaryExpression};
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
        let operator =
            tokens.next_if(|x| x.kind == TokenKind::EqualEqual || x.kind == TokenKind::BangEqual);
        let expr = match operator {
            Some(x) => {
                let right = parse_comparison(tokens)?;
                match x.kind {
                    TokenKind::EqualEqual => {
                        Expr::Equality(Box::new(left), BinaryOperator::Equal, Box::new(right))
                    }
                    TokenKind::BangEqual => {
                        Expr::Equality(Box::new(left), BinaryOperator::NotEqual, Box::new(right))
                    }
                    token => return Err(Error::UnexpecedCharacter(token)),
                }
            }
            None => left,
        };
        Ok(expr)
    }

    pub fn parse_comparison<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let mut left = parse_term(tokens)?;
        while let Some(operator) = tokens.next_if(|x| {
            x.kind == TokenKind::Greather
                || x.kind == TokenKind::GreatherEqual
                || x.kind == TokenKind::Less
                || x.kind == TokenKind::LessEqual
        }) {

                let right = parse_term(tokens)?;
                left = match operator.kind {
                    TokenKind::Greather => {
                        Expr::Compare(Box::new(left), BinaryOperator::Greater, Box::new(right))
                    }
                    TokenKind::Less => {
                        Expr::Compare(Box::new(left), BinaryOperator::Less, Box::new(right))
                    }
                    TokenKind::GreatherEqual => {
                        Expr::Compare(Box::new(left), BinaryOperator::GreaterEqual, Box::new(right))
                    }
                    TokenKind::LessEqual => {
                        Expr::Compare(Box::new(left), BinaryOperator::LessEqual, Box::new(right))
                    }
                    token => return Err(Error::UnexpecedCharacter(token)),
                }
            };
        Ok(left)
    }

    pub fn parse_term<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let mut left = parse_factor(tokens)?;
        while let Some(operator) = tokens.next_if(|x| x.kind == TokenKind::Plus || x.kind == TokenKind::Minus) {

                let right = parse_factor(tokens)?;
                left = match operator.kind {
                    TokenKind::Plus => Expr::Arithmetic(BinaryExpression::add(left, right)),
                    TokenKind::Minus => Expr::Arithmetic(BinaryExpression::sub(left, right)),
                    token => return Err(Error::UnexpecedCharacter(token)),
                }
        };
        Ok(left)
    }

    pub fn parse_factor<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let mut left = parse_unary(tokens)?;
        while let Some(operator) = tokens.next_if(|x| x.kind == TokenKind::Slash || x.kind == TokenKind::Star) {

                let right = parse_unary(tokens)?;
                left = match operator.kind {
                    TokenKind::Star => Expr::Arithmetic(BinaryExpression::mult(left, right)),
                    TokenKind::Slash => Expr::Arithmetic(BinaryExpression::div(left, right)),
                    token => return Err(Error::UnexpecedCharacter(token)),
                }
        };
        Ok(left)
    }

    pub fn parse_unary<I: Iterator<Item = Token>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expr, Error> {
        let operator = tokens.next_if(|x| x.kind == TokenKind::Minus || x.kind == TokenKind::Bang);
        match operator {
            Some(x) => {
                let right = parse_unary(tokens)?;
                match x.kind {
                    TokenKind::Minus => Ok(Expr::Unary(UnaryOperator::Minus, Box::new(right))),
                    TokenKind::Bang => Ok(Expr::Unary(UnaryOperator::Bang, Box::new(right))),
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
            TokenKind::Keyword(Keyword::True) => Expr::Literal(LiteralOperator::True),
            TokenKind::Keyword(Keyword::False) => Expr::Literal(LiteralOperator::False),
            TokenKind::Keyword(Keyword::Nil) => Expr::Literal(LiteralOperator::Nil),
            TokenKind::Number(literal) => Expr::Literal(LiteralOperator::Number(literal)),
            TokenKind::String(literal) => Expr::Literal(LiteralOperator::String(literal)),
            TokenKind::LeftParen => {
                let expr = parse_expression(tokens)?;
                let _ = tokens
                    .next_if(|x| x.kind == TokenKind::RightParen)
                    .ok_or(Error::ExpectRightParen)?;
                Expr::Grouping(Box::new(expr))
            }
            token => return Err(Error::UnexpecedCharacter(token))
        };
        Ok(expr)
    }

#[cfg(test)]
mod tests {
    use crate::expression::{BinaryOperator, Expr, LiteralOperator, UnaryOperator, BinaryExpression};
    use crate::lexer::Lexer;

    use super::{
        parse_comparison, parse_equality, parse_factor, parse_primary, parse_term, parse_unary,
    };

    #[test]
    fn test_parser_parser_equality_equal() {
        let mut tokens = Lexer::from_iter("2 == 2".chars()).peekable();
        let expect = Expr::Equality(
            Box::new(Expr::Literal(LiteralOperator::Number(2_f64))),
            BinaryOperator::Equal,
            Box::new(Expr::Literal(LiteralOperator::Number(2_f64))),
        );
        assert_eq!(expect, parse_equality(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_compare_greater() {
        let mut tokens = Lexer::from_iter("2 > 2".chars()).peekable();
        let expect = Expr::Compare(
            Box::new(Expr::Literal(LiteralOperator::Number(2_f64))),
            BinaryOperator::Greater,
            Box::new(Expr::Literal(LiteralOperator::Number(2_f64))),
        );
        assert_eq!(expect, parse_comparison(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_presidence() {
        let mut tokens = Lexer::from_iter("2 + 2 * 2".chars()).peekable();
        let expect = Expr::Arithmetic( BinaryExpression::add(
                Expr::Literal(LiteralOperator::Number(2_f64)),
            Expr::Arithmetic(BinaryExpression::mult(
                Expr::Literal(LiteralOperator::Number(2_f64)),
                Expr::Literal(LiteralOperator::Number(2_f64)),
            )),
        ));
        assert_eq!(expect, parse_term(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_factor_add_negative() {
        let mut tokens = Lexer::from_iter("2 + -2".chars()).peekable();
        let expect = Expr::Arithmetic( BinaryExpression::add(
            Expr::Literal(LiteralOperator::Number(2_f64)),
            Expr::Unary(UnaryOperator::Minus, Box::new(Expr::Literal(LiteralOperator::Number(2_f64)))),
        ));
        assert_eq!(expect, parse_term(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_term_add() {
        let mut tokens = Lexer::from_iter("2 + 2".chars()).peekable();
        let expect = Expr::Arithmetic( BinaryExpression::add(
            Expr::Literal(LiteralOperator::Number(2_f64)),
            Expr::Literal(LiteralOperator::Number(2_f64)),
        ));
        assert_eq!(expect, parse_term(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_term_sub() {
        let mut tokens = Lexer::from_iter("2 - 2".chars()).peekable();
        let expect = Expr::Arithmetic( BinaryExpression::sub(
            Expr::Literal(LiteralOperator::Number(2_f64)),
            Expr::Literal(LiteralOperator::Number(2_f64)),
        ));
        assert_eq!(expect, parse_term(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_factor_multiply_negative() {
        let mut tokens = Lexer::from_iter("2 * -2".chars()).peekable();
        let expect = Expr::Arithmetic( BinaryExpression::mult(
            Expr::Literal(LiteralOperator::Number(2_f64)),
            Expr::Unary(UnaryOperator::Minus, Box::new(Expr::Literal(LiteralOperator::Number(2_f64)))),
        ));
        assert_eq!(expect, parse_factor(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_factor_multiply() {
        let mut tokens = Lexer::from_iter("2 * 2".chars()).peekable();
        let expect = Expr::Arithmetic( BinaryExpression::mult(
            Expr::Literal(LiteralOperator::Number(2_f64)),
            Expr::Literal(LiteralOperator::Number(2_f64)),
        ));
        assert_eq!(expect, parse_factor(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_factor_divide() {
        let mut tokens = Lexer::from_iter("2 / 2".chars()).peekable();
        let expect = Expr::Arithmetic( BinaryExpression::div(
            Expr::Literal(LiteralOperator::Number(2_f64)),
            Expr::Literal(LiteralOperator::Number(2_f64)),
        ));
        assert_eq!(expect, parse_factor(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_unary() {
        let mut tokens = Lexer::from_iter("-123".chars()).peekable();
        let expect = Expr::Unary(UnaryOperator::Minus, Box::new(Expr::Literal(LiteralOperator::Number(123_f64))));
        assert_eq!(expect, parse_unary(&mut tokens).unwrap());

        let mut tokens = Lexer::from_iter("!true".chars()).peekable();
        let expect = Expr::Unary(UnaryOperator::Bang, Box::new(Expr::Literal(LiteralOperator::True)));
        assert_eq!(expect, parse_unary(&mut tokens).unwrap());

        let mut tokens = Lexer::from_iter("!!false".chars()).peekable();
        let expect = Expr::Unary(
            UnaryOperator::Bang,
            Box::new(Expr::Unary(
                UnaryOperator::Bang,
                Box::new(Expr::Literal(LiteralOperator::False)),
            )),
        );
        assert_eq!(expect, parse_unary(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_primary_group() {
        let mut tokens = Lexer::from_iter("( \"grouped\" )".chars()).peekable();
        let expect = Expr::Grouping(Box::new(Expr::Literal(LiteralOperator::String("grouped".to_string()))));
        assert_eq!(expect, parse_unary(&mut tokens).unwrap());
    }

    #[test]
    fn test_parser_parse_primary_literals() {
        let mut tokens = Lexer::from_iter("true false nil 123 \"string\"".chars()).peekable();
        assert_eq!(
            Expr::Literal(LiteralOperator::True),
            parse_primary(&mut tokens).unwrap()
        );
        assert_eq!(
            Expr::Literal(LiteralOperator::False),
            parse_primary(&mut tokens).unwrap()
        );
        assert_eq!(
            Expr::Literal(LiteralOperator::Nil),
            parse_primary(&mut tokens).unwrap()
        );
        assert_eq!(Expr::Literal(LiteralOperator::Number(123_f64)), parse_primary(&mut tokens).unwrap());
        assert_eq!(
            Expr::Literal(LiteralOperator::String("string".to_string())),
            parse_primary(&mut tokens).unwrap()
        );
    }
}
