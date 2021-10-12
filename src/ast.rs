/*
* ISC License
*
* Copyright <2021> <Phone Pyae Kyaw>
*
* Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

use crate::file::Folder;
use crate::parser::Rule;

#[derive(Debug)]
pub struct Attribute{
    pub key: String,
    pub value: String
}

#[derive(Debug)]
pub struct Statement{
    pub key: String,
    pub value: String,
    pub attributes: Vec<Attribute>
}

#[derive(Debug)]
pub enum Node{
    Statement(Statement),
    Section(Folder),
    Error
}

impl Attribute{
    pub fn new(key: String, value: String) -> Attribute{Attribute{key, value}}

    /// Build Attribute from pair
    pub fn from(pair: pest::iterators::Pair<Rule>) -> Attribute{
        let mut key = String::new();
        let mut value = String::new();
        for pair1 in pair.into_inner(){
            match pair1.as_rule(){
                Rule::AttrKey => key = String::from(pair1.as_str()),
                Rule::AttrValue => value = String::from(pair1.as_str()),
                _ => println!("Something went wrong while parsing Attribute\nContent: {}", pair1.as_str())
            }
        }
        Attribute::new(key, value)
    }
}

impl Statement{
    pub fn new(key: String, value: String, attributes: Vec<Attribute>) -> Statement{Statement{key, value, attributes}}

    /// Build Statement from pair
    pub fn from(pair: pest::iterators::Pair<Rule>) -> Statement{
        let mut key = String::new();
        let mut value = String::new();
        let mut attributes = vec![];
        for pair1 in pair.into_inner(){
            match pair1.as_rule(){
                Rule::Key => key = String::from(pair1.as_str()),
                Rule::Value => value = String::from(pair1.as_str()),
                Rule::Attribute => {for pair2 in pair1.into_inner(){
                    attributes.push(Attribute::from(pair2));
                }},
                _ => println!("Something went wrong while parsing Statement\nContent: {}", pair1.as_str())
            }
        }
        Statement::new(key, value, attributes)
    }
}