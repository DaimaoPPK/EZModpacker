#![allow(dead_code)]

pub mod parser;
pub mod file;
pub mod ast;
pub mod operations;

#[macro_use]
extern crate pest_derive;