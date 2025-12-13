use std::fmt::Formatter;
use std::rc::Rc;
use crate::ast::{Expr, AST, Operator, Stmt};
use crate::evaluate::LoxValue::LBoolean;

// the goal of the evaluator is to convert the AST into a LoxValue.
#[derive(Debug, PartialEq, Clone)]
pub enum LoxValue{
    LNil,
    LBoolean(bool),
    LNumber(f64),
    LString(String),
}
pub type Output = ();
type Environment = crate::environ::Environment<LoxValue>;

impl LoxValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            LoxValue::LNil | LBoolean(false) => false,
            _ => true,
        }
    }
}

impl std::fmt::Display for LoxValue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Operator::*;
        match self {
            LoxValue::LNil => formatter.write_str("nil"),
            LoxValue::LBoolean(v) => formatter.write_str(&format!("{v}")),
            LoxValue::LNumber(v) => formatter.write_str(&format!("{v}")),
            LoxValue::LString(v) => formatter.write_str(v),
        }?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    ZeroDivision,
    UnsupportedBinOp(LoxValue, Operator, LoxValue),
    UnsupportedUnaryOp(Operator, LoxValue),
}

pub struct Interpreter {
    top_level: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter{top_level: Environment::new(None)}
    }

    pub fn evaluate(&mut self,  ast: AST) -> Result<Output, Error> {
        execute_statements(&ast.top, &self.top_level)?;
        Ok(())
    }
}

pub fn evaluate(ast: AST) -> Result<Output, Error> {
    println!("Evaluating");
    let mut environ = Environment::new(None);
    execute_statements(&ast.top, &environ)?;
    Ok(())
}

pub fn execute_statements(statements: &Vec<Stmt>, environ: &Rc<Environment>) -> Result<(), Error> {
    // execute zero or more statements
    for stmt in statements.iter() {
        execute_statement(stmt, environ)?;
    };
    Ok(())
}

pub fn execute_statement(stmt: &Stmt, environ: &Rc<Environment>) -> Result<(), Error> {
    // execute a single statement
    match stmt {
        Stmt::SPrint{expr} => {
            let value = evaluate_expression(expr, environ)?;
            println!("{value:?}");
        },
        Stmt::SExpression{expr} => {

            evaluate_expression(expr, environ)?;
        },
        Stmt::SVarDecl {name, initializer} => {
            let iv = match initializer {
                Some(v) => evaluate_expression(v, environ)?,
                None => LoxValue::LNil
            };
            environ.declare(name, iv)
        }
    }
    Ok(()) // statements don't produce values
}

pub fn evaluate_expression(expr: &Expr, environ: &Rc<Environment>) -> Result<LoxValue, Error> {
    Ok(match expr {
        Expr::ENumber {value} => {
            LoxValue::LNumber(value.parse().unwrap())
        },
        Expr::EString {value} => {
            LoxValue::LString(value.clone())
        }
        Expr::EBool {value} => {
            LoxValue::LBoolean(*value)
        }
        Expr::ENil =>{
            LoxValue::LNil
        },
        Expr::EVariable {name} => {
            environ.lookup(name).unwrap().clone()
        }
        Expr::EBinary {left, op, right} => {
            use LoxValue::*;
            use Operator::*;
            let lv = evaluate_expression(left, environ)?;
            let rv = evaluate_expression(right, environ)?;
            match (lv, op, rv) {
                (LNumber(x), OAdd, LNumber(y))=> LNumber(x + y),
                (LNumber(x), OSub, LNumber(y))=> LNumber(x - y),
                (LNumber(x), OMul, LNumber(y))=> LNumber(x * y),
                (LNumber(x), ODiv, LNumber(y))=> {
                    if y == 0.0 {
                        return Err(Error::ZeroDivision)
                    }else{
                        LNumber(x / y)
                    }
                },
                (LNumber(x), OLt, LNumber(y))=> LBoolean(x < y),
                (LNumber(x), OLe, LNumber(y))=> LBoolean(x <= y),
                (LNumber(x), OGt, LNumber(y))=> LBoolean(x > y),
                (LNumber(x), OGe, LNumber(y))=> LBoolean(x >= y),
                // string
                (LString(x), OAdd, LString(y))=> LString(format!("{}{}", x, y)),

                // equality works with any combination of values
                (x, OEq, y)=> LBoolean(x == y),
                (x, ONe, y)=> LBoolean(x != y),
                (lv, op, rv) => {
                    return Err(Error::UnsupportedBinOp(lv, *op, rv))
                }
            }
        }
        Expr::EUnary {op, right} => {
            use LoxValue::*;
            use Operator::*;
            let rv = evaluate_expression(right, environ)?;
            match (op, rv) {
                (OSub, LNumber(x)) => LNumber(-x),
                (ONot, x) => LBoolean(!x.is_truthy()),
                (op, rv) => {
                    return Err(Error::UnsupportedUnaryOp(*op, rv));
                }
            }
        }
        Expr::EGrouping { expr} => {
            evaluate_expression(expr, environ)?
        },
        Expr::EAssign { name, value } => {
            let v = evaluate_expression(value, environ)?;
            environ.assign(name, v.clone());
            v
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}
