use crate::parser::Error;

mod evaluate;
mod parser;
mod reader;
mod tokenize;

fn run() -> Result<(), Error> {
    let source = reader::read_source("somefile.lox")?;
    let tokens = tokenize::tokenize(source)?;
    let ast = parser::parse(tokens)?;
    let out = evaluate::evaluate(ast)?;
    Ok(out)
}

fn main() {
    println!("Hello, Lox!");
    match run() {
        Ok(_) => {
            println!("It worked")
        }
        Err(e) => {
            println!("Failed: {e:?}")
        }
    }
}
