use crate::ast::{AST, Expr, Operator, Stmt};
use crate::tokenize::TokenType::*;
use crate::tokenize::{Token, TokenType, Tokens};

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
            _ => panic!("Not an operator {:?}", tok.toktype),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    SyntaxError { line: usize, msg: String },
}

// pub type Error = ();  // roughly playing the same role as Python’s None

pub struct Parser {
    // parsing involves a left-to-right scan over tokens. Sometimes
    // peeking ahead is necessary. This struct is used for managing that.
    tokens: Vec<Token>,
    n: usize,
}

impl Parser {
    pub fn new(tokens: Tokens) -> Self {
        Self {
            tokens: tokens.tokens,
            n: 0,
        }
    }

    fn accept(&mut self, toktype: TokenType) -> bool {
        if !self.at_end() && self.tokens[self.n].toktype == toktype {
            self.n += 1;
            true
        } else {
            false
        }
    }

    // accept any token from a list of possible types
    fn accepts<const N: usize>(&mut self, toktypes: [TokenType; N]) -> bool {
        if !self.at_end() && toktypes.contains(&self.tokens[self.n].toktype) {
            self.n += 1;
            true
        } else {
            false
        }
    }

    fn consume(&mut self, toktype: TokenType, msg: &str) -> Result<(), Error> {
        // require the next token to exactly match the given tokentype or else error
        if !self.accept(toktype) {
            Err(self.syntax_error(msg))
        } else {
            Ok(())
        }
    }

    // helper function to create a syntax error
    fn syntax_error(&self, msg: &str) -> Error {
        Error::SyntaxError {
            line: self.tokens[self.n].line,
            msg: format!("{msg} at {:?}", self.tokens[self.n].lexeme),
        }
    }

    // return the last matched token (a borrow)
    fn last_token(&self) -> &Token {
        &self.tokens[self.n - 1]
    }

    fn last_lexeme(&self) -> &String {
        &self.tokens[self.n - 1].lexeme
    }

    fn at_end(&self) -> bool {
        self.n >= self.tokens.len() || self.tokens[self.n].toktype == TEof
    }

    fn parse_top(&mut self) -> Result<AST, Error> {
        let top = self.parse_statements()?;

        if !self.at_end() {
            return Err(self.syntax_error("Unparsed input"));
        }
        Ok(AST { top })
    }

    fn parse_statements(&mut self) -> Result<Vec<Stmt>, Error> {
        // zero or more statements
        let mut statements= Vec::new();
        while !self.at_end() {
            statements.push(self.parse_declaration()?);
        }
        Ok(statements)
    }

    fn parse_var_declaration(&mut self) -> Result<Stmt, Error> {
        self.consume(TIdentifier, "Expect variable name")?;
        let name = self.last_lexeme().clone();
        let  mut initializer = None;
        if self.accept(TEqual) {
            initializer = Some(self.parse_expression()?);
        }
        self.consume(TSemicolon, "Expect ';' after variable declaration")?;
        Ok(Stmt::vardecl(name, initializer))
    }

    fn parse_declaration(&mut self) -> Result<Stmt, Error> {
        // parse a declaration or a statement.
        if self.accept(TVar) {
            self.parse_var_declaration()
        }else {
            self.parse_statement()
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt, Error> {
        // parse a single statement
        if self.accept(TPrint) {
            self.parse_print_statement()
        }else {
            self.parse_expression_statement()
        }
    }

    fn parse_print_statement(&mut self) -> Result<Stmt, Error> {
        // print expression
        let value = self.parse_expression()?;
        self.consume(TSemicolon, "Expected ';' after value.")?;
        Ok(Stmt::print(value))
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, Error> {
        // expression
        let value = self.parse_expression()?;
        self.consume(TSemicolon, "Expected ';' after value.")?;
        Ok(Stmt::expression(value))
    }

    pub fn parse_expression(&mut self) -> Result<Expr, Error> {
        self.parse_assignment()
    }

    pub fn parse_assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.parse_binary()?;
        if self.accept(TEqual) {
            let value = self.parse_assignment()?;
            if let Expr::EVariable {name} = expr {
                return Ok(Expr::assign(name, value));
            }else {
                panic!("invalid assignment target");
            }
        }
        Ok(expr)
    }

    pub fn parse_binary(&mut self) -> Result<Expr, Error> {
        let left = self.parse_unary()?;
        if self.accepts([
            TPlus, TMinus, TStar, TSlash, TLess, TLessEqual, TGreater, TGreaterEqual, TEqualEqual, TBangEqual
        ]) {
            let op = Operator::from(self.last_token());
            let right = self.parse_unary()?;
            Ok(Expr::binary(left, op, right))
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, Error> {
        if self.accepts([TMinus, TBang]){
            let op = Operator::from(self.last_token());
            Ok(Expr::unary(op, self.parse_expression()?))
        }else {
            Ok(self.parse_primary()?)
        }
    }

    // parse a single value (like a literal number, string, etc.)
    fn parse_primary(&mut self) -> Result<Expr, Error> {
        Ok(if self.accept(TNumber) {
            Expr::number(self.last_lexeme())
        } else if self.accept(TString) {
            let lexeme = self.last_lexeme();
            Expr::string(&lexeme[1..lexeme.len() - 1])
        } else if self.accept(TNil) {
            Expr::nil()
        } else if self.accept(TTrue) {
            Expr::bool(true)
        } else if self.accept(TFalse) {
            Expr::bool(false)
        } else if self.accept(TLeftParen) {
            let expr = self.parse_expression()?;
            self.consume(TRightParen, "Expect ')' after expression.")?;
            Expr::grouping(expr)

        } else if self.accept(TIdentifier) {
            Expr::variable(self.last_lexeme())
        }
        else {
            return Err(self.syntax_error("Expected primary"));
        })
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
    fn its_alive() {
        assert_eq!(true, true);
    }
}
