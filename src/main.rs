mod reader;
mod tokenize;
mod parser;
mod evaluate;

fn main() {
    println!("Hello, Lox!");
    reader::read_source();
    tokenize::tokenize();
    parser::parse();
    evaluate::evaluate();
}
