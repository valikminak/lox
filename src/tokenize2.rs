// tokenize2.rs
//
// A Re-envisioned tokenizer
use crate::reader::Source;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    TLeftParen,
    TRightParen,
    TLeftBrace,
    TRightBrace,
    TComma,
    TDot,
    TMinus,
    TPlus,
    TSemiColon,
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

    TIgnore,
    TEof,
}

use TokenType::*;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub toktype: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(toktype: TokenType, lexeme: impl Into<String>, line: usize) -> Token {
        Token {
            toktype,
            lexeme: lexeme.into(),
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
pub struct Error(Vec<ScanError>);

// This makes it easier to iterate over all of the scan errors
impl Error {
    pub fn iter(&self) -> std::slice::Iter<'_, ScanError> {
        self.0.iter()
    }
}

use std::iter::Peekable;
use std::str::CharIndices;
use std::ops::Range;

type Chars<'a> = Peekable<CharIndices<'a>>;

fn accept(chars: &mut Chars, toktype: TokenType, start: usize) ->
Option<(TokenType, Range<usize>)> {
    let (n, ch) = chars.next()?;
    Some((toktype, start..n+1))
}

fn peek(chars: &mut Chars, ch: char) -> bool {
    if let Some(&(_, c)) = chars.peek() {
        c == ch
    } else {
        false
    }
}

fn scan_tokens(s: String) -> Result<Tokens, Error> {
    let mut chars = s.char_indices().peekable();
    let mut result = Vec::new();
    let mut line = 1;
    while let Some((toktype, range)) = scan_token(&mut chars) {
        if toktype != TIgnore {
            result.push(Token::new(toktype, &s[range], line));
        }
    }
    result.push(Token::new(TEof, "", line));
    Ok(Tokens { tokens: result })
}

fn scan_token(chars : &mut Chars) -> Option<(TokenType, Range<usize>)> {
    scan_simple_symbol(chars)
        .or_else(|| scan_compare_symbol(chars))
        .or_else(|| ignore_whitespace(chars))
        .or_else(|| scan_number(chars))
        .or_else(|| scan_identifier(chars))
        .or_else(|| scan_string(chars))
}

fn ignore_whitespace(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start + 1;
    if ch.is_whitespace() {
        while let Some(&(n, ch)) = chars.peek() {
            if ch.is_whitespace() {
                end = n;
                let _ = chars.next();
            } else {
                break;
            }
        }
        Some((TIgnore, start..end+1))
    } else {
        None
    }
}

fn scan_simple_symbol(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    match ch {
        '+' => accept(chars, TPlus, start),
        '-' => accept(chars, TMinus, start),
        '*' => accept(chars, TStar, start),
        '(' => accept(chars, TLeftParen, start),
        ')' => accept(chars, TRightParen, start),
        '{' => accept(chars, TLeftBrace, start),
        '}' => accept(chars, TRightBrace, start),
        ';' => accept(chars, TSemiColon, start),
        ',' => accept(chars, TComma, start),
        '.' => accept(chars, TDot, start),
        _ => None
    }
}

fn scan_compare_symbol(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    match ch {
        '<' => {
            let _ = chars.next();
            if peek(chars, '=') {
                accept(chars, TLessEqual, start)
            } else {
                Some((TLess, start..start+1))
            }
        },
        '>' => {
            let _ = chars.next();
            if peek(chars, '=') {
                accept(chars, TGreaterEqual, start)
            } else {
                Some((TGreater, start..start+1))
            }
        },
        '=' => {
            let _ = chars.next();
            if peek(chars, '=') {
                accept(chars, TEqualEqual, start)
            } else {
                Some((TEqual, start..start+1))
            }
        },
        '!' => {
            let _ = chars.next();
            if peek(chars, '=') {
                accept(chars, TBangEqual, start)
            } else {
                Some((TBang, start..start+1))
            }
        },
        _ => None
    }
}

fn scan_number(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start;
    if ch.is_digit(10) {
        while let Some(&(n, ch)) = chars.peek() {
            if ch.is_digit(10) {
                end = n;
                chars.next().unwrap();
            } else {
                break;
            }
        }
        if peek(chars, '.') {
            chars.next().unwrap();
            while let Some(&(n, ch)) = chars.peek() {
                if ch.is_digit(10) {
                    end = n;
                    chars.next().unwrap();
                } else {
                    break;
                }
            }
        }
        Some((TNumber, start..end+1))
    } else {
        None
    }
}

fn scan_string(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start+1;
    if ch == '"' {
        chars.next().unwrap();
        while let Some((n, ch)) = chars.next() {
            end = n;
            if ch == '"' {
                break;
            }
        }
        Some((TString, start..end+1))
    } else {
        None
    }
}

fn scan_identifier(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start;
    if ch.is_alphabetic() || ch == '_' {
        while let Some(&(n, ch)) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                end = n;
                chars.next().unwrap();
            } else {
                break;
            }
        }
        Some((TIdentifier, start..end+1))
    } else {
        None
    }
}

pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    // println!("Tokenizing");
    scan_tokens(source.contents)
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
        let tokens = scan_tokens(String::from("(){},.-+;*"));
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TLeftParen, "(", 1),
                Token::new(TRightParen, ")", 1),
                Token::new(TLeftBrace, "{", 1),
                Token::new(TRightBrace, "}", 1),
                Token::new(TComma, ",", 1),
                Token::new(TDot, ".", 1),
                Token::new(TMinus, "-", 1),
                Token::new(TPlus, "+", 1),
                Token::new(TSemiColon, ";", 1),
                Token::new(TStar, "*", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }

    #[test]
    fn two_character() {
        let tokens = scan_tokens(String::from("! != < <= > >= == ="));
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TBang, "!", 1),
                Token::new(TBangEqual, "!=", 1),
                Token::new(TLess, "<", 1),
                Token::new(TLessEqual, "<=", 1),
                Token::new(TGreater, ">", 1),
                Token::new(TGreaterEqual, ">=", 1),
                Token::new(TEqualEqual, "==", 1),
                Token::new(TEqual, "=", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }

    #[test]
    fn strings() {
        let tokens = scan_tokens(String::from("\"hello\" \"world\""));
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TString, "\"hello\"", 1),
                Token::new(TString, "\"world\"", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }

    #[test]
    fn numbers() {
        let tokens = scan_tokens(String::from("12345 123.45"));
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TNumber, "12345", 1),
                Token::new(TNumber, "123.45", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }
    #[test]
    fn identifiers() {
        let tokens = scan_tokens(String::from("abc def123 ab_cd"));
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TIdentifier, "abc", 1),
                Token::new(TIdentifier, "def123", 1),
                Token::new(TIdentifier, "ab_cd", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }
    #[test]
    fn keywords() {
        let tokens = scan_tokens(String::from(
            "and class else false for fun if nil or print return super this true var while",
        ));
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TAnd, "and", 1),
                Token::new(TClass, "class", 1),
                Token::new(TElse, "else", 1),
                Token::new(TFalse, "false", 1),
                Token::new(TFor, "for", 1),
                Token::new(TFun, "fun", 1),
                Token::new(TIf, "if", 1),
                Token::new(TNil, "nil", 1),
                Token::new(TOr, "or", 1),
                Token::new(TPrint, "print", 1),
                Token::new(TReturn, "return", 1),
                Token::new(TSuper, "super", 1),
                Token::new(TThis, "this", 1),
                Token::new(TTrue, "true", 1),
                Token::new(TVar, "var", 1),
                Token::new(TWhile, "while", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }
}
