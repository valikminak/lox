mod reader;
mod tokenize;
mod parser;
mod evaluate;

fn main() {
    println!("Hello, Lox!");
    let source = reader::read_source("somefile.lox");
    let tokens = tokenize::tokenize(source);
    let ast = parser::parse(tokens);
    let out = evaluate::evaluate(ast);
}
