extern crate ezmodpacker;
use ezmodpacker::{file, parser, operations};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = file::File::read(args[1].clone());

    let ast = parser::parse(file.content);
    let ast = ast.unwrap();

    let mut interpreter = operations::Interpreter::new();
    interpreter.eval(ast);
}