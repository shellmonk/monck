use std::fs;

mod cli;
mod ast;
mod lexer;
mod parser;
mod err;

fn main() {
    let content = fs::read_to_string("examples/v3.monck").unwrap();
    //let parser = Parser::new(&content);

    println!("{content}");

    //for stmt in parser.ast {
    //    println!("statement: {stmt}");
    //}
}
