mod reader;
mod tokenize;
mod parser;
mod evaluate;

fn main() {
    println!("Hello, Lox!");
    let source = reader::read_source("somefile.lox").unwrap();
    let tokens = tokenize::tokenize(source).unwrap();
    let ast = parser::parse(tokens).unwrap();
    let out = evaluate::evaluate(ast).unwrap();
}
