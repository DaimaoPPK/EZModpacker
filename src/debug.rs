use crate::parser::{Rule, pest::iterators::Pairs};

pub fn pretty_print(pairs: Pairs<Rule>){
    for pair1 in pairs{match pair1.as_rule(){
        Rule::Section => println!("Section: {}", pair1.as_str()),
        _ => {for pair2 in pair1.into_inner(){
            match pair2.as_rule(){
            Rule::Key => println!("Key: {}", pair2.as_str()),
            Rule::Value => println!("Value: {}", pair2.as_str()),
            _ => {for pair3 in pair2.into_inner(){
                for pair4 in pair3.into_inner(){
                    match pair4.as_rule(){
                        Rule::AttrKey => println!("Attrkey: {}", pair4.as_str()),
                        Rule::AttrValue => println!("Attrvalue: {}", pair4.as_str()),
                        _ => ()
                }
            }}
        }}
    }}
}}}