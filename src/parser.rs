/*
* ISC License
*
* Copyright <2021> <Phone Pyae Kyaw>
*
* Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

pub extern crate pest;

use pest::Parser;
use crate::ast::{Statement, Node};
use crate::file::{Folder};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EZParser;

/// Parse String into Parse Tree using grammar.pest with Pest and then build AST from Parse Tree
/// # Examples
/// ```
/// use std::env;
/// use ezmodpacker::parser;
/// for arg in env::args(){
///     let file = fs::read_to_string(arg)
///         .expect("Something went wrong while reading File");
///     parse(parser::parse(file));
/// }
/// ```
pub fn parse(source: String) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>>{
    let mut ast = vec![];
    let pairs = EZParser::parse(Rule::Program, source.as_str()).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs{
        ast.push(build_ast(pair));
    }
    Ok(ast)
}

/// Convert pair from Parse Tree into Node
fn build_ast(pair: pest::iterators::Pair<Rule>) -> Node{
    match pair.as_rule(){
        Rule::Statement => Node::Statement(Statement::from(pair)),
        Rule::Section => Node::Section(Folder::new(String::from(pair.as_str()).replace("[", "").replace("]", ""))),
        _ => {println!("Something went wrong while building Abstract Syntax Tree\nContent: {}", pair.as_str()); Node::Error}
    }
}