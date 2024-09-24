use crate::{token, Literal, Sao, Token};
use bytes::Buf;
use std::{collections::HashMap, io::Cursor};

macro_rules! add_token {
    ($self:expr, $typ:expr) => {
        add_token!($self, $typ, None)
    };
    ($self:expr, $typ:expr, $lit:expr) => {{
        let text = $self.string_from_bytes($self.start, $self.current);
        $self.tokens.push(Token::new($typ, text, $lit, $self.line));
    }};
}

pub struct Scanner {
    src: Cursor<Vec<u8>>,
    tokens: Vec<Token>,
    keywords: HashMap<&'static str, token::Type>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: Cursor<Vec<u8>>) -> Self {
        Self {
            src,
            tokens: Vec::new(),
            keywords: Self::keywords(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn keywords() -> HashMap<&'static str, token::Type> {
        #[allow(clippy::enum_glob_use)]
        use super::token::Type::*;

        HashMap::from([
            ("and", And),
            ("class", Class),
            ("else", Else),
            ("false", False),
            ("for", For),
            ("fun", Fun),
            ("if", If),
            ("nil", Nil),
            ("or", Or),
            ("print", Print),
            ("return", Return),
            ("super", Super),
            ("this", This),
            ("true", True),
            ("var", Var),
            ("while", While),
        ])
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        use super::token::Type::Eof;

        while self.src.has_remaining() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(Eof, String::new(), None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        #[allow(clippy::enum_glob_use)]
        use super::token::Type::*;

        match self.pop() {
            b'(' => add_token!(self, LeftParen),
            b')' => add_token!(self, RightParen),
            b'{' => add_token!(self, LeftBrace),
            b'}' => add_token!(self, RightBrace),
            b',' => add_token!(self, Comma),
            b'.' => add_token!(self, Dot),
            b'-' => add_token!(self, Minus),
            b'+' => add_token!(self, Plus),
            b';' => add_token!(self, Semicolon),
            b'*' => add_token!(self, Star),
            b'!' if self.match_next(b'=') => add_token!(self, BangEqual),
            b'!' => add_token!(self, Bang),
            b'=' if self.match_next(b'=') => add_token!(self, EqualEqual),
            b'=' => add_token!(self, Equal),
            b'<' if self.match_next(b'=') => add_token!(self, LessEqual),
            b'<' => add_token!(self, Less),
            b'>' if self.match_next(b'=') => add_token!(self, GreaterEqual),
            b'>' => add_token!(self, Greater),
            b'/' if self.match_next(b'/') => self.comment(),
            b'/' => add_token!(self, Slash),
            b' ' | b'\r' | b'\t' => Self::ignore(),
            b'\n' => self.line += 1,
            b'"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() => self.identifier(),
            _ => Sao::error(self.line, "Unexpected character."),
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let bytes = &self.src.get_ref()[self.start..self.current];
        let binding = String::from_utf8(bytes.into()).expect("failed to convert bytes to string");
        let mut token_type = token::Type::Identifier;
        if let Some(tt) = self.keywords.get(&binding.as_str()) {
            tt.clone_into(&mut token_type);
        }
        add_token!(self, token_type);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let text = self.string_from_bytes(self.start, self.current);
        let value = Literal::Number(text.parse().expect("failed to parse string to number"));
        add_token!(self, token::Type::Number, Some(value));
    }

    fn string(&mut self) {
        while self.peek() != b'"' && self.src.has_remaining() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if !self.src.has_remaining() {
            return Sao::error(self.line, "Unterminated string.");
        }

        self.advance();

        let value = Literal::String(self.string_from_bytes(self.start + 1, self.current - 1));
        add_token!(self, token::Type::String, Some(value));
    }

    fn comment(&mut self) {
        while self.peek() != b'\n' && self.src.has_remaining() {
            self.advance();
        }
    }

    const fn ignore() {}

    fn match_next(&mut self, expected: u8) -> bool {
        if !self.src.has_remaining() || self.peek() != expected {
            return false;
        }
        self.current += 1;
        self.src.advance(1);
        true
    }

    fn peek(&self) -> u8 {
        if self.src.has_remaining() {
            self.src.chunk()[0]
        } else {
            b'\0'
        }
    }

    fn peek_next(&self) -> u8 {
        if self.src.remaining() >= 2 {
            self.src.chunk()[1]
        } else {
            b'\0'
        }
    }

    fn advance(&mut self) {
        // TODO: error if out of bounds
        self.current += 1;
        self.src.advance(1);
    }

    fn pop(&mut self) -> u8 {
        // TODO: error if out of bounds
        self.current += 1;
        self.src.get_u8()
    }

    fn string_from_bytes(&self, start: usize, end: usize) -> String {
        let bytes = &self.src.get_ref()[start..end];
        String::from_utf8(bytes.into()).expect("failed to convert bytes to string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_scanner() -> Scanner {
        let src = Cursor::new(vec![b'h', b'e', b'y', b'\n']);
        Scanner::new(src)
    }

    #[test]
    fn test_match_next() {
        let mut scanner = create_scanner();
        let next = scanner.match_next(b'a');
        assert_eq!(next, false);
        let next = scanner.match_next(b'h');
        assert_eq!(next, true);
        let next = scanner.match_next(b'e');
        assert_eq!(next, true);
        let next = scanner.match_next(b'y');
        assert_eq!(next, true);
        let next = scanner.match_next(b'\n');
        assert_eq!(next, true);
        let next = scanner.match_next(b'\n');
        assert_eq!(next, false);
    }

    #[test]
    fn test_pop() {
        let mut scanner = create_scanner();
        assert_eq!(scanner.pop(), b'h');
        assert_eq!(scanner.pop(), b'e');
        assert_eq!(scanner.pop(), b'y');
        assert_eq!(scanner.pop(), b'\n');
        // TODO: test that cannot overflow
    }

    #[test]
    fn test_peek() {
        let mut scanner = create_scanner();
        assert_eq!(scanner.peek(), b'h');
        assert_ne!(scanner.peek(), b'e');
        scanner.pop();
        assert_eq!(scanner.peek(), b'e');
        scanner.pop();
        assert_eq!(scanner.peek(), b'y');
        scanner.pop();
        assert_eq!(scanner.peek(), b'\n');
        scanner.pop();
        assert_eq!(scanner.peek(), b'\0');
    }

    #[test]
    fn test_peek_next() {
        let mut scanner = create_scanner();
        assert_eq!(scanner.peek_next(), b'e');
        assert_ne!(scanner.peek(), b'l');
        scanner.pop();
        assert_eq!(scanner.peek_next(), b'y');
        scanner.pop();
        assert_eq!(scanner.peek_next(), b'\n');
        scanner.pop();
        assert_eq!(scanner.peek_next(), b'\0');
    }

    #[test]
    fn test_string_from_bytes() {
        let scanner = create_scanner();
        assert_eq!(
            scanner.string_from_bytes(scanner.start, scanner.current + 3),
            "hey"
        );
    }
}
