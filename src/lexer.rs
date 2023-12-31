use crate::token::{Error, Keyword, Token, TokenKind};

pub struct Lexer<Chars: Iterator<Item = char>> {
    source: std::iter::Peekable<Chars>,
    line: u32,
}

impl<Chars: Iterator<Item = char>> Lexer<Chars> {
    pub fn from_iter(chars: Chars) -> Self {
        Self {
            source: chars.peekable(),
            line: 0,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.source.peek().is_none()
    }

    fn trim_while<F>(&mut self, f: F)
    where
        F: FnOnce(&char) -> bool + Copy,
    {
        while self.source.next_if(f).is_some() {}
    }

    fn new_token(&self, text: String, kind: TokenKind) -> Token {
        Token::new(text, self.line, kind)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.trim_while(|x| x.is_whitespace());
        if let Some(x) = self.source.next() {
            let mut text = String::new();
            text.push(x);
            match x {
                x if x.is_whitespace() => None,
                '(' => Some(self.new_token(text, TokenKind::LeftParen)),
                ')' => Some(self.new_token(text, TokenKind::RightParen)),
                '{' => Some(self.new_token(text, TokenKind::LeftBrace)),
                '}' => Some(self.new_token(text, TokenKind::RightBrace)),
                ',' => Some(self.new_token(text, TokenKind::Comma)),
                '.' => Some(self.new_token(text, TokenKind::Dot)),
                '-' => Some(self.new_token(text, TokenKind::Minus)),
                '+' => Some(self.new_token(text, TokenKind::Plus)),
                ';' => Some(self.new_token(text, TokenKind::Semicolon)),
                '*' => Some(self.new_token(text, TokenKind::Star)),
                '!' => match self.source.next_if_eq(&'=') {
                    Some(x) => {
                        text.push(x);
                        Some(self.new_token(text, TokenKind::BangEqual))
                    }
                    None => Some(self.new_token(text, TokenKind::Bang)),
                },
                '=' => match self.source.next_if_eq(&'=') {
                    Some(x) => {
                        text.push(x);
                        Some(self.new_token(text, TokenKind::EqualEqual))
                    }
                    None => Some(self.new_token(text, TokenKind::Equal)),
                },
                '<' => match self.source.next_if_eq(&'=') {
                    Some(x) => {
                        text.push(x);
                        Some(self.new_token(text, TokenKind::LessEqual))
                    }
                    None => Some(self.new_token(text, TokenKind::Less)),
                },
                '>' => match self.source.next_if_eq(&'=') {
                    Some(x) => {
                        text.push(x);
                        Some(self.new_token(text, TokenKind::GreatherEqual))
                    }
                    None => Some(self.new_token(text, TokenKind::Greather)),
                },
                '/' => match self.source.next_if_eq(&'/') {
                    Some(_) => {
                        self.trim_while(|&x| x != '\n');
                        None
                    }
                    None => Some(self.new_token(text, TokenKind::Slash)),
                },
                '"' => {
                    while let Some(x) = self.source.next_if(|&x| x != '"') {
                        if x == '\n' {
                            self.line += 1;
                        }
                        text.push(x)
                    }

                    if self.is_at_end() {
                        return Some(
                            self.new_token(text, TokenKind::Error(Error::UnterminatedString)),
                        );
                    }

                    // skip the remaining '"'
                    self.source.next();

                    // Trim the surrounding quotes.
                    if !text.is_empty() {
                        text.remove(0);
                    }
                    Some(self.new_token(text.clone(), TokenKind::String(text)))
                }
                '0'..='9' => {
                    while let Some(x) = self.source.next_if(|&x| x.is_numeric()) {
                        text.push(x)
                    }

                    if let Some(x) = self.source.next_if(|&x| x == '.') {
                        if let Some(&y) = self.source.peek() {
                            if y.is_numeric() {
                                text.push(x);
                            }

                            while let Some(x) = self.source.next_if(|&x| x.is_numeric()) {
                                text.push(x)
                            }
                        }
                    }
                    let token: Token = match text.parse() {
                        Ok(literal) => self.new_token(text, TokenKind::Number(literal)),
                        Err(_) => self.new_token(text, TokenKind::Error(Error::UnterminatedString)),
                    };
                    Some(token)
                }
                'a'..='z' | 'A'..='B' | '_' => {
                    while let Some(x) = self.source.next_if(|&x| x.is_alphanumeric()) {
                        text.push(x);
                    }

                    let keyword = match text.as_str() {
                        "and" => Some(Keyword::And),
                        "class" => Some(Keyword::Class),
                        "else" => Some(Keyword::Else),
                        "false" => Some(Keyword::False),
                        "for" => Some(Keyword::For),
                        "fun" => Some(Keyword::Fun),
                        "if" => Some(Keyword::If),
                        "nil" => Some(Keyword::Nil),
                        "or" => Some(Keyword::Or),
                        "print" => Some(Keyword::Print),
                        "return" => Some(Keyword::Return),
                        "super" => Some(Keyword::Super),
                        "this" => Some(Keyword::This),
                        "true" => Some(Keyword::True),
                        "var" => Some(Keyword::Var),
                        "while" => Some(Keyword::While),
                        _ => None,
                    };
                    if let Some(x) = keyword {
                        Some(self.new_token(text, TokenKind::Keyword(x)))
                    } else {
                        Some(self.new_token(text, TokenKind::Identifiter))
                    }
                }
                _ => Some(self.new_token(text, TokenKind::Error(Error::UnexpectedCharacter))),
            }
        } else {
            None
        }
    }
}

impl<Chars: Iterator<Item = char>> Iterator for Lexer<Chars> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_expresion_multiply() {
        let mut scanner = Lexer::from_iter("2*2".chars());
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Number(2_f64));
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Star);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Number(2_f64));
    }

    #[test]
    fn scan_expression() {
        let source = "print \"Hello, World!\";".chars();
        let tokens: Vec<Token> = Lexer::from_iter(source).collect();
        assert_eq!(tokens[0].lexeme, "print");
        assert_eq!(tokens[0].kind, TokenKind::Keyword(Keyword::Print));
        assert_eq!(tokens[1].lexeme, "Hello, World!");
        assert_eq!(
            tokens[1].kind,
            TokenKind::String("Hello, World!".to_string())
        );
        assert_eq!(tokens[2].lexeme, ";");
        assert_eq!(tokens[2].kind, TokenKind::Semicolon);
    }

    #[test]
    fn scan_identifier() {
        let source = "blafs".chars();
        let mut scanner = Lexer::from_iter(source);
        let token = scanner.next().expect("Should be some");
        assert_eq!(token.kind, TokenKind::Identifiter);
        assert_eq!(token.lexeme, "blafs");
    }

    #[test]
    fn scan_reserved_words() {
        let source = "or".chars();
        let mut scanner = Lexer::from_iter(source);
        let token = scanner.next().expect("Should be some");
        assert_eq!(token.kind, TokenKind::Keyword(Keyword::Or));
        assert_eq!(token.lexeme, "or");
    }

    #[test]
    fn scan_numeric_literals() {
        let source = "1234".chars();
        let mut scanner = Lexer::from_iter(source);
        let token = scanner.next().expect("Should be some");
        assert_eq!(token.kind, TokenKind::Number(1234_f64));
        assert_eq!(token.lexeme, "1234", "Should handle integers");

        let source = "12.34".chars();
        let mut scanner = Lexer::from_iter(source);
        let token = scanner.next().expect("Should be some");
        assert_eq!(token.kind, TokenKind::Number(12.34_f64));
        assert_eq!(token.lexeme, "12.34", "Should handle floats");
    }

    #[test]
    fn scan_string_literals_ok() {
        let source = "\"Hello, \nWorld!\"".chars();
        let mut scanner = Lexer::from_iter(source);
        let token = scanner.next().expect("Should be some");
        assert_eq!(token.kind, TokenKind::String(token.lexeme.clone()));
        assert_eq!(token.lexeme, "Hello, \nWorld!");
        assert_eq!(token.line, 1);
    }

    #[test]
    fn scan_string_literals_unterminated() {
        let source = "\"".chars();
        let mut scanner = Lexer::from_iter(source);
        let token = scanner.next().expect("Should be some");
        assert_eq!(token.kind, TokenKind::Error(Error::UnterminatedString));
        assert_eq!(token.lexeme, "\"");
    }

    #[test]
    fn scan_error() {
        let source = "@".chars();
        let mut scanner = Lexer::from_iter(source);
        let token = scanner.next().expect("Should be some");
        assert_eq!(token.kind, TokenKind::Error(Error::UnexpectedCharacter));
    }

    #[test]
    fn scan_single_character_tokens() {
        let source = "() {} , . - + ;  * /".chars();
        let mut scanner = Lexer::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::LeftParen);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::RightParen);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::LeftBrace);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::RightBrace);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Comma);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Dot);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Minus);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Plus);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Semicolon);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Star);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Slash);
        assert!(scanner.next().is_none(), "End of file");
    }

    #[test]
    fn scan_two_character_tokens() {
        let source = "!=!".chars();
        let mut scanner = Lexer::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::BangEqual);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Bang);

        let source = "===".chars();
        let mut scanner = Lexer::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::EqualEqual);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Equal);

        let source = "<=<".chars();
        let mut scanner = Lexer::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::LessEqual);

        let source = ">=>".chars();
        let mut scanner = Lexer::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::GreatherEqual);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Greather);
    }

    #[test]
    fn scan_comment() {
        let source = "/ //This is a comment".chars();
        let mut scanner = Lexer::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Slash);
        assert!(scanner.next().is_none(), "Comment should be discarded");

        let source = "//This is a comment".chars();
        let mut scanner = Lexer::from_iter(source);
        assert!(scanner.next().is_none(), "Comment should be discarded");
    }
}
