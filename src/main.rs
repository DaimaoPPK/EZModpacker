extern crate ezmodpacker;
use ezmodpacker::{file, parser, operations};

use std::env;

fn main() {
    for arg in env::args().skip(1){
        let file = file::File::read(arg);

        let ast = parser::parse(file.content).unwrap();

        let mut interpreter = operations::Interpreter::new();
        interpreter.eval(ast);
    }
}