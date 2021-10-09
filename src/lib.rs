#![allow(dead_code)]

pub mod parser; // parse .modpackinfo file into AST
pub mod file; // file structs
pub mod ast; // structs for elements in AST
pub mod operations; // interpret AST and do operations

#[macro_use]
extern crate pest_derive;