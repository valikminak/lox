use crate::tokenize::Token;

#[derive(Debug, PartialEq)]
pub struct AST {
    pub top: Option<Expr>, // could have an empty program
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    OAdd,
    OSub,
    OMul,
    ODiv,
    OLt,
    OLe,
    OGt,
    OGe,
    OEq,
    ONe,
    ONot,
    OAnd,
    OOr,
}

use Operator::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    // Literal { value: Literal },
    ENumber {value: String},
    EString {value: String},
    EBool {value: bool},
    ENil,

    EBinary {left: Box<Expr>, op: Operator, right: Box<Expr>},
    EUnary { op: Operator, right: Box<Expr> },
    EGrouping { expr: Box<Expr> },
}

use Expr::*;

impl Expr {
    pub fn number(value: impl Into<String>) -> Expr {
        ENumber {value: value.into()}
    }
    pub fn string(value: impl Into<String>) -> Expr {
        EString {value: value.into()}
    }

    pub fn bool(value: bool) -> Expr {
        EBool {value}
    }
    pub fn nil() -> Expr {
        ENil
    }

    // the .into puts the value in the Box in this case

    pub fn binary(left: Expr, op: Operator, right: Expr) -> Expr {
        EBinary {left: left.into(), op, right: right.into()}
    }

    pub fn unary(op: Operator, right: Expr) -> Expr {
        EUnary {op, right: right.into()}
    }

    pub fn grouping(expr: Expr) -> Expr {
        EGrouping { expr: expr.into() }
    }
}

pub fn format_op(o: &Operator) -> &'static str {
    match o {
        OAdd => "+",
        OSub => "-",
        OMul => "*",
        ODiv => "/",
        OLt => "<",
        OLe => ">",
        OGt => ">",
        OGe => ">=",
        OEq => "==",
        ONe => "!=",
        OAnd => "and",
        OOr => "or",
        ONot => "!",
    }
}

pub fn format_expr(e: &Expr) -> String {
    match e {
        ENumber { value } => format!("{}", value),
        EString { value } => format!("\"{}\"", value),
        EBool { value } => format!("{}", value),
        ENil => "nil".to_string(),
        EBinary { left, op, right } => {
            format!("({} {} {})", format_op(op), format_expr(left), format_expr(right))
        },
        EUnary { op, right } => {
            format!("({}{})", format_op(op), format_expr(right))
        },
        EGrouping { expr } => format!("group ({})", format_expr(expr) ),

    }
}

pub fn main() {
    let expression = Expr::binary(
        Expr::unary(Operator::OSub, Expr::number("123")),
        OMul,
        Expr::grouping(Expr::number("45.67")),
    );
    println!("{}", format_expr(&expression));
}