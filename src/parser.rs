use std::hash::Hash;
use crate::tokenize::{Token, TokenType, Tokens};
use crate::ast::{Expr, Operator, AST};
use crate::tokenize::TokenType::*;


impl From<&Token> for Operator {
    fn from(tok: &Token) -> Self {
        match tok.toktype {
            TPlus => Operator::OAdd,
            TMinus => Operator::OSub,
            TStar => Operator::OMul,
            TSlash => Operator::ODiv,
            TLess => Operator::OLt,
            TLessEqual => Operator::OLe,
            TGreater => Operator::OGt,
            TGreaterEqual => Operator::OGe,
            TEqualEqual => Operator::OEq,
            TBangEqual => Operator::ONe,
            TAnd => Operator::OAnd,
            TOr => Operator::OOr,
            TBang => Operator::ONot,
            _ => panic!("Not an operator {:?}", tok.toktype)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    SyntaxError {line: usize, msg: String}
}

// pub type Error = ();  // roughly playing the same role as Python’s None

struct Parser {
    // parsing involves a left-to-right scan over tokens. Sometimes
    // peeking ahead is necessary. This struct is used for managing that.
    tokens: Vec<Token>,
    n: usize,
}


impl Parser {

    fn new(tokens: Tokens) -> Self {
        Self { tokens: tokens.tokens, n: 0 }
    }

    fn accept(&mut self, toktype: TokenType) -> bool {
        if !self.at_end() && self.tokens[self.n].toktype == toktype {
            self.n += 1;
            true
        }else {
            false
        }
    }


    // accept any token from a list of possible types
    fn accepts<const N: usize>(&mut self, toktypes: [TokenType; N]) -> bool {
        if !self.at_end() && toktypes.contains(&self.tokens[self.n].toktype) {
            self.n += 1;
            true
        }else {
            false
        }
    }
    
    fn expect(&mut self, toktype: TokenType, msg: &str) -> Result<(), Error> {
        // require the next token to exactly match the given tokentype or else error
        if !self.accept(toktype) {
            Err(self.syntax_error(msg))
        }else {
            Ok(())
        }
    }

    // helper function to create a syntax error
    fn syntax_error(&self, msg: &str) -> Error {
        Error::SyntaxError{
            line: self.tokens[self.n].line,
            msg: msg.into()
        }
    }
    
    // return the last matched token (a borrow)
    fn last_token(&self) -> &Token {
        &self.tokens[self.n-1]
    }

    fn last_lexeme(&self) -> &String {
        &self.tokens[self.n-1].lexeme
    }

    fn at_end(&self) -> bool {
        self.n >= self.tokens.len()
    }

    fn parse_top(&mut self) -> Result<AST, Error> {
        Ok(AST{top: Some(self.parse_expression()?)})
    }

    fn parse_expression(&mut self) -> Result<Expr, Error> {
        let left = self.parse_primary()?;
        if self.accepts([TPlus, TMinus, TStar, TSlash]) {
            let op = Operator::from(self.last_token());
            let right = self.parse_primary()?;
            Ok(Expr::binary(left, op, right))
        }else {
            Ok(left)
        }
    }

    // parse a single value (like a literal number, string, etc.)
    fn parse_primary(&mut self) -> Result<Expr, Error>  {
        Ok(
            if self.accept(TNumber) {
                Expr::number(self.last_lexeme())
            }else if self.accept(TString){
                Expr::string(self.last_lexeme())
            }else if self.accept(TLeftParen){
                let expr = self.parse_expression()?;
                self.expect(TRightParen, "Expect ')' after expression.");
                Expr::grouping(expr)
            }
            else {
                return Err(self.syntax_error("Expected primary"));
            }
        )
    }

}

pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    println!("Parsing");
    Parser::new(tokens).parse_top()
    // Ok(AST {top: None})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive(){
        assert_eq!(true, true);
    }
}