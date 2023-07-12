#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greather,
    GreatherEqual,
    Less,
    LessEqual,

    // Literals
    Identifiter,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

// Token(TokenType type, String lexeme, Object literal, int line) {
//   this.type = type;
//   this.lexeme = lexeme;
//   this.literal = literal;
//   this.line = line;
// }

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    // literal: String,
    // line: u32,
}

impl Token {
    pub fn new(lexeme: String, kind: TokenKind) -> Self {
        Self { kind, lexeme }
    }
}

pub struct Scanner<Chars: Iterator<Item = char>> {
    source: std::iter::Peekable<Chars>,
}

impl<Chars: Iterator<Item = char>> Scanner<Chars> {
    pub fn from_iter(chars: Chars) -> Self {
        Self {
            source: chars.peekable(),
        }
    }

    fn trim_while<F>(&mut self, f: F) where F:FnOnce(&char) -> bool + Copy {
        while self.source.next_if(f).is_some() {}
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.trim_while(|x| x.is_whitespace());
        if let Some(x) = self.source.next() {
            let mut text = String::new();
            text.push(x);
            match x {
                x if x.is_whitespace() => None,
                '(' => Some(Token::new(text, TokenKind::LeftParen)),
                ')' => Some(Token::new(text, TokenKind::RightParen)),
                '{' => Some(Token::new(text, TokenKind::LeftBrace)),
                '}' => Some(Token::new(text, TokenKind::RightBrace)),
                ',' => Some(Token::new(text, TokenKind::Comma)),
                '.' => Some(Token::new(text, TokenKind::Dot)),
                '-' => Some(Token::new(text, TokenKind::Minus)),
                ';' => Some(Token::new(text, TokenKind::Semicolon)),
                '*' => Some(Token::new(text, TokenKind::Star)),
                '!' => match self.source.next_if(|x| x.eq(&'=')) {
                    Some(x) => {
                        text.push(x);
                        Some(Token::new(text, TokenKind::BangEqual))
                    }
                    None => Some(Token::new(text, TokenKind::Bang)),
                },
                '=' => match self.source.next_if(|x| x.eq(&'=')) {
                    Some(x) => {
                        text.push(x);
                        Some(Token::new(text, TokenKind::EqualEqual))
                    }
                    None => Some(Token::new(text, TokenKind::Equal)),
                },
                '<' => match self.source.next_if(|x| x.eq(&'=')) {
                    Some(x) => {
                        text.push(x);
                        Some(Token::new(text, TokenKind::LessEqual))
                    }
                    None => Some(Token::new(text, TokenKind::Less)),
                },
                '>' => match self.source.next_if(|x| x.eq(&'=')) {
                    Some(x) => {
                        text.push(x);
                        Some(Token::new(text, TokenKind::GreatherEqual))
                    }
                    None => Some(Token::new(text, TokenKind::Greather)),
                },
                '/' => match self.source.next_if(|x| x.eq(&'/')) {
                    Some(_) => {
                        self.trim_while(|&x| x != '\n');
                        None
                    }
                    None => Some(Token::new(text, TokenKind::Slash)),
                },
                _ => {
                    todo!()
                }
            }
        } else {
            None
        }
    }
}

impl<Chars: Iterator<Item = char>> Iterator for Scanner<Chars> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_single_character_tokens() {
        let source = "() {} , . - ;  * /".chars();
        let mut scanner = Scanner::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::LeftParen);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::RightParen);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::LeftBrace);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::RightBrace);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Comma);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Dot);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Minus);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Semicolon);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Star);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Slash);
        assert!(scanner.next().is_none());
    }
    #[test]
    fn scan_two_character_tokens() {
        let source = "!=!".chars();
        let mut scanner = Scanner::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::BangEqual);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Bang);

        let source = "===".chars();
        let mut scanner = Scanner::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::EqualEqual);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Equal);

        let source = "<=<".chars();
        let mut scanner = Scanner::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::LessEqual);

        let source = ">=>".chars();
        let mut scanner = Scanner::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::GreatherEqual);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Greather);
    }

    #[test]
    fn scan_comment() {
        let source = "/ //This is a comment".chars();
        let mut scanner = Scanner::from_iter(source);
        assert_eq!(scanner.next().unwrap().kind, TokenKind::Slash);
        assert!(scanner.next().is_none(), "Comment should be discarded");

        let source = "//This is a comment".chars();
        let mut scanner = Scanner::from_iter(source);
        assert!(scanner.next().is_none(), "Comment should be discarded");
    }
}
