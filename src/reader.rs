pub struct Source {
    pub contents: String,
}

impl Source {
    pub fn from(s: impl Into<String>) -> Source {
        Source { contents: s.into() }
    }
}

#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error {
            msg: format!("{}", e),
        }
    }
}

pub fn read_source(filename: &str) -> Result<Source, Error> {
    println!("Reading source");
    let contents = std::fs::read_to_string(filename)?;
    Ok(Source { contents })
}

#[cfg(test)]
mod tests {
    use crate::ast::{AST, Expr, Operator};
    use crate::parser::{parse, Parser};

    // helper
    fn parse_string(s: &str) -> AST {
        use crate::reader::Source;
        use crate::tokenize::tokenize;
        let source = Source::from(s);
        let tokens = tokenize(source).unwrap();
        parse(tokens).unwrap()
    }

    fn parse_expr_string(s: &str) -> Expr {
        use crate::reader::Source;
        use crate::tokenize::tokenize;
        let source = Source::from(s);
        let tokens = tokenize(source).unwrap();
        Parser::new(tokens).parse_expression().unwrap()
    }

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_primary() {
        assert_eq!(parse_expr_string("123"), Expr::number("123"));
        assert_eq!(parse_expr_string("\"hello\""), Expr::string("hello"));
        assert_eq!(parse_expr_string("(2)"), Expr::grouping(Expr::number("2")));
        assert_eq!(parse_expr_string("nil"), Expr::nil());
        assert_eq!(parse_expr_string("true"), Expr::bool(true));
        assert_eq!(parse_expr_string("false"), Expr::bool(false));
    }

    #[test]
    fn test_binary() {
        assert_eq!(
            parse_expr_string("1 + 2"),
            Expr::binary(Expr::number("1"), Operator::OAdd, Expr::number("2"))
        );
    }
}
