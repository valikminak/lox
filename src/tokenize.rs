use crate::reader::Source;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // single-character tokens
    TLeftParen,
    TRightParen,
    TLeftBrace,
    TRightBrace,
    TComma,
    TDot,
    TMinus,
    TPlus,
    TSemicolon,
    TSlash,
    TStar,

    // One or two character tokens
    TBang,
    TBangEqual,
    TEqual,
    TEqualEqual,
    TGreater,
    TGreaterEqual,
    TLess,
    TLessEqual,

    // Literals
    TIdentifier,
    TString,
    TNumber,

    // Keywords
    TAnd,
    TClass,
    TElse,
    TFalse,
    TFun,
    TFor,
    TIf,
    TNil,
    TOr,
    TPrint,
    TReturn,
    TSuper,
    TThis,
    TTrue,
    TVar,
    TWhile,
    TEof,
}

use TokenType::*;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Num(f64),
    None,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub toktype: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(
        toktype: TokenType,
        lexeme: impl Into<String>,
        literal: Literal,
        line: usize,
    ) -> Token {
        Token {
            toktype,
            lexeme: lexeme.into(),
            literal,
            line,
        }
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum ScanError {
    UnexpectedCharacter { line: usize, ch: char },
    UnterminatedString { line: usize },
}
#[derive(Debug)]
pub struct Error (Vec<ScanError>);


struct Scanner {
    // we're converting the input source text into a Vec<char>.
    // The 'char' Rust type represents a Unicode code point in the range
    // 0 to 0x10FFFFF. Internally, char is 32-bits.
    // The main reason for doing this is that scanning works
    // naturally with characters. We need to make it easier
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<ScanError>,
}

impl Scanner {
    fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    fn error(&mut self, err: ScanError) {
        self.errors.push(err);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(mut self) -> Result<Tokens, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TEof, "", Literal::None, self.line));

        if self.errors.len() == 0 {
            Ok(Tokens {
                tokens: self.tokens,
            })
        }else{
            Err(Error(self.errors))
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn lexeme(&self) -> String {
        // return the current lexeme as a string
        self.source[self.start..self.current].iter().collect()
    }

    fn add_token(&mut self, toktype: TokenType) {
        self.add_token_with_literal(toktype, Literal::None);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\x00';
        } else {
            self.source[self.current]
        }
    }

    fn add_token_with_literal(&mut self, toktype: TokenType, literal: Literal) {
        self.tokens
            .push(Token::new(toktype, self.lexeme(), literal, self.line));
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TLeftParen),
            ')' => self.add_token(TRightParen),
            '{' => self.add_token(TLeftBrace),
            '}' => self.add_token(TRightBrace),
            ',' => self.add_token(TComma),
            '.' => self.add_token(TDot),
            '-' => self.add_token(TMinus),
            '+' => self.add_token(TPlus),
            ';' => self.add_token(TSemicolon),
            '*' => self.add_token(TStar),
            '!' => {
                let toktype = if self.matches('=') {
                    TBangEqual
                } else {
                    TBang
                };
                self.add_token(toktype);
            }
            '=' => {
                let toktype = if self.matches('=') {
                    TEqualEqual
                } else {
                    TEqual
                };
                self.add_token(toktype);
            }
            '<' => {
                let toktype = if self.matches('=') {
                    TLessEqual
                } else {
                    TLess
                };
                self.add_token(toktype);
            }
            '>' => {
                let toktype = if self.matches('=') {
                    TGreaterEqual
                } else {
                    TGreater
                };
                self.add_token(toktype);
            }
            '/' => {
                if self.matches('/') {
                    // comment goes to end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TSlash);
                }
            }
            // Ignore whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c if c.is_digit(10) => {
                self.number();
            }
            c if c.is_alphabetic() => self.identifier(),
            e => {
                self.errors.push(ScanError::UnexpectedCharacter {line: self.line, ch: e});
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            todo!("Unterminated string")
        }
        self.advance();
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_with_literal(TString, Literal::Str(value));
    }
    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let literal = Literal::Num(self.lexeme().parse().unwrap());
        self.add_token_with_literal(TNumber, literal);
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        // also HashMap is a way to coup with it
        let toktype = match &self.lexeme()[..] {
            "and" => TAnd,
            "class" => TClass,
            "else" => TElse,
            "false" => TFalse,
            "for" => TFor,
            "fun" => TFun,
            "if" => TIf,
            "nil" => TNil,
            "or" => TOr,
            "print" => TPrint,
            "return" => TReturn,
            "super" => TSuper,
            "this" => TThis,
            "true" => TTrue,
            "var" => TVar,
            "while" => TWhile,
            _ => TIdentifier,
        };
        self.add_token(toktype);
    }
}

pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing");

    Scanner::new(&source.contents).scan_tokens()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }

    #[test]
    fn single_character() {
        let scanner = Scanner::new("(){},.-+;*");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TLeftParen, "(", Literal::None, 1),
                Token::new(TRightParen, ")", Literal::None, 1),
                Token::new(TLeftBrace, "{", Literal::None, 1),
                Token::new(TRightBrace, "}", Literal::None, 1),
                Token::new(TComma, ",", Literal::None, 1),
                Token::new(TDot, ".", Literal::None, 1),
                Token::new(TMinus, "-", Literal::None, 1),
                Token::new(TPlus, "+", Literal::None, 1),
                Token::new(TSemicolon, ";", Literal::None, 1),
                Token::new(TStar, "*", Literal::None, 1),
                Token::new(TEof, "", Literal::None, 1),
            ]
        );
    }

    #[test]
    fn two_characters() {
        let scanner = Scanner::new("!  !=    < <=  > >= ==    =");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TBang, "!", Literal::None, 1),
                Token::new(TBangEqual, "!=", Literal::None, 1),
                Token::new(TLess, "<", Literal::None, 1),
                Token::new(TLessEqual, "<=", Literal::None, 1),
                Token::new(TGreater, ">", Literal::None, 1),
                Token::new(TGreaterEqual, ">=", Literal::None, 1),
                Token::new(TEqualEqual, "==", Literal::None, 1),
                Token::new(TEqual, "=", Literal::None, 1),
                Token::new(TEof, "", Literal::None, 1),
            ]
        );
    }

    #[test]
    fn strings() {
        let scanner = Scanner::new("\"hello\" \"world\"");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(
                    TString,
                    "\"hello\"",
                    Literal::Str("hello".to_string()),
                    1
                ),
                Token::new(
                    TString,
                    "\"world\"",
                    Literal::Str("world".to_string()),
                    1
                ),
                Token::new(TEof, "", Literal::None, 1),
            ]
        );
    }

    #[test]
    fn numbers() {
        let scanner = Scanner::new("12345 123.45");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TNumber, "12345", Literal::Num(12345.0), 1),
                Token::new(TNumber, "123.45", Literal::Num(123.45), 1),
                Token::new(TEof, "", Literal::None, 1),
            ]
        );
    }

    #[test]
    fn identifiers() {
        let scanner = Scanner::new("abc def123 ab_cd");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TIdentifier, "abc", Literal::None, 1),
                Token::new(TIdentifier, "def123", Literal::None, 1),
                Token::new(TIdentifier, "ab_cd", Literal::None, 1),
                Token::new(TEof, "", Literal::None, 1),
            ]
        );
    }

    #[test]
    fn keywords() {
        let scanner = Scanner::new(
            "and class else false for fun if nil or print return super this true var while",
        );
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TAnd, "and", Literal::None, 1),
                Token::new(TClass, "class", Literal::None, 1),
                Token::new(TElse, "else", Literal::None, 1),
                Token::new(TFalse, "false", Literal::None, 1),
                Token::new(TFor, "for", Literal::None, 1),
                Token::new(TFun, "fun", Literal::None, 1),
                Token::new(TIf, "if", Literal::None, 1),
                Token::new(TNil, "nil", Literal::None, 1),
                Token::new(TOr, "or", Literal::None, 1),
                Token::new(TPrint, "print", Literal::None, 1),
                Token::new(TReturn, "return", Literal::None, 1),
                Token::new(TSuper, "super", Literal::None, 1),
                Token::new(TThis, "this", Literal::None, 1),
                Token::new(TTrue, "true", Literal::None, 1),
                Token::new(TVar, "var", Literal::None, 1),
                Token::new(TWhile, "while", Literal::None, 1),
                Token::new(TEof, "", Literal::None, 1),
            ]
        );
    }
}
