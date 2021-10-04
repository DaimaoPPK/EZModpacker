#![allow(dead_code)]

mod parser;
mod file;
mod ast;
mod operations;

#[macro_use]
extern crate pest_derive;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = file::File::read(args[1].clone());

    let ast = parser::parse(file.content);
    let ast = ast.unwrap();

    let mut interpreter = operations::Interpreter::new();
    interpreter.eval(ast);
}