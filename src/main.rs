use std::io::Write;

mod evaluate;
mod parser;
mod reader;
mod tokenize;
mod ast;

// top-level error
#[derive(Debug)]
pub enum Error {
    Reade(reader::Error),
    Tokenize(tokenize::Error),
    Parse(parser::Error),
    Evaluate(evaluate::Error),
}

impl From<reader::Error> for Error {
    fn from(error: reader::Error) -> Error {
        Error::Reade(error)
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

fn run_prompt() {
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();
    loop {
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let source = reader::Source {contents: buffer};
        match run(source) {
            Ok(_) => {},
            Err(error) => {
                println!("{error:?}")
            }
        }
    }
}

fn run(source: reader::Source) -> Result<(), Error> {
    let tokens = tokenize::tokenize(source)?;
    println!("tokens: {:?}", tokens);
    let ast = parser::parse(tokens)?;
    let _out = evaluate::evaluate(ast)?;
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
                println!("Failed: {e:?}")
            }
        }
    } else {
        eprintln!("Usage: lox [filename]");
    }
}
