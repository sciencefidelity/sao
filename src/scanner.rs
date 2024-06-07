use crate::{Lox, Token, TokenType};
use bytes::Buf;
use std::{collections::HashMap, io::Cursor};

pub struct Scanner {
    src: Cursor<Vec<u8>>,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
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

    pub fn keywords() -> HashMap<String, TokenType> {
        use super::TokenType::*;
        let keywords = HashMap::from([
            ("and".to_owned(), And),
            ("class".to_owned(), Class),
            ("else".to_owned(), Else),
            ("false".to_owned(), False),
            ("for".to_owned(), For),
            ("fun".to_owned(), Fun),
            ("if".to_owned(), If),
            ("nil".to_owned(), Nil),
            ("or".to_owned(), Or),
            ("print".to_owned(), Print),
            ("return".to_owned(), Return),
            ("super".to_owned(), Super),
            ("this".to_owned(), This),
            ("true".to_owned(), True),
            ("var".to_owned(), Var),
            ("while".to_owned(), While),
        ]);
        keywords
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        use super::TokenType::EOF;

        while self.src.has_remaining() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(EOF, "".to_owned(), self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        use super::TokenType::*;
        match self.get_u8() {
            b'(' => self.add_token(LeftParen),
            b')' => self.add_token(RightParen),
            b'{' => self.add_token(LeftBrace),
            b'}' => self.add_token(RightBrace),
            b',' => self.add_token(Comma),
            b'.' => self.add_token(Dot),
            b'-' => self.add_token(Minus),
            b'+' => self.add_token(Plus),
            b';' => self.add_token(Semicolon),
            b'*' => self.add_token(Star),
            b'!' if self.match_next(b'=') => self.add_token(BangEqual),
            b'!' => self.add_token(Bang),
            b'=' if self.match_next(b'=') => self.add_token(EqualEqual),
            b'=' => self.add_token(Equal),
            b'<' if self.match_next(b'=') => self.add_token(LessEqual),
            b'<' => self.add_token(Less),
            b'>' if self.match_next(b'=') => self.add_token(GreaterEqual),
            b'>' => self.add_token(Greater),
            b'/' if self.match_next(b'/') => {
                while self.peek_u8() != b'\n' && self.src.has_remaining() {
                    self.get_u8();
                }
            }
            b'/' => self.add_token(Slash),
            b' ' | b'\r' | b'\t' => {}
            b'\n' => self.line += 1,
            b'"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() => self.identifier(),
            _ => Lox::error(self.line, "Unexpected character.".to_owned()),
        }
    }

    fn identifier(&mut self) {
        while self.peek_u8().is_ascii_alphanumeric() {
            self.get_u8();
        }

        // TODO: combine with add_token() or add helper function
        let bytes = &self.src.get_ref()[self.start..self.current];
        // TODO: fail without panic
        let text = String::from_utf8(bytes.into()).expect("failed to convert bytes to string");
        if let Some(token_type) = self.keywords.get(&text) {
            self.add_token(token_type.to_owned());
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn number(&mut self) {
        while self.peek_u8().is_ascii_digit() {
            self.get_u8();
        }

        if self.peek_u8() == b'.' && self.peek_next_u8().is_ascii_digit() {
            self.get_u8();
            while self.peek_u8().is_ascii_digit() {
                self.get_u8();
            }
        }

        // TODO: find a way to convert this to f64
        self.add_token(TokenType::Number);
    }

    fn string(&mut self) {
        while self.peek_u8() != b'"' && self.src.has_remaining() {
            if self.peek_u8() == b'\n' {
                self.line += 1;
            }
            self.get_u8();
        }

        if !self.src.has_remaining() {
            return Lox::error(self.line, "Unterminated string.".to_owned());
        }

        self.get_u8();
        self.start += 1;
        self.current -= 1;
        self.add_token(TokenType::String);
        self.current += 1;
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if !self.src.has_remaining() {
            false
        } else if self.peek_u8() != expected {
            false
        } else {
            self.current += 1;
            self.src.advance(1);
            true
        }
    }

    fn get_u8(&mut self) -> u8 {
        self.current += 1;
        self.src.get_u8()
    }

    fn peek_u8(&self) -> u8 {
        if !self.src.has_remaining() {
            return b'\0';
        }
        self.src.chunk()[0]
    }

    fn peek_next_u8(&self) -> u8 {
        if !self.src.remaining() <= 2 {
            return b'\0';
        }
        self.src.chunk()[1]
    }

    fn add_token(&mut self, token_type: TokenType) {
        let bytes = &self.src.get_ref()[self.start..self.current];
        // TODO: fail without panic
        let text = String::from_utf8(bytes.into()).expect("failed to convert bytes to string");
        self.tokens.push(Token::new(token_type, text, self.line));
    }
}
