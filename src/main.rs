use std::io::Write;

mod evaluate;
mod parser;
mod reader;
mod tokenize;
mod ast;
mod environ;

// top-level error
#[derive(Debug)]
pub enum Error {
    Read(reader::Error),
    Tokenize(tokenize::Error),
    Parse(parser::Error),
    Evaluate(evaluate::Error),
}

impl From<reader::Error> for Error {
    fn from(error: reader::Error) -> Error {
        Error::Read(error)
    }
}

impl From<tokenize::Error> for Error {
    fn from(error: tokenize::Error) -> Error {
        Error::Tokenize(error)
    }
}

impl From<parser::Error> for Error {
    fn from(error: parser::Error) -> Error {
        Error::Parse(error)
    }
}

impl From<evaluate::Error> for Error {
    fn from(error: evaluate::Error) -> Error {
        Error::Evaluate(error)
    }
}

fn report_errors(err: Error) {
    match err {
        Error::Read(e) => {
            eprintln!("{}", e.msg);
        }
        Error::Tokenize(e) => {
            use crate::tokenize::ScanError;
            // for scan_error in e.iter() {
            //     match scan_error {
            //         ScanError::UnexpectedCharacter { line, ch } => {
            //             eprintln!("Line {line}: Unexpected character {ch:?}");
            //         }
            //         ScanError::UnterminatedString { line } => {
            //             eprintln!("Line {line}: Unterminated string");
            //         }
            //     }
            // }
        }
        Error::Parse(e) => {
            use crate::parser::Error;
            match e {
                Error::SyntaxError { line, msg } => {
                    eprintln!("Line {line}: Syntax error: {msg}");
                }
            }
        }
        Error::Evaluate(e) => {
            use crate::evaluate::Error::*;
            match e {
                ZeroDivision => {
                    eprintln!("Division by zero");
                }
                UnsupportedBinOp(left, op, right) => {
                    eprintln!("Unsupported operation: {left:?} {op} {right:?}");
                }
                UnsupportedUnaryOp(op, value) => {
                    eprintln!("Unsupported operation: {op}{value:?}");
                }
            }
        }
    }
}

fn run_prompt() {
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let mut interpreter = evaluate::Interpreter::new();
    loop {
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let source = reader::Source {contents: buffer};
        match run_interp(&mut interpreter, source) {
            Ok(_) => {},
            Err(e) => {
                report_errors(e);
            }
        }
    }
}

fn run(source: reader::Source) -> Result<(), Error> {
    let mut interpreter = evaluate::Interpreter::new();
    run_interp(&mut interpreter, source)
}

fn run_interp(interp: &mut evaluate::Interpreter, source: reader::Source) -> Result<(), Error> {
    let tokens = tokenize::tokenize(source)?;
    println!("tokens: {:?}", tokens);
    let ast = parser::parse(tokens)?;
    println!("ast: {:?}", ast);
    interp.evaluate(ast)?;
    Ok(())
}

fn run_file(filename: &str) -> Result<(), Error> {
    let source = reader::read_source(filename)?;
    run(source)
}

fn main() {
    println!("Hello, Lox!");
    ast::main();
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        run_prompt();
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => {
                println!("It worked")
            }
            Err(e) => {
                report_errors(e);
            }
        }
    } else {
        eprintln!("Usage: lox [filename]");
    }
}
