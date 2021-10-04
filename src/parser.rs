pub extern crate pest;

use pest::Parser;
use crate::ast::{Statement, Node};
use crate::file::{Folder};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EZParser;

pub fn parse(source: String) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>>{
    let mut ast = vec![];
    let pairs = EZParser::parse(Rule::Program, source.as_str()).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs{
        ast.push(build_ast(pair));
    }
    Ok(ast)
}

fn build_ast(pair: pest::iterators::Pair<Rule>) -> Node{
    match pair.as_rule(){
        Rule::Statement => Node::Statement(Statement::from(pair)),
        Rule::Section => Node::Section(Folder::new(String::from(pair.as_str()).replace("[", "").replace("]", ""))),
        _ => {println!("Something went wrong while building Abstract Syntax Tree\nContent: {}", pair.as_str()); Node::Error}
    }
}